// Glossary Delete Operations
// Handles deleting glossary entries

import { executeStatement } from '../useDatabase'
import { executeDBOperation } from '../useDBOperation'
import type { GlossaryOperationResult } from './types'

/**
 * Delete a single glossary entry
 */
export async function deleteGlossaryEntry(
  entryId: number
): Promise<GlossaryOperationResult> {
  return executeDBOperation(async () => {
    const result = await executeStatement(
      'DELETE FROM glossary_entries WHERE id = ?',
      [entryId]
    )

    if (!result) {
      throw new Error('Failed to delete glossary entry')
    }
  }, 'deleting glossary entry')
}

/**
 * Delete multiple glossary entries by IDs
 */
export async function bulkDeleteGlossaryEntries(
  entryIds: number[]
): Promise<GlossaryOperationResult<{ deleted_count: number; errors: string[] }>> {
  return executeDBOperation(async () => {
    const errors: string[] = []
    let deletedCount = 0

    for (const id of entryIds) {
      const result = await deleteGlossaryEntry(id)
      if (result.success) {
        deletedCount++
      } else {
        errors.push(`Failed to delete entry ${id}: ${result.error || 'Unknown error'}`)
      }
    }

    return {
      deleted_count: deletedCount,
      errors
    }
  }, 'bulk deleting glossary entries')
}

/**
 * Delete all glossary entries for a specific language pair
 */
export async function deleteGlossaryEntriesForLanguages(
  source_language: string,
  target_language: string
): Promise<GlossaryOperationResult<{ deleted_count: number }>> {
  return executeDBOperation(async () => {
    const result = await executeStatement(
      'DELETE FROM glossary_entries WHERE source_language = ? AND target_language = ?',
      [source_language, target_language]
    )

    if (!result) {
      throw new Error('Failed to delete glossary entries for language pair')
    }

    // Note: SQLite doesn't return affected rows count directly
    // We would need to query before deletion to get the count
    return {
      deleted_count: 0  // Could be enhanced to return actual count
    }
  }, 'deleting glossary entries for languages')
}

/**
 * Delete all glossary entries for a specific category
 */
export async function deleteGlossaryEntriesByCategory(
  category: string
): Promise<GlossaryOperationResult> {
  return executeDBOperation(async () => {
    const result = await executeStatement(
      'DELETE FROM glossary_entries WHERE category = ?',
      [category]
    )

    if (!result) {
      throw new Error('Failed to delete glossary entries by category')
    }
  }, 'deleting glossary entries by category')
}

