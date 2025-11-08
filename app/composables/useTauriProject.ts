// Tauri Project Store composable
// Fournit une API unifiée pour la persistance avec tauri-plugin-store

import { useStore } from '~/composables/useStore'

export interface StoreData {
  [key: string]: any
}

export interface TauriStoreOptions {
  storeName?: string
  serialize?: boolean
}

/**
 * Composable pour gérer la persistance avec Tauri Store
 * Fournit des méthodes sûres pour sauvegarder/charger des données
 */
export function useTauriStore(options: TauriStoreOptions = {}) {
  const { storeName = 'ludolingo.json' } = options

  let storeInstance: any = null

  /**
   * Obtenir ou initialiser l'instance du store Tauri
   */
  const getStore = async () => {
    if (!storeInstance) {
      const { useProjectStore } = useStore()
      storeInstance = await useProjectStore(storeName)
    }
    return storeInstance
  }

  /**
   * Sauvegarder une valeur dans le store
   * Gère automatiquement la suppression des clés null/undefined
   */
  const setItem = async <T>(key: string, value: T | null | undefined): Promise<void> => {
    const store = await getStore()

    if (value === null || value === undefined) {
      // Supprimer la clé si la valeur est null/undefined
      try {
        await store.delete(key)
      } catch {
        // Ignore si la clé n'existe pas
      }
    } else {
      // Sauvegarder la valeur
      await store.set(key, value)
    }
  }

  /**
   * Récupérer une valeur du store
   */
  const getItem = async <T>(key: string): Promise<T | null> => {
    try {
      const store = await getStore()
      const value = await store.get(key)
      return value ?? null
    } catch (error) {
      console.warn(`Erreur lors de la récupération de ${key}:`, error)
      return null
    }
  }

  /**
   * Vérifier si une clé existe dans le store
   */
  const hasItem = async (key: string): Promise<boolean> => {
    try {
      const store = await getStore()
      const value = await store.get(key)
      return value !== null && value !== undefined
    } catch {
      return false
    }
  }

  /**
   * Supprimer une clé du store
   */
  const removeItem = async (key: string): Promise<void> => {
    try {
      const store = await getStore()
      await store.delete(key)
    } catch {
      // Ignore si la clé n'existe pas
    }
  }

  /**
   * Sauvegarder plusieurs valeurs en une fois
   */
  const setItems = async (data: StoreData): Promise<void> => {
    const store = await getStore()

    for (const [key, value] of Object.entries(data)) {
      await setItem(key, value)
    }
  }

  /**
   * Charger plusieurs valeurs en une fois
   */
  const getItems = async (keys: string[]): Promise<StoreData> => {
    const result: StoreData = {}

    for (const key of keys) {
      result[key] = await getItem(key)
    }

    return result
  }

  /**
   * Sauvegarder et persister toutes les modifications
   */
  const save = async (): Promise<void> => {
    const store = await getStore()
    await store.save()
  }

  /**
   * Sauvegarder des données et les persister immédiatement
   */
  const setAndSave = async <T>(key: string, value: T | null | undefined): Promise<void> => {
    await setItem(key, value)
    await save()
  }

  /**
   * Sauvegarder plusieurs données et les persister immédiatement
   */
  const setItemsAndSave = async (data: StoreData): Promise<void> => {
    await setItems(data)
    await save()
  }

  /**
   * Charger des données avec gestion d'erreur
   */
  const loadData = async <T>(key: string, defaultValue: T): Promise<T> => {
    const value = await getItem<T>(key)
    return value ?? defaultValue
  }

  /**
   * Nettoyer le cache de l'instance store
   */
  const clearCache = (): void => {
    storeInstance = null
  }

  return {
    // Instance
    getStore,

    // Opérations de base
    setItem,
    getItem,
    hasItem,
    removeItem,

    // Opérations batch
    setItems,
    getItems,

    // Persistance
    save,
    setAndSave,
    setItemsAndSave,

    // Utilitaires
    loadData,
    clearCache
  }
}
