/**
 * Base store composable for common Pinia store patterns
 * Provides shared state and helpers for loading, error handling, and async operations
 */

import { ref, type Ref } from 'vue'

export interface BaseStoreState {
  isLoading: Ref<boolean>
  error: Ref<string | null>
}

export interface BaseStoreActions {
  clearError: () => void
}

/**
 * Creates base store state (isLoading, error)
 * @returns Base store state and actions
 */
export function useBaseStoreState(): BaseStoreState & BaseStoreActions {
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const clearError = () => {
    error.value = null
  }

  return {
    isLoading,
    error,
    clearError
  }
}

/**
 * Generic helper for async operations with loading and error handling
 * @param operation - The async operation to execute
 * @param errorMessage - Error message to use if operation fails
 * @param options - Options for controlling loading state
 * @returns Result of the operation
 */
export async function executeAsyncOperation<T>(
  operation: () => Promise<T>,
  errorMessage: string,
  state: BaseStoreState,
  options: { skipLoading?: boolean } = {}
): Promise<T> {
  try {
    if (!options.skipLoading) {
      state.isLoading.value = true
    }
    state.error.value = null

    return await operation()
  } catch (err) {
    const message = err instanceof Error ? err.message : errorMessage
    state.error.value = message
    console.error(`Error: ${errorMessage}`, err)
    throw err
  } finally {
    if (!options.skipLoading) {
      state.isLoading.value = false
    }
  }
}

