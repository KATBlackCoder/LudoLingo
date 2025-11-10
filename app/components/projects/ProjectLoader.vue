<template>
  <UCard>
    <template #header>
      <div class="flex items-center gap-3">
        <UIcon name="i-heroicons-folder-open" class="h-5 w-5 text-primary" />
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
          Charger un projet existant
        </h3>
      </div>
      <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
        Sélectionnez un projet précédemment extrait pour afficher ses textes
      </p>
    </template>

    <div class="space-y-4">
      <!-- Liste des projets -->
      <div v-if="availableProjects.length > 0" class="space-y-3">
        <h4 class="text-sm font-medium text-gray-900 dark:text-white">
          Projets disponibles ({{ availableProjects.length }})
        </h4>

        <div class="grid gap-3">
          <div
            v-for="project in availableProjects"
            :key="project.id"
            class="p-4 border rounded-lg cursor-pointer transition-all duration-200 hover:border-primary hover:bg-primary/5 dark:hover:bg-primary/10"
            :class="{
              'border-primary bg-primary/5 dark:bg-primary/10': selectedProjectId === project.id,
              'border-gray-200 dark:border-gray-700': selectedProjectId !== project.id
            }"
            @click="selectProject(project.id)"
          >
            <div class="flex items-start justify-between">
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-2">
                  <UIcon name="i-heroicons-folder" class="h-4 w-4 text-gray-500" />
                  <h5 class="font-medium text-gray-900 dark:text-white truncate">
                    {{ project.name }}
                  </h5>
                  <UBadge
                    :color="getEngineColor(project.gameEngine)"
                    size="xs"
                    variant="subtle"
                  >
                    {{ project.gameEngine === 'Unknown' ? 'Auto' : project.gameEngine }}
                  </UBadge>
                </div>

                <div class="flex items-center gap-4 text-sm text-gray-600 dark:text-gray-400">
                  <span class="flex items-center gap-1">
                    <UIcon name="i-heroicons-document-text" class="h-3 w-3" />
                    {{ project.totalTexts }} textes
                  </span>
                  <span class="flex items-center gap-1">
                    <UIcon name="i-heroicons-check-circle" class="h-3 w-3" />
                    {{ project.translatedTexts }} traduits
                  </span>
                  <span class="flex items-center gap-1">
                    <UIcon name="i-heroicons-clock" class="h-3 w-3" />
                    {{ formatDate(project.lastAccessedAt) }}
                  </span>
                </div>

                <div class="mt-2 text-xs text-gray-500 dark:text-gray-500 truncate">
                  {{ project.gamePath }}
                </div>
              </div>

              <div class="ml-4 flex-shrink-0 flex gap-2">
                <UButton
                  size="xs"
                  :loading="loadingProjectId === project.id"
                  :disabled="loadingProjectId !== null || deletingProjectId === project.id"
                  @click.stop="loadProject(project.id)"
                >
                  {{ loadingProjectId === project.id ? 'Chargement...' : 'Charger' }}
                </UButton>

                <UButton
                  size="xs"
                  color="error"
                  variant="outline"
                  :loading="deletingProjectId === project.id"
                  :disabled="loadingProjectId !== null || deletingProjectId !== null"
                  @click.stop="handleDeleteProject(project.id, project.name)"
                >
                  <UIcon name="i-heroicons-trash" class="h-3 w-3" />
                </UButton>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Aucun projet disponible -->
      <div v-else class="text-center py-8">
        <UIcon name="i-heroicons-folder" class="h-12 w-12 text-gray-400 dark:text-gray-500 mx-auto mb-3" />
        <h4 class="text-sm font-medium text-gray-900 dark:text-white mb-1">
          Aucun projet extrait
        </h4>
        <p class="text-xs text-gray-600 dark:text-gray-400">
          Créez d'abord un projet en scannant un jeu depuis l'accueil
        </p>
      </div>

      <!-- Bouton d'action -->
      <div v-if="selectedProjectId && !loadingProjectId" class="flex justify-end pt-4 border-t">
        <UButton
          color="primary"
          @click="loadSelectedProject"
        >
          Charger le projet sélectionné
        </UButton>
      </div>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { computed, ref, onMounted } from 'vue'
