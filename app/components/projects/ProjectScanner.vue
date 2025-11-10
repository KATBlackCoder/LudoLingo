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
import { writeTextFile, readTextFile, exists } from '@tauri-apps/plugin-fs'
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
const { createProject, setCurrentProject, updateProjectTexts, loadProjectTextsFromDB } = projectsStore

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

    // Créer ou trouver un projet existant pour ce dossier
    let project = projectsStore.projects.find(p => p.gamePath === selected)

    if (!project) {
      // Vérifier d'abord le fichier marqueur pour une vérification rapide
      const markerData = await readProjectMarkerFile(selected)

      if (markerData) {
        // Le fichier marqueur existe, vérifier si le projet en DB a des textes
        const projectExistsWithTexts = await hasProjectTexts(markerData.projectId)

        if (projectExistsWithTexts) {
          // Charger le projet depuis la DB en utilisant l'ID du marqueur
          const { getProject } = await import('~/composables/db/project')
          const dbResult = await getProject(markerData.projectId)

          if (dbResult.success && dbResult.data) {
            const dbProject = dbResult.data
            const loadedProject = {
              id: dbProject.id,
              name: dbProject.name,
              gamePath: dbProject.game_path,
              gameEngine: (dbProject.game_engine || 'Unknown') as 'RPG Maker MV' | 'RPG Maker MZ' | 'Unknown',
              createdAt: dbProject.created_at,
              lastAccessedAt: new Date().toISOString(),
              scanHistory: [],
              totalTexts: 0,
              translatedTexts: 0,
              extractedTexts: []
            }

            projectsStore.projects.push(loadedProject)
            project = loadedProject
            await projectsStore.saveProjects()
            await notifyInfo(`Projet existant "${projectName}" chargé depuis le marqueur.`)
          }
        }
      }

      // Si pas de projet chargé depuis le marqueur, vérifier en DB ou créer nouveau
      if (!project) {
        const { getProjects } = await import('~/composables/db/project')
        const dbResult = await getProjects({ game_path: selected })

        if (dbResult.success && dbResult.data && dbResult.data.projects.length > 0) {
          // Le projet existe en DB, le charger dans le store Pinia
          const dbProject = dbResult.data.projects[0]!
          const loadedProject = {
            id: dbProject.id,
            name: dbProject.name,
            gamePath: dbProject.game_path,
            gameEngine: (dbProject.game_engine || 'Unknown') as 'RPG Maker MV' | 'RPG Maker MZ' | 'Unknown',
            createdAt: dbProject.created_at,
            lastAccessedAt: new Date().toISOString(),
            scanHistory: [],
            totalTexts: 0,
            translatedTexts: 0,
            extractedTexts: []
          }

          projectsStore.projects.push(loadedProject)
          project = loadedProject
          await projectsStore.saveProjects()
          await notifyInfo(`Projet existant "${projectName}" chargé.`)
        } else {
          // Créer un nouveau projet
          project = await createProject({
            name: projectName,
            gamePath: selected,
            gameEngine: 'Unknown' // Sera détecté automatiquement
          })
          await notifyInfo(`Nouveau projet "${projectName}" créé.`)
        }
      }
    }

    // Vérifier que le projet a été créé/chargé
    if (!project) {
      throw new Error('Échec de création ou chargement du projet')
    }

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
      await createProjectMarkerFile(selected, project.id)

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
      console.log(`${index + 1}. [${text.entry_type}] "${text.source_text}" ${text.context ? `(contexte: ${text.context})` : ''}`)
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

// Créer un fichier marqueur dans le dossier du projet
async function createProjectMarkerFile(projectPath: string, projectId: number) {
  try {
    const markerData = {
      projectId: projectId,
      createdAt: new Date().toISOString(),
      version: '1.0'
    }

    const markerPath = `${projectPath}/.ludolingo.json`
    await writeTextFile(markerPath, JSON.stringify(markerData, null, 2))
    console.log(`📄 Fichier marqueur créé: ${markerPath}`)
  } catch (error) {
    console.warn('Impossible de créer le fichier marqueur:', error)
    // Ne pas échouer l'extraction pour autant
  }
}

// Lire le fichier marqueur d'un projet
async function readProjectMarkerFile(projectPath: string): Promise<{ projectId: number } | null> {
  try {
    const markerPath = `${projectPath}/.ludolingo.json`
    const existsMarker = await exists(markerPath)

    if (!existsMarker) {
      return null
    }

    const markerContent = await readTextFile(markerPath)
    const markerData = JSON.parse(markerContent)

    if (markerData.projectId && typeof markerData.projectId === 'number') {
      return { projectId: markerData.projectId }
    }

    return null
  } catch (error) {
    console.warn('Erreur lors de la lecture du fichier marqueur:', error)
    return null
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
