import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification'

export interface NotificationOptions {
  title: string
  body: string
  icon?: string
}

/**
 * Composable pour gérer les notifications natives Tauri
 * Fournit une API unifiée pour envoyer des notifications système
 */
export function useNotifications() {

  /**
   * Vérifie si la permission de notification est accordée
   */
  const checkPermission = async (): Promise<boolean> => {
    try {
      return await isPermissionGranted()
    } catch (error) {
      console.warn('Erreur lors de la vérification des permissions de notification:', error)
      return false
    }
  }

  /**
   * Demande la permission de notification si nécessaire
   */
  const requestNotificationPermission = async (): Promise<boolean> => {
    try {
      const permission = await requestPermission()
      return permission === 'granted'
    } catch (error) {
      console.warn('Erreur lors de la demande de permission de notification:', error)
      return false
    }
  }

  /**
   * Envoie une notification système
   */
  const notify = async (options: NotificationOptions): Promise<void> => {
    try {
      let permissionGranted = await checkPermission()

      if (!permissionGranted) {
        permissionGranted = await requestNotificationPermission()
      }

      if (permissionGranted) {
        await sendNotification({
          title: options.title,
          body: options.body,
          icon: options.icon || 'icon.png'
        })
      } else {
        // Fallback vers console.log si pas de permission
        console.log(`🔔 ${options.title}: ${options.body}`)
      }
    } catch (error) {
      console.warn('Erreur lors de l\'envoi de notification:', error)
      // Fallback toujours disponible
      console.log(`🔔 ${options.title}: ${options.body}`)
    }
  }

  /**
   * Notifications prédéfinies pour les cas courants
   */
  const notifySuccess = async (message: string, title = 'LudoLingo') => {
    await notify({
      title,
      body: message
    })
  }

  const notifyError = async (message: string, title = 'LudoLingo - Erreur') => {
    await notify({
      title,
      body: message
    })
  }

  const notifyInfo = async (message: string, title = 'LudoLingo') => {
    await notify({
      title,
      body: message
    })
  }

  const notifyWarning = async (message: string, title = 'LudoLingo - Attention') => {
    await notify({
      title,
      body: message
    })
  }

  return {
    // Méthodes de base
    checkPermission,
    requestNotificationPermission,
    notify,

    // Méthodes prédéfinies
    notifySuccess,
    notifyError,
    notifyInfo,
    notifyWarning
  }
}
