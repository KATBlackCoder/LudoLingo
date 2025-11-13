/**
 * Centralized database operation helpers
 * Provides reusable patterns for DB operations with consistent error handling
 */

import type { DBOperationResult } from './project/types'
import type { TextOperationResult } from './texts/types'

/**
 * Wraps a database operation with consistent error handling
 * @param operation - The async operation to execute
 * @param errorContext - Context for error messages (e.g., 'creating project')
 * @returns Result with success flag and data or error
 */
export async function executeDBOperation<T>(
  operation: () => Promise<T>,
  errorContext: string
): Promise<DBOperationResult<T>> {
  try {
    const data = await operation()
    return {
      success: true,
      data
    }
  } catch (error) {
    console.error(`Error ${errorContext}:`, error)
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error occurred'
    }
  }
}

/**
 * Wraps a database operation with consistent error handling (TextOperationResult variant)
 * @param operation - The async operation to execute
 * @param errorContext - Context for error messages (e.g., 'creating text entry')
 * @returns Result with success flag and data or error
 */
export async function executeTextOperation<T>(
  operation: () => Promise<T>,
  errorContext: string
): Promise<TextOperationResult<T>> {
  try {
    const data = await operation()
    return {
      success: true,
      data
    }
  } catch (error) {
    console.error(`Error ${errorContext}:`, error)
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error occurred'
    }
  }
}

/**
 * Checks if a record exists in the database
 * @param checkQuery - SQL query to check existence (should return at least one row if exists)
 * @param params - Query parameters
 * @returns true if record exists, false otherwise
 */
export async function checkRecordExists(
  checkQuery: string,
  params: unknown[]
): Promise<boolean> {
  try {
    const { executeQuery } = await import('./useDatabase')
    const result = await executeQuery<{ count: number }>(checkQuery, params)
    return result.length > 0
  } catch (error) {
    console.error('Error checking record existence:', error)
    return false
  }
}

/**
 * Builds a SQL WHERE clause dynamically from filters
 * @param filters - Object with filter keys and values
 * @param filterMap - Map of filter keys to SQL column names and operators
 * @returns Object with SQL WHERE clause and parameters
 */
export interface FilterMap {
  [key: string]: {
    column: string
    operator?: '=' | 'LIKE' | 'IN' | '>=' | '<=' | '>'
    transform?: (value: unknown) => unknown
  }
}

export function buildWhereClause(
  filters: Record<string, unknown>,
  filterMap: FilterMap
): { whereClause: string; params: unknown[] } {
  const conditions: string[] = []
  const params: unknown[] = []

  for (const [key, value] of Object.entries(filters)) {
    if (value === undefined || value === null) continue

    const config = filterMap[key]
    if (!config) continue

    const { column, operator = '=', transform } = config
    const transformedValue = transform ? transform(value) : value

    if (operator === 'LIKE' && typeof transformedValue === 'string') {
      conditions.push(`${column} LIKE ?`)
      params.push(`%${transformedValue}%`)
    } else if (operator === 'IN' && Array.isArray(transformedValue)) {
      const placeholders = transformedValue.map(() => '?').join(',')
      conditions.push(`${column} IN (${placeholders})`)
      params.push(...transformedValue)
    } else {
      conditions.push(`${column} ${operator} ?`)
      params.push(transformedValue)
    }
  }

  const whereClause = conditions.length > 0
    ? `WHERE ${conditions.join(' AND ')}`
    : ''

  return { whereClause, params }
}

