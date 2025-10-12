/**
 * This is a basic example of how to connect to a RabbitMQ instance and how to send a simple
 * message
 */

use futures_lite::StreamExt;
use lapin::{
    BasicProperties, Connection, ConnectionProperties, Result,
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    publisher_confirm::Confirmation,
    types::FieldTable,
};

#[tokio::main]
async fn main() -> Result<()> {
    let addr: String = "amqp://user:password@localhost:5672".into();
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;

    let channel = conn.create_channel().await?;

    let queue = channel
        .queue_declare(
            "hello",
            QueueDeclareOptions {
                exclusive: true,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;

    let mut consumer = channel
        .basic_consume(
            "hello",
            "my_consumer",
            BasicConsumeOptions {
                exclusive: true,
                ..BasicConsumeOptions::default()
            },
            FieldTable::default(),
        )
        .await?;

    tokio::spawn(async move {
        while let Some(delivery) = consumer.next().await {
            let delivery = delivery.expect("error in consumer");
            delivery.ack(BasicAckOptions::default()).await.expect("ack");
        }
    });

    loop {
        let payload = b"Hello World!";
        let confirm = channel
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await?
            .await?;
        assert_eq!(confirm, Confirmation::NotRequested);
    }
}
