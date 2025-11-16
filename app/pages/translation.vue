<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useProjectsStore } from '~/stores/projects'
import { useTranslationStore } from '~/stores/translation'
import { useNotifications } from '~/composables/useNotifications'
import { isConnectionError } from '~/utils/connectionErrors'
import RawTextsTable from '~/components/translations/RawTextsTable.vue'
import InProgressTable from '~/components/translations/InProgressTable.vue'
import FinalTextsTable from '~/components/translations/FinalTextsTable.vue'
import TranslationControls from '~/components/translations/TranslationControls.vue'

const projectsStore = useProjectsStore()
const translationStore = useTranslationStore()
const { notifyError, notifyWarning, notifySuccess } = useNotifications()

const currentTab = ref<'raw' | 'in-progress' | 'final'>('raw')

// Stores réactifs pour les sessions de traduction
const { hasActiveSessions } = storeToRefs(translationStore)

// État pour le nombre de textes sélectionnés
const selectedTextsCount = ref(0)

// Handler pour les changements de sélection
const handleSelectionChange = (count: number) => {
  selectedTextsCount.value = count
}

// Statistiques globales
const stats = computed(() => {
  const project = projectsStore.currentProject
  if (!project) return { raw: 0, inProgress: 0, final: 0 }
  
  const raw = project.extractedTexts.filter(
    t => !t.translated_text || t.status === 'NotTranslated'
  ).length
  
  const inProgress = project.extractedTexts.filter(t => {
    const textIdNum = parseInt(t.id, 10)
    const isNumericId = !isNaN(textIdNum)
    return t.status === 'InProgress' || 
           (isNumericId && translationStore.textsBeingTranslated.has(textIdNum))
  }).length
  
  const final = project.extractedTexts.filter(
    t => t.translated_text && t.status === 'Translated'
  ).length
  
  return { raw, inProgress, final }
})

// Charger les sessions de traduction au montage
onMounted(async () => {
  if (projectsStore.currentProject) {
    await translationStore.loadProjectSessions(projectsStore.currentProject.id)
  }
})

// Surveiller les changements de projet
watch(() => projectsStore.currentProject, async (project) => {
  if (project) {
    await translationStore.loadProjectSessions(project.id)
    // Réinitialiser la sélection quand on change de projet
    selectedTextsCount.value = 0
  }
})

// Surveiller les changements d'onglet pour réinitialiser la sélection
watch(currentTab, () => {
  if (currentTab.value !== 'final') {
    selectedTextsCount.value = 0
  }
})

// Surveiller les sessions complétées pour afficher une notification
watch(
  () => translationStore.activeSessions.filter(s => s.status === 'completed'),
  async (completedSessions, oldCompletedSessions) => {
    if (completedSessions.length > 0) {
      // Trouver les nouvelles sessions complétées (qui n'étaient pas complétées avant)
      const newCompletedSessions = completedSessions.filter(
        session => !oldCompletedSessions?.some(old => old.session_id === session.session_id)
      )
      
      if (newCompletedSessions.length > 0) {
        const latestCompleted = newCompletedSessions[newCompletedSessions.length - 1]
        if (latestCompleted) {
          const progress = translationStore.getSessionProgress(latestCompleted.session_id)
          
          // Recharger les textes depuis la DB pour mettre à jour les statistiques
          const project = projectsStore.currentProject
          if (project) {
            await projectsStore.loadProjectTextsFromDB(project.id)
          }
          
          if (progress) {
            const totalProcessed = progress.processed_count
            const totalCount = progress.total_count
            const errorCount = progress.errors.length
            const successCount = progress.successful_translations.length
            
            if (errorCount === 0) {
              notifySuccess(
                'Traduction terminée',
                `${successCount} texte(s) traduit(s) avec succès sur ${totalCount} texte(s).`
              )
            } else {
              notifyWarning(
                'Traduction terminée avec erreurs',
                `${successCount} texte(s) traduit(s) avec succès sur ${totalCount} texte(s). ${errorCount} erreur(s) rencontrée(s).`
              )
            }
          } else {
            notifySuccess(
              'Traduction terminée',
              'La traduction s\'est terminée avec succès.'
            )
          }
        }
      }
    }
  },
  { deep: true }
)

