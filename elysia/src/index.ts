import { Elysia } from "elysia";

const app = new Elysia()
  .ws("/ws", {
    message: (ws, msg) => {
      if (typeof msg == "string") {
        ws.send(msg + msg);
      }
    },
  })
  .get("/", () => "Hello Elysia")
  .listen(3000);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
