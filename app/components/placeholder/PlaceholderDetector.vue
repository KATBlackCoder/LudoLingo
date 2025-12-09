<template>
  <div class="placeholder-detector" v-if="shouldShowDetector">
    <!-- Bouton d'analyse -->
    <div class="mb-4">
      <UButton
        icon="i-heroicons-magnifying-glass"
        variant="outline"
        size="sm"
        :loading="isAnalyzing"
        @click="analyzePlaceholders"
      >
        {{ isAnalyzing ? 'Analyse en cours...' : 'Analyser les placeholders manquants' }}
      </UButton>
    </div>

    <!-- Résultats de l'analyse -->
    <UCard v-if="detectionResult && detectionResult.unconvertedCodes.length > 0">
      <template #header>
        <div class="flex items-center gap-3">
          <div class="text-orange-600 dark:text-orange-400">
            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
            </svg>
          </div>
          <div>
            <h3 class="text-lg font-semibold">
              Codes de formatage non convertis détectés
            </h3>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {{ detectionResult.unconvertedCodes.length }} codes trouvés • {{ detectionResult.totalTextsAnalyzed }} textes analysés • {{ detectionResult.analysisDuration }}ms
            </p>
          </div>
        </div>
      </template>

      <div class="space-y-4">
        <!-- Tableau des codes -->
        <div class="border rounded-lg overflow-hidden">
          <div class="bg-gray-50 dark:bg-gray-800 px-4 py-3 border-b">
            <div class="grid grid-cols-12 gap-4 font-medium text-sm">
              <div class="col-span-3">Code détecté</div>
              <div class="col-span-2">Occurrences</div>
              <div class="col-span-3">Type d'entrée</div>
              <div class="col-span-4">Exemple de contexte</div>
            </div>
          </div>

          <div class="divide-y max-h-96 overflow-y-auto">
            <div
              v-for="(code, index) in detectionResult.unconvertedCodes"
              :key="index"
              class="px-4 py-3 hover:bg-gray-50 dark:hover:bg-gray-800/50"
            >
              <div class="grid grid-cols-12 gap-4 items-center text-sm">
                <div class="col-span-3">
                  <code class="bg-red-100 dark:bg-red-900/20 text-red-800 dark:text-red-300 px-2 py-1 rounded text-xs font-mono">
                    {{ code.code }}
                  </code>
                </div>
                <div class="col-span-2">
                  <UBadge
                    color="primary"
                    variant="subtle"
                    class="rounded-full"
                  >
                    {{ code.occurrences }}
                  </UBadge>
                </div>
                <div class="col-span-3 text-gray-600 dark:text-gray-400">
                  {{ formatEntryType(code.entryType) }}
                </div>
                <div class="col-span-4 text-gray-700 dark:text-gray-300 truncate">
                  <span :title="code.exampleTexts[0]">
                    {{ code.exampleTexts[0] }}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Section d'aide -->
        <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
          <div class="flex items-start gap-3">
            <div class="text-blue-600 dark:text-blue-400 mt-0.5">
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path>
              </svg>
            </div>
            <div>
              <h4 class="font-medium text-blue-900 dark:text-blue-100 mb-2">
                Comment ajouter ces codes dans les formatters
              </h4>
              <ol class="text-sm text-blue-800 dark:text-blue-200 space-y-1 mb-3">
                <li>• Ouvrez <code class="bg-blue-200 dark:bg-blue-800 px-1 rounded text-xs">wolf_rpg_formatter.rs</code> ou <code class="bg-blue-200 dark:bg-blue-800 px-1 rounded text-xs">rpg_maker_formatter.rs</code></li>
                <li>• Ajoutez une regex pour chaque code détecté</li>
                <li>• Ajoutez la conversion vers un placeholder universel</li>
                <li>• Ajoutez la restauration pour la conversion inverse</li>
              </ol>
              <div class="bg-blue-100 dark:bg-blue-800/50 rounded p-2">
                <p class="text-xs text-blue-900 dark:text-blue-100">
                  <strong>Exemple :</strong> Pour <code class="bg-white dark:bg-blue-700 px-1 rounded">\unknown</code> →
                  <code class="bg-white dark:bg-blue-700 px-1 rounded">result.replace("\\unknown", "[UNKNOWN]")</code>
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </UCard>

    <!-- Message si aucun code trouvé -->
    <UCard v-else-if="detectionResult && detectionResult.unconvertedCodes.length === 0" class="border-green-200 bg-green-50 dark:bg-green-900/20">
      <div class="flex items-center gap-3">
        <div class="text-green-600 dark:text-green-400">
          <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
          </svg>
        </div>
        <div>
          <h3 class="text-lg font-semibold text-green-900 dark:text-green-100">
            Aucun code non converti détecté
          </h3>
          <p class="text-sm text-green-700 dark:text-green-300">
            Tous les codes de formatage sont déjà pris en charge par les formatters actuels.
          </p>
        </div>
      </div>
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useProjectsStore } from '~/stores/projects'
import { analyzeProjectPlaceholders, type DatabaseDetectionResult } from '~/composables/placeholderDetector'

interface Props {
  autoAnalyze?: boolean // Analyser automatiquement au montage
}

const props = withDefaults(defineProps<Props>(), {
  autoAnalyze: false
})

// État
const projectsStore = useProjectsStore()
const isAnalyzing = ref(false)
const detectionResult = ref<DatabaseDetectionResult | null>(null)

// Computed
const currentProject = computed(() => projectsStore.currentProject)

// Vérifier si le projet a des textes (donc a été scanné)
const projectHasTexts = computed(() => {
  if (!currentProject.value) return false
  return projectsStore.getProjectTexts(currentProject.value.id).length > 0
})

// Le composant n'est visible que si on a un projet actif avec des textes
const shouldShowDetector = computed(() => {
  return currentProject.value && projectHasTexts.value
})

// Fonctions
async function analyzePlaceholders() {
  if (!currentProject.value) return

  try {
    isAnalyzing.value = true
    detectionResult.value = await analyzeProjectPlaceholders(currentProject.value.id)
  } catch (error) {
    console.error('Erreur lors de l\'analyse des placeholders:', error)
  } finally {
    isAnalyzing.value = false
  }
}

function formatEntryType(entryType: string): string {
  const typeMap: Record<string, string> = {
    'actor_text_unit': 'Acteurs',
    'item_text_unit': 'Objets',
    'weapon_text_unit': 'Armes',
    'armor_text_unit': 'Armures',
    'skill_text_unit': 'Compétences',
    'state_text_unit': 'États',
    'class_text_unit': 'Classes',
    'troop_text_unit': 'Groupes',
    'common_event_text_unit': 'Événements communs',
    'map_info_text_unit': 'Infos cartes',
    'map_data_text_unit': 'Données cartes',
    'system_text_unit': 'Système'
  }

  return typeMap[entryType] || entryType.replace('_text_unit', '').replace('_', ' ')
}

// Analyser automatiquement si demandé et que le projet a des textes
if (props.autoAnalyze && shouldShowDetector.value) {
  analyzePlaceholders()
}

// Exposer les fonctions pour utilisation externe
defineExpose({
  analyzePlaceholders,
  isAnalyzing,
  detectionResult
})
</script>
