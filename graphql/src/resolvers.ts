import { PrismaClient, type Like, type Post, type User } from "@prisma/client";

// Prisma is the ORM I am using to simplify the SQL queries.
// Could be replaced with raw SQL, but input would still need
// to be sanitized to prevent SQL injections.
const prisma = new PrismaClient();

export default {
  /**
   * QUERIES:
   * Queries are reads on your API. Each function below
   * is treated as a "starter", for where to enter the graph.
   */
  Query: {
    hello: () => "Hi!",

    greeting: (_: undefined, params: ({ name: string })) => ("Hello, " + params.name),

    allPosts: () => prisma.post.findMany(),

    getUserByName: (_: undefined, params: ({ username: string })) => {
      return prisma.user.findUnique({
        where: {
          "username": (params.username ?? "")
        }
      })
    }
  },

  /**
   * Once you're in the graph, we need to specify how to get from
   * one table to the other.
   */
  User: {
    posts: (user: User) => (prisma.post.findMany({ where: { author_id: user.id } })),
    likes: (user: User) => (prisma.post.findMany({ where: { likes: { some: { user_id: user.id } } } }))
  },
  Post: {
    author: (post: Post) => prisma.user.findUnique({ where: { id: post.author_id } }),
    likes: (post: Post) => prisma.user.findMany({ where: { likes: { some: { post_id: post.id } } } })
  },
  Like: {
    author: (like: Like) => prisma.user.findUnique({ where: { id: like.user_id } }),
    post: (like: Like) => prisma.post.findUnique({ where: { id: like.post_id } })
  },

  /**
   * Here is where the mutations are specified.
   */
  Mutation: {
    makePost: (_: undefined, params: { content: string, username: string }) => (
      prisma.post.create({
        data: {
          author: {
            connectOrCreate: {
              where: { username: params.username },
              create: { username: params.username }
            }
          },
          content: params.content
        }
      }))
  }
}