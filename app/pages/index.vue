<template>
  <UContainer class="py-8">
    <!-- Titre principal -->
    <div class="text-center mb-12">
      <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">
        {{ tmReactive('welcome', 'title').value }}
      </h1>
      <p class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
        {{ tmReactive('welcome', 'subtitle').value }}
      </p>
    </div>

      <!-- Section principale avec onglets -->
      <div class="mb-12">
        <UTabs v-model="currentSection" :items="[
          { label: 'Nouveau projet', value: 'new' },
          { label: 'Projets existants', value: 'existing' }
        ]" />

        <!-- Section nouveau projet -->
        <div v-if="currentSection === 'new'" class="text-center mt-6">
          <ProjectScanner
            button-text="Scanner un jeu"
            size="xl"
            @scan-started="onScanStarted"
            @scan-completed="onScanCompleted"
            @scan-error="onScanError"
          />
          <p class="text-sm text-gray-600 dark:text-gray-400 mt-2">
            Sélectionnez un dossier de jeu RPG Maker pour commencer l'extraction des textes
          </p>
        </div>

        <!-- Section projets existants -->
        <div v-else class="mt-6">
        <ProjectLoader @project-loaded="onProjectLoaded" />
        </div>
    </div>

      <!-- Bouton vers les traductions (affiché après extraction réussie) -->
    <div v-if="extractedTexts.length > 0" class="text-center">
      <UButton
        icon="i-heroicons-arrow-right"
        color="primary"
        variant="outline"
        size="lg"
          to="/translation"
      >
          Voir les textes ({{ extractedTexts.length }} textes)
      </UButton>
      <p class="text-sm text-gray-600 dark:text-gray-400 mt-2">
          Accédez à l'interface de traduction avec les textes bruts, en cours et traduits
      </p>
    </div>
  </UContainer>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAppLocale } from '~/composables/useLocale'
import { useProjectsStore } from '~/stores/projects'
import { TextsTable, ProjectScanner, ProjectStats } from '~/components/projects'
import ProjectLoader from '~/components/projects/ProjectLoader.vue'
import type { TextEntry } from '~/types/scanning-commands'

const { tmReactive } = useAppLocale()
const projectsStore = useProjectsStore()
const { getProjectTexts } = projectsStore

// État pour l'affichage
const hasAttemptedExtraction = ref(false)
const currentSection = ref<'new' | 'existing'>('new')

// Textes extraits depuis le store (projet actuel)
const extractedTexts = computed(() => {
  if (projectsStore.currentProject) {
    return getProjectTexts(projectsStore.currentProject.id)
  }
  return []
})

// Initialiser les projets au montage
onMounted(async () => {
  try {
    // Les projets sont chargés automatiquement par le store
    // Rien à faire ici
  } catch (error) {
    console.error('Erreur lors de l\'initialisation des projets:', error)
  }
})

// Gestionnaires d'événements du ProjectScanner
function onScanStarted(projectName: string) {
  hasAttemptedExtraction.value = true
  console.log(`🔄 Démarrage de l'extraction pour le projet: ${projectName}`)
}

function onScanCompleted(texts: TextEntry[], projectId: number) {
  console.log(`✅ Extraction terminée: ${texts.length} textes pour le projet ${projectId}`)
}

function onScanError(error: Error) {
  console.error('❌ Erreur lors de l\'extraction:', error)
}

// Gestionnaire pour le chargement d'un projet existant
function onProjectLoaded() {
  // Le projet est maintenant chargé et affiché automatiquement via les computed
  console.log('Projet chargé avec succès')
  // Basculer vers la section nouveau projet après chargement
  currentSection.value = 'new'
}
</script>
