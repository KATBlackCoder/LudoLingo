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

      <!-- Section principale -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
        <ProjectSection
          @scan-projects="handleScanProjects"
        />

        <SupportedGamesSection />
      </div>

      <!-- Section dons -->
      <DonationSection />
  </UContainer>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useLocale } from '~/composables/useLocale'
import { useNotifications } from '~/composables/useNotifications'
import { useProjectsStore } from '~/stores/projects'
import ProjectSection from '~/components/projects/ProjectSection.vue'
import SupportedGamesSection from '~/components/projects/SupportedGamesSection.vue'
import DonationSection from '~/components/projects/DonationSection.vue'
import { extractTextsFromFolder } from '~/composables/db/scanning'
import { createProject, setCurrentProject, updateProjectStats, initializeProjects } from '~/composables/db/projects'
import { open } from '@tauri-apps/plugin-dialog'

const { tmReactive } = useLocale()
const { notifySuccess, notifyError, notifyInfo } = useNotifications()
const projectsStore = useProjectsStore()

// Initialiser les projets au montage
onMounted(async () => {
  try {
    await initializeProjects()
  } catch (error) {
    console.error('Erreur lors de l\'initialisation des projets:', error)
  }
})

async function handleScanProjects() {
  try {
    // Ouvrir directement le sélecteur de dossier
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
      // Créer un nouveau projet
      project = await createProject({
        name: projectName,
        gamePath: selected,
        gameEngine: 'Unknown' // Sera détecté automatiquement
      })

      await notifyInfo(`Nouveau projet "${projectName}" créé.`)
    }

    // Définir comme projet actuel
    await setCurrentProject(project.id)

    // Afficher notification de début
    await notifyInfo('Extraction des textes en cours...')

    // Extraire directement les textes
    const texts = await extractTextsFromFolder(selected)

    // Mettre à jour les statistiques du projet
    await updateProjectStats(project.id, texts.length, 0) // 0 traductions pour le moment

    // Afficher notification de succès
    await notifySuccess(`Extraction terminée ! ${texts.length} textes trouvés dans "${projectName}".`)

    // Garder les logs détaillés en console pour le développement
    console.log(`✅ Extraction terminée ! ${texts.length} textes trouvés:`)
    texts.forEach((text, index) => {
      console.log(`${index + 1}. [${text.entry_type}] "${text.source_text}" ${text.context ? `(contexte: ${text.context})` : ''}`)
    })

  } catch (error) {
    console.error('Extraction failed:', error)

    // Notification d'erreur
    await notifyError('Impossible d\'extraire les textes du dossier sélectionné.')

    console.error('❌ Erreur: Impossible d\'extraire les textes du dossier sélectionné')
  }
}
</script>
