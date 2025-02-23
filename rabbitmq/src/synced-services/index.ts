import amqp from "amqplib"

try {
  const connection = await amqp.connect("amqp://localhost")

  const channel = await connection.createChannel()
  const exchange = 'logs';

  channel.assertExchange(exchange, 'fanout', { durable: false })

  // Create ephemeral queue
  const q = await channel.assertQueue("", { exclusive: true });
  console.log("[x] Created channel %s", q.queue);

  // Bind queue to fanout exchange
  channel.bindQueue(q.queue, exchange, '')

  channel.consume(q.queue, (msg) => {
    if (!msg) return;
    console.log("[x] %s", msg.content.toString())
  })

  setInterval(() => {
    const msg = `Hello from process ${process.pid}!`;
    channel.publish(exchange, '', Buffer.from(msg))
  }, 1000)
} catch (e) {
  console.error(e)
}