import { useProjectsStore } from '~/stores/projects'
import { useNotifications } from '~/composables/useNotifications'
import { confirm } from '@tauri-apps/plugin-dialog'

interface Props {
  // Aucune prop nécessaire pour ce composant
}

interface Emits {
  (e: 'project-loaded'): void
}

defineProps<Props>()
const emit = defineEmits<Emits>()

const projectsStore = useProjectsStore()
const { loadProjectTextsFromDB, setCurrentProject, loadProjectsFromDB, deleteProject } = projectsStore
const { notifySuccess, notifyError, notifyWarning } = useNotifications()

// État local
const selectedProjectId = ref<number | null>(null)
const loadingProjectId = ref<number | null>(null)
const deletingProjectId = ref<number | null>(null)

// Charger les projets depuis DB au montage du composant
onMounted(async () => {
  await loadProjectsFromDB()
})

// Projets disponibles (ceux qui ont des textes extraits)
const availableProjects = computed(() =>
  projectsStore.projects.filter(project => project.totalTexts > 0)
)

// Sélection d'un projet
function selectProject(projectId: number) {
  selectedProjectId.value = projectId
}

// Chargement d'un projet spécifique
async function loadProject(projectId: number) {
  try {
    loadingProjectId.value = projectId

    console.log(`🔄 Chargement du projet ${projectId}...`)

    // Charger les textes depuis la DB
    await loadProjectTextsFromDB(projectId)

    // Définir comme projet actuel
    await setCurrentProject(projectId)

    console.log(`✅ Projet ${projectId} chargé avec succès`)

    emit('project-loaded')
  } catch (error) {
    console.error('Erreur lors du chargement du projet:', error)
    // TODO: Afficher une notification d'erreur
  } finally {
    loadingProjectId.value = null
  }
}

// Chargement du projet sélectionné
function loadSelectedProject() {
  if (selectedProjectId.value) {
    loadProject(selectedProjectId.value)
  }
}

// Suppression d'un projet
async function handleDeleteProject(projectId: number, projectName: string) {
  try {
    // Demander confirmation avec dialog Tauri natif
    const confirmed = await confirm(
      `Êtes-vous sûr de vouloir supprimer le projet "${projectName}" ?\n\n` +
      `Cette action va :\n` +
      `• Supprimer définitivement tous les textes extraits\n` +
      `• Supprimer le fichier marqueur .ludolingo.json\n` +
      `• Retirer le projet de la liste\n\n` +
      `Cette action est IRRÉVERSIBLE.`,
      {
        title: 'Confirmation de suppression',
        kind: 'warning',
        okLabel: 'Supprimer',
        cancelLabel: 'Annuler'
      }
    )

    if (!confirmed) {
      return
    }

    deletingProjectId.value = projectId

    await deleteProject(projectId)

    notifySuccess(`Projet "${projectName}" supprimé avec succès`)

    // Recharger la liste des projets
    await loadProjectsFromDB()
  } catch (error) {
    console.error('Erreur lors de la suppression:', error)
    notifyError(
      `Échec de la suppression du projet "${projectName}": ${
        error instanceof Error ? error.message : 'Erreur inconnue'
      }`
    )
  } finally {
    deletingProjectId.value = null
  }
}

// Formatage de la date
function formatDate(dateString: string): string {
  try {
    const date = new Date(dateString)
    return date.toLocaleDateString('fr-FR', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    })
  } catch {
    return 'Date inconnue'
  }
}

// Couleur pour le badge du moteur
function getEngineColor(engine: string): "error" | "warning" | "secondary" | "neutral" | "primary" | "success" | "info" {
  switch (engine) {
    case 'RPG Maker MZ':
      return 'warning'
    case 'RPG Maker MV':
      return 'secondary'
    case 'Unknown':
      return 'neutral'
    default:
      return 'primary'
  }
}
</script>
