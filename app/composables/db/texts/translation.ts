// Translation Operations
// Handles translation-related database operations and Ollama integration

import type {
  TextOperationResult,
  BulkTextOperationResult
} from './types'
import { invokeTauri, invokeTauriVoid } from '../useTauriInvoke'

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
 * Translate a single text entry
 */
export interface SingleTranslationResult {
  translated_text: string
  model_used: string
  confidence?: number
  processing_time_ms: number
}

export async function translateSingleText(
  sourceText: string,
  sourceLanguage?: string,
  targetLanguage?: string,
  context?: string,
  model?: string
): Promise<TextOperationResult<SingleTranslationResult>> {
  return invokeTauri<SingleTranslationResult>('translate_single_text', {
    sourceText,
    sourceLanguage,
    targetLanguage,
    context,
    model
  })
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
  translatedText: string
): Promise<TextOperationResult> {
  // Use the existing updateTextTranslation function from update.ts
  const { updateTextTranslation } = await import('./update')
  return updateTextTranslation(textId, translatedText)
}

/**
 * Bulk update text entries with translations
 */
export async function bulkUpdateTranslations(
  updates: Array<{
    textId: number
    translatedText: string
  }>
): Promise<BulkTextOperationResult> {
  const result = await invokeTauri('bulk_update_translations', {
    updates: updates.map(u => ({
      textId: u.textId,
      translatedText: u.translatedText
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
