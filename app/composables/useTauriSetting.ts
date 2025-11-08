// Tauri Settings composable
// Example of how to use the Tauri store composable for settings

import { useTauriStore } from '~/composables/useTauriProject'

export interface AppSettings {
  ollama: {
    mode: 'local' | 'online'
    endpoint: string
    port: number
    model: string
  }
  ui: {
    language: string
  }
}

/**
 * Composable pour gérer les paramètres de l'application
 * Utilise le Tauri store pour la persistance
 */
export function useSettings() {
  const tauriStore = useTauriStore({ storeName: 'settings.json' })

  const defaultSettings: AppSettings = {
    ollama: {
      mode: 'local',
      endpoint: 'http://localhost',
      port: 11434,
      model: 'llama2:13b'
    },
    ui: {
      language: 'fr'
    }
  }

  /**
   * Charger les paramètres sauvegardés
   */
  const loadSettings = async (): Promise<AppSettings> => {
    const savedSettings = await tauriStore.getItem<AppSettings>('settings')
    return { ...defaultSettings, ...savedSettings }
  }

  /**
   * Sauvegarder tous les paramètres
   */
  const saveSettings = async (settings: AppSettings): Promise<void> => {
    await tauriStore.setAndSave('settings', settings)
  }

  /**
   * Sauvegarder un paramètre spécifique
   */
  const saveSetting = async <K extends keyof AppSettings>(
    key: K,
    value: AppSettings[K]
  ): Promise<void> => {
    const currentSettings = await loadSettings()
    const newSettings = { ...currentSettings, [key]: value }
    await saveSettings(newSettings)
  }

  /**
   * Charger un paramètre spécifique
   */
  const getSetting = async <K extends keyof AppSettings>(
    key: K
  ): Promise<AppSettings[K]> => {
    const settings = await loadSettings()
    return settings[key]
  }

  /**
   * Réinitialiser tous les paramètres
   */
  const resetSettings = async (): Promise<void> => {
    await saveSettings(defaultSettings)
  }

  return {
    loadSettings,
    saveSettings,
    saveSetting,
    getSetting,
    resetSettings,
    defaultSettings
  }
}
