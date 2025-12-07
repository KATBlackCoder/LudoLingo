<template>
  <div class="glossary-import-export flex gap-2">
    <!-- Import Button with Dropdown Menu -->
    <UDropdownMenu :items="importMenuItems">
      <UButton
        icon="i-heroicons-arrow-down-tray"
        color="primary"
        variant="outline"
        :loading="isImporting"
        trailing-icon="i-heroicons-chevron-down"
      >
        Importer
      </UButton>
    </UDropdownMenu>

    <!-- Export Button with Dropdown Menu -->
    <UDropdownMenu :items="exportMenuItems">
      <UButton
        icon="i-heroicons-arrow-up-tray"
        color="primary"
        variant="outline"
        :loading="isExporting"
        trailing-icon="i-heroicons-chevron-down"
      >
        Exporter
      </UButton>
    </UDropdownMenu>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useNotifications } from '~/composables/useNotifications'
import {
  importGlossaryFromJSON,
  importGlossaryFromCSV,
  importGlossaryFromXLSX,
  exportGlossaryToJSON,
  exportGlossaryToCSV,
  exportGlossaryToXLSX
} from '~/composables/db/glossary'
import { useGlossaryStore } from '~/stores/glossary'
import type { DropdownMenuItem } from '@nuxt/ui'

const { notifySuccess, notifyError, notifyWarning } = useNotifications()
const glossaryStore = useGlossaryStore()

const isImporting = ref(false)
const isExporting = ref(false)

// Import menu items - Always skip duplicates by default
const importMenuItems = computed<DropdownMenuItem[][]>(() => [
  [
    {
      label: 'Importer depuis JSON',
      icon: 'i-heroicons-document-text',
      onSelect: () => handleImport('json')
    },
    {
      label: 'Importer depuis CSV',
      icon: 'i-heroicons-table-cells',
      onSelect: () => handleImport('csv')
    },
    {
      label: 'Importer depuis Excel',
      icon: 'i-heroicons-document-duplicate',
      onSelect: () => handleImport('xlsx')
    }
  ]
])

// Export menu items
const exportMenuItems = computed<DropdownMenuItem[][]>(() => [
  [
    {
      label: 'Exporter en JSON',
      icon: 'i-heroicons-document-text',
      onSelect: () => handleExport('json')
    },
    {
      label: 'Exporter en CSV',
      icon: 'i-heroicons-table-cells',
      onSelect: () => handleExport('csv')
    },
    {
      label: 'Exporter en Excel',
      icon: 'i-heroicons-document-duplicate',
      onSelect: () => handleExport('xlsx')
    }
  ]
])

// Handle import action - Always skip duplicates by default
const handleImport = async (format: 'json' | 'csv' | 'xlsx') => {
  try {
    isImporting.value = true

    // Get current project_id from filters if available
    const projectId = glossaryStore.filters.project_id === 'current'
      ? null // Will be handled by the import function
      : typeof glossaryStore.filters.project_id === 'number'
        ? glossaryStore.filters.project_id
        : null

    // Always skip duplicates - entries with same source_term, translated_term, 
    // source_language, target_language, and project_id will be ignored
    const skipDuplicates = true

    let result

    if (format === 'json') {
      result = await importGlossaryFromJSON({ project_id: projectId, skipDuplicates })
    } else if (format === 'csv') {
      result = await importGlossaryFromCSV({ project_id: projectId, skipDuplicates })
    } else {
      result = await importGlossaryFromXLSX({ project_id: projectId, skipDuplicates })
    }

    if (!result.success) {
      notifyError(
        'Erreur d\'importation',
        result.error || 'Impossible d\'importer le fichier'
      )
      return
    }

    const { imported_count, skipped_count, errors } = result.data!

    // Reload glossary entries after import
    await glossaryStore.loadEntries(glossaryStore.filters)
    await glossaryStore.loadStats()

    // Show success notification with details
    if (errors.length > 0) {
      notifyWarning(
        'Importation partielle',
        `${imported_count} entrée(s) importée(s), ${skipped_count} doublon(s) ignoré(s). ${errors.length > 0 ? `Erreurs: ${errors.slice(0, 3).join('; ')}${errors.length > 3 ? '...' : ''}` : ''}`
      )
    } else if (skipped_count > 0) {
      notifySuccess(
        'Importation réussie',
        `${imported_count} entrée(s) importée(s), ${skipped_count} doublon(s) ignoré(s)`
      )
    } else {
      notifySuccess(
        'Importation réussie',
        `${imported_count} entrée(s) importée(s) avec succès`
      )
    }
  } catch (error) {
    notifyError(
      'Erreur d\'importation',
      error instanceof Error ? error.message : 'Une erreur inattendue s\'est produite'
    )
  } finally {
    isImporting.value = false
  }
}

// Handle export action
const handleExport = async (format: 'json' | 'csv' | 'xlsx') => {
  try {
    isExporting.value = true

    // Get current filters for export
    const filters: { source_language?: string; target_language?: string; project_id?: number | null } = {}
    
    if (glossaryStore.filters.source_language) {
      filters.source_language = glossaryStore.filters.source_language
    }
    if (glossaryStore.filters.target_language) {
      filters.target_language = glossaryStore.filters.target_language
    }
    if (typeof glossaryStore.filters.project_id === 'number') {
      filters.project_id = glossaryStore.filters.project_id
    } else if (glossaryStore.filters.project_id === 'global') {
      filters.project_id = null
    }

    let result

    if (format === 'json') {
      result = await exportGlossaryToJSON(filters)
    } else if (format === 'csv') {
      result = await exportGlossaryToCSV(filters)
    } else {
      result = await exportGlossaryToXLSX(filters)
    }

    if (!result.success) {
      notifyError(
        'Erreur d\'exportation',
        result.error || 'Impossible d\'exporter le fichier'
      )
      return
    }

    const { exported_count, file_path } = result.data!

    notifySuccess(
      'Exportation réussie',
      `${exported_count} entrée(s) exportée(s) vers:\n${file_path}`
    )
  } catch (error) {
    notifyError(
      'Erreur d\'exportation',
      error instanceof Error ? error.message : 'Une erreur inattendue s\'est produite'
    )
  } finally {
    isExporting.value = false
  }
}
</script>

