// Text Creation Operations
// Handles saving extracted texts to database

import { executeQuery, executeStatement } from '../useDatabase'
import type { TextEntry } from '~/types/scanning-commands'
import type {
  CreateTextEntry,
  CreateGameFile,
  TextOperationResult,
  BulkTextOperationResult,
  DBGameFile
} from './types'

// Helper function to map TextEntry (frontend) to CreateTextEntry (DB)
function mapTextEntryToDB(text: TextEntry, projectId: number, gameFileId?: number): CreateTextEntry {
  // Map status
  const statusMap: Record<string, CreateTextEntry['status']> = {
    'NotTranslated': 'extracted',
    'Translated': 'translated',
    'Ignored': 'reviewed',
    'InProgress': 'extracted'
  }

  // Map prompt_type to text_type for database
  const textTypeMap: Record<string, CreateTextEntry['text_type']> = {
    'Character': 'dialogue',
    'Dialogue': 'dialogue',
    'Item': 'item',
    'Skill': 'skill',
    'System': 'system'
  }

  // Ensure location is never empty (required by DB)
  const location = text.location?.trim() || ''
  if (!location) {
    console.warn(`Text entry "${text.id}" has empty location, using fallback`)
  }

  // Ensure text_type is never undefined (has default in DB but better to be explicit)
  const textType = textTypeMap[text.prompt_type] || 'other'

  return {
    project_id: projectId,
    game_file_id: gameFileId,
    source_text: text.source_text,
    translated_text: text.translated_text || undefined,
    location,  // Structured format from parsers: "object_type:object_id:field"
    text_type: textType,
    status: statusMap[text.status] || 'extracted'
  }
}

// Create or get game file record
export async function createOrGetGameFile(
  projectId: number,
  filePath: string,
  fileFormat: string = 'json'
): Promise<TextOperationResult<DBGameFile>> {
  const { executeTextOperation } = await import('../useDBOperation')
  
  return executeTextOperation(async (): Promise<DBGameFile> => {
    // Check if file already exists
    const existingFiles = await executeQuery<DBGameFile>(
      'SELECT * FROM game_files WHERE project_id = ? AND file_path = ?',
      [projectId, filePath]
    )

    if (existingFiles && existingFiles.length > 0 && existingFiles[0]) {
      return existingFiles[0]
    }

    // Create new game file record
    const createData: CreateGameFile = {
      project_id: projectId,
      file_path: filePath,
      file_format: fileFormat,
      scan_status: 'completed'
    }

    const result = await executeStatement(
      `INSERT INTO game_files (project_id, file_path, file_format, scan_status, created_at)
       VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)`,
      [createData.project_id, createData.file_path, createData.file_format, createData.scan_status]
    )

    if (!result) {
      throw new Error('Failed to create game file record')
    }

    // Get the created record
    const newFile = await executeQuery<DBGameFile>(
      'SELECT * FROM game_files WHERE id = last_insert_rowid()'
    )

    if (!newFile || newFile.length === 0 || !newFile[0]) {
      throw new Error('Failed to retrieve created game file')
    }

    return newFile[0]
  }, 'creating or getting game file')
}

// Save a single text entry
export async function createTextEntry(
  projectId: number,
  text: TextEntry,
  gameFileId?: number
): Promise<TextOperationResult<number>> {
  try {
    const dbEntry = mapTextEntryToDB(text, projectId, gameFileId)

    const result = await executeStatement(
      `INSERT INTO translation_entries (
        project_id, game_file_id, source_text, translated_text, location,
        text_type, status, created_at, updated_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)`,
      [
        dbEntry.project_id,
        dbEntry.game_file_id || null,
        dbEntry.source_text,
        dbEntry.translated_text || null,
        dbEntry.location,
        dbEntry.text_type,
        dbEntry.status
      ]
    )

    if (!result) {
      return { success: false, error: 'Failed to create text entry' }
    }

    return { success: true, data: result.lastInsertId }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

// Check if project already has texts in database
export async function hasProjectTexts(projectId: number): Promise<boolean> {
  try {
    const result = await executeQuery<{ count: number }>(
      'SELECT COUNT(*) as count FROM translation_entries WHERE project_id = ?',
      [projectId]
    )
    return (result[0]?.count || 0) > 0
  } catch (error) {
    console.error('Error checking if project has texts:', error)
    return false
  }
}

// Save multiple text entries in bulk (only if project doesn't have texts yet)
export async function createBulkTextEntries(
  projectId: number,
  texts: TextEntry[],
  filePath?: string
): Promise<BulkTextOperationResult> {
  const errors: string[] = []
  let insertedCount = 0

  try {
    // Check if project already has texts
    const hasExistingTexts = await hasProjectTexts(projectId)

    if (hasExistingTexts) {
      console.log(`ℹ️ Projet ${projectId} a déjà ${await getProjectTextCount(projectId)} textes en DB - skipping insertion`)
      return {
        success: true,
        inserted_count: 0,
        errors: []
      }
    }

    console.log(`💾 Insertion de ${texts.length} nouveaux textes pour le projet ${projectId}...`)

    // Group texts by file_path to create game_file records
    const textsByFile = new Map<string, TextEntry[]>()
    for (const text of texts) {
      const textFilePath = text.file_path || filePath || 'unknown'
      if (!textsByFile.has(textFilePath)) {
        textsByFile.set(textFilePath, [])
      }
      textsByFile.get(textFilePath)!.push(text)
    }

    // Create/get game_file records for each unique file_path
    const gameFileMap = new Map<string, number>()
    for (const [filePath] of textsByFile.entries()) {
      if (filePath && filePath !== 'unknown') {
      const gameFileResult = await createOrGetGameFile(projectId, filePath, 'json')
      if (gameFileResult.success && gameFileResult.data) {
          gameFileMap.set(filePath, gameFileResult.data.id)
      } else {
          errors.push(`Failed to create/get game file ${filePath}: ${gameFileResult.error}`)
        }
      }
    }

    // Process texts in batches to avoid overwhelming the database
    const batchSize = 50
    for (let i = 0; i < texts.length; i += batchSize) {
      const batch = texts.slice(i, i + batchSize)

      for (const text of batch) {
        // Get game_file_id for this text's file_path
        const textFilePath = text.file_path || filePath || 'unknown'
        const textGameFileId = textFilePath !== 'unknown' ? gameFileMap.get(textFilePath) : undefined

        const result = await createTextEntry(projectId, text, textGameFileId)
        if (result.success) {
          insertedCount++
        } else {
          errors.push(`Failed to save text "${text.source_text.substring(0, 50)}...": ${result.error}`)
        }
      }
    }

    console.log(`✅ ${insertedCount} textes insérés pour le projet ${projectId}`)

    return {
      success: errors.length === 0,
      inserted_count: insertedCount,
      errors
    }
  } catch (error) {
    return {
      success: false,
      inserted_count: insertedCount,
      errors: [...errors, `Bulk operation failed: ${error instanceof Error ? error.message : 'Unknown error'}`]
    }
  }
}

// Helper function to get text count for a project
async function getProjectTextCount(projectId: number): Promise<number> {
  try {
    const result = await executeQuery<{ count: number }>(
      'SELECT COUNT(*) as count FROM translation_entries WHERE project_id = ?',
      [projectId]
    )
    return result[0]?.count || 0
  } catch (error) {
    console.error('Error getting project text count:', error)
    return 0
  }
}