// Surveiller les sessions en erreur pour détecter les problèmes de connexion
watch(
  () => translationStore.activeSessions.filter(s => s.status === 'error'),
  (errorSessions, oldErrorSessions) => {
    if (errorSessions.length > 0) {
      // Trouver les nouvelles sessions en erreur (qui n'étaient pas en erreur avant)
      const newErrorSessions = errorSessions.filter(
        session => !oldErrorSessions?.some(old => old.session_id === session.session_id)
      )
      
      if (newErrorSessions.length > 0) {
        const latestError = newErrorSessions[newErrorSessions.length - 1]
        if (latestError) {
          const progress = translationStore.getSessionProgress(latestError.session_id)
          
          // Vérifier si c'est une erreur de connexion
          if (progress) {
            const hasConnectionError = progress.errors.some(err => 
              isConnectionError(err.error_message)
            )
            
            if (hasConnectionError) {
              notifyError(
                'Connexion Ollama perdue',
                'La connexion à Ollama a été perdue. La traduction a été arrêtée automatiquement. Veuillez vérifier votre configuration Ollama.'
              )
            } else {
              // Erreur générale
              const errorCount = progress.errors.length
              notifyError(
                'Erreur de traduction',
                `La traduction a échoué avec ${errorCount} erreur(s). Vérifiez les détails dans l'onglet "En cours".`
              )
            }
          } else {
            notifyError(
              'Erreur de traduction',
              'Une erreur est survenue lors de la traduction.'
            )
          }
        }
      }
    }
  },
  { deep: true }
)
</script>

<template>
  <div class="space-y-6">
    <!-- Premier Container : En-tête, contrôles et statistiques -->
    <UContainer class="py-8">
      <div class="space-y-6">
        <!-- En-tête -->
        <div class="text-center">
          <h1 class="text-3xl font-bold mb-4">Interface de Traduction</h1>
          <p class="text-lg text-gray-600 dark:text-gray-400">
            Suivez la progression de vos traductions en temps réel
          </p>
        </div>

        <!-- Vérification projet -->
        <div v-if="!projectsStore.currentProject" class="text-center py-12">
          <UAlert
            icon="i-heroicons-exclamation-triangle"
            color="warning"
            title="Aucun projet sélectionné"
            description="Commencez par scanner un jeu depuis la page d'accueil pour extraire des textes"
          />
          <div class="mt-4">
            <UButton
              icon="i-heroicons-arrow-left"
              color="primary"
              variant="outline"
              to="/"
            >
              Retour à l'accueil
            </UButton>
          </div>
        </div>

        <!-- Interface avec contrôles -->
        <div v-else class="space-y-6">
          <!-- Boutons de contrôle de traduction -->
          <TranslationControls :selected-texts-count="selectedTextsCount" />

          <!-- Message si traductions en cours -->
          <div v-if="hasActiveSessions" class="text-center">
            <UAlert
              icon="i-heroicons-language"
              color="info"
              variant="subtle"
              title="Traductions en cours"
              description="Les traductions sont en cours d'exécution. Vous pouvez continuer à travailler pendant que la traduction se déroule en arrière-plan."
            />
          </div>

          <!-- Statistiques rapides -->
          <div class="grid grid-cols-3 gap-4">
            <UCard>
              <div class="text-center">
                <div class="text-3xl font-bold text-gray-900 dark:text-white">
                  {{ stats.raw }}
                </div>
                <div class="text-sm text-gray-600 dark:text-gray-400 mt-1">
                  Textes bruts
                </div>
              </div>
            </UCard>
            
            <UCard>
              <div class="text-center">
                <div class="text-3xl font-bold text-warning">
                  {{ stats.inProgress }}
                </div>
                <div class="text-sm text-gray-600 dark:text-gray-400 mt-1">
                  En cours
                </div>
              </div>
            </UCard>
            
            <UCard>
              <div class="text-center">
                <div class="text-3xl font-bold text-success">
                  {{ stats.final }}
                </div>
                <div class="text-sm text-gray-600 dark:text-gray-400 mt-1">
                  Traduits
                </div>
              </div>
            </UCard>
          </div>

          <!-- Onglets -->
          <UTabs v-model="currentTab" :items="[
            { label: `Textes Bruts (${stats.raw})`, value: 'raw' },
            { label: `En Cours (${stats.inProgress})`, value: 'in-progress' },
            { label: `Résultats (${stats.final})`, value: 'final' }
          ]" />
        </div>
      </div>
    </UContainer>

    <!-- Deuxième Container : Tables de traduction -->
    <UContainer v-if="projectsStore.currentProject" class="py-6">
      <div class="mt-6">
        <RawTextsTable v-if="currentTab === 'raw'" />
        <InProgressTable v-if="currentTab === 'in-progress'" />
        <FinalTextsTable 
          v-if="currentTab === 'final'" 
          @selection-change="handleSelectionChange"
        />
      </div>
    </UContainer>
  </div>
</template>

