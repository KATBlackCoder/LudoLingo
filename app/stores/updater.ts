// Updater store using Pinia
// Manages update state, checking, downloading, and installation

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useBaseStoreState } from '~/composables/stores/useBaseStore'
import { check } from '@tauri-apps/plugin-updater'
import type { Update } from '@tauri-apps/plugin-updater'

// Type helper for the return type of check()
type CheckResult = Awaited<ReturnType<typeof check>>

export const useUpdaterStore = defineStore('updater', () => {
  // Base store state (isLoading, error, clearError)
  const { isLoading, error, clearError } = useBaseStoreState()

  // State
  const availableUpdate = ref<CheckResult>(null)
  const isChecking = ref(false)
  const isDownloading = ref(false)
  const downloadProgress = ref(0)
  const downloadedUpdate = ref<Update | null>(null)

  // Getters
  const hasUpdate = computed(() => availableUpdate.value !== null)
  const canInstall = computed(() => downloadedUpdate.value !== null)
  const isReadyToInstall = computed(() => canInstall.value && !isDownloading.value)

  // Actions
  const setAvailableUpdate = (update: CheckResult) => {
    availableUpdate.value = update
  }

  const setDownloadedUpdate = (update: Update | null) => {
    downloadedUpdate.value = update
  }

  const setChecking = (checking: boolean) => {
    isChecking.value = checking
  }

  const setDownloading = (downloading: boolean) => {
    isDownloading.value = downloading
  }

  const setDownloadProgress = (progress: number) => {
    downloadProgress.value = Math.max(0, Math.min(100, progress))
  }

  const reset = () => {
    availableUpdate.value = null
    downloadedUpdate.value = null
    isChecking.value = false
    isDownloading.value = false
    downloadProgress.value = 0
    clearError()
  }

  return {
    // State
    availableUpdate,
    isChecking,
    isDownloading,
    downloadProgress,
    downloadedUpdate,
    isLoading,
    error,

    // Getters
    hasUpdate,
    canInstall,
    isReadyToInstall,

    // Actions
    setAvailableUpdate,
    setDownloadedUpdate,
    setChecking,
    setDownloading,
    setDownloadProgress,
    clearError,
    reset
  }
})

