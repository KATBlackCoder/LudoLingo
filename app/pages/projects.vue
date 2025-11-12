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

      <!-- Bouton de lancement des traductions -->
      <div v-if="projectsStore.currentProject && extractedTexts.length > 0 && !hasActiveSessions" class="flex justify-center gap-4 mb-6">
        <UButton
          icon="i-heroicons-play-circle"
          color="primary"
          size="lg"
          @click="startAllTranslations"
        >
          Commencer toutes les traductions
        </UButton>
      </div>

      <!-- Message si traductions en cours -->
      <div v-else-if="hasActiveSessions" class="text-center mb-6">
        <UAlert
          icon="i-heroicons-language"
          color="info"
          variant="subtle"
          title="Traductions en cours"
          description="Les traductions sont en cours d'exécution. Vous pouvez continuer à travailler pendant que la traduction se déroule en arrière-plan."
        />
      </div>


      <!-- Résultats d'extraction (tous les textes) -->
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
import { storeToRefs } from 'pinia'
import { useProjectsStore } from '~/stores/projects'
import { useTranslationStore } from '~/stores/translation'
import { useSettings } from '~/composables/useTauriSetting'
import { TextsTable, ProjectStats } from '~/components/projects'
import ProjectLoader from '~/components/projects/ProjectLoader.vue'

const projectsStore = useProjectsStore()
const translationStore = useTranslationStore()
const { getProjectTexts } = projectsStore

// Stores réactifs pour les sessions de traduction
const { hasActiveSessions } = storeToRefs(translationStore)

// Textes extraits depuis le store (projet actuel) - optimisé
const extractedTexts = computed(() => {
  const project = projectsStore.currentProject
  if (!project) return []

  // Utiliser la fonction memoïsée du store
  return getProjectTexts(project.id) || []
})

// Fonctions pour les boutons globaux
async function startAllTranslations() {
  const untranslatedTexts = extractedTexts.value
    .filter(text => text.status === 'NotTranslated')

  if (untranslatedTexts.length === 0) {
    console.log('Aucun texte à traduire')
    return
  }

  try {
    // Récupérer les settings utilisateur
    const settings = useSettings()
    const userSettings = await settings.loadSettings()

    console.log('🔧 Utilisation des settings utilisateur:', {
      model: userSettings.ollama.model,
      sourceLanguage: userSettings.translation.sourceLanguage,
      targetLanguage: userSettings.translation.targetLanguage
    })

    // Valider et filtrer les textes avec des IDs valides
    const validTexts = untranslatedTexts
      .filter(text => {
        const id = parseInt(text.id)
        if (isNaN(id) || id <= 0) {
          console.warn(`Texte invalide ignoré (ID: ${text.id}):`, text)
          return false
        }
        return true
      })
      .map(text => ({
        id: parseInt(text.id),
        sourceText: text.source_text,
        context: text.context || undefined
      }))

    if (validTexts.length === 0) {
      console.error('Aucun texte valide trouvé pour la traduction')
      return
    }

    console.log(`Envoi de ${validTexts.length} textes valides sur ${untranslatedTexts.length} trouvés`)

    await translationStore.startTranslation({
      projectId: projectsStore.currentProject!.id,
      texts: validTexts,
      sourceLanguage: userSettings.translation.sourceLanguage,
      targetLanguage: userSettings.translation.targetLanguage,
      model: userSettings.ollama.model
    })
    console.log(`Traduction démarrée pour ${validTexts.length} textes avec le modèle ${userSettings.ollama.model}`)

    // Rediriger vers la page de traduction
    //await navigateTo('/translation')
  } catch (error) {
    console.error('Erreur lors du démarrage de la traduction:', error)
  }
}


// Gestionnaire pour le chargement d'un projet
function onProjectLoaded() {
  // Le projet est maintenant chargé et affiché automatiquement via les computed
  console.log('Projet chargé avec succès')
  // Les textes seront chargés en arrière-plan par ProjectLoader
}

</script>
