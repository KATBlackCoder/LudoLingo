// Glossary store using Pinia
// Manages glossary state, entries, and filters

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useBaseStoreState, executeAsyncOperation } from '~/composables/stores/useBaseStore'
import {
  createGlossaryEntry,
  getGlossaryEntries,
  getGlossaryEntry,
  updateGlossaryEntry,
  deleteGlossaryEntry,
  getGlossaryStats,
  type GlossaryEntry,
  type CreateGlossaryEntry,
  type UpdateGlossaryEntry,
  type GlossaryFilters
} from '~/composables/db/glossary'

export const useGlossaryStore = defineStore('glossary', () => {
  // Base store state (isLoading, error, clearError)
  const { isLoading, error, clearError } = useBaseStoreState()

  // State
  const entries = ref<GlossaryEntry[]>([])
  const filters = ref<GlossaryFilters>({
    category: undefined,
    source_language: undefined,
    target_language: undefined,
    search: undefined,
    project_id: undefined,
    limit: undefined,
    offset: undefined
  })
  const stats = ref<{
    total_entries: number
    categories_breakdown: Record<string, number>
    language_pairs: Array<{ source_language: string; target_language: string; count: number }>
  } | null>(null)

  // Getters
  const filteredEntries = computed(() => {
    let result = entries.value

    // Filter by category
    if (filters.value.category && filters.value.category.length > 0) {
      result = result.filter(entry => filters.value.category!.includes(entry.category))
    }

    // Filter by source language
    if (filters.value.source_language) {
      result = result.filter(entry => entry.source_language === filters.value.source_language)
    }

    // Filter by target language
    if (filters.value.target_language) {
      result = result.filter(entry => entry.target_language === filters.value.target_language)
    }

    // Filter by project_id (client-side filtering as backup, but DB filtering is primary)
    if (filters.value.project_id !== undefined) {
      if (filters.value.project_id === 'global' || filters.value.project_id === null) {
        // Only global terms
        result = result.filter(entry => entry.project_id === null)
      } else if (typeof filters.value.project_id === 'number') {
        // Project-specific: include global + project-specific
        result = result.filter(entry => entry.project_id === null || entry.project_id === filters.value.project_id)
      }
      // 'current' is handled by converting to project ID in GlossaryFilters component
    }

    // Filter by search term
    if (filters.value.search) {
      const searchLower = filters.value.search.toLowerCase()
      result = result.filter(entry =>
        entry.source_term.toLowerCase().includes(searchLower) ||
        entry.translated_term.toLowerCase().includes(searchLower)
      )
    }

    return result
  })

  const totalEntries = computed(() => entries.value.length)
  const filteredCount = computed(() => filteredEntries.value.length)

  const categories = computed(() => {
    const cats = new Set(entries.value.map(e => e.category))
    return Array.from(cats).sort()
  })

  const languagePairs = computed(() => {
    const pairs = new Set(
      entries.value.map(e => `${e.source_language}-${e.target_language}`)
    )
    return Array.from(pairs).map(pair => {
      const [source, target] = pair.split('-')
      return { source_language: source, target_language: target }
    })
  })

  // Actions

  /**
   * Load glossary entries from database
   * If customFilters are provided, they are passed to the DB query (for project_id filtering)
   * Otherwise, uses current filters from store
   * Client-side filtering is also applied via filteredEntries computed for other filters
   */
  const loadEntries = async (customFilters?: GlossaryFilters) => {
    return executeAsyncOperation(async () => {
      // Use custom filters if provided, otherwise use store filters
      const filtersToUse = customFilters || filters.value
      
      // Pass filters to DB query (especially for project_id which should be filtered server-side)
      const result = await getGlossaryEntries(filtersToUse)

      if (!result.success || !result.data) {
        throw new Error(result.error || 'Failed to load glossary entries')
      }

      entries.value = result.data.entries
      return result.data
    }, 'loading glossary entries', { isLoading, error })
  }

  /**
   * Create a new glossary entry
   */
  const createEntry = async (data: CreateGlossaryEntry) => {
    return executeAsyncOperation(async () => {
      const result = await createGlossaryEntry(data)

      if (!result.success || !result.data) {
        throw new Error(result.error || 'Failed to create glossary entry')
      }

      // Add to local state
      entries.value.push(result.data)
      // Reload stats
      await loadStats()
      return result.data
    }, 'creating glossary entry', { isLoading, error })
  }

  /**
   * Update an existing glossary entry
   */
  const updateEntry = async (entryId: number, updates: Partial<UpdateGlossaryEntry>) => {
    return executeAsyncOperation(async () => {
      const result = await updateGlossaryEntry(entryId, updates)

      if (!result.success || !result.data) {
        throw new Error(result.error || 'Failed to update glossary entry')
      }

      // Update local state
      const index = entries.value.findIndex(e => e.id === entryId)
      if (index !== -1) {
        entries.value[index] = result.data
      }
      // Reload stats
      await loadStats()
      return result.data
    }, 'updating glossary entry', { isLoading, error })
  }

  /**
   * Delete a glossary entry
   */
  const deleteEntry = async (entryId: number) => {
    return executeAsyncOperation(async () => {
      const result = await deleteGlossaryEntry(entryId)

      if (!result.success) {
        throw new Error(result.error || 'Failed to delete glossary entry')
      }

      // Remove from local state
      entries.value = entries.value.filter(e => e.id !== entryId)
      // Reload stats
      await loadStats()
      return undefined
    }, 'deleting glossary entry', { isLoading, error })
  }

  /**
   * Get a single glossary entry by ID
   */
  const getEntry = async (entryId: number) => {
    return executeAsyncOperation(async () => {
      const result = await getGlossaryEntry(entryId)

      if (!result.success || !result.data) {
        throw new Error(result.error || 'Failed to get glossary entry')
      }

      return result.data
    }, 'getting glossary entry', { isLoading, error })
  }

  /**
   * Load glossary statistics
   */
  const loadStats = async () => {
    return executeAsyncOperation(async () => {
      const result = await getGlossaryStats()

      if (!result.success || !result.data) {
        throw new Error(result.error || 'Failed to load glossary statistics')
      }

      stats.value = result.data
      return result.data
    }, 'loading glossary statistics', { isLoading, error }, { skipLoading: true })
  }

  /**
   * Update filters
   */
  const setFilters = (newFilters: Partial<GlossaryFilters>) => {
    filters.value = {
      ...filters.value,
      ...newFilters
    }
  }

  /**
   * Clear all filters
   */
  const clearFilters = () => {
    filters.value = {
      category: undefined,
      source_language: undefined,
      target_language: undefined,
      search: undefined,
      project_id: undefined,
      limit: undefined,
      offset: undefined
    }
  }

  /**
   * Reset store state
   */
  const reset = () => {
    entries.value = []
    clearFilters()
    stats.value = null
    clearError()
  }

  return {
    // State
    entries,
    filters,
    stats,

    // Getters
    filteredEntries,
    totalEntries,
    filteredCount,
    categories,
    languagePairs,

    // Base state
    isLoading,
    error,
    clearError,

    // Actions
    loadEntries,
    createEntry,
    updateEntry,
    deleteEntry,
    getEntry,
    loadStats,
    setFilters,
    clearFilters,
    reset
  }
})

