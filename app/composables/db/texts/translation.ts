// Translation Operations
// Handles translation-related database operations and Ollama integration

import { invoke } from '@tauri-apps/api/core'
import type {
  TextOperationResult,
  BulkTextOperationResult
} from './types'

/**
 * Generic helper for Tauri invoke operations with error handling
 */
async function invokeTauri<T>(
  command: string,
  args: Record<string, any>
): Promise<TextOperationResult<T>> {
  try {
    console.log(`🔵 [Frontend] Calling Tauri command: ${command}`, args)
    const result = await invoke(command, args)
    console.log(`✅ [Frontend] Command ${command} succeeded:`, result)
    return { success: true, data: result as T }
  } catch (error) {
    console.error(`❌ [Frontend] Command ${command} failed:`, error)
    console.error(`❌ [Frontend] Error type:`, typeof error)
    console.error(`❌ [Frontend] Error details:`, JSON.stringify(error, null, 2))
    return {
      success: false,
      error: `Failed to ${command.replace(/_/g, ' ')}: ${error instanceof Error ? error.message : String(error)}`
    }
  }
}

/**
 * Generic helper for Tauri invoke operations without return data
 */
async function invokeTauriVoid(
  command: string,
  args: Record<string, any>
): Promise<TextOperationResult> {
  try {
    await invoke(command, args)
    return { success: true }
  } catch (error) {
    return {
      success: false,
      error: `Failed to ${command.replace(/_/g, ' ')}: ${error instanceof Error ? error.message : 'Unknown error'}`
    }
  }
}

// Translation session types
export interface TranslationSession {
  session_id: string
  project_id: number
  current_entry?: number
  processed_count: number
  total_count: number
  status: 'idle' | 'running' | 'paused' | 'completed' | 'error'
  estimated_time_remaining?: number
  error_count: number
}

export interface TranslationProgress {
  session_id: string
  current_entry?: number
  processed_count: number
  total_count: number
  status: 'idle' | 'running' | 'paused' | 'completed' | 'error'
  estimated_time_remaining?: number
  errors: Array<{
    entry_id: number
    error_message: string
    timestamp: string
  }>
  successful_translations: Array<{
    entry_id: number
    translated_text: string
    timestamp: string
  }>
}


export interface TranslationSuggestion {
  suggestion: string
  confidence: number
  source: 'ollama' | 'glossary' | 'similar'
}

export interface TranslationText {
  id: number
  sourceText: string
  context?: string
}

export interface StartTranslationRequest {
  projectId: number
  texts: TranslationText[]
  startFrom?: number
  sourceLanguage?: string
  targetLanguage?: string
  model?: string
}


/**
 * Start a sequential translation session
 */
export async function startSequentialTranslation(
  request: StartTranslationRequest
): Promise<TextOperationResult<{ session_id: string; total_entries: number }>> {
  return invokeTauri('start_sequential_translation', {
    projectId: request.projectId,
    texts: request.texts,
    startFrom: request.startFrom,
    sourceLanguage: request.sourceLanguage,
    targetLanguage: request.targetLanguage,
    model: request.model
  })
}

/**
 * Get translation session progress
 */
export async function getTranslationProgress(
  sessionId: string
): Promise<TextOperationResult<TranslationProgress>> {
  return invokeTauri('get_sequential_progress', { sessionId })
}

/**
 * Pause a translation session
 */
export async function pauseTranslationSession(
  sessionId: string
): Promise<TextOperationResult> {
  return invokeTauriVoid('pause_sequential_session', { sessionId })
}

/**
 * Resume a translation session
 */
export async function resumeTranslationSession(
  sessionId: string
): Promise<TextOperationResult> {
  return invokeTauriVoid('resume_sequential_session', { sessionId })
}

/**
 * Stop a translation session
 */
export async function stopTranslationSession(
  sessionId: string
): Promise<TextOperationResult> {
  return invokeTauriVoid('stop_sequential_session', { sessionId })
}

/**
 * Get active translation sessions for a project
 */
export async function getProjectTranslationSessions(
  projectId: number
): Promise<TextOperationResult<TranslationSession[]>> {
  return invokeTauri('get_project_sessions', { projectId })
}

/**
 * Get translation suggestions for a text
 */
export async function getTranslationSuggestions(
  sourceText: string,
  context?: string,
  maxSuggestions = 3
): Promise<TextOperationResult<TranslationSuggestion[]>> {
  const result = await invokeTauri<TranslationSuggestion[]>('get_translation_suggestions', {
    sourceText,
    context
  })

  if (result.success && result.data) {
    // Limit to maxSuggestions
    const limitedSuggestions = result.data.slice(0, maxSuggestions)
    return { success: true, data: limitedSuggestions }
  }

  return result as TextOperationResult<TranslationSuggestion[]>
}

/**
 * Update a text entry with translation
 */
export async function updateTextWithTranslation(
  textId: number,
  translatedText: string,
  translationSource: 'manual' | 'ollama' | 'glossary' = 'manual'
): Promise<TextOperationResult> {
  // Use the existing updateTextTranslation function from update.ts
  const { updateTextTranslation } = await import('./update')
  return updateTextTranslation(textId, translatedText, translationSource)
}

/**
 * Bulk update text entries with translations
 */
export async function bulkUpdateTranslations(
  updates: Array<{
    textId: number
    translatedText: string
    translationSource?: 'manual' | 'ollama' | 'glossary'
  }>
): Promise<BulkTextOperationResult> {
  const result = await invokeTauri('bulk_update_translations', {
    updates: updates.map(u => ({
      textId: u.textId,
      translatedText: u.translatedText,
      translationSource: u.translationSource || 'manual'
    }))
  })

  if (result.success && result.data) {
    const data = result.data as { inserted_count: number; errors: string[] }
    return {
      success: data.errors.length === 0,
      inserted_count: data.inserted_count,
      errors: data.errors
    }
  }

  return {
    success: false,
    inserted_count: 0,
    errors: result.error ? [result.error] : ['Bulk update failed']
  }
}
