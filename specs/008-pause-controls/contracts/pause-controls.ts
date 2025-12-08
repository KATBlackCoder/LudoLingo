// Contrats TypeScript pour les contrôles de pause configurables
// Spécification 008: Contrôles de Pause Configurables

/**
 * ARCHITECTURE: SÉPARATION BACKEND/FRONTEND
 *
 * ⚠️  IMPORTANT: Les vraies pauses matérielles sont gérées côté BACKEND (Rust)
 *    pour garantir la protection contre la surchauffe même en cas de crash frontend.
 *
 * 🔹 BACKEND (Rust/Tauri):
 *    - Gère les pauses physiques avec tokio::time::sleep()
 *    - Configure batch_size et pause_duration_minutes
 *    - Fournit pause_time_remaining pour l'affichage
 *
 * 🔹 FRONTEND (Vue/TypeScript):
 *    - Affiche les contrôles de configuration utilisateur
 *    - Sauvegarde les paramètres utilisateur
 *    - Affiche le compteur (lecture seule) pendant les pauses
 *
 * Cette séparation garantit robustesse et sécurité matérielle.
 */

/**
 * Configuration des pauses automatiques pour les traductions
 *
 * Ces paramètres sont utilisés côté BACKEND pour contrôler les vraies pauses matérielles.
 * Ils étendent la structure SequentialSession commune et utilisent le batch_counter existant.
 */
export interface PauseSettings {
  /** Active ou désactive les pauses automatiques */
  enabled: boolean
  /** Nombre de traductions avant de déclencher une pause */
  batchSize: number
  /** Durée de la pause en minutes */
  pauseDurationMinutes: number
}

/**
 * Extension des paramètres utilisateur pour inclure la configuration des pauses
 */
export interface AppSettings {
  provider: 'ollama' | 'runpod'
  ollama: {
    endpoint: string
    port: number
    model: string
  }
  runpod: {
    pod_id: string
    model: string
  }
  translation: {
    sourceLanguage: string
    targetLanguage: string
    pause: PauseSettings
  }
  updater: {
    autoCheck: boolean
    checkFrequency: 'daily' | 'weekly' | 'manual'
    lastCheckDate?: string
  }
}

/**
 * Requête de traduction séquentielle étendue avec paramètres de pause
 */
export interface SequentialTranslationRequest {
  project_id: number
  texts: TranslationText[]
  start_from?: number
  source_language?: string
  target_language?: string
  model?: string
  /** Paramètres de pause optionnels */
  pause_settings?: PauseSettings
}

/**
 * Texte de traduction (structure existante)
 */
export interface TranslationText {
  id: number
  source_text: string
  context?: string
  text_type?: string
}

/**
 * Progrès de traduction étendu avec compteur de pause
 */
export interface SequentialProgress {
  session_id: string
  current_entry?: number
  processed_count: number
  total_count: number
  status: 'idle' | 'running' | 'paused' | 'completed' | 'error'
  estimated_time_remaining?: number
  errors: SequentialError[]
  successful_translations: SuccessfulTranslation[]
  /** Temps restant en pause en secondes (optionnel) */
  pause_time_remaining?: number
}

/**
 * Erreur de traduction séquentielle (structure existante)
 */
export interface SequentialError {
  entry_id: number
  error_message: string
  timestamp: number
}

/**
 * Traduction réussie (structure existante)
 */
export interface SuccessfulTranslation {
  entry_id: number
  translated_text: string
  model_used: string
  timestamp: number
  processing_time_ms: number
}

/**
 * État du compteur de pause dans la page translation (LECTURE SEULE)
 *
 * ATTENTION: Ce compteur est purement informatif et affiché dans app/pages/translation.vue.
 * Les vraies pauses sont gérées côté BACKEND (Rust) pour la sécurité matérielle.
 * Le composant PauseControls gère seulement la configuration, pas l'affichage du compteur.
 */
