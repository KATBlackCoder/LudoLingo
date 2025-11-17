// Auto-update composable
// Handles automatic update checking based on user preferences

import { useUpdater } from './useUpdater'
import { useSettings } from '~/composables/useTauriSetting'

let autoCheckInterval: ReturnType<typeof setInterval> | null = null
let isInitialized = false

/**
 * Composable pour gérer la vérification automatique des mises à jour
 * Respecte les préférences utilisateur (fréquence, activation)
 */
export function useAutoUpdate() {
  const updater = useUpdater()
  const settings = useSettings()

  /**
   * Vérifier si une vérification est nécessaire selon la fréquence configurée
   */
  const shouldCheckForUpdates = async (): Promise<boolean> => {
    const currentSettings = await settings.loadSettings()
    
    // Si la vérification automatique est désactivée
    if (!currentSettings.updater.autoCheck) {
      return false
    }

    // Si la fréquence est manuelle, ne pas vérifier automatiquement
    if (currentSettings.updater.checkFrequency === 'manual') {
      return false
    }

    // Vérifier la dernière date de vérification
    const lastCheckDate = currentSettings.updater.lastCheckDate
    if (!lastCheckDate) {
      return true // Première vérification
    }

    const lastCheck = new Date(lastCheckDate)
    const now = new Date()
    const daysSinceLastCheck = Math.floor(
      (now.getTime() - lastCheck.getTime()) / (1000 * 60 * 60 * 24)
    )

    // Vérifier selon la fréquence
    switch (currentSettings.updater.checkFrequency) {
      case 'daily':
        return daysSinceLastCheck >= 1
      case 'weekly':
        return daysSinceLastCheck >= 7
      default:
        return false
    }
  }

  /**
   * Effectuer une vérification et mettre à jour la date de dernière vérification
   */
  const performAutoCheck = async (): Promise<void> => {
    try {
      await updater.checkForUpdates()
      
      // Mettre à jour la date de dernière vérification
      const currentSettings = await settings.loadSettings()
      await settings.saveSetting('updater', {
        ...currentSettings.updater,
        lastCheckDate: new Date().toISOString()
      })
    } catch (error) {
      // Erreurs silencieuses pour la vérification automatique
      console.debug('[AutoUpdate] Silent check failed:', error)
    }
  }

  /**
   * Initialiser la vérification automatique
   * Vérifie immédiatement si nécessaire, puis configure l'intervalle
   */
  const initializeAutoCheck = async (): Promise<void> => {
    if (isInitialized) {
      console.warn('[AutoUpdate] Already initialized')
      return
    }

    // Vérification initiale si nécessaire
    if (await shouldCheckForUpdates()) {
      await performAutoCheck()
    }

    // Configurer l'intervalle pour les vérifications périodiques
    // Vérifier toutes les heures si une vérification est nécessaire
    autoCheckInterval = setInterval(async () => {
      if (await shouldCheckForUpdates()) {
        await performAutoCheck()
      }
    }, 60 * 60 * 1000) // 1 heure

    isInitialized = true
    console.log('[AutoUpdate] Initialized')
  }

  /**
   * Arrêter la vérification automatique
   */
  const stopAutoCheck = (): void => {
    if (autoCheckInterval) {
      clearInterval(autoCheckInterval)
      autoCheckInterval = null
    }
    isInitialized = false
    console.log('[AutoUpdate] Stopped')
  }

  /**
   * Réinitialiser la vérification automatique (utile après changement de paramètres)
   */
  const restartAutoCheck = async (): Promise<void> => {
    stopAutoCheck()
    await initializeAutoCheck()
  }

  return {
    initializeAutoCheck,
    stopAutoCheck,
    restartAutoCheck,
    performAutoCheck,
    shouldCheckForUpdates
  }
}

