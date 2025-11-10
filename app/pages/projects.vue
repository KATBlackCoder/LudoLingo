<template>
  <UContainer class="py-8">
    <div class="space-y-8">
      <!-- En-tête de la page -->
      <div class="text-center">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">Résultats d'Extraction</h1>
        <p class="text-lg text-gray-600 dark:text-gray-400">Statistiques et table complète des textes extraits</p>
      </div>

      <!-- Statistiques du projet -->
      <div v-if="projectsStore.currentProject">
        <ProjectStats />
      </div>

      <!-- Résultats d'extraction -->
      <div v-if="extractedTexts.length > 0" class="space-y-6">
        <TextsTable :texts="extractedTexts" />
      </div>

        <!-- État vide (si aucun texte extrait) -->
        <div v-else class="space-y-6">
          <!-- Composant de chargement de projet -->
          <ProjectLoader @project-loaded="onProjectLoaded" />

          <!-- Message alternatif si pas de projets -->
          <div v-if="projectsStore.projects.length === 0" class="text-center py-12">
            <UIcon name="i-heroicons-document-text" class="w-16 h-16 text-gray-400 dark:text-gray-500 mx-auto mb-4" />
            <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">Aucun projet disponible</h3>
            <p class="text-gray-600 dark:text-gray-400 mb-4">Commencez par extraire des textes depuis la page d'accueil.</p>
            <UButton
              icon="i-heroicons-arrow-left"
              color="primary"
              variant="outline"
              :to="{ name: 'index' }"
            >
              Retour à l'accueil
            </UButton>
          </div>
        </div>
    </div>
  </UContainer>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useProjectsStore } from '~/stores/projects'
import { TextsTable, ProjectStats } from '~/components/projects'
import ProjectLoader from '~/components/projects/ProjectLoader.vue'

const projectsStore = useProjectsStore()
const { getProjectTexts } = projectsStore

// Textes extraits depuis le store (projet actuel)
const extractedTexts = computed(() => {
  if (projectsStore.currentProject) {
    return getProjectTexts(projectsStore.currentProject.id)
  }
  return []
})

// Gestionnaire pour le chargement d'un projet
function onProjectLoaded() {
  // Le projet est maintenant chargé et affiché automatiquement via les computed
  console.log('Projet chargé avec succès')
}
</script>
