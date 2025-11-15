// Text Read Operations
// Handles retrieving text entries from database

import { executeQuery } from '../useDatabase'
import type { TextEntry } from '~/types/scanning-commands'
import type {
  DBTextEntry,
  TextFilters,
  TextQueryResult,
  DBGameFile,
  TextOperationResult
} from './types'

// Helper function to map DBTextEntry to TextEntry (frontend format)
function mapDBToFrontendText(dbText: DBTextEntry, filePath?: string): TextEntry {
  // Map status back to frontend format
  const statusMap: Record<string, TextEntry['status']> = {
    'extracted': 'NotTranslated',
    'translated': 'Translated',
    'reviewed': 'Ignored'
  }

  // Map text_type back to prompt_type
  // text_type values: 'character', 'dialogue', 'system', 'item', 'skill', 'general', 'other'
  const promptTypeMap: Record<string, TextEntry['prompt_type']> = {
    'character': 'Character',
    'dialogue': 'Dialogue',  // Dialogue maps back to 'Dialogue'
    'system': 'System',
    'item': 'Item',
    'skill': 'Skill',
    'general': 'General',  // General purpose text
    'other': 'Other'       // Other/uncategorized text
  }

  return {
    id: dbText.id.toString(),
    source_text: dbText.source_text,
    translated_text: dbText.translated_text || '',
    field_type: dbText.text_type,
    status: statusMap[dbText.status] || 'NotTranslated',
    prompt_type: promptTypeMap[dbText.text_type] || 'Character',
    location: dbText.location || '',  // Structured identifier from database
    entry_type: 'text',
    file_path: filePath
  }
}

// Get all text entries for a project
export async function getProjectTexts(
  projectId: number,
  filters?: TextFilters
): Promise<TextOperationResult<TextEntry[]>> {
  try {
    let query = `
      SELECT te.*, gf.file_path
      FROM translation_entries te
      LEFT JOIN game_files gf ON te.game_file_id = gf.id
      WHERE te.project_id = ?
    `
    const params: any[] = [projectId]

    // Apply filters
    if (filters) {
      if (filters.status && filters.status.length > 0) {
        const placeholders = filters.status.map(() => '?').join(',')
        query += ` AND te.status IN (${placeholders})`
        params.push(...filters.status)
      }

      if (filters.text_type && filters.text_type.length > 0) {
        const placeholders = filters.text_type.map(() => '?').join(',')
        query += ` AND te.text_type IN (${placeholders})`
        params.push(...filters.text_type)
      }

      if (filters.game_file_id) {
        query += ` AND te.game_file_id = ?`
        params.push(filters.game_file_id)
      }
    }

    // Add ordering
    query += ` ORDER BY te.created_at ASC`

    // Apply pagination
    if (filters?.limit) {
      query += ` LIMIT ?`
      params.push(filters.limit)

      if (filters?.offset) {
        query += ` OFFSET ?`
        params.push(filters.offset)
      }
    }

    const result = await executeQuery<DBTextEntry & { file_path?: string }>(query, params)

    if (!result) {
      return { success: false, error: 'Failed to fetch project texts' }
    }

    const texts = result.map((dbText: DBTextEntry & { file_path?: string }) => mapDBToFrontendText(dbText, dbText.file_path))

    return { success: true, data: texts }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

// Get text entries with pagination and total count
export async function getProjectTextsWithCount(
  projectId: number,
  filters?: TextFilters
): Promise<TextOperationResult<TextQueryResult>> {
  try {
    // Get total count
    let countQuery = `SELECT COUNT(*) as total FROM translation_entries WHERE project_id = ?`
    const countParams: any[] = [projectId]

    if (filters) {
      if (filters.status && filters.status.length > 0) {
        const placeholders = filters.status.map(() => '?').join(',')
        countQuery += ` AND status IN (${placeholders})`
        countParams.push(...filters.status)
      }

      if (filters.text_type && filters.text_type.length > 0) {
        const placeholders = filters.text_type.map(() => '?').join(',')
        countQuery += ` AND text_type IN (${placeholders})`
        countParams.push(...filters.text_type)
      }

      if (filters.game_file_id) {
        countQuery += ` AND game_file_id = ?`
        countParams.push(filters.game_file_id)
      }
    }

    const countResult = await executeQuery<{ total: number }>(countQuery, countParams)

    if (!countResult) {
      return { success: false, error: 'Failed to get total count' }
    }

    const totalCount = countResult[0]?.total || 0

    // Get texts with pagination
    const textsResult = await getProjectTexts(projectId, filters)

    if (!textsResult.success) {
      return { success: false, error: textsResult.error }
    }

    return {
      success: true,
      data: {
        entries: textsResult.data || [],
        total_count: totalCount
      }
    }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

// Get a single text entry by ID
export async function getTextEntry(textId: number): Promise<TextOperationResult<TextEntry>> {
  try {
    const result = await executeQuery<DBTextEntry & { file_path?: string }>(
      `SELECT te.*, gf.file_path
       FROM translation_entries te
       LEFT JOIN game_files gf ON te.game_file_id = gf.id
       WHERE te.id = ?`,
      [textId]
    )

    if (!result || result.length === 0) {
      return { success: false, error: 'Text entry not found' }
    }

    const dbText = result[0] as DBTextEntry & { file_path?: string }
    const text = mapDBToFrontendText(dbText, dbText.file_path)

    return { success: true, data: text }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

// Get game files for a project
export async function getProjectFiles(projectId: number): Promise<TextOperationResult<DBGameFile[]>> {
  try {
    const result = await executeQuery<DBGameFile>(
      'SELECT * FROM game_files WHERE project_id = ? ORDER BY file_path ASC',
      [projectId]
    )

    if (!result) {
      return { success: false, error: 'Failed to fetch project files' }
    }

    return { success: true, data: result }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

// Get text statistics for a project
export async function getProjectTextStats(projectId: number): Promise<TextOperationResult<{
  total_texts: number
  translated_texts: number
  by_status: Record<string, number>
  by_type: Record<string, number>
}>> {
  try {
    // Get total counts
    const totalResult = await executeQuery<{ count: number }>(
      'SELECT COUNT(*) as count FROM translation_entries WHERE project_id = ?',
      [projectId]
    )

    const translatedResult = await executeQuery<{ count: number }>(
      'SELECT COUNT(*) as count FROM translation_entries WHERE project_id = ? AND status = ?',
      [projectId, 'translated']
    )

    // Get counts by status
    const statusResult = await executeQuery<{ status: string; count: number }>(
      'SELECT status, COUNT(*) as count FROM translation_entries WHERE project_id = ? GROUP BY status',
      [projectId]
    )

    // Get counts by type
    const typeResult = await executeQuery<{ text_type: string; count: number }>(
      'SELECT text_type, COUNT(*) as count FROM translation_entries WHERE project_id = ? GROUP BY text_type',
      [projectId]
    )

    if (!totalResult || !translatedResult) {
      return { success: false, error: 'Failed to fetch text statistics' }
    }

    const stats = {
      total_texts: totalResult[0]?.count || 0,
      translated_texts: translatedResult[0]?.count || 0,
      by_status: (statusResult || []).reduce((acc: Record<string, number>, item) => {
        acc[item.status] = item.count
        return acc
      }, {}),
      by_type: (typeResult || []).reduce((acc: Record<string, number>, item) => {
        acc[item.text_type] = item.count
        return acc
      }, {})
    }

    return { success: true, data: stats }
  } catch (error) {
    return {
      success: false,
      error: `Database error: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}
