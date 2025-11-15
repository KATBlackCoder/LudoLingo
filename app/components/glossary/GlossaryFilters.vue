<script setup lang="ts">
import { ref, watch, onMounted, computed } from 'vue'
import { useGlossaryStore } from '~/stores/glossary'
import { useProjectsStore } from '~/stores/projects'
import type { GlossaryFilters } from '~/composables/db/glossary'

interface Emits {
  (e: 'filter-changed', filters: GlossaryFilters): void
}

const emit = defineEmits<Emits>()

const glossaryStore = useGlossaryStore()
const projectsStore = useProjectsStore()

const categoryOptions = [
  { label: 'Général', value: 'general' },
  { label: 'Personnage', value: 'character' },
  { label: 'Objet', value: 'item' },
  { label: 'Lieu', value: 'location' },
  { label: 'Système', value: 'system' },
  { label: 'Compétence', value: 'skill' }
]

const languageOptions = [
  { label: 'Japonais', value: 'ja' },
  { label: 'Français', value: 'fr' },
  { label: 'Anglais', value: 'en' },
  { label: 'Espagnol', value: 'es' },
  { label: 'Allemand', value: 'de' },
  { label: 'Italien', value: 'it' },
  { label: 'Portugais', value: 'pt' },
  { label: 'Chinois', value: 'zh' },
  { label: 'Coréen', value: 'ko' }
]

const localFilters = ref<{
  category: string | undefined
  source_language: string | undefined
  target_language: string | undefined
  search: string
  project_scope: 'all' | 'global' | 'current' | undefined
}>({
  category: undefined,
  source_language: undefined,
  target_language: undefined,
  search: '',
  project_scope: undefined
})

// Scope options for project filter
const scopeOptions = computed(() => {
  const options = [
    { label: 'Tous', value: 'all' },
    { label: '🌍 Globaux uniquement', value: 'global' }
  ]
  
  // Add current project option if available
  if (projectsStore.currentProject) {
    options.push({
      label: `📁 Projet actuel uniquement`,
      value: 'current'
    })
  }
  
  return options
})

// Initialiser avec les filtres du store au montage uniquement
onMounted(() => {
  const storeFilters = glossaryStore.filters
  localFilters.value = {
    category: Array.isArray(storeFilters.category) ? storeFilters.category[0] : storeFilters.category,
    source_language: storeFilters.source_language,
    target_language: storeFilters.target_language,
    search: storeFilters.search || '',
    project_scope: storeFilters.project_id === 'global' ? 'global' : 
                    storeFilters.project_id === 'current' ? 'current' :
                    storeFilters.project_id === undefined ? undefined : 'all'
  }
})

// Émettre les changements de filtres
const applyFilters = () => {
  // Convert project_scope to project_id filter
  let projectId: GlossaryFilters['project_id'] = undefined
  if (localFilters.value.project_scope === 'global') {
    projectId = 'global'
  } else if (localFilters.value.project_scope === 'current' && projectsStore.currentProject) {
    projectId = projectsStore.currentProject.id
  } else if (localFilters.value.project_scope === 'all') {
    // 'all' means no filter (show everything)
    projectId = undefined
  }
  
  const filters: GlossaryFilters = {
    category: localFilters.value.category ? [localFilters.value.category] : undefined,
    source_language: localFilters.value.source_language,
    target_language: localFilters.value.target_language,
    search: localFilters.value.search.trim() || undefined,
    project_id: projectId
  }
  
  glossaryStore.setFilters(filters)
  emit('filter-changed', filters)
}

// Réinitialiser les filtres
const clearFilters = () => {
  localFilters.value = {
    category: undefined,
    source_language: undefined,
    target_language: undefined,
    search: '',
    project_scope: undefined
  }
  glossaryStore.clearFilters()
  emit('filter-changed', {})
}

// Watchers pour appliquer automatiquement les filtres
watch(() => localFilters.value.category, () => {
  applyFilters()
})

watch(() => localFilters.value.source_language, () => {
  applyFilters()
})

watch(() => localFilters.value.target_language, () => {
  applyFilters()
})

watch(() => localFilters.value.project_scope, () => {
  applyFilters()
})

// Debounce pour la recherche
let searchTimeout: ReturnType<typeof setTimeout> | null = null
watch(() => localFilters.value.search, () => {
  if (searchTimeout) {
    clearTimeout(searchTimeout)
  }
  searchTimeout = setTimeout(() => {
    applyFilters()
  }, 300)
})
</script>

<template>
  <div class="glossary-filters">
    <div class="flex flex-wrap gap-4 p-4 bg-default rounded-lg border border-accented">
      <UFormField label="Recherche" class="flex-1 min-w-[200px]">
        <UInput
          v-model="localFilters.search"
          placeholder="Rechercher un terme..."
          icon="i-heroicons-magnifying-glass"
        />
      </UFormField>

      <UFormField label="Catégorie" class="min-w-[150px]">
        <USelect
          v-model="localFilters.category"
          :items="categoryOptions"
          value-key="value"
          placeholder="Toutes les catégories"
        />
      </UFormField>

      <UFormField label="Langue source" class="min-w-[150px]">
        <USelect
          v-model="localFilters.source_language"
          :items="languageOptions"
          value-key="value"
          placeholder="Toutes les langues"
        />
      </UFormField>

      <UFormField label="Langue cible" class="min-w-[150px]">
        <USelect
          v-model="localFilters.target_language"
          :items="languageOptions"
          value-key="value"
          placeholder="Toutes les langues"
        />
      </UFormField>

      <UFormField label="Portée" class="min-w-[180px]">
        <USelect
          v-model="localFilters.project_scope"
          :items="scopeOptions"
          value-key="value"
          placeholder="Tous"
        />
      </UFormField>

      <div class="flex items-end">
        <UButton
          variant="ghost"
          color="neutral"
          size="sm"
          @click="clearFilters"
          :disabled="!localFilters.category && !localFilters.source_language && !localFilters.target_language && !localFilters.search && !localFilters.project_scope"
        >
          Réinitialiser
        </UButton>
      </div>
    </div>
  </div>
</template>

