// Text Update Operations
// Handles updating translated texts and text metadata

import { executeStatement, executeQuery } from '../useDatabase'
import type { TextEntry } from '~/types/scanning-commands'
import type {
  UpdateTextEntry,
  TextOperationResult,
  BulkTextOperationResult
} from './types'

// Update a single text entry
export async function updateTextEntry(
  textId: number,
  updates: Partial<UpdateTextEntry>
): Promise<TextOperationResult> {
  try {
    const updateFields: string[] = []
    const params: any[] = []

    // Build dynamic update query
    if (updates.translated_text !== undefined) {
      updateFields.push('translated_text = ?')
      params.push(updates.translated_text)
    }

    if (updates.context !== undefined) {
      updateFields.push('context = ?')
      params.push(updates.context)
    }

    if (updates.text_type !== undefined) {
      updateFields.push('text_type = ?')
      params.push(updates.text_type)
    }

    if (updates.status !== undefined) {
      updateFields.push('status = ?')
      params.push(updates.status)
    }

    if (updates.translation_source !== undefined) {
      updateFields.push('translation_source = ?')
      params.push(updates.translation_source)
    }

    if (updateFields.length === 0) {
      return { success: false, error: 'No fields to update' }
    }

    // Add updated_at timestamp
    updateFields.push('updated_at = CURRENT_TIMESTAMP')

    // Add WHERE condition
    const query = `UPDATE translation_entries SET ${updateFields.join(', ')} WHERE id = ?`
    params.push(textId)

    const result = await executeStatement(query, params)

    if (!result) {
      return { success: false, error: 'Failed to update text entry' }
    }

    return { success: true }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

// Update text translation and status
export async function updateTextTranslation(
  textId: number,
  translatedText: string,
  translationSource: 'manual' | 'ollama' | 'glossary' = 'manual'
): Promise<TextOperationResult> {
  return updateTextEntry(textId, {
    translated_text: translatedText,
    status: 'translated',
    translation_source: translationSource
  })
}

// Update text status only
export async function updateTextStatus(
  textId: number,
  status: 'extracted' | 'translated' | 'reviewed' | 'finalized'
): Promise<TextOperationResult> {
  return updateTextEntry(textId, { status })
}

// Bulk update text translations
export async function bulkUpdateTextTranslations(
  updates: Array<{ textId: number; translatedText: string; translationSource?: 'manual' | 'ollama' | 'glossary' }>
): Promise<BulkTextOperationResult> {
  const errors: string[] = []
  let updatedCount = 0

  try {
    // Process updates in batches
    const batchSize = 25
    for (let i = 0; i < updates.length; i += batchSize) {
      const batch = updates.slice(i, i + batchSize)

      for (const update of batch) {
        const result = await updateTextTranslation(
          update.textId,
          update.translatedText,
          update.translationSource || 'manual'
        )

        if (result.success) {
          updatedCount++
        } else {
          errors.push(`Failed to update text ${update.textId}: ${result.error}`)
        }
      }
    }

    return {
      success: errors.length === 0,
      inserted_count: updatedCount,
      errors
    }
  } catch (error) {
    return {
      success: false,
      inserted_count: updatedCount,
      errors: [...errors, `Bulk update failed: ${error instanceof Error ? error.message : 'Unknown error'}`]
    }
  }
}

// Bulk update text statuses
export async function bulkUpdateTextStatuses(
  updates: Array<{ textId: number; status: 'extracted' | 'translated' | 'reviewed' | 'finalized' }>
): Promise<BulkTextOperationResult> {
  const errors: string[] = []
  let updatedCount = 0

  try {
    // Process updates in batches
    const batchSize = 50
    for (let i = 0; i < updates.length; i += batchSize) {
      const batch = updates.slice(i, i + batchSize)

      for (const update of batch) {
        const result = await updateTextStatus(update.textId, update.status)

        if (result.success) {
          updatedCount++
        } else {
          errors.push(`Failed to update text ${update.textId}: ${result.error}`)
        }
      }
    }

    return {
      success: errors.length === 0,
      inserted_count: updatedCount,
      errors
    }
  } catch (error) {
    return {
      success: false,
      inserted_count: updatedCount,
      errors: [...errors, `Bulk status update failed: ${error instanceof Error ? error.message : 'Unknown error'}`]
    }
  }
}

// Update game file status
export async function updateGameFileStatus(
  gameFileId: number,
  status: 'pending' | 'scanning' | 'completed' | 'error'
): Promise<TextOperationResult> {
  try {
    const result = await executeStatement(
      'UPDATE game_files SET scan_status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?',
      [status, gameFileId]
    )

    if (!result) {
      return { success: false, error: 'Failed to update game file status' }
    }

    return { success: true }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}
