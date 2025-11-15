// Glossary Import/Export Operations
// Handles bulk import from CSV/JSON and export to CSV/JSON

import { open, save } from '@tauri-apps/plugin-dialog'
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'
import { createBulkGlossaryEntries, getGlossaryEntries } from './index'
import type {
  CreateGlossaryEntry,
  GlossaryEntry,
  GlossaryOperationResult
} from './types'

/**
 * Export glossary entries to JSON file
 */
export async function exportGlossaryToJSON(
  filters?: { source_language?: string; target_language?: string; project_id?: number | null }
): Promise<GlossaryOperationResult<{ exported_count: number; file_path: string }>> {
  try {
    // Get all entries matching filters
    const entriesResult = await getGlossaryEntries(filters || {})
    
    if (!entriesResult.success || !entriesResult.data) {
      return {
        success: false,
        error: entriesResult.error || 'Failed to fetch glossary entries'
      }
    }

    const entries = entriesResult.data.entries

    // Open save dialog
    const filePath = await save({
      title: 'Exporter le glossaire en JSON',
      defaultPath: `glossary_export_${new Date().toISOString().split('T')[0]}.json`,
      filters: [
        {
          name: 'JSON',
          extensions: ['json']
        }
      ]
    })

    if (!filePath) {
      return {
        success: false,
        error: 'Export cancelled by user'
      }
    }

    // Format entries for export (exclude internal fields if needed)
    const exportData = entries.map(entry => ({
      source_term: entry.source_term,
      translated_term: entry.translated_term,
      source_language: entry.source_language,
      target_language: entry.target_language,
      category: entry.category,
      project_id: entry.project_id
    }))

    // Write JSON file
    await writeTextFile(filePath, JSON.stringify(exportData, null, 2))

    return {
      success: true,
      data: {
        exported_count: entries.length,
        file_path: filePath
      }
    }
  } catch (error) {
    return {
      success: false,
      error: `Export failed: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

/**
 * Export glossary entries to CSV file
 */
export async function exportGlossaryToCSV(
  filters?: { source_language?: string; target_language?: string; project_id?: number | null }
): Promise<GlossaryOperationResult<{ exported_count: number; file_path: string }>> {
  try {
    // Get all entries matching filters
    const entriesResult = await getGlossaryEntries(filters || {})
    
    if (!entriesResult.success || !entriesResult.data) {
      return {
        success: false,
        error: entriesResult.error || 'Failed to fetch glossary entries'
      }
    }

    const entries = entriesResult.data.entries

    // Open save dialog
    const filePath = await save({
      title: 'Exporter le glossaire en CSV',
      defaultPath: `glossary_export_${new Date().toISOString().split('T')[0]}.csv`,
      filters: [
        {
          name: 'CSV',
          extensions: ['csv']
        }
      ]
    })

    if (!filePath) {
      return {
        success: false,
        error: 'Export cancelled by user'
      }
    }

    // CSV header
    const headers = ['source_term', 'translated_term', 'source_language', 'target_language', 'category', 'project_id']
    
    // Escape CSV values (handle quotes and commas)
    const escapeCSV = (value: string | number | null): string => {
      if (value === null || value === undefined) return ''
      const str = String(value)
      if (str.includes(',') || str.includes('"') || str.includes('\n')) {
        return `"${str.replace(/"/g, '""')}"`
      }
      return str
    }

    // Build CSV content
    const csvRows = [
      headers.join(','),
      ...entries.map(entry => 
        [
          escapeCSV(entry.source_term),
          escapeCSV(entry.translated_term),
          escapeCSV(entry.source_language),
          escapeCSV(entry.target_language),
          escapeCSV(entry.category),
          escapeCSV(entry.project_id)
        ].join(',')
      )
    ]

    // Write CSV file
    await writeTextFile(filePath, csvRows.join('\n'))

    return {
      success: true,
      data: {
        exported_count: entries.length,
        file_path: filePath
      }
    }
  } catch (error) {
    return {
      success: false,
      error: `Export failed: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

/**
 * Import glossary entries from JSON file
 */
export async function importGlossaryFromJSON(
  options?: { skipDuplicates?: boolean; project_id?: number | null }
): Promise<GlossaryOperationResult<{ imported_count: number; skipped_count: number; errors: string[] }>> {
  try {
    // Open file dialog
    const filePath = await open({
      title: 'Importer le glossaire depuis JSON',
      filters: [
        {
          name: 'JSON',
          extensions: ['json']
        }
      ],
      multiple: false
    })

    if (!filePath || typeof filePath !== 'string') {
      return {
        success: false,
        error: 'Import cancelled by user'
      }
    }

    // Read JSON file
    const fileContent = await readTextFile(filePath)
    const entries: CreateGlossaryEntry[] = JSON.parse(fileContent)

    if (!Array.isArray(entries)) {
      return {
        success: false,
        error: 'Invalid JSON format: expected an array of entries'
      }
    }

    // Validate and prepare entries
    const validEntries: CreateGlossaryEntry[] = []
    const errors: string[] = []

    for (const entry of entries) {
      // Validate required fields
      if (!entry.source_term || !entry.translated_term) {
        errors.push(`Entry missing required fields (source_term or translated_term): ${JSON.stringify(entry)}`)
        continue
      }

      // Apply project_id if provided
      const entryToImport: CreateGlossaryEntry = {
        ...entry,
        project_id: options?.project_id !== undefined ? options.project_id : entry.project_id
      }

      validEntries.push(entryToImport)
    }

    if (validEntries.length === 0) {
      return {
        success: false,
        error: `No valid entries found. ${errors.length > 0 ? `Errors: ${errors.join('; ')}` : ''}`
      }
    }

    // Import entries
    const importResult = await createBulkGlossaryEntries(validEntries)

    if (!importResult.success) {
      return {
        success: false,
        error: importResult.error || 'Failed to import entries'
      }
    }

    const importedCount = importResult.data?.inserted_count || 0
    const importErrors = importResult.data?.errors || []

    return {
      success: true,
      data: {
        imported_count: importedCount,
        skipped_count: validEntries.length - importedCount,
        errors: [...errors, ...importErrors]
      }
    }
  } catch (error) {
    return {
      success: false,
      error: `Import failed: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

/**
 * Import glossary entries from CSV file
 */
export async function importGlossaryFromCSV(
  options?: { skipDuplicates?: boolean; project_id?: number | null }
): Promise<GlossaryOperationResult<{ imported_count: number; skipped_count: number; errors: string[] }>> {
  try {
    // Open file dialog
    const filePath = await open({
      title: 'Importer le glossaire depuis CSV',
      filters: [
        {
          name: 'CSV',
          extensions: ['csv']
        }
      ],
      multiple: false
    })

    if (!filePath || typeof filePath !== 'string') {
      return {
        success: false,
        error: 'Import cancelled by user'
      }
    }

    // Read CSV file
    const fileContent = await readTextFile(filePath)
    const lines = fileContent.split('\n').filter((line): line is string => line !== undefined && line.trim().length > 0)

    if (lines.length < 2) {
      return {
        success: false,
        error: 'CSV file must contain at least a header row and one data row'
      }
    }

    // Parse CSV header
    const headerLine = lines[0]
    if (!headerLine) {
      return {
        success: false,
        error: 'CSV file is empty or invalid'
      }
    }
    const headers = headerLine.split(',').map(h => h.trim().replace(/^"|"$/g, ''))
    const requiredHeaders = ['source_term', 'translated_term']
    
    for (const required of requiredHeaders) {
      if (!headers.includes(required)) {
        return {
          success: false,
          error: `CSV file missing required column: ${required}`
        }
      }
    }

    // Parse CSV rows
    const entries: CreateGlossaryEntry[] = []
    const errors: string[] = []

    for (let i = 1; i < lines.length; i++) {
      const line = lines[i]
      if (!line) continue
      
      // Simple CSV parsing (handle quoted values)
      const values: string[] = []
      let currentValue = ''
      let inQuotes = false

      for (let j = 0; j < line.length; j++) {
        const char = line[j]
        const nextChar = line[j + 1]

        if (char === '"') {
          if (inQuotes && nextChar === '"') {
            currentValue += '"'
            j++ // Skip next quote
          } else {
            inQuotes = !inQuotes
          }
        } else if (char === ',' && !inQuotes) {
          values.push(currentValue.trim())
          currentValue = ''
        } else {
          currentValue += char
        }
      }
      values.push(currentValue.trim()) // Add last value

      // Map values to entry object
      const entry: Partial<CreateGlossaryEntry> = {}
      
      headers.forEach((header, index) => {
        const value = values[index]?.trim() || ''
        if (value) {
          if (header === 'project_id') {
            entry.project_id = value === 'null' || value === '' ? null : parseInt(value, 10)
          } else {
            (entry as Record<string, unknown>)[header] = value
          }
        }
      })

      // Validate required fields
      if (!entry.source_term || !entry.translated_term) {
        errors.push(`Row ${i + 1} missing required fields (source_term or translated_term)`)
        continue
      }

      // Apply project_id if provided
      if (options?.project_id !== undefined) {
        entry.project_id = options.project_id
      }

      entries.push(entry as CreateGlossaryEntry)
    }

    if (entries.length === 0) {
      return {
        success: false,
        error: `No valid entries found. ${errors.length > 0 ? `Errors: ${errors.join('; ')}` : ''}`
      }
    }

    // Import entries
    const importResult = await createBulkGlossaryEntries(entries)

    if (!importResult.success) {
      return {
        success: false,
        error: importResult.error || 'Failed to import entries'
      }
    }

    const importedCount = importResult.data?.inserted_count || 0
    const importErrors = importResult.data?.errors || []

    return {
      success: true,
      data: {
        imported_count: importedCount,
        skipped_count: entries.length - importedCount,
        errors: [...errors, ...importErrors]
      }
    }
  } catch (error) {
    return {
      success: false,
      error: `Import failed: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

