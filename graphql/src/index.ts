/**
 * This file loads the web frontend, which allows you to interact with the repo's
 * GraphQL API.
 */

import express from "express";
import { ruruHTML } from "ruru/server"
import { readFileSync } from "fs";
import { graphqlHTTP } from "express-graphql";
import { makeExecutableSchema } from "@graphql-tools/schema";
import resolvers from "./resolvers"

const typeDefs = (readFileSync("./src/schema.gql").toString()); // Pull Schema from schema file

const schema = makeExecutableSchema({
  typeDefs,
  resolvers // Pull Functions from "resolvers" file
})


const app = express();
app.all(
  "/graphql", // Create a GraphQL endpoint at /graphql
  graphqlHTTP({
    schema,
  })
).get(
  "/", (_req, res) => { // Create the frontend page.
    res.type("html");
    res.end(ruruHTML({ endpoint: "/graphql" }))
  }
).listen(5173)