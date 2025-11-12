// Ollama store using Pinia
// Manages Ollama connection status, model listing, and connection testing

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { TextOperationResult } from '~/composables/db/texts/types'

export interface OllamaStatus {
  available: boolean
  models_available?: string[]
  version?: string
  error?: string
}

export const useOllamaStore = defineStore('ollama', () => {
  // State
  const status = ref<OllamaStatus | null>(null)
  const isCheckingConnection = ref(false)
  const availableModels = ref<string[]>([])
  const lastChecked = ref<Date | null>(null)

  // Getters
  const isConnected = computed(() => status.value?.available === true)
  const hasError = computed(() => !!status.value?.error)
  const errorMessage = computed(() => status.value?.error || '')
  const modelCount = computed(() => availableModels.value.length)

  // Private helper function for Ollama API calls
  const callOllamaStatus = async (
    host?: string,
    port?: number
  ): Promise<TextOperationResult<OllamaStatus>> => {
    try {
      const result = await invoke('check_ollama_status', {
        host,
        port
      }) as OllamaStatus

      return { success: true, data: result }
    } catch (error) {
      return {
        success: false,
        error: `Failed to check Ollama status: ${error instanceof Error ? error.message : 'Unknown error'}`
      }
    }
  }

  // Actions
  const checkStatus = async (
    host?: string,
    port?: number
  ): Promise<TextOperationResult<OllamaStatus>> => {
    return await callOllamaStatus(host, port)
  }

  const checkConnection = async (host?: string, port?: number): Promise<boolean> => {
    isCheckingConnection.value = true
    status.value = null

    try {
      const result = await callOllamaStatus(host, port)

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

        console.log('🔗 Ollama connection check:', status.value)
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
      console.error('❌ Ollama connection error:', error)
      return false
    } finally {
      isCheckingConnection.value = false
    }
  }

  const refreshModels = async (): Promise<string[]> => {
    if (!isConnected.value) {
      console.warn('⚠️ Cannot refresh models: Ollama not connected')
      return []
    }

    try {
      const result = await callOllamaStatus()

      if (result.success && result.data) {
        const data = result.data
        if (data.available && data.models_available) {
          availableModels.value = data.models_available
          console.log('📚 Models refreshed:', availableModels.value.length, 'models')
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
      await checkConnection()
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
