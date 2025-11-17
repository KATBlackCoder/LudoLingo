/**
 * Composable for RunPod connection checking before translation
 * Extracts the RunPod verification logic
 */

import { useRunPodStore } from '~/stores/runpod'
import { useSettings } from '~/composables/useTauriSetting'
import { useNotifications } from '~/composables/useNotifications'

export function useRunpodCheck() {
  const runpodStore = useRunPodStore()
  const settings = useSettings()
  const { notifyError, notifyWarning } = useNotifications()

  /**
   * Extract pod_id from URL or return as-is if it's already a pod_id
   */
  const extractPodId = (input: string): string => {
    if (!input || input.trim() === '') return ''
    const trimmed = input.trim()
    const urlMatch = trimmed.match(/https?:\/\/([a-z0-9]+)-11434\.proxy\.runpod\.net/)
    if (urlMatch && urlMatch[1]) {
      return urlMatch[1]
    }
    return trimmed
  }

  /**
   * Check RunPod connection before starting translation
   * Uses cached status if recent (less than 30 seconds old)
   * @returns true if RunPod is available and ready, false otherwise
   */
  const checkRunpodBeforeTranslation = async (): Promise<boolean> => {
    const userSettings = await settings.loadSettings()
    const podIdInput = userSettings.runpod.pod_id || ''
    const podId = extractPodId(podIdInput)
    
    if (!podId || podId.trim() === '') {
      notifyError(
        'RunPod non configuré',
        'Le POD_ID RunPod n\'est pas configuré. Veuillez configurer RunPod dans les paramètres.'
      )
      return false
    }

    // Use cached status if recent (less than 30 seconds)
    const lastChecked = runpodStore.lastChecked
    const shouldRecheck = !lastChecked || (Date.now() - lastChecked.getTime()) > 30000
    
    if (shouldRecheck) {
      // Check with timeout (7 seconds - Rust timeout: 5s for test + 5s for list = max 10s)
      const checkPromise = runpodStore.checkConnection(podId)
      const timeoutPromise = new Promise<boolean>((resolve) => {
        setTimeout(() => resolve(false), 12000)
      })
      
      const isConnected = await Promise.race([checkPromise, timeoutPromise])
      
      if (!isConnected) {
        // Check if it was a timeout or a real error
        if (!runpodStore.isConnected && runpodStore.errorMessage) {
          const errorMsg = runpodStore.errorMessage
          notifyError(
            'RunPod indisponible',
            `Impossible de se connecter à RunPod. ${errorMsg}. Veuillez vérifier votre configuration dans les paramètres.`
          )
        } else {
          notifyError(
            'RunPod indisponible',
            'La vérification de connexion a pris trop de temps. RunPod ne répond pas. Veuillez vérifier votre configuration dans les paramètres.'
          )
        }
        return false
      }
    } else {
      // Use already checked status
      if (!runpodStore.isConnected) {
        const errorMsg = runpodStore.errorMessage || 'RunPod n\'est pas disponible'
        notifyError(
          'RunPod indisponible',
          `RunPod n'est pas disponible. ${errorMsg}. Veuillez vérifier votre configuration dans les paramètres.`
        )
        return false
      }
    }

    return true
  }

  return {
    checkRunpodBeforeTranslation
  }
}

