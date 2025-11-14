// Glossary Read Operations
// Handles retrieving glossary entries from database

import { executeQuery } from '../useDatabase'
import { executeDBOperation } from '../useDBOperation'
import type {
  GlossaryEntry,
  GlossaryFilters,
  GlossaryQueryResult,
  GlossaryOperationResult
} from './types'

/**
 * Get all glossary entries for a specific language pair
 * This is the main function used by the backend to enrich translation prompts
 */
export async function getGlossaryTermsForLanguages(
  source_language: string,
  target_language: string
): Promise<GlossaryOperationResult<GlossaryEntry[]>> {
  return executeDBOperation(async () => {
    const entries = await executeQuery<GlossaryEntry>(
      `SELECT * FROM glossary_entries 
       WHERE source_language = ? AND target_language = ?
       ORDER BY category, source_term`,
      [source_language, target_language]
    )

    return entries || []
  }, 'getting glossary terms for languages')
}

/**
 * Get all glossary entries with optional filters
 */
export async function getGlossaryEntries(
  filters?: GlossaryFilters
): Promise<GlossaryOperationResult<GlossaryQueryResult>> {
  return executeDBOperation(async () => {
    let query = 'SELECT * FROM glossary_entries'
    const params: unknown[] = []
    const conditions: string[] = []

    // Build WHERE clause from filters
    if (filters?.category && filters.category.length > 0) {
      const placeholders = filters.category.map(() => '?').join(',')
      conditions.push(`category IN (${placeholders})`)
      params.push(...filters.category)
    }

    if (filters?.source_language) {
      conditions.push('source_language = ?')
      params.push(filters.source_language)
    }

    if (filters?.target_language) {
      conditions.push('target_language = ?')
      params.push(filters.target_language)
    }

    if (filters?.search) {
      conditions.push('(source_term LIKE ? OR translated_term LIKE ?)')
      const searchTerm = `%${filters.search}%`
      params.push(searchTerm, searchTerm)
    }

    if (conditions.length > 0) {
      query += ` WHERE ${conditions.join(' AND ')}`
    }

    // Get total count
    const countQuery = query.replace('SELECT *', 'SELECT COUNT(*) as count')
    const countResult = await executeQuery<{ count: number }>(countQuery, params)
    const total_count = countResult[0]?.count || 0

    // Add ordering and pagination
    query += ' ORDER BY category, source_term'

    if (filters?.limit) {
      query += ` LIMIT ?`
      params.push(filters.limit)

      if (filters?.offset) {
        query += ` OFFSET ?`
        params.push(filters.offset)
      }
    }

    const entries = await executeQuery<GlossaryEntry>(query, params)

    return {
      entries: entries || [],
      total_count
    }
  }, 'getting glossary entries')
}

/**
 * Get a single glossary entry by ID
 */
export async function getGlossaryEntry(
  id: number
): Promise<GlossaryOperationResult<GlossaryEntry>> {
  return executeDBOperation(async () => {
    const entries = await executeQuery<GlossaryEntry>(
      'SELECT * FROM glossary_entries WHERE id = ?',
      [id]
    )

    if (entries.length === 0) {
      throw new Error(`Glossary entry with id ${id} not found`)
    }

    return entries[0]
  }, 'getting glossary entry')
}

/**
 * Search for glossary entries by source term (exact match or partial)
 */
export async function searchGlossaryByTerm(
  source_term: string,
  source_language?: string,
  target_language?: string,
  exactMatch: boolean = false
): Promise<GlossaryOperationResult<GlossaryEntry[]>> {
  return executeDBOperation(async () => {
    let query = 'SELECT * FROM glossary_entries WHERE '
    const params: unknown[] = []

    if (exactMatch) {
      query += 'source_term = ?'
      params.push(source_term)
    } else {
      query += 'source_term LIKE ?'
      params.push(`%${source_term}%`)
    }

    if (source_language) {
      query += ' AND source_language = ?'
      params.push(source_language)
    }

    if (target_language) {
      query += ' AND target_language = ?'
      params.push(target_language)
    }

    query += ' ORDER BY category, source_term'

    const entries = await executeQuery<GlossaryEntry>(query, params)

    return entries || []
  }, 'searching glossary by term')
}

/**
 * Get glossary statistics
 */
export async function getGlossaryStats(): Promise<GlossaryOperationResult<{
  total_entries: number
  categories_breakdown: Record<string, number>
  language_pairs: Array<{ source_language: string; target_language: string; count: number }>
}>> {
  return executeDBOperation(async () => {
    // Total entries
    const totalResult = await executeQuery<{ count: number }>(
      'SELECT COUNT(*) as count FROM glossary_entries'
    )
    const total_entries = totalResult[0]?.count || 0

    // Categories breakdown
    const categoriesResult = await executeQuery<{ category: string; count: number }>(
      'SELECT category, COUNT(*) as count FROM glossary_entries GROUP BY category'
    )
    const categories_breakdown: Record<string, number> = {}
    for (const row of categoriesResult) {
      categories_breakdown[row.category] = row.count
    }

    // Language pairs breakdown
    const pairsResult = await executeQuery<{
      source_language: string
      target_language: string
      count: number
    }>(
      `SELECT source_language, target_language, COUNT(*) as count 
       FROM glossary_entries 
       GROUP BY source_language, target_language`
    )

    return {
      total_entries,
      categories_breakdown,
      language_pairs: pairsResult || []
    }
  }, 'getting glossary statistics')
}

