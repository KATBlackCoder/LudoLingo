// Store utilities composable
// Provides access to persistent store via tauri-plugin-store

import { Store } from '@tauri-apps/plugin-store'

// Global settings store
let settingsStore: Store | null = null

/**
 * Get or initialize global settings store (settings.json)
 */
export async function useSettingsStore(): Promise<Store> {
  if (!settingsStore) {
    settingsStore = await Store.load('settings.json')
  }
  return settingsStore
}

/**
 * Get project-specific store (ludolingo.json for all projects)
 */
export async function useProjectStore(): Promise<Store> {
  // Use a fixed filename for all projects to avoid conflicts
  const storeName = 'ludolingo.json'
  return await Store.load(storeName)
}

/**
 * Generic store operations
 */
export const useStore = () => {
  return {
    useSettingsStore,
    useProjectStore,
  }
}

