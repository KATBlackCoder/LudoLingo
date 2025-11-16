// Text Delete Operations
// Handles deleting text entries and game files

import { executeStatement, executeQuery } from '../useDatabase'
import type { TextOperationResult } from './types'

// Delete a single text entry
export async function deleteTextEntry(textId: number): Promise<TextOperationResult> {
  const { executeTextOperation } = await import('../useDBOperation')
  
  return executeTextOperation(async () => {
    const result = await executeStatement(
      'DELETE FROM translation_entries WHERE id = ?',
      [textId]
    )

    if (!result) {
      throw new Error('Failed to delete text entry')
    }
  }, 'deleting text entry')
}

// Delete all text entries for a project
export async function deleteProjectTexts(projectId: number): Promise<TextOperationResult> {
  try {
    const result = await executeStatement(
      'DELETE FROM translation_entries WHERE project_id = ?',
      [projectId]
    )

    if (!result) {
      return { success: false, error: 'Failed to delete project texts' }
    }

    return { success: true }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

// Delete all text entries for a specific game file
export async function deleteGameFileTexts(gameFileId: number): Promise<TextOperationResult> {
  try {
    const result = await executeStatement(
      'DELETE FROM translation_entries WHERE game_file_id = ?',
      [gameFileId]
    )

    if (!result) {
      return { success: false, error: 'Failed to delete game file texts' }
    }

    return { success: true }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

// Delete a game file record (and its associated texts via CASCADE)
export async function deleteGameFile(gameFileId: number): Promise<TextOperationResult> {
  try {
    // First delete associated texts (though CASCADE should handle this)
    await deleteGameFileTexts(gameFileId)

    // Then delete the game file record
    const result = await executeStatement(
      'DELETE FROM game_files WHERE id = ?',
      [gameFileId]
    )

    if (!result) {
      return { success: false, error: 'Failed to delete game file' }
    }

    return { success: true }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

// Delete all game files for a project (and their texts)
export async function deleteProjectGameFiles(projectId: number): Promise<TextOperationResult> {
  try {
    // Get all game files for the project
    const filesResult = await executeQuery<{ id: number }>(
      'SELECT id FROM game_files WHERE project_id = ?',
      [projectId]
    )

    if (!filesResult) {
      return { success: false, error: 'Failed to get project files' }
    }

    // Delete each game file and its texts
    const fileIds = filesResult.map((f: { id: number }) => f.id)
    for (const fileId of fileIds) {
      const deleteResult = await deleteGameFile(fileId)
      if (!deleteResult.success) {
        return { success: false, error: `Failed to delete game file ${fileId}: ${deleteResult.error}` }
      }
    }

    return { success: true }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

// Clean up orphaned records (useful for maintenance)
export async function cleanupOrphanedTexts(): Promise<TextOperationResult<{ deleted_count: number }>> {
  try {
    // Find texts with invalid project references
    const orphanedResult = await executeQuery<{ count: number }>(
      'SELECT COUNT(*) as count FROM translation_entries te LEFT JOIN projects p ON te.project_id = p.id WHERE p.id IS NULL'
    )

    if (!orphanedResult) {
      return { success: false, error: 'Failed to check for orphaned texts' }
    }

    const orphanedCount = orphanedResult[0]?.count || 0

    if (orphanedCount > 0) {
      // Delete orphaned texts
      const deleteResult = await executeStatement(
        'DELETE FROM translation_entries WHERE project_id NOT IN (SELECT id FROM projects)'
      )

      if (!deleteResult) {
        return { success: false, error: 'Failed to delete orphaned texts' }
      }
    }

    return { success: true, data: { deleted_count: orphanedCount } }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}
