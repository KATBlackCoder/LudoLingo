// RunPod store using Pinia
// Manages RunPod connection status and connection testing

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { TextOperationResult } from '~/composables/db/texts/types'

export interface RunPodStatus {
  available: boolean
  models_available?: string[]
  error?: string
}

export const useRunPodStore = defineStore('runpod', () => {
  // State
  const status = ref<RunPodStatus | null>(null)
  const isCheckingConnection = ref(false)
  const availableModels = ref<string[]>([])
  const lastChecked = ref<Date | null>(null)

  // Getters
  const isConnected = computed(() => status.value?.available === true)
  const hasError = computed(() => !!status.value?.error)
  const errorMessage = computed(() => status.value?.error || '')
  const modelCount = computed(() => availableModels.value.length)

  // Private helper function for RunPod API calls
  const callRunPodStatus = async (
    pod_id: string
  ): Promise<TextOperationResult<RunPodStatus>> => {
    try {
      console.log('🔍 [TS] Calling check_runpod_status with pod_id:', pod_id)
      
      if (!pod_id || pod_id.trim() === '') {
        return {
          success: false,
          error: 'POD_ID is required'
        }
      }
      
      const result = await invoke('check_runpod_status', {
        podId: pod_id.trim()
      }) as RunPodStatus

      console.log('✅ [TS] check_runpod_status result:', result)
      return { success: true, data: result }
    } catch (error) {
      console.error('❌ [TS] check_runpod_status error:', error)
      return {
        success: false,
        error: `Failed to check RunPod status: ${error instanceof Error ? error.message : String(error)}`
      }
    }
  }

  // Actions
  const checkStatus = async (
    pod_id: string
  ): Promise<TextOperationResult<RunPodStatus>> => {
    return await callRunPodStatus(pod_id)
  }

  const checkConnection = async (pod_id: string): Promise<boolean> => {
    // Prevent multiple simultaneous checks
    if (isCheckingConnection.value) {
      console.warn('⚠️ RunPod check already in progress, skipping duplicate request')
      return isConnected.value
    }

    isCheckingConnection.value = true
    status.value = null

    try {
      // If pod_id not provided, load from settings
      let checkPodId = pod_id
      
      if (!checkPodId || checkPodId.trim() === '') {
        const { useSettings } = await import('~/composables/useTauriSetting')
        const settings = useSettings()
        const userSettings = await settings.loadSettings()
        checkPodId = userSettings.runpod.pod_id
      }
      
      // Extract pod_id from URL if needed
      const extractPodId = (input: string): string => {
        if (!input || input.trim() === '') return ''
        const trimmed = input.trim()
        const urlMatch = trimmed.match(/https?:\/\/([a-z0-9]+)-11434\.proxy\.runpod\.net/)
        if (urlMatch && urlMatch[1]) {
          return urlMatch[1]
        }
        return trimmed
      }
      
      const extractedPodId = extractPodId(checkPodId)
      
      if (!extractedPodId || extractedPodId.trim() === '') {
        status.value = {
          available: false,
          error: 'POD_ID is required'
        }
        lastChecked.value = new Date()
        return false
      }
      
      const result = await callRunPodStatus(extractedPodId)

      if (result.success && result.data) {
        const data = result.data
        status.value = {
          available: data.available,
          models_available: data.models_available || [],
          error: data.error
        }

        // Update available models if connection successful
        if (data.available && data.models_available) {
          availableModels.value = data.models_available
        }

        lastChecked.value = new Date()

        return data.available
      } else {
        status.value = {
          available: false,
          error: result.error || 'Unknown error occurred'
        }
        lastChecked.value = new Date()
        return false
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Connection failed'
      status.value = {
        available: false,
        error: errorMessage
      }
      lastChecked.value = new Date()
      console.error('❌ RunPod connection error:', error)
      return false
    } finally {
      // Always reset checking state, even on error
      isCheckingConnection.value = false
    }
  }

  const refreshModels = async (pod_id?: string): Promise<string[]> => {
    // If not connected, try to connect first
    if (!isConnected.value) {
      let checkPodId = pod_id
      if (!checkPodId) {
        const { useSettings } = await import('~/composables/useTauriSetting')
        const settings = useSettings()
        const userSettings = await settings.loadSettings()
        checkPodId = userSettings.runpod.pod_id
      }
      
      const extractPodId = (input: string): string => {
        if (!input || input.trim() === '') return ''
        const trimmed = input.trim()
        const urlMatch = trimmed.match(/https?:\/\/([a-z0-9]+)-11434\.proxy\.runpod\.net/)
        if (urlMatch && urlMatch[1]) {
          return urlMatch[1]
        }
        return trimmed
      }
      
      const extractedPodId = extractPodId(checkPodId || '')
      if (extractedPodId) {
        await checkConnection(extractedPodId)
      }
    }

    // If still not connected after attempt, return empty
    if (!isConnected.value) {
      console.warn('⚠️ Cannot refresh models: RunPod not connected')
      return []
    }

    try {
      let checkPodId = pod_id
      if (!checkPodId) {
        const { useSettings } = await import('~/composables/useTauriSetting')
        const settings = useSettings()
        const userSettings = await settings.loadSettings()
        checkPodId = userSettings.runpod.pod_id
      }
      
      const extractPodId = (input: string): string => {
        if (!input || input.trim() === '') return ''
        const trimmed = input.trim()
        const urlMatch = trimmed.match(/https?:\/\/([a-z0-9]+)-11434\.proxy\.runpod\.net/)
        if (urlMatch && urlMatch[1]) {
          return urlMatch[1]
        }
        return trimmed
      }
      
      const extractedPodId = extractPodId(checkPodId || '')
      const result = await callRunPodStatus(extractedPodId)

      if (result.success && result.data) {
        const data = result.data
        if (data.available && data.models_available) {
          availableModels.value = data.models_available
          return availableModels.value
        }
      }
    } catch (error) {
      console.error('❌ Failed to refresh models:', error)
    }

    return availableModels.value
  }

  const clearStatus = () => {
    status.value = null
    lastChecked.value = null
  }

  const clearError = () => {
    if (status.value) {
      status.value.error = undefined
    }
  }

  // Auto-check connection on store initialization (optional)
  const initialize = async () => {
    // Only check if not recently checked (within last 30 seconds)
    if (!lastChecked.value || (Date.now() - lastChecked.value.getTime()) > 30000) {
      await checkConnection('')
    }
  }

  return {
    // State
    status,
    isCheckingConnection,
    availableModels,
    lastChecked,

    // Getters
    isConnected,
    hasError,
    errorMessage,
    modelCount,

    // Actions
    checkStatus,
    checkConnection,
    refreshModels,
    clearStatus,
    clearError,
    initialize
  }
})

