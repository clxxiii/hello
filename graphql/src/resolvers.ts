import { PrismaClient, type Like, type Post, type User } from "@prisma/client";

const prisma = new PrismaClient();

export default {
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
  }
}