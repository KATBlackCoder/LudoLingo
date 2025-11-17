// Updater composable
// Provides methods to check, download, and install updates

import { check } from '@tauri-apps/plugin-updater'
import type { Update, DownloadEvent } from '@tauri-apps/plugin-updater'
import { useUpdaterStore } from '~/stores/updater'
import { storeToRefs } from 'pinia'

export interface UpdateCheckOptions {
  proxy?: string
  timeout?: number
  headers?: Record<string, string>
}

// Type helper for the return type of check()
type CheckResult = Awaited<ReturnType<typeof check>>

/**
 * Composable pour gérer les mises à jour de l'application
 * Utilise le store Pinia pour gérer l'état
 */
export function useUpdater() {
  const store = useUpdaterStore()
  const { error: storeError, downloadProgress: downloadProgressRef } = storeToRefs(store)

  // Garder une référence à l'objet Update original (ne pas le stocker dans Pinia)
  // Pinia sérialise les objets et fait perdre le contexte de classe
  const currentUpdateRef = ref<CheckResult | null>(null)

  /**
   * Vérifier les mises à jour disponibles
   */
  const checkForUpdates = async (options?: UpdateCheckOptions): Promise<CheckResult> => {
    if (store.isChecking) {
      console.warn('⚠️ Update check already in progress')
      return currentUpdateRef.value
    }

    store.setChecking(true)
    store.clearError()

    try {
      const update = await check(options || {})

      if (update) {
        // Stocker l'objet original dans le composable (pas dans Pinia)
        currentUpdateRef.value = update
        // Stocker seulement les métadonnées dans le store pour l'affichage
        store.setAvailableUpdate(update)
        console.log('✅ Update available:', update.version)
      } else {
        currentUpdateRef.value = null
        store.setAvailableUpdate(null)
        console.log('ℹ️ No update available')
      }

      return update
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to check for updates'
      storeError.value = message
      console.error('❌ Error checking for updates:', err)
      // Don't throw - silently handle errors to not disturb user
      return null
    } finally {
      store.setChecking(false)
    }
  }

  /**
   * Télécharger la mise à jour avec progression
   * Utilise l'objet Update stocké dans le composable (pas dans Pinia)
   */
  const downloadUpdate = async (update?: CheckResult | Update): Promise<void> => {
    if (store.isDownloading) {
      console.warn('⚠️ Download already in progress')
      return
    }

    // Utiliser l'objet passé en paramètre ou celui stocké dans le composable
    const updateObj = (update || currentUpdateRef.value) as any
    if (!updateObj) {
      throw new Error('No update available to download')
    }

    store.setDownloading(true)
    store.setDownloadProgress(0)
    store.clearError()

    try {
      // Utiliser l'objet directement depuis le composable (conserve le contexte de classe)
      await updateObj.download((event: DownloadEvent) => {
        switch (event.event) {
          case 'Started':
            store.setDownloadProgress(0)
            console.log('📥 Download started')
            break
          case 'Progress':
            // Track progress using chunk length (contentLength not always available)
            if (event.data.chunkLength) {
              // Increment progress (simplified - actual progress tracking would need total size)
              const currentProgress = downloadProgressRef.value
              store.setDownloadProgress(Math.min(95, currentProgress + 1))
            }
            break
          case 'Finished':
            store.setDownloadProgress(100)
            // Stocker l'objet Update après téléchargement dans le composable
            currentUpdateRef.value = updateObj
            store.setDownloadedUpdate(updateObj as Update)
            console.log('✅ Download finished')
            break
        }
      })

      // Stocker l'objet Update après téléchargement
      currentUpdateRef.value = updateObj
      store.setDownloadedUpdate(updateObj as Update)
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to download update'
      storeError.value = message
      console.error('❌ Error downloading update:', err)
      throw err
    } finally {
      store.setDownloading(false)
    }
  }

  /**
   * Installer la mise à jour téléchargée
   * Utilise l'objet Update stocké dans le composable (pas dans Pinia)
   */
  const installUpdate = async (update?: CheckResult | Update): Promise<void> => {
    store.clearError()

    // Utiliser l'objet passé en paramètre ou celui stocké dans le composable (priorité au composable)
    // Ne pas utiliser store.downloadedUpdate car il peut avoir perdu le contexte de classe
    const updateObj = (update || currentUpdateRef.value) as any
    if (!updateObj) {
      throw new Error('No update available to install')
    }

    try {
      // Utiliser l'objet directement depuis le composable (conserve le contexte de classe)
      await updateObj.install()
      console.log('✅ Update installation started')
      // Note: On Windows, the app will automatically exit
      // On Linux, the user may need to manually restart
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to install update'
      storeError.value = message
      console.error('❌ Error installing update:', err)
      throw err
    }
  }

  /**
   * Télécharger et installer en une seule opération
   * Utilise l'objet Update stocké dans le composable (pas dans Pinia)
   */
  const downloadAndInstallUpdate = async (update?: CheckResult | Update): Promise<void> => {
    if (store.isDownloading) {
      console.warn('⚠️ Download already in progress')
      return
    }

    // Utiliser l'objet passé en paramètre ou celui stocké dans le composable
    const updateObj = (update || currentUpdateRef.value) as any
    if (!updateObj) {
      throw new Error('No update available to download and install')
    }

    store.setDownloading(true)
    store.setDownloadProgress(0)
    store.clearError()

    try {
      // Utiliser l'objet directement depuis le composable (conserve le contexte de classe)
      await updateObj.downloadAndInstall((event: DownloadEvent) => {
        switch (event.event) {
          case 'Started':
            store.setDownloadProgress(0)
            console.log('📥 Download and install started')
            break
          case 'Progress':
            // Track progress using chunk length
            if (event.data.chunkLength) {
              const currentProgress = downloadProgressRef.value
              store.setDownloadProgress(Math.min(95, currentProgress + 1))
            }
            break
          case 'Finished':
            store.setDownloadProgress(100)
            console.log('✅ Download finished, installation will start')
            break
        }
      })

      console.log('✅ Update download and install completed')
      // Note: On Windows, the app will automatically exit
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to download and install update'
      storeError.value = message
      console.error('❌ Error downloading and installing update:', err)
      throw err
    } finally {
      store.setDownloading(false)
    }
  }

  return {
    // Methods
    checkForUpdates,
    downloadUpdate,
    installUpdate,
    downloadAndInstallUpdate,

    // Store state (reactive)
    availableUpdate: computed(() => store.availableUpdate),
    isChecking: computed(() => store.isChecking),
    isDownloading: computed(() => store.isDownloading),
    downloadProgress: computed(() => store.downloadProgress),
    downloadedUpdate: computed(() => store.downloadedUpdate),
    hasUpdate: computed(() => store.hasUpdate),
    canInstall: computed(() => store.canInstall),
    isReadyToInstall: computed(() => store.isReadyToInstall),
    error: computed(() => store.error),
    isLoading: computed(() => store.isLoading),

    // Actions
    clearError: store.clearError,
    reset: store.reset
  }
}

