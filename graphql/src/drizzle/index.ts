import { drizzle } from 'drizzle-orm/bun-sqlite'
import { Database } from "bun:sqlite"
import { env } from 'bun'

const client = new Database(env.DATABASE_URL)

export const db = drizzle(client);