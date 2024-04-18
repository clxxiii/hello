import express from "express";
import { createHandler } from "graphql-http/lib/use/express";
import { ruruHTML } from "ruru/server"
import { readFileSync } from "fs";
import { buildSchema } from "graphql";
import * as rootValue from "./endpoints"

const schema = buildSchema(readFileSync("./schema.gql").toString());

const app = express();
app.all(
  "/graphql",
  createHandler({
    schema,
    rootValue
  })
).get(
  "/", (_req, res) => {
    res.type("html");
    res.end(ruruHTML({ endpoint: "/graphql" }))
  }
).listen(5173)