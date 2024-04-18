import express from "express";
// import { createHandler } from "graphql-http/lib/use/express";
import { ruruHTML } from "ruru/server"
import { readFileSync } from "fs";
import { graphqlHTTP } from "express-graphql";
// import { buildSchema } from "graphql";
import { makeExecutableSchema } from "@graphql-tools/schema";
import resolvers from "./resolvers"

const typeDefs = (readFileSync("./src/schema.gql").toString());

const schema = makeExecutableSchema({
  typeDefs,
  resolvers
})

const app = express();
app.all(
  "/graphql",
  graphqlHTTP({
    schema,
  })
).get(
  "/", (_req, res) => {
    res.type("html");
    res.end(ruruHTML({ endpoint: "/graphql" }))
  }
).listen(5173)