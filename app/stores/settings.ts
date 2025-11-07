// Application settings store
// Using Pinia setup store style

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

import type { SupportedLanguage } from '~/i18n/locales'

type OllamaMode = 'local' | 'online'
export type SupportedLocale = SupportedLanguage

interface Settings {
  ollama: {
    mode: OllamaMode
    endpoint: string
    port: number
    model: string
  }
  ui: {
    theme: 'light' | 'dark' | 'auto'
    language: SupportedLocale
  }
}

const defaultSettings: Settings = {
  ollama: {
    mode: 'local',
    endpoint: 'http://localhost',
    port: 11434,
    model: 'llama2:13b',
  },
  ui: {
    theme: 'auto',
    language: 'fr',
  },
}

export const useSettingsStore = defineStore('settings', () => {
  // State
  const settings = ref<Settings>({ ...defaultSettings })
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const ollamaUrl = computed(() => {
    const { mode, endpoint, port } = settings.value.ollama
    if (mode === 'local') {
      return `${endpoint}:${port}`
    }
    return endpoint
  })

  // Actions
  async function loadSettings() {
    loading.value = true
    error.value = null
    try {
      // TODO: Implement store loading via tauri-plugin-store
      // const store = await useStore()
      // const savedSettings = await store.get('settings')
      // if (savedSettings) {
      //   settings.value = savedSettings
      // }
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to load settings'
    } finally {
      loading.value = false
    }
  }

  async function saveSettings() {
    loading.value = true
    error.value = null
    try {
      // TODO: Implement store saving via tauri-plugin-store
      // const store = await useStore()
      // await store.set('settings', settings.value)
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to save settings'
    } finally {
      loading.value = false
    }
  }

  function updateOllamaMode(mode: OllamaMode) {
    settings.value.ollama.mode = mode
  }

  function updateOllamaEndpoint(endpoint: string) {
    settings.value.ollama.endpoint = endpoint
  }

  function updateOllamaPort(port: number) {
    settings.value.ollama.port = port
  }

  function updateOllamaModel(model: string) {
    settings.value.ollama.model = model
  }

  function updateTheme(theme: 'light' | 'dark' | 'auto') {
    settings.value.ui.theme = theme
  }

  async function updateLanguage(language: SupportedLocale) {
    settings.value.ui.language = language
    // La synchronisation avec l'UI se fait automatiquement via app.vue
  }

  function resetToDefaults() {
    settings.value = { ...defaultSettings }
  }

  return {
    // State
    settings,
    loading,
    error,
    
    // Getters
    ollamaUrl,
    
    // Actions
    loadSettings,
    saveSettings,
    updateOllamaMode,
    updateOllamaEndpoint,
    updateOllamaPort,
    updateOllamaModel,
    updateTheme,
    updateLanguage,
    resetToDefaults,
  }
})

