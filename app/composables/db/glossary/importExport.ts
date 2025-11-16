// Glossary Import/Export Operations
// Handles bulk import from CSV/JSON/XLSX and export to CSV/JSON/XLSX

import { open, save } from '@tauri-apps/plugin-dialog'
import { readTextFile, writeTextFile, readFile, writeFile } from '@tauri-apps/plugin-fs'
import { read, utils, write } from 'xlsx'
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
 * Export glossary entries to CSV file using SheetJS
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

    // Prepare data for SheetJS
    const worksheetData = entries.map(entry => ({
      source_term: entry.source_term,
      translated_term: entry.translated_term,
      source_language: entry.source_language,
      target_language: entry.target_language,
      category: entry.category,
      project_id: entry.project_id ?? ''
    }))

    // Create workbook and worksheet using SheetJS
    const worksheet = utils.json_to_sheet(worksheetData)
    const workbook = utils.book_new()
    utils.book_append_sheet(workbook, worksheet, 'Glossary')

    // Convert to CSV string
    const csvString = utils.sheet_to_csv(worksheet)

    // Write CSV file
    await writeTextFile(filePath, csvString)

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
 * Export glossary entries to Excel (XLSX) file using SheetJS
 */
export async function exportGlossaryToXLSX(
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
      title: 'Exporter le glossaire en Excel',
      defaultPath: `glossary_export_${new Date().toISOString().split('T')[0]}.xlsx`,
      filters: [
        {
          name: 'Excel',
          extensions: ['xlsx']
        }
      ]
    })

    if (!filePath) {
      return {
        success: false,
        error: 'Export cancelled by user'
      }
    }

    // Prepare data for SheetJS
    const worksheetData = entries.map(entry => ({
      source_term: entry.source_term,
      translated_term: entry.translated_term,
      source_language: entry.source_language,
      target_language: entry.target_language,
      category: entry.category,
      project_id: entry.project_id ?? ''
    }))

    // Create workbook and worksheet using SheetJS
    const worksheet = utils.json_to_sheet(worksheetData)
    const workbook = utils.book_new()
    utils.book_append_sheet(workbook, worksheet, 'Glossary')

    // Convert to XLSX buffer (Uint8Array)
    const xlsxBuffer = write(workbook, { type: 'array', bookType: 'xlsx' })

    // Write XLSX file as binary
    await writeFile(filePath, new Uint8Array(xlsxBuffer))

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
 * Import glossary entries from CSV file using SheetJS
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

    // Parse CSV using SheetJS
    const workbook = read(fileContent, { type: 'string' })
    const firstSheetName = workbook.SheetNames[0]
    if (!firstSheetName) {
      return {
        success: false,
        error: 'CSV file is empty or invalid'
      }
    }

    const worksheet = workbook.Sheets[firstSheetName]
    if (!worksheet) {
      return {
        success: false,
        error: 'CSV file worksheet is invalid'
      }
    }
    const jsonData = utils.sheet_to_json<Record<string, unknown>>(worksheet)

    if (!Array.isArray(jsonData) || jsonData.length === 0) {
      return {
        success: false,
        error: 'CSV file must contain at least a header row and one data row'
      }
    }

    // Validate required headers
    const requiredHeaders = ['source_term', 'translated_term']
    const firstRow = jsonData[0]
    if (!firstRow) {
      return {
        success: false,
        error: 'CSV file is empty'
      }
    }

    const headers = Object.keys(firstRow)
    for (const required of requiredHeaders) {
      if (!headers.includes(required)) {
        return {
          success: false,
          error: `CSV file missing required column: ${required}`
        }
      }
    }

    // Parse entries
    const entries: CreateGlossaryEntry[] = []
    const errors: string[] = []

    for (let i = 0; i < jsonData.length; i++) {
      const row = jsonData[i]
      if (!row) continue

      // Map row to entry object
      const entry: Partial<CreateGlossaryEntry> = {}
      
      headers.forEach(header => {
        const value = row[header]
        if (value !== undefined && value !== null && value !== '') {
          if (header === 'project_id') {
            entry.project_id = value === 'null' || value === '' ? null : typeof value === 'number' ? value : parseInt(String(value), 10)
          } else {
            (entry as Record<string, unknown>)[header] = String(value)
          }
        }
      })

      // Validate required fields
      if (!entry.source_term || !entry.translated_term) {
        errors.push(`Row ${i + 2} missing required fields (source_term or translated_term)`)
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

/**
 * Import glossary entries from Excel (XLSX) file using SheetJS
 */
export async function importGlossaryFromXLSX(
  options?: { skipDuplicates?: boolean; project_id?: number | null }
): Promise<GlossaryOperationResult<{ imported_count: number; skipped_count: number; errors: string[] }>> {
  try {
    // Open file dialog
    const filePath = await open({
      title: 'Importer le glossaire depuis Excel',
      filters: [
        {
          name: 'Excel',
          extensions: ['xlsx', 'xls']
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

    // Read Excel file as binary
    const fileBuffer = await readFile(filePath)

    // Parse Excel using SheetJS
    const workbook = read(fileBuffer, { type: 'array' })
    const firstSheetName = workbook.SheetNames[0]
    if (!firstSheetName) {
      return {
        success: false,
        error: 'Excel file is empty or invalid'
      }
    }

    const worksheet = workbook.Sheets[firstSheetName]
    if (!worksheet) {
      return {
        success: false,
        error: 'Excel file worksheet is invalid'
      }
    }
    const jsonData = utils.sheet_to_json<Record<string, unknown>>(worksheet)

    if (!Array.isArray(jsonData) || jsonData.length === 0) {
      return {
        success: false,
        error: 'Excel file must contain at least a header row and one data row'
      }
    }

    // Validate required headers
    const requiredHeaders = ['source_term', 'translated_term']
    const firstRow = jsonData[0]
    if (!firstRow) {
      return {
        success: false,
        error: 'Excel file is empty'
      }
    }

    const headers = Object.keys(firstRow)
    for (const required of requiredHeaders) {
      if (!headers.includes(required)) {
        return {
          success: false,
          error: `Excel file missing required column: ${required}`
        }
      }
    }

    // Parse entries
    const entries: CreateGlossaryEntry[] = []
    const errors: string[] = []

    for (let i = 0; i < jsonData.length; i++) {
      const row = jsonData[i]
      if (!row) continue

      // Map row to entry object
      const entry: Partial<CreateGlossaryEntry> = {}
      
      headers.forEach(header => {
        const value = row[header]
        if (value !== undefined && value !== null && value !== '') {
          if (header === 'project_id') {
            entry.project_id = value === 'null' || value === '' ? null : typeof value === 'number' ? value : parseInt(String(value), 10)
          } else {
            (entry as Record<string, unknown>)[header] = String(value)
          }
        }
      })

      // Validate required fields
      if (!entry.source_term || !entry.translated_term) {
        errors.push(`Row ${i + 2} missing required fields (source_term or translated_term)`)
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

