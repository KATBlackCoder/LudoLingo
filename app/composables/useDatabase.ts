// Database utilities composable
// Provides access to SQLite database via tauri-plugin-sql

import Database, { type QueryResult } from '@tauri-apps/plugin-sql'

let db: Database | null = null

/**
 * Get or initialize database connection
 */
export async function useDatabase(): Promise<Database> {
  if (!db) {
    db = await Database.load('sqlite:ludolingo.db')
  }
  return db
}

/**
 * Execute a SQL SELECT query
 */
export async function executeQuery<T extends Record<string, unknown> = Record<string, unknown>>(
  sql: string,
  params: unknown[] = []
): Promise<T[]> {
  const database = await useDatabase()
  const result = await database.select(sql, params)
  return result as T[]
}

/**
 * Execute a SQL statement (INSERT, UPDATE, DELETE)
 */
export async function executeStatement(
  sql: string,
  params: unknown[] = []
): Promise<QueryResult> {
  const database = await useDatabase()
  return database.execute(sql, params)
}

