import amqp from "amqplib/callback_api"

amqp.connect('amqp://localhost', (e0, connection) => {
  if (e0) throw e0;

  connection.createChannel((e1, channel) => {
    if (e1) throw e1;

    const queue = 'hello';

    channel.assertQueue(queue, {
      durable: false
    })

    console.log(" [*] Waiting for messages in %s. To exit press CTRL+C", queue);

    channel.consume(queue, (msg) => {
      console.log(" [x] Recieved %s", msg?.content.toString());
    }, {
      noAck: true
    })
  })
})