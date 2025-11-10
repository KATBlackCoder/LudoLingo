<template>
  <div class="project-stats">
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- Textes extraits -->
      <UCard class="text-center">
        <div class="flex items-center justify-center mb-4">
          <UIcon name="i-heroicons-document-text" class="w-8 h-8 text-blue-600 dark:text-blue-400" />
        </div>
        <div class="text-3xl font-bold text-gray-900 dark:text-white mb-2">{{ totalTexts }}</div>
        <div class="text-sm text-gray-700 dark:text-gray-300">
          {{ totalTexts === 1 ? 'Texte extrait' : 'Textes extraits' }}
        </div>
      </UCard>

      <!-- Textes traduits -->
      <UCard class="text-center">
        <div class="flex items-center justify-center mb-4">
          <UIcon name="i-heroicons-check-circle" class="w-8 h-8 text-green-600 dark:text-green-400" />
        </div>
        <div class="text-3xl font-bold text-gray-900 dark:text-white mb-2">{{ translatedTexts }}</div>
        <div class="text-sm text-gray-700 dark:text-gray-300">
          {{ translatedTexts === 1 ? 'Texte traduit' : 'Textes traduits' }}
        </div>
        <div v-if="totalTexts > 0" class="text-xs text-gray-600 dark:text-gray-400 mt-1">
          ({{ Math.round((translatedTexts / totalTexts) * 100) }}%)
        </div>
      </UCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useProjectsStore } from '~/stores/projects'

interface Props {
  projectId?: number
}

const props = withDefaults(defineProps<Props>(), {
  projectId: undefined
})

const projectsStore = useProjectsStore()

// Récupérer le projet (actuel ou spécifié)
const project = computed(() => {
  if (props.projectId) {
    return projectsStore.projects.find(p => p.id === props.projectId) || null
  }
  return projectsStore.currentProject
})

// Statistiques du projet
const totalTexts = computed(() => project.value?.totalTexts || 0)
const translatedTexts = computed(() => project.value?.translatedTexts || 0)
</script>
