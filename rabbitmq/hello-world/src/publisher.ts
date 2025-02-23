import amqp from "amqplib/callback_api"

amqp.connect("amqp://localhost", (e1, connection) => {
  if (e1) throw e1;

  connection.createChannel((e1, channel) => {
    if (e1) throw e1;

    const queue = 'hello';
    const msg = 'Hello World!'

    channel.assertQueue(queue, {
      durable: false
    })
    channel.sendToQueue(queue, Buffer.from(msg));

    console.log(" [x] Sent %s", msg);
  })
  setTimeout(() => {
    connection.close();
    process.exit(0);
  }, 500)
})