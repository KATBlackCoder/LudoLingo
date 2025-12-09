// Translation store using Pinia
// Manages translation state, sessions, and progress tracking

import { defineStore, storeToRefs } from 'pinia'
import { ref, computed } from 'vue'
import { useBaseStoreState, executeAsyncOperation } from '~/composables/stores/useBaseStore'
import { useOllamaStore } from '~/stores/ollama'
import { useProjectsStore } from '~/stores/projects'
import { isConnectionError } from '~/utils/connectionErrors'
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
  // Base store state (isLoading, error, clearError)
  const { isLoading, error, clearError } = useBaseStoreState()
  
  // State - only translation-specific state
  const activeSessions = ref<TranslationSession[]>([])
  const sessionProgress = ref<Map<string, TranslationProgress>>(new Map())

  // Temporary state to track texts currently being translated
  const textsBeingTranslated = ref<Set<number>>(new Set())

  // State for selected texts for retranslation
  const selectedTextsForRetranslation = ref<Array<{ id: string; source_text: string; location?: string; prompt_type: string }>>([])

  // Pause management state
  const sessionBatchCounters = ref<Map<string, number>>(new Map()) // Track translations per session
  const activeCountdowns = ref<Map<string, { timer: number; remaining: number }>>(new Map()) // Active countdowns
  const sessionPauseSettings = ref<Map<string, { enabled: boolean; batchSize: number; pauseDurationMinutes: number }>>(new Map()) // Pause settings per session

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
          }

          // Clear any remaining texts being translated from this session
          // Note: This is a simplified approach - in a real implementation,
          // we would track which session contains which texts
        } else if (options.newStatus) {
          // Update session status
          const session = activeSessions.value.find(s => s.session_id === sessionId)
          if (session) {
            session.status = options.newStatus
            if (options.onSuccess) options.onSuccess(session)
          }
        }

        const emoji = options.newStatus === 'paused' ? '⏸️' : options.newStatus === 'running' ? '▶️' : '🛑'
      } else {
        throw new Error(result.error || `Failed to ${operationName.toLowerCase()} session`)
      }
    } catch (err) {
      // Better error message extraction
      let errorMessage: string
      if (err instanceof Error) {
        errorMessage = err.message
      } else if (typeof err === 'string') {
        errorMessage = err
      } else if (err && typeof err === 'object' && 'message' in err) {
        errorMessage = String((err as { message: unknown }).message)
      } else {
        errorMessage = String(err) || 'Unknown error'
      }
      
      error.value = `Failed to ${operationName.toLowerCase()} session: ${errorMessage}`
      console.error(`Error ${operationName.toLowerCase()} session:`, err)
      throw err
    }
  }

  // Getters

  const hasActiveSessions = computed(() => {
    // Only consider sessions that are actually active (running or paused)
    // Completed and error sessions should not be considered active
    return activeSessions.value.some(session => 
      session.status === 'running' || session.status === 'paused'
    )
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
      // Add text IDs to the "being translated" set
      request.texts.forEach(text => {
        textsBeingTranslated.value.add(text.id)
      })

    try {
      return await executeAsyncOperation(async () => {
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

        // Store pause settings for this session
        if (request.pauseSettings) {
          sessionPauseSettings.value.set(result.data.session_id, request.pauseSettings)
        }

        // Start progress monitoring
        monitorSessionProgress(result.data.session_id)

        return session
      } else {
          // Nettoyer les textes en cours si le démarrage échoue
          request.texts.forEach(text => {
            textsBeingTranslated.value.delete(text.id)
          })
        throw new Error(result.error || 'Failed to start translation')
      }
      }, 'Failed to start translation', { isLoading, error })
    } catch (err) {
      // Nettoyer les textes en cours si une erreur survient
      request.texts.forEach(text => {
        textsBeingTranslated.value.delete(text.id)
      })
      throw err
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
    return executeAsyncOperation(async () => {
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
      } else {
        // Log warning but don't throw - sessions might just be empty
        console.warn('⚠️ Failed to load project sessions:', result.error)
      }
    }, 'Failed to load project sessions', { isLoading, error }, { skipLoading: true })
  }

  // Pause management functions
  const startPauseCountdown = async (sessionId: string, pauseDurationMinutes: number) => {
    // Clear any existing countdown for this session
    clearPauseCountdown(sessionId)

    const totalSeconds = pauseDurationMinutes * 60
    let remainingSeconds = totalSeconds

    console.log(`⏸️ Starting pause countdown for session ${sessionId}: ${pauseDurationMinutes} minutes (${totalSeconds}s)`)

    const timer = setInterval(() => {
      remainingSeconds--
      console.log(`⏱️ Pause countdown ${sessionId}: ${remainingSeconds}s remaining`)

      if (remainingSeconds <= 0) {
        clearPauseCountdown(sessionId)
        console.log(`▶️ Pause countdown finished for session ${sessionId}, resuming...`)
        resumeSession(sessionId).catch(err => {
          console.error(`❌ Failed to resume session ${sessionId}:`, err)
        })
      }
    }, 1000)

    activeCountdowns.value.set(sessionId, { timer, remaining: remainingSeconds })
  }

  const clearPauseCountdown = (sessionId: string) => {
    const countdown = activeCountdowns.value.get(sessionId)
    if (countdown) {
      clearInterval(countdown.timer)
      activeCountdowns.value.delete(sessionId)
      console.log(`🧹 Cleared countdown for session ${sessionId}`)
    }
  }

  const getPauseTimeRemaining = (sessionId: string): number | null => {
    const countdown = activeCountdowns.value.get(sessionId)
    return countdown ? countdown.remaining : null
  }

  const checkAndTriggerPause = async (sessionId: string, newTranslationsCount: number) => {
    // Get pause settings for this session
    const pauseSettings = sessionPauseSettings.value.get(sessionId)
    if (!pauseSettings?.enabled) return

    // Find the session
    const session = activeSessions.value.find(s => s.session_id === sessionId)
    if (!session) return

    // Increment batch counter
    const currentCount = sessionBatchCounters.value.get(sessionId) || 0
    const newCount = currentCount + newTranslationsCount
    sessionBatchCounters.value.set(sessionId, newCount)

    console.log(`📊 Session ${sessionId}: ${newCount}/${pauseSettings.batchSize} translations in current batch`)

    // Check if batch size is reached
    if (newCount >= pauseSettings.batchSize) {
      console.log(`🎯 Batch size reached for session ${sessionId} (${newCount} >= ${pauseSettings.batchSize})`)

      // Trigger pause
      await pauseSession(sessionId)

      // Reset batch counter for next batch
      sessionBatchCounters.value.set(sessionId, 0)

      // Start countdown
      await startPauseCountdown(sessionId, pauseSettings.pauseDurationMinutes)
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
              } catch (saveError) {
                console.error(`❌ [DB] Failed to save translation for entry ${translation.entry_id}:`, saveError)
              }
            }

            // Check if we need to trigger a pause after processing successful translations
            await checkAndTriggerPause(sessionId, progress.successful_translations.length)
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
          console.log('📡 Backend progress reçu pour session', sessionId, ':', {
            status: progress.status,
            pause_time_remaining: progress.pause_time_remaining,
            processed_count: progress.processed_count,
            full_progress: progress
          })
          sessionProgress.value.set(sessionId, progress)

          // Stop monitoring if session is completed or errored
          if (progress.status === 'completed' || progress.status === 'error') {
            clearInterval(intervalId)
            progressMonitoring.delete(sessionId)

            // Clean up pause-related state
            sessionBatchCounters.value.delete(sessionId)
            clearPauseCountdown(sessionId)
            sessionPauseSettings.value.delete(sessionId)
          }
        } else {
          // Si erreur de connexion Ollama, arrêter automatiquement la session
          if (isConnectionError(result.error)) {
            console.error(`❌ Erreur de connexion Ollama détectée pour la session ${sessionId}, arrêt automatique`)
            
            // Marquer la session comme erreur
            const sessionIndex = activeSessions.value.findIndex(s => s.session_id === sessionId)
            if (sessionIndex !== -1) {
              const session = activeSessions.value[sessionIndex]
              if (session) {
                session.status = 'error'
                session.error_count += 1
              }
            }
            
            // Arrêter le monitoring
            clearInterval(intervalId)
            progressMonitoring.delete(sessionId)
            
            // Nettoyer les textes en cours de traduction pour cette session
            // Récupérer les IDs des textes de cette session depuis le progress si disponible
            const progress = sessionProgress.value.get(sessionId)
            if (progress) {
              // Marquer les textes non traduits comme "NotTranslated" pour les remettre dans la queue
              // Note: On ne peut pas facilement savoir quels textes appartiennent à cette session exacte
              // donc on nettoie tous les textes en cours si la session échoue
            }
            
            // Nettoyer tous les textes en cours de traduction car la session a échoué
            textsBeingTranslated.value.clear()
        } else {
          console.warn(`⚠️ Failed to get progress for session ${sessionId}:`, result.error)
          }
        }
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Unknown error'
        
        // Si erreur de connexion réseau, arrêter automatiquement
        if (isConnectionError(errorMessage)) {
          console.error(`❌ Erreur de connexion détectée pour la session ${sessionId}, arrêt automatique`)
          
          // Marquer la session comme erreur
          const sessionIndex = activeSessions.value.findIndex(s => s.session_id === sessionId)
          if (sessionIndex !== -1) {
            const session = activeSessions.value[sessionIndex]
            if (session) {
              session.status = 'error'
              session.error_count += 1
            }
          }
          
          // Arrêter le monitoring
          clearInterval(intervalId)
          progressMonitoring.delete(sessionId)
          
          // Nettoyer les textes en cours de traduction car la connexion a échoué
          textsBeingTranslated.value.clear()
        } else {
        console.error(`Error monitoring session ${sessionId}:`, err)
        }
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
    source: 'manual' | 'ollama' | 'glossary' = 'manual',
    status: 'NotTranslated' | 'InProgress' | 'Translated' | 'Ignored' = 'Translated'
  ) => {
    return executeAsyncOperation(async () => {
      // 1. Mise à jour de la base de données
      const result = await updateTextWithTranslation(textId, translatedText, status)

      if (result.success) {
        // 2. Mettre à jour directement dans le store pour une réactivité immédiate (comme dans glossary)
        const projectsStore = useProjectsStore()
        const project = projectsStore.currentProject

        if (project) {
          const textIndex = project.extractedTexts.findIndex(t => parseInt(t.id, 10) === textId)
          if (textIndex !== -1) {
            const existingText = project.extractedTexts[textIndex]
            if (existingText) {
              // Mettre à jour le texte directement dans le tableau (comme glossary.updateEntry)
              // Préserver toutes les propriétés existantes et mettre à jour seulement ce qui change
              existingText.translated_text = translatedText
              existingText.status = status
              
              // Mettre à jour les statistiques
              project.translatedTexts = project.extractedTexts.filter(t => t.status === 'Translated').length
            } else {
              // Si le texte n'est pas trouvé, recharger depuis la DB (cas rare)
              await projectsStore.loadProjectTextsFromDB(project.id)
            }
          } else {
            // Si le texte n'est pas trouvé, recharger depuis la DB (cas rare)
            await projectsStore.loadProjectTextsFromDB(project.id)
          }
        } else {
          console.warn('⚠️ No current project found for UI update')
        }
      } else {
        throw new Error(result.error || 'Failed to apply translation')
      }
    }, 'Failed to apply translation', { isLoading, error }, { skipLoading: true })
  }

  // Action pour définir les textes sélectionnés pour retraduction
  const setSelectedTextsForRetranslation = (texts: Array<{ id: string; source_text: string; location?: string; prompt_type: string }>) => {
    selectedTextsForRetranslation.value = texts
  }

  // Action pour mettre un texte en statut InProgress
  const setTextInProgress = async (textId: number) => {
    const projectsStore = useProjectsStore()
    const project = projectsStore.currentProject
    if (!project) return

    // Mettre à jour directement dans le store (comme glossary)
    const textIndex = project.extractedTexts.findIndex(t => parseInt(t.id, 10) === textId)
    if (textIndex !== -1) {
      const existingText = project.extractedTexts[textIndex]
      if (existingText) {
        // Mettre à jour le texte directement dans le tableau (comme glossary.updateEntry)
        // Mettre à jour seulement le statut
        existingText.status = 'InProgress'
        textsBeingTranslated.value.add(textId)
        
        // Mettre à jour dans la DB
        await applyTranslation(textId, existingText.translated_text || '', 'manual', 'InProgress')
      }
    }
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
    }
    progressMonitoring.clear()
  }

  return {
    // State
    activeSessions,
    sessionProgress: computed(() => sessionProgress.value),
    textsBeingTranslated,
    selectedTextsForRetranslation,
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
    getPauseTimeRemaining,

    // Actions
    startTranslation,
    pauseSession,
    resumeSession,
    stopSession,
    loadProjectSessions,
    getSuggestions,
    applyTranslation,
    setSelectedTextsForRetranslation,
    setTextInProgress,
    clearError,
    cleanupCompletedSessions,
    cleanupMonitoringIntervals,
    updateDefaultSettings
  }
})
