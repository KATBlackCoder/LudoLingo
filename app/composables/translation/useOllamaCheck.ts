/**
 * Composable for Ollama connection checking before translation
 * Extracts the Ollama verification logic from translation.vue
 */

import { useOllamaStore } from '~/stores/ollama'
import { useSettings } from '~/composables/useTauriSetting'
import { useNotifications } from '~/composables/useNotifications'

export function useOllamaCheck() {
  const ollamaStore = useOllamaStore()
  const settings = useSettings()
  const { notifyError, notifyWarning } = useNotifications()

  /**
   * Check Ollama connection before starting translation
   * Uses cached status if recent (less than 30 seconds old)
   * @returns true if Ollama is available and ready, false otherwise
   */
  const checkOllamaBeforeTranslation = async (): Promise<boolean> => {
    // Use cached status if recent (less than 30 seconds)
    const lastChecked = ollamaStore.lastChecked
    const shouldRecheck = !lastChecked || (Date.now() - lastChecked.getTime()) > 30000
    
    if (shouldRecheck) {
      const userSettings = await settings.loadSettings()
      const ollamaEndpoint = userSettings.ollama.endpoint
      const ollamaPort = userSettings.ollama.port
      
      // Extract host from endpoint (remove http:// or https://)
      const host = ollamaEndpoint.replace(/^https?:\/\//, '').split(':')[0]
      
      // Check with timeout (7 seconds - Rust timeout: 3s for test + 3s for list = max 6s)
      const checkPromise = ollamaStore.checkConnection(host, ollamaPort)
      const timeoutPromise = new Promise<boolean>((resolve) => {
        setTimeout(() => resolve(false), 7000)
      })
      
      const isConnected = await Promise.race([checkPromise, timeoutPromise])
      
      if (!isConnected) {
        // Check if it was a timeout or a real error
        if (!ollamaStore.isConnected && ollamaStore.errorMessage) {
          const errorMsg = ollamaStore.errorMessage
          notifyError(
            'Ollama indisponible',
            `Impossible de se connecter à Ollama. ${errorMsg}. Veuillez vérifier votre configuration dans les paramètres.`
          )
        } else {
          notifyError(
            'Ollama indisponible',
            'La vérification de connexion a pris trop de temps. Ollama ne répond pas. Veuillez vérifier votre configuration dans les paramètres.'
          )
        }
        return false
      }
    } else {
      // Use already checked status
      if (!ollamaStore.isConnected) {
        const errorMsg = ollamaStore.errorMessage || 'Ollama n\'est pas disponible'
        notifyError(
          'Ollama indisponible',
          `Ollama n'est pas disponible. ${errorMsg}. Veuillez vérifier votre configuration dans les paramètres.`
        )
        return false
      }
    }

    // Verify that the requested model is available
    const userSettings = await settings.loadSettings()
    if (userSettings.ollama.model && !ollamaStore.availableModels.includes(userSettings.ollama.model)) {
      notifyWarning(
        'Modèle non disponible',
        `Le modèle "${userSettings.ollama.model}" n'est pas disponible. Modèles disponibles: ${ollamaStore.availableModels.join(', ') || 'aucun'}`
      )
      return false
    }

    return true
  }

  return {
    checkOllamaBeforeTranslation
  }
}

