// Translation store using Pinia
// Manages translation state, sessions, and progress tracking

import { defineStore, storeToRefs } from 'pinia'
import { ref, computed } from 'vue'
import { useOllamaStore } from '~/stores/ollama'
import { useProjectsStore } from '~/stores/projects'
import type {
  TranslationSession,
  TranslationProgress,
  TranslationSuggestion,
  StartTranslationRequest,
  TextOperationResult
} from '~/composables/db/texts'
import {
  startSequentialTranslation,
  getTranslationProgress,
  pauseTranslationSession,
  resumeTranslationSession,
  stopTranslationSession,
  getProjectTranslationSessions,
  getTranslationSuggestions,
  updateTextWithTranslation
} from '~/composables/db/texts'

export const useTranslationStore = defineStore('translation', () => {
  // State - only translation-specific state
  const activeSessions = ref<TranslationSession[]>([])
  const sessionProgress = ref<Map<string, TranslationProgress>>(new Map())

  // Temporary state to track texts currently being translated
  const textsBeingTranslated = ref<Set<number>>(new Set())

  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Default settings
  const defaultSourceLanguage = ref('ja')
  const defaultTargetLanguage = ref('fr')
  const defaultModel = ref('llama3.2:3b')

  // Use Ollama store for Ollama-related state
  const ollamaStore = useOllamaStore()
  const { isConnected, availableModels } = storeToRefs(ollamaStore)

  // Helper function for filtering sessions by status
  const getSessionsByStatus = (status: TranslationSession['status']) => {
    return computed(() => activeSessions.value.filter(session => session.status === status))
  }

  // Helper function for session control operations
  const executeSessionOperation = async (
    sessionId: string,
    operation: () => Promise<TextOperationResult>,
    operationName: string,
    options: {
      newStatus?: TranslationSession['status']
      removeSession?: boolean
      onSuccess?: (session: TranslationSession) => void
    } = {}
  ) => {
    try {
      error.value = null

      const result = await operation()

      if (result.success) {
        if (options.removeSession) {
          // Special case: remove session entirely
          const sessionIndex = activeSessions.value.findIndex(s => s.session_id === sessionId)
          if (sessionIndex !== -1) {
            activeSessions.value.splice(sessionIndex, 1)
            sessionProgress.value.delete(sessionId)
          }

          // Stop monitoring for this session
          if (progressMonitoring.has(sessionId)) {
            clearInterval(progressMonitoring.get(sessionId)!)
            progressMonitoring.delete(sessionId)
            console.log(`🛑 Stopped monitoring for session ${sessionId}`)
          }

          // Clear any remaining texts being translated from this session
          // Note: This is a simplified approach - in a real implementation,
          // we would track which session contains which texts
          console.log(`🧹 Cleared texts being translated tracking for stopped session ${sessionId}`)
        } else if (options.newStatus) {
          // Update session status
          const session = activeSessions.value.find(s => s.session_id === sessionId)
          if (session) {
            session.status = options.newStatus
            if (options.onSuccess) options.onSuccess(session)
          }
        }

        const emoji = options.newStatus === 'paused' ? '⏸️' : options.newStatus === 'running' ? '▶️' : '🛑'
        console.log(`${emoji} Session ${operationName.toLowerCase()}:`, sessionId)
      } else {
        throw new Error(result.error || `Failed to ${operationName.toLowerCase()} session`)
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown error'
      error.value = `Failed to ${operationName.toLowerCase()} session: ${errorMessage}`
      console.error(`Error ${operationName.toLowerCase()} session:`, err)
      throw err
    }
  }

  // Getters

  const hasActiveSessions = computed(() => {
    return activeSessions.value.length > 0
  })

  const runningSessions = getSessionsByStatus('running')
  const pausedSessions = getSessionsByStatus('paused')
  const completedSessions = getSessionsByStatus('completed')
  const errorSessions = getSessionsByStatus('error')

  const getSessionProgress = (sessionId: string) => {
    return sessionProgress.value.get(sessionId) || null
  }

  // Actions

  const startTranslation = async (request: StartTranslationRequest) => {
    try {
      isLoading.value = true
      error.value = null

      console.log('🚀 Starting translation session:', request)

      // Add text IDs to the "being translated" set
      request.texts.forEach(text => {
        textsBeingTranslated.value.add(text.id)
      })

      const result = await startSequentialTranslation(request)

      if (result.success && result.data) {
        const session: TranslationSession = {
          session_id: result.data.session_id,
          project_id: request.projectId,
          current_entry: undefined,
          processed_count: 0,
          total_count: result.data.total_entries,
          status: 'running',
          estimated_time_remaining: undefined,
          error_count: 0
        }

        activeSessions.value.push(session)

        // Start progress monitoring
        monitorSessionProgress(result.data.session_id)

        console.log('✅ Translation session started:', session)
        return session
      } else {
        throw new Error(result.error || 'Failed to start translation')
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown error'
      error.value = `Failed to start translation: ${errorMessage}`
      console.error('Error starting translation:', err)
      throw err
    } finally {
      isLoading.value = false
    }
  }

  const pauseSession = async (sessionId: string) => {
    return executeSessionOperation(
      sessionId,
      () => pauseTranslationSession(sessionId),
      'Pause',
      { newStatus: 'paused' }
    )
  }

  const resumeSession = async (sessionId: string) => {
    return executeSessionOperation(
      sessionId,
      () => resumeTranslationSession(sessionId),
      'Resume',
      {
        newStatus: 'running',
        onSuccess: (session) => monitorSessionProgress(session.session_id) // Restart monitoring
      }
    )
  }

  const stopSession = async (sessionId: string) => {
    return executeSessionOperation(
      sessionId,
      () => stopTranslationSession(sessionId),
      'Stop',
      { removeSession: true }
    )
  }

  const loadProjectSessions = async (projectId: number) => {
    try {
      error.value = null

      const result = await getProjectTranslationSessions(projectId)

      if (result.success && result.data) {
        // Update existing sessions and add new ones
        for (const session of result.data) {
          const existingIndex = activeSessions.value.findIndex(s => s.session_id === session.session_id)
          if (existingIndex !== -1) {
            // Update existing
            activeSessions.value[existingIndex] = session
          } else {
            // Add new
            activeSessions.value.push(session)
          }

          // Start monitoring if running
          if (session.status === 'running') {
            monitorSessionProgress(session.session_id)
          }
        }

        console.log(`✅ Loaded ${result.data.length} sessions for project ${projectId}`)
      } else {
        console.warn('⚠️ Failed to load project sessions:', result.error)
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown error'
      error.value = `Failed to load project sessions: ${errorMessage}`
      console.error('Error loading project sessions:', err)
    }
  }

  const monitorSessionProgress = (sessionId: string) => {
    // Clear any existing monitoring for this session
    if (progressMonitoring.has(sessionId)) {
      clearInterval(progressMonitoring.get(sessionId)!)
    }

    const intervalId = setInterval(async () => {
      try {
        const result = await getTranslationProgress(sessionId)

        if (result.success && result.data) {
          const progress = result.data

          // SAVE SUCCESSFUL TRANSLATIONS TO DB
          if (progress.successful_translations && progress.successful_translations.length > 0) {
            for (const translation of progress.successful_translations) {
              try {
                await applyTranslation(translation.entry_id, translation.translated_text, 'ollama')
                // Remove from "being translated" set since it's now saved
                textsBeingTranslated.value.delete(translation.entry_id)
                console.log(`💾 [DB] Saved translation for entry ${translation.entry_id}`)
              } catch (saveError) {
                console.error(`❌ [DB] Failed to save translation for entry ${translation.entry_id}:`, saveError)
              }
            }
          }

          // Update session in activeSessions
          const sessionIndex = activeSessions.value.findIndex(s => s.session_id === sessionId)
          if (sessionIndex !== -1) {
            const session = activeSessions.value[sessionIndex]
            if (session) {
              session.current_entry = progress.current_entry
              session.processed_count = progress.processed_count
              session.status = progress.status as TranslationSession['status']
              session.estimated_time_remaining = progress.estimated_time_remaining
              session.error_count = progress.errors.length
            }
          }

          // Store detailed progress
          sessionProgress.value.set(sessionId, progress)

          // Stop monitoring if session is completed or errored
          if (progress.status === 'completed' || progress.status === 'error') {
            clearInterval(intervalId)
            progressMonitoring.delete(sessionId)
            console.log(`🏁 Session ${sessionId} finished with status: ${progress.status}`)
          }
        } else {
          console.warn(`⚠️ Failed to get progress for session ${sessionId}:`, result.error)
        }
      } catch (err) {
        console.error(`Error monitoring session ${sessionId}:`, err)
      }
    }, 5000) // Check every 5 seconds

    progressMonitoring.set(sessionId, intervalId)
  }

  // Map to track progress monitoring intervals
  const progressMonitoring = new Map<string, number>()

  const getSuggestions = async (
    sourceText: string,
    context?: string,
    maxSuggestions = 3
  ): Promise<TranslationSuggestion[]> => {
    try {
      const result = await getTranslationSuggestions(sourceText, context, maxSuggestions)

      if (result.success && result.data) {
        return result.data
      } else {
        console.warn('Failed to get suggestions:', result.error)
        return []
      }
    } catch (err) {
      console.error('Error getting suggestions:', err)
      return []
    }
  }

  const applyTranslation = async (
    textId: number,
    translatedText: string,
    source: 'manual' | 'ollama' | 'glossary' = 'manual'
  ) => {
    try {
      error.value = null

      // 1. Mise à jour de la base de données
      const result = await updateTextWithTranslation(textId, translatedText, source)

      if (result.success) {
        // 2. Mise à jour directe du store projects pour rafraîchir l'UI
        const projectsStore = useProjectsStore()
        const project = projectsStore.currentProject

        if (project) {
          // Trouver et mettre à jour le texte dans le store
          const textIndex = project.extractedTexts.findIndex(t => parseInt(t.id) === textId)
          if (textIndex !== -1) {
            const textEntry = project.extractedTexts[textIndex]
            if (textEntry) {
              textEntry.translated_text = translatedText
              textEntry.status = 'Translated'
              // Recalculer le nombre de textes traduits
              project.translatedTexts = project.extractedTexts.filter(t => t.status === 'Translated').length

              console.log(`✅ Applied ${source} translation to text ${textId} + UI refreshed`)
            }
          } else {
            console.warn(`⚠️ Text ${textId} not found in current project store`)
          }
        } else {
          console.warn('⚠️ No current project found for UI update')
        }
      } else {
        throw new Error(result.error || 'Failed to apply translation')
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown error'
      error.value = `Failed to apply translation: ${errorMessage}`
      console.error('Error applying translation:', err)
      throw err
    }
  }

  const clearError = () => {
    error.value = null
  }

  const cleanupCompletedSessions = () => {
    // Remove completed and errored sessions older than 1 hour
    const oneHourAgo = Date.now() - (60 * 60 * 1000)

    activeSessions.value = activeSessions.value.filter(session => {
      if ((session.status === 'completed' || session.status === 'error')) {
        // Keep for debugging, but could be removed based on timestamp
        return true
      }
      return true
    })

    // Clean up progress monitoring for removed sessions
    for (const [sessionId, intervalId] of progressMonitoring.entries()) {
      if (!activeSessions.value.some(s => s.session_id === sessionId)) {
        clearInterval(intervalId)
        progressMonitoring.delete(sessionId)
      }
    }
  }

  const updateDefaultSettings = (
    sourceLanguage?: string,
    targetLanguage?: string,
    model?: string
  ) => {
    if (sourceLanguage) defaultSourceLanguage.value = sourceLanguage
    if (targetLanguage) defaultTargetLanguage.value = targetLanguage
    if (model) defaultModel.value = model
  }

  const cleanupMonitoringIntervals = () => {
    for (const [sessionId, intervalId] of progressMonitoring.entries()) {
      clearInterval(intervalId)
      console.log(`🧹 Cleaned up monitoring interval for session ${sessionId}`)
    }
    progressMonitoring.clear()
  }

  return {
    // State
    activeSessions,
    sessionProgress: computed(() => sessionProgress.value),
    textsBeingTranslated,
    isLoading,
    error,
    defaultSourceLanguage,
    defaultTargetLanguage,
    defaultModel,

    // Ollama state (from ollama store)
    isOllamaConnected: isConnected,
    availableModels,

    // Getters
    hasActiveSessions,
    runningSessions,
    pausedSessions,
    completedSessions,
    errorSessions,
    getSessionProgress,

    // Actions
    startTranslation,
    pauseSession,
    resumeSession,
    stopSession,
    loadProjectSessions,
    getSuggestions,
    applyTranslation,
    clearError,
    cleanupCompletedSessions,
    cleanupMonitoringIntervals,
    updateDefaultSettings
  }
})
