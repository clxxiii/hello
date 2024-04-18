import { sqliteTable, text } from "drizzle-orm/sqlite-core";
import { randomUUID } from "crypto";
import { relations, sql } from "drizzle-orm";

export const post = sqliteTable('Post', {
  id: text('id').$default(randomUUID).primaryKey(),
  date_posted: text('date_posted').notNull().default(sql`(CURRENT_TIMESTAMP)`),
  content: text('content').notNull()
})

export const postRelations = relations(post, ({ one }) => ({
  author: one(user, { fields: [post.id], references: [user.username] })
}))

export const user = sqliteTable('User', {
  username: text('username').notNull().primaryKey(),
})

export const userRelations = relations(user, ({ many }) => ({
  posts: many(post)
}))

