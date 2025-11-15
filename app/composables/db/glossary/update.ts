// Glossary Update Operations
// Handles updating glossary entries

import { executeStatement, executeQuery } from '../useDatabase'
import { executeDBOperation } from '../useDBOperation'
import type {
  UpdateGlossaryEntry,
  GlossaryEntry,
  GlossaryOperationResult
} from './types'

/**
 * Update a single glossary entry
 */
export async function updateGlossaryEntry(
  entryId: number,
  updates: Partial<UpdateGlossaryEntry>
): Promise<GlossaryOperationResult<GlossaryEntry>> {
  return executeDBOperation(async () => {
    const updateFields: string[] = []
    const params: unknown[] = []

    // Build dynamic update query
    if (updates.source_term !== undefined) {
      updateFields.push('source_term = ?')
      params.push(updates.source_term)
    }

    if (updates.translated_term !== undefined) {
      updateFields.push('translated_term = ?')
      params.push(updates.translated_term)
    }

    if (updates.source_language !== undefined) {
      updateFields.push('source_language = ?')
      params.push(updates.source_language)
    }

    if (updates.target_language !== undefined) {
      updateFields.push('target_language = ?')
      params.push(updates.target_language)
    }

    if (updates.category !== undefined) {
      updateFields.push('category = ?')
      params.push(updates.category)
    }

    if (updates.project_id !== undefined) {
      updateFields.push('project_id = ?')
      params.push(updates.project_id ?? null)
    }

    if (updateFields.length === 0) {
      throw new Error('No fields to update')
    }

    // Add updated_at timestamp
    updateFields.push('updated_at = CURRENT_TIMESTAMP')

    // Add WHERE condition
    params.push(entryId)

    const query = `UPDATE glossary_entries 
                   SET ${updateFields.join(', ')} 
                   WHERE id = ?`

    const result = await executeStatement(query, params)

    if (!result) {
      throw new Error('Failed to update glossary entry')
    }

    // Retrieve the updated entry
    const updatedResult = await executeQuery<GlossaryEntry>(
      'SELECT * FROM glossary_entries WHERE id = ?',
      [entryId]
    )

    if (updatedResult.length === 0) {
      throw new Error('Glossary entry not found after update')
    }

    return updatedResult[0]
  }, 'updating glossary entry')
}

/**
 * Bulk update glossary entries
 */
export async function bulkUpdateGlossaryEntries(
  updates: Array<{ id: number; updates: Partial<UpdateGlossaryEntry> }>
): Promise<GlossaryOperationResult<{ updated_count: number; errors: string[] }>> {
  return executeDBOperation(async () => {
    const errors: string[] = []
    let updatedCount = 0

    for (const { id, updates: entryUpdates } of updates) {
      const result = await updateGlossaryEntry(id, entryUpdates)
      if (result.success) {
        updatedCount++
      } else {
        errors.push(`Failed to update entry ${id}: ${result.error || 'Unknown error'}`)
      }
    }

    return {
      updated_count: updatedCount,
      errors
    }
  }, 'bulk updating glossary entries')
}

