// Translation Operations
// Handles translation-related database operations and Ollama/RunPod integration

import type {
  TextOperationResult,
  BulkTextOperationResult
} from './types'
import { invokeTauri, invokeTauriVoid } from '../useTauriInvoke'
import { useSettings } from '../../useTauriSetting'

/**
 * Helper function to get provider settings
 */
async function getProviderSettings(): Promise<{ provider: string; pod_id?: string }> {
  const settings = useSettings()
  const userSettings = await settings.loadSettings()
  return {
    provider: userSettings.provider,
    pod_id: userSettings.provider === 'runpod' ? userSettings.runpod.pod_id : undefined
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
  textType?: string  // Text type for category filtering: 'dialogue', 'system', 'item', 'skill', 'other'
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
  const { provider, pod_id } = await getProviderSettings()
  
  // Convert texts to backend format (camelCase for Rust serde rename_all)
  const backendTexts = request.texts.map(text => ({
    id: text.id,
    sourceText: text.sourceText,
    context: text.context,
    textType: text.textType
  }))
  
  return invokeTauri('start_sequential_translation', {
    provider,
    projectId: request.projectId,
    texts: backendTexts,
    startFrom: request.startFrom,
    sourceLanguage: request.sourceLanguage,
    targetLanguage: request.targetLanguage,
    model: request.model,
    podId: pod_id
  })
}

/**
 * Get translation session progress
 */
export async function getTranslationProgress(
  sessionId: string
): Promise<TextOperationResult<TranslationProgress>> {
  const { provider, pod_id } = await getProviderSettings()
  return invokeTauri('get_sequential_progress', { 
    sessionId,
    provider,
    podId: pod_id
  })
}

/**
 * Pause a translation session
 */
export async function pauseTranslationSession(
  sessionId: string
): Promise<TextOperationResult> {
  const { provider, pod_id } = await getProviderSettings()
  return invokeTauriVoid('pause_sequential_session', { 
    sessionId,
    provider,
    podId: pod_id
  })
}

/**
 * Resume a translation session
 */
export async function resumeTranslationSession(
  sessionId: string
): Promise<TextOperationResult> {
  const { provider, pod_id } = await getProviderSettings()
  return invokeTauriVoid('resume_sequential_session', { 
    sessionId,
    provider,
    podId: pod_id
  })
}

/**
 * Stop a translation session
 */
export async function stopTranslationSession(
  sessionId: string
): Promise<TextOperationResult> {
  const { provider, pod_id } = await getProviderSettings()
  return invokeTauriVoid('stop_sequential_session', { 
    sessionId,
    provider,
    podId: pod_id
  })
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
  const { provider, pod_id } = await getProviderSettings()
  const settings = useSettings()
  const userSettings = await settings.loadSettings()
  
  // Use model from settings if not provided
  let finalModel = model
  if (!finalModel) {
    if (provider === 'ollama') {
      finalModel = userSettings.ollama.model
    } else if (provider === 'runpod') {
      finalModel = userSettings.runpod.model || undefined
      // If RunPod model is empty, we'll let backend handle it (it should use first available model)
    }
  }
  
  return invokeTauri<SingleTranslationResult>('translate_single_text', {
    provider,
    sourceText,
    sourceLanguage,
    targetLanguage,
    context,
    model: finalModel,
    podId: pod_id
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
  const { provider, pod_id } = await getProviderSettings()
  const result = await invokeTauri<TranslationSuggestion[]>('get_translation_suggestions', {
    provider,
    sourceText,
    context,
    podId: pod_id
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
  status: 'NotTranslated' | 'InProgress' | 'Translated' | 'Ignored' = 'Translated'
): Promise<TextOperationResult> {
  // Use the existing updateTextEntry function from update.ts with status
  const { updateTextEntry } = await import('./update')
  
  // Map frontend status to DB status
  const statusMap: Record<'NotTranslated' | 'InProgress' | 'Translated' | 'Ignored', 'extracted' | 'translated' | 'reviewed'> = {
    'NotTranslated': 'extracted',
    'InProgress': 'extracted', // InProgress is temporary, keep as extracted in DB
    'Translated': 'translated',
    'Ignored': 'reviewed'
  }
  
  return updateTextEntry(textId, {
    translated_text: translatedText,
    status: statusMap[status]
  })
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
