<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import { confirm } from '@tauri-apps/plugin-dialog'
import { useGlossaryStore } from '~/stores/glossary'
import { useNotifications } from '~/composables/useNotifications'
import { GlossaryTable, GlossaryEditor, GlossaryFilters } from '~/components/glossary'
import type { GlossaryEntry } from '~/composables/db/glossary'

const glossaryStore = useGlossaryStore()
const { notifySuccess, notifyError, notifyWarning } = useNotifications()

const { entries, isLoading } = storeToRefs(glossaryStore)

// Utiliser computed local pour filteredEntries pour garantir la réactivité au premier rendu
// storeToRefs peut avoir des problèmes de timing avec les computed au premier chargement
const filteredEntries = computed(() => glossaryStore.filteredEntries)

// État du modal d'édition
const isEditorOpen = ref(false)
const editingEntry = ref<GlossaryEntry | null>(null)

// Charger les entrées au montage
onMounted(async () => {
  try {
    // Toujours charger les données au montage pour s'assurer qu'elles sont à jour
    await glossaryStore.loadEntries({})
    await glossaryStore.loadStats()
    // Forcer la mise à jour après le chargement
    await nextTick()
  } catch (error) {
    notifyError(
      'Erreur de chargement',
      error instanceof Error ? error.message : 'Impossible de charger les entrées du glossaire'
    )
  }
})

// Ouvrir le modal pour créer une nouvelle entrée
const handleCreate = () => {
  editingEntry.value = null
  isEditorOpen.value = true
}

// Ouvrir le modal pour éditer une entrée
const handleEdit = (entry: GlossaryEntry) => {
  editingEntry.value = entry
  isEditorOpen.value = true
}

// Supprimer une entrée avec confirmation
const handleDelete = async (entry: GlossaryEntry) => {
  const confirmed = await confirm(
    `Êtes-vous sûr de vouloir supprimer l'entrée "${entry.source_term}" ?`,
    {
      title: 'Supprimer l\'entrée du glossaire',
      kind: 'warning',
      okLabel: 'Supprimer',
      cancelLabel: 'Annuler'
    }
  )

  if (!confirmed) return

  try {
    await glossaryStore.deleteEntry(entry.id)
    notifySuccess('Entrée supprimée', `L'entrée "${entry.source_term}" a été supprimée avec succès`)
  } catch (error) {
    notifyError(
      'Erreur de suppression',
      error instanceof Error ? error.message : 'Impossible de supprimer l\'entrée'
    )
  }
}

// Gérer la sauvegarde depuis le modal
const handleSaved = async () => {
  // Le store met déjà à jour entries.value automatiquement lors de createEntry/updateEntry
  // On recharge seulement les stats car entries.value est déjà à jour
  try {
    await glossaryStore.loadStats()
    // Note: Pas besoin de recharger entries car le store les met déjà à jour localement
    // filteredEntries est réactif et se mettra à jour automatiquement
  } catch (error) {
    notifyError(
      'Erreur de rechargement',
      error instanceof Error ? error.message : 'Impossible de recharger les statistiques'
    )
  }
}

// Gérer les changements de filtres
const handleFilterChanged = async () => {
  // Les filtres sont appliqués automatiquement côté client via filteredEntries computed
  // Pas besoin de recharger depuis la DB, les filtres sont appliqués sur entries.value
  // Cette fonction est appelée mais ne fait rien car filteredEntries est réactif
}

// Statistiques pour affichage
const stats = computed(() => {
  return {
    total: entries.value.length,
    filtered: filteredEntries.value.length,
    categories: glossaryStore.categories.length,
    languagePairs: glossaryStore.languagePairs.length
  }
})
</script>

<template>
  <UContainer class="py-8">
    <!-- Header -->
    <div class="mb-8">
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-3">
          <UIcon name="i-heroicons-book-open" class="h-7 w-7 text-primary" />
          <div>
            <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
              Glossaire
            </h1>
            <p class="text-gray-600 dark:text-gray-400 mt-1">
              Gérez les termes de traduction pour enrichir vos traductions
            </p>
          </div>
        </div>
        <UButton
          icon="i-heroicons-plus"
          color="primary"
          @click="handleCreate"
        >
          Nouvelle entrée
        </UButton>
      </div>

      <!-- Statistiques -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        <UCard>
          <div class="flex items-center gap-3">
            <UIcon name="i-heroicons-book-open" class="h-5 w-5 text-primary" />
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Total</p>
              <p class="text-2xl font-bold">{{ stats.total }}</p>
            </div>
          </div>
        </UCard>
        <UCard>
          <div class="flex items-center gap-3">
            <UIcon name="i-heroicons-funnel" class="h-5 w-5 text-info" />
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Filtrées</p>
              <p class="text-2xl font-bold">{{ stats.filtered }}</p>
            </div>
          </div>
        </UCard>
        <UCard>
          <div class="flex items-center gap-3">
            <UIcon name="i-heroicons-tag" class="h-5 w-5 text-success" />
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Catégories</p>
              <p class="text-2xl font-bold">{{ stats.categories }}</p>
            </div>
          </div>
        </UCard>
        <UCard>
          <div class="flex items-center gap-3">
            <UIcon name="i-heroicons-globe-alt" class="h-5 w-5 text-warning" />
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Paires de langues</p>
              <p class="text-2xl font-bold">{{ stats.languagePairs }}</p>
            </div>
          </div>
        </UCard>
      </div>
    </div>

    <!-- Filtres -->
    <div class="mb-6">
      <GlossaryFilters @filter-changed="handleFilterChanged" />
    </div>

    <!-- Table des entrées -->
    <UCard>
      <GlossaryTable
        :key="`glossary-table-${filteredEntries.length}-${entries.length}`"
        :entries="filteredEntries || []"
        :loading="isLoading"
        @edit="handleEdit"
        @delete="handleDelete"
      />
    </UCard>

    <!-- Modal d'édition/création -->
    <GlossaryEditor
      v-model:open="isEditorOpen"
      :entry="editingEntry"
      @saved="handleSaved"
    />
  </UContainer>
</template>

