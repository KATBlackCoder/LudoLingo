// Scanning composables for game file analysis and text extraction
// Provides frontend interface to scanning commands

import { invoke } from '@tauri-apps/api/core'
import type {
  ScanRequest,
  ScanProgress,
  ScanResult,
  FileValidationResult,
  GameFile,
  TranslationEntry,
  TextEntry
} from '~/types/scanning-commands'

/**
 * Start scanning a game folder for translatable content
 */
export async function scanFolder(request: ScanRequest): Promise<ScanResult> {
  return await invoke('scan_folder', { request })
}

/**
 * Get current scan progress by scan ID
 */
export async function getScanProgress(scanId: string): Promise<ScanProgress> {
  return await invoke('get_scan_progress', { scanId })
}

/**
 * Cancel an ongoing scan
 */
export async function cancelScan(scanId: string): Promise<void> {
  return await invoke('cancel_scan', { scanId })
}

/**
 * Validate if a file format is supported for scanning
 */
export async function validateFileFormat(filePath: string): Promise<FileValidationResult> {
  return await invoke('validate_file_format', { filePath })
}

/**
 * Poll scan progress until completion
 * Returns the final progress when scan is done
 */
export async function waitForScanCompletion(
  scanId: string,
  onProgress?: (progress: ScanProgress) => void,
  pollInterval: number = 1000
): Promise<ScanProgress> {
  return new Promise((resolve, reject) => {
    const poll = async () => {
      try {
        const progress = await getScanProgress(scanId)

        if (onProgress) {
          onProgress(progress)
        }

        if (progress.status === 'Completed') {
          resolve(progress)
        } else if (progress.status === 'Failed') {
          reject(new Error(`Scan failed: ${progress.errors.join(', ')}`))
        } else {
          // Continue polling
          setTimeout(poll, pollInterval)
        }
      } catch (error) {
        reject(error)
      }
    }

    poll()
  })
}

/**
 * Get all game files for a project
 * Note: This would need a corresponding backend command
 */
export async function getProjectFiles(projectId: number): Promise<GameFile[]> {
  // TODO: Implement when backend command exists
  throw new Error('Not implemented yet')
}

/**
 * Get extracted translation entries for a project
 * Note: This would need a corresponding backend command
 */
export async function getProjectEntries(
  projectId: number,
  filters?: {
    status?: string[]
    textType?: string[]
    limit?: number
    offset?: number
  }
): Promise<{
  entries: TranslationEntry[]
  totalCount: number
}> {
  // TODO: Implement when backend command exists
  throw new Error('Not implemented yet')
}

/**
 * Utility function to check if a scan is still in progress
 */
export function isScanInProgress(progress: ScanProgress): boolean {
  return progress.status === 'InProgress'
}

/**
 * Utility function to check if a scan completed successfully
 */
export function isScanCompleted(progress: ScanProgress): boolean {
  return progress.status === 'Completed'
}

/**
 * Utility function to check if a scan failed
 */
export function isScanFailed(progress: ScanProgress): boolean {
  return progress.status === 'Failed'
}

/**
 * Extract texts directly from a game folder
 */
export async function extractTextsFromFolder(folderPath: string): Promise<TextEntry[]> {
  return await invoke('extract_texts_from_folder', { folderPath })
}