export interface PauseCountdownState {
  /** Le compteur est-il actif (visible) - basé sur l'état backend */
  isActive: boolean
  /** Temps restant en secondes - fourni par le backend */
  timeRemaining: number
  /** ID de la session en pause - fourni par le backend */
  sessionId: string | null
}

/**
 * Props pour le composant TranslationControls
 */
export interface TranslationControlsProps {
  /** Nombre de textes sélectionnés pour retraduction */
  selectedTextsCount?: number
}

/**
 * Événements émis par TranslationControls
 */
export interface TranslationControlsEmits {
  /** Démarrage de la traduction */
  startTranslation: [request: StartTranslationRequest]
  /** Arrêt de toutes les traductions */
  stopAllTranslations: []
  /** Retraduction des textes sélectionnés */
  retransalateSelected: [texts: TranslationText[]]
}

/**
 * Requête de démarrage de traduction (structure existante étendue)
 */
export interface StartTranslationRequest {
  projectId: number
  texts: TranslationText[]
  sourceLanguage: string
  targetLanguage: string
  model?: string
  /** Paramètres de pause pour cette session */
  pauseSettings?: PauseSettings
}

/**
 * Valeurs par défaut pour les paramètres de pause
 */
export const DEFAULT_PAUSE_SETTINGS: PauseSettings = {
  enabled: true,
  batchSize: 150,
  pauseDurationMinutes: 5
} as const

/**
 * Limites de validation pour les paramètres de pause
 */
export const PAUSE_SETTINGS_LIMITS = {
  batchSize: {
    min: 1,
    max: 1000,
    default: 150
  },
  pauseDurationMinutes: {
    min: 1,
    max: 60,
    default: 5
  }
} as const

/**
 * Fonctions utilitaires pour la validation des paramètres de pause
 */
export const PauseSettingsValidators = {
  /**
   * Valide le nombre de traductions avant pause
   */
  validateBatchSize: (value: number): boolean => {
    return Number.isInteger(value) &&
           value >= PAUSE_SETTINGS_LIMITS.batchSize.min &&
           value <= PAUSE_SETTINGS_LIMITS.batchSize.max
  },

  /**
   * Valide la durée de pause en minutes
   */
  validatePauseDuration: (value: number): boolean => {
    return Number.isInteger(value) &&
           value >= PAUSE_SETTINGS_LIMITS.pauseDurationMinutes.min &&
           value <= PAUSE_SETTINGS_LIMITS.pauseDurationMinutes.max
  },

  /**
   * Corrige automatiquement une valeur invalide
   */
  sanitizeBatchSize: (value: number): number => {
    if (!Number.isInteger(value)) return PAUSE_SETTINGS_LIMITS.batchSize.default
    return Math.max(
      PAUSE_SETTINGS_LIMITS.batchSize.min,
      Math.min(PAUSE_SETTINGS_LIMITS.batchSize.max, value)
    )
  },

  /**
   * Corrige automatiquement la durée de pause
   */
  sanitizePauseDuration: (value: number): number => {
    if (!Number.isInteger(value)) return PAUSE_SETTINGS_LIMITS.pauseDurationMinutes.default
    return Math.max(
      PAUSE_SETTINGS_LIMITS.pauseDurationMinutes.min,
      Math.min(PAUSE_SETTINGS_LIMITS.pauseDurationMinutes.max, value)
    )
  }
}

/**
 * Fonctions utilitaires pour le formatage du compteur
 */
export const PauseCountdownFormatters = {
  /**
   * Formate le temps restant en MM:SS
   */
  formatTimeRemaining: (seconds: number): string => {
    if (seconds < 0) return '00:00'
    const minutes = Math.floor(seconds / 60)
    const remainingSeconds = seconds % 60
    return `${minutes.toString().padStart(2, '0')}:${remainingSeconds.toString().padStart(2, '0')}`
  },

  /**
   * Parse un temps formaté MM:SS en secondes
   */
  parseTimeString: (timeString: string): number => {
    const [minutes, seconds] = timeString.split(':').map(Number)
    return (minutes || 0) * 60 + (seconds || 0)
  }
}
