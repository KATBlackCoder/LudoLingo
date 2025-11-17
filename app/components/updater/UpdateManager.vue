<template>
  <div>
    <!-- Notification discrète -->
    <UpdateNotification
      v-if="hasUpdate"
      @show-dialog="showDialog = true"
    />

    <!-- Dialog de mise à jour -->
    <UpdateDialog
      v-if="availableUpdate"
      v-model:open="showDialog"
      :available-update="(availableUpdate as any)"
      :current-version="currentVersion"
      :is-checking="isChecking"
      :is-downloading="isDownloading"
      :is-ready-to-install="isReadyToInstall"
      :download-progress="downloadProgress"
      @download="handleDownload"
      @install="handleInstall"
      @cancel-download="handleCancelDownload"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useUpdater } from '~/composables/updater/useUpdater'
import { UpdateNotification, UpdateDialog } from './index'
import { getVersion } from '@tauri-apps/api/app'
import { platform } from '@tauri-apps/plugin-os'

const updater = useUpdater()
const showDialog = ref(false)

// Version actuelle de l'application
const currentVersion = ref('0.2.0')

// Charger la version au montage
onMounted(async () => {
  try {
    currentVersion.value = await getVersion()
  } catch (error) {
    console.warn('Failed to get app version:', error)
  }
})

// Computed properties depuis le composable (déjà réactifs)
const availableUpdate = updater.availableUpdate
const hasUpdate = updater.hasUpdate
const isChecking = updater.isChecking
const isDownloading = updater.isDownloading
const isReadyToInstall = updater.isReadyToInstall
const downloadProgress = updater.downloadProgress

// Ouvrir le dialog quand une mise à jour est disponible (si pas déjà ouvert)
watch(hasUpdate, (newValue) => {
  if (newValue && !showDialog.value) {
    // Ne pas ouvrir automatiquement, laisser l'utilisateur choisir via la notification
  }
})

// Gérer le téléchargement
const handleDownload = async () => {
  const update = updater.availableUpdate.value
  if (!update) return

  try {
    // Le type CheckResult retourné par check() a les méthodes download() et install()
    // TypeScript ne reconnaît pas la compatibilité à cause de la propriété privée #private
    // En runtime, cela fonctionne car les méthodes sont présentes
    await updater.downloadUpdate(update as any)
  } catch (error) {
    console.error('Error downloading update:', error)
    // L'erreur est déjà gérée dans le composable
  }
}

// Gérer l'installation
const handleInstall = async () => {
  const update = updater.downloadedUpdate.value
  if (!update) return

  try {
    // Afficher un message d'avertissement avant l'installation
    // Sur Windows, l'app se ferme automatiquement
    // Sur Linux, l'utilisateur devra redémarrer manuellement
    const currentPlatform = await platform()
    
    if (currentPlatform === 'win32') {
      // Sur Windows, l'app se ferme automatiquement lors de l'installation
      // On peut afficher un message informatif
      console.log('⚠️ L\'application va se fermer pour installer la mise à jour')
    } else {
      // Sur Linux/Mac, l'utilisateur devra redémarrer manuellement
      console.log('⚠️ Veuillez redémarrer l\'application après l\'installation')
    }

    // Même problème de typage que pour downloadUpdate
    await updater.installUpdate(update as any)
    // Note: Sur Windows, l'app se ferme automatiquement
  } catch (error) {
    console.error('Error installing update:', error)
    // L'erreur est déjà gérée dans le composable et affichée dans le store
  }
}

// Gérer l'annulation du téléchargement
const handleCancelDownload = () => {
  // Réinitialiser l'état (le téléchargement ne peut pas être annulé directement via l'API)
  // On peut juste fermer le dialog
  showDialog.value = false
}
</script>

