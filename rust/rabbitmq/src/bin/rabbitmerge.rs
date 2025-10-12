/**
 * This is my implementation of merge sort that utilized RabbitMQ to distribute tasks
 * among as many machines as you'd like. 
 */ 
use std::io::Write;

use futures_lite::StreamExt;
use lapin::{
    options::{BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions, QueueDeleteOptions}, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties, Result
};

#[tokio::main]
async fn main() -> Result<()> {
    let addr: String = std::env::var("RABBITMQ_URL").unwrap_or("amqp://user:password@localhost:5672".into());
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;

    let channel = conn.create_channel().await?;

    channel
        .queue_declare(
            "merge_queue",
            QueueDeclareOptions {
                auto_delete: true,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;

    let mut consumer = channel
        .basic_consume(
            "merge_queue",
            "merge_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // Spawn Listener for merges
    println!("Started listener for merges");
    tokio::spawn(async move {
        loop {
            if let Some(delivery) = consumer.next().await {
                let delivery = delivery.expect("Delivery failed");
                if let Some(x) = delivery.properties.reply_to() {
                    if let Some(y) = delivery.properties.correlation_id() {
                        let reply_key = String::from(x.as_str());
                        let correlation_id = String::from(y.as_str());
                        let channel = conn.create_channel().await.unwrap();
                        let unsorted = bytes_to_vec(&delivery.data);

                        // Complete task on new thread
                        tokio::spawn(async move {
                            mergesort(&unsorted, channel, &reply_key, &correlation_id)
                                .await
                                .unwrap();
                        });
                    }
                }
            }
        }
    });

    let response_queue = channel
        .queue_declare(
            "",
            QueueDeclareOptions {
                exclusive: true,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;

    let mut response_consumer = channel
        .basic_consume(
            response_queue.name().as_str(),
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
    loop {
        print!("Enter several space-separated numbers on one line: ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // This is a "pretty" solution but it is unbelievably slow
        let numbers: Vec<i32> = input
            .split(" ")
            .map(|s| s.parse::<i32>().map(|i| Some(i)).unwrap_or(None))
            .filter(|o| o.is_some())
            .map(|o| o.expect("Empty elements have already been filtered out"))
            .collect();

        println!("[+] Unsorted: {numbers:?}");

        channel
            .basic_publish(
                "",
                "merge_queue",
                BasicPublishOptions::default(),
                &vec_to_bytes(&numbers),
                BasicProperties::default()
                    .with_reply_to(response_queue.name().as_str().into())
                    .with_correlation_id("numbers".into()),
            )
            .await?;

        if let Some(x) = response_consumer.next().await {
            let delivery = x.unwrap();
            let response = bytes_to_vec(&delivery.data);
            println!("[+] Complete: {response:?}");
        }
    }
}

fn vec_to_bytes(vec: &[i32]) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for i in vec {
        for b in i.to_be_bytes() {
            bytes.push(b);
        }
    }
    bytes
}

fn bytes_to_vec(vec: &[u8]) -> Vec<i32> {
    let mut vector = Vec::new();
    for i in vec.windows(4).step_by(4) {
        vector.push(i32::from_be_bytes([i[0], i[1], i[2], i[3]]));
    }
    vector
}

async fn mergesort(
    vector: &[i32],
    channel: Channel,
    reply_to: &str,
    correlation_id: &str,
) -> Result<()> {
    println!("[.] Recieved request to sort {vector:?}");

    // Base Case
    if vector.len() <= 1 {
        println!("[.] {vector:?} is complete");
        channel
            .basic_publish(
                "",
                reply_to,
                BasicPublishOptions::default(),
                &vec_to_bytes(vector),
                BasicProperties::default().with_correlation_id(correlation_id.into()),
            )
            .await?;
        return Ok(());
    }

    // Recursively run on array halves
    let midpoint: usize = vector.len() / 2;
    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();
    vector[..midpoint].clone_into(&mut arr1);
    vector[midpoint..].clone_into(&mut arr2);

    println!("[.] Requesting sort for {arr1:?} and {arr2:?}");
    let response_queue = channel
        .queue_declare(
            "",
            QueueDeclareOptions {
                exclusive: true,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;

    let mut consumer = channel
        .basic_consume(
            response_queue.name().as_str(),
            "merge_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // Request merges
    let id1 = "arr1";
    let id2 = "arr2";
    channel
        .basic_publish(
            "",
            "merge_queue",
            BasicPublishOptions::default(),
            &vec_to_bytes(&arr1),
            BasicProperties::default()
                .with_reply_to(response_queue.name().as_str().into())
                .with_correlation_id(id1.into()),
        )
        .await?;
    channel
        .basic_publish(
            "",
            "merge_queue",
            BasicPublishOptions::default(),
            &vec_to_bytes(&arr2),
            BasicProperties::default()
                .with_reply_to(response_queue.name().as_str().into())
                .with_correlation_id(id2.into()),
        )
        .await?;

    // Wait for responses
    let mut sorted1: Option<Vec<i32>> = None;
    let mut sorted2: Option<Vec<i32>> = None;
    for _ in 0..2 {
        if let Some(delivery) = consumer.next().await {
            let delivery = delivery?;
            if let Some(id) = delivery.properties.correlation_id() {
                if id.as_str() == id1 {
                    sorted1 = Some(bytes_to_vec(&delivery.data));
                    println!("[.] Recieved sorted array {sorted1:?}");
                } else if id.as_str() == id2 {
                    sorted2 = Some(bytes_to_vec(&delivery.data));
                    println!("[.] Recieved sorted array {sorted2:?}");
                }
            }
        }
    }

    let sorted1 = match sorted1 {
        Some(x) => x,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to recieve arrays properly",
            )
            .into());
        }
    };

    let sorted2 = match sorted2 {
        Some(x) => x,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to recieve arrays properly",
            )
            .into());
        }
    };

    // Actually do merge sort
    let result_len = arr1.len() + arr2.len();
    let mut result = Vec::new();
    let mut arr1pos = 0;
    let mut arr2pos = 0;
    for _i in 0..result_len {
        let num1 = sorted1.get(arr1pos).unwrap_or(&std::i32::MAX);
        let num2 = sorted2.get(arr2pos).unwrap_or(&std::i32::MAX);

        if *num1 < *num2 {
            result.push(*num1);
            arr1pos += 1;
        } else {
            result.push(*num2);
            arr2pos += 1;
        };
    }

    println!("[.] {result:?} is complete");
    channel
        .basic_publish(
            "",
            reply_to,
            BasicPublishOptions::default(),
            &vec_to_bytes(&result),
            BasicProperties::default().with_correlation_id(correlation_id.into()),
        )
        .await?;
    channel.queue_delete(response_queue.name().as_str(), QueueDeleteOptions::default()).await?;
    channel.close(200, "Safely Closed").await.unwrap();

    Ok(())
}
