<template>
  <div class="project-scanner">
    <slot name="trigger" :isScanning="isExtracting" :handleScan="handleScanProjects">
      <UButton
        icon="i-heroicons-folder-open"
        :color="color"
        :size="size"
        :loading="isExtracting"
        @click="handleScanProjects"
      >
        {{ isExtracting ? 'Extraction en cours...' : buttonText }}
      </UButton>
    </slot>

    <slot name="results" :texts="extractedTexts" :project="currentProject">
      <div v-if="extractedTexts.length > 0" class="mt-4">
        <UAlert
          icon="i-heroicons-check-circle"
          color="success"
          variant="soft"
          :title="`Extraction terminée`"
          :description="`${extractedTexts.length} textes trouvés`"
        />
      </div>
    </slot>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useNotifications } from '~/composables/useNotifications'
import { useProjectsStore } from '~/stores/projects'
import { extractTextsFromFolder } from '~/composables/db/scanning'
import { hasProjectTexts } from '~/composables/db/texts/create'
import { open } from '@tauri-apps/plugin-dialog'
import type { TextEntry } from '~/types/scanning-commands'

interface Props {
  buttonText?: string
  color?: 'primary' | 'secondary' | 'success' | 'info' | 'warning' | 'error'
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  showResults?: boolean
}

withDefaults(defineProps<Props>(), {
  buttonText: 'Scanner un jeu',
  color: 'primary',
  size: 'xl',
  showResults: true
})

const emit = defineEmits<{
  'scan-started': [projectName: string]
  'scan-completed': [texts: TextEntry[], projectId: number]
  'scan-error': [error: Error]
}>()

const { notifySuccess, notifyError, notifyInfo } = useNotifications()
const projectsStore = useProjectsStore()
const { loadOrCreateProject, setCurrentProject, updateProjectTexts, loadProjectTextsFromDB } = projectsStore

// État local
const isExtracting = ref(false)

// Computed pour les données du projet actuel
const currentProject = computed(() => projectsStore.currentProject)
const extractedTexts = computed(() => {
  if (currentProject.value) {
    return projectsStore.getProjectTexts(currentProject.value.id)
  }
  return []
})

// Fonction principale de scan
async function handleScanProjects() {
  try {
    isExtracting.value = true

    // Ouvrir le sélecteur de dossier
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Sélectionnez un dossier de jeu RPG Maker'
    })

    if (!selected || typeof selected !== 'string') {
      return // Annulé par l'utilisateur
    }

    // Générer un nom de projet basé sur le nom du dossier
    const folderName = selected.split('/').pop() || 'Unknown Project'
    const projectName = `Projet ${folderName}`

    // Charger ou créer le projet de manière centralisée
    const project = await loadOrCreateProject(selected, projectName)

    // Émettre l'événement de début
    emit('scan-started', projectName)

    // Définir comme projet actuel
    await setCurrentProject(project.id)

    // Afficher notification de début
    await notifyInfo('Extraction des textes en cours...')

    // Vérifier si le projet a déjà des textes en DB
    const hasTexts = await hasProjectTexts(project.id)
    let texts: TextEntry[] = []

    if (hasTexts) {
      console.log(`ℹ️ Projet ${project.id} a déjà des textes en DB - chargement direct`)
      // Charger les textes existants depuis la DB
      texts = await loadProjectTextsFromDB(project.id)
    } else {
      console.log(`🔄 Extraction des textes pour le projet ${project.id}...`)
      // Extraire les textes du dossier
      texts = await extractTextsFromFolder(selected)

      // Créer le fichier .ludolingo.json avec l'ID du projet
      const { createProjectMarker } = await import('~/composables/projectMarkers')
      await createProjectMarker(selected, project.id)

      // Stocker les textes extraits en DB et dans le store
      await updateProjectTexts(project.id, texts)
    }

    // Émettre l'événement de succès
    emit('scan-completed', texts, project.id)

    // Afficher notification de succès
    await notifySuccess(`Extraction terminée ! ${texts.length} textes trouvés dans "${projectName}".`)

    // Logs détaillés pour le développement
    console.log(`✅ Extraction terminée ! ${texts.length} textes trouvés:`)
    texts.forEach((text, index) => {
      console.log(`${index + 1}. [${text.entry_type}] "${text.source_text}" ${text.location ? `(location: ${text.location})` : ''}`)
    })

  } catch (error) {
    console.error('Extraction failed:', error)
    const err = error instanceof Error ? error : new Error('Erreur inconnue')

    // Émettre l'événement d'erreur
    emit('scan-error', err)

    // Notification d'erreur
    await notifyError('Impossible d\'extraire les textes du dossier sélectionné.')
  } finally {
    isExtracting.value = false
  }
}


// Exposer la fonction pour utilisation externe
defineExpose({
  handleScanProjects,
  isExtracting,
  extractedTexts,
  currentProject
})
</script>
