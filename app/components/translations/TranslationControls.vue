<script setup lang="ts">
import { ref, computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useProjectsStore } from '~/stores/projects'
import { useTranslationStore } from '~/stores/translation'
import { useNotifications } from '~/composables/useNotifications'
import { useSettings } from '~/composables/useTauriSetting'
import { useOllamaCheck } from '~/composables/translation/useOllamaCheck'
import { useRunpodCheck } from '~/composables/translation/useRunpodCheck'
import { invoke } from '@tauri-apps/api/core'
import type { TextEntry } from '~/types/scanning-commands'

const projectsStore = useProjectsStore()
const translationStore = useTranslationStore()
const { notifySuccess, notifyError, notifyWarning } = useNotifications()
const { checkOllamaBeforeTranslation } = useOllamaCheck()
const { checkRunpodBeforeTranslation } = useRunpodCheck()
const settings = useSettings()

const { hasActiveSessions, selectedTextsForRetranslation } = storeToRefs(translationStore)

// Props pour recevoir le nombre de textes sélectionnés
const props = defineProps<{
  selectedTextsCount?: number
}>()

// État pour la retraduction en masse
const isRetranslatingSelected = ref(false)

// État de l'injection
const isInjecting = ref(false)
const isValidating = ref(false)
const isStartingTranslation = ref(false)
const injectionProgress = ref<{
  injection_id: string
  current_file: string
  files_processed: number
  total_files: number
  entries_injected: number
  errors: Array<{ file_path: string; error_message: string }>
  status: 'Pending' | 'InProgress' | 'Completed' | 'Partial' | 'Failed' | 'Cancelled'
} | null>(null)

// Computed pour les textes traduits
const translatedTexts = computed(() => {
  const project = projectsStore.currentProject
  if (!project) return []
  return project.extractedTexts.filter(
    text => text.translated_text && text.translated_text.trim() !== '' && text.status === 'Translated'
  )
})

// Fonction pour reconstruire le parser_id depuis location
const reconstructParserId = (location: string): string => {
  if (!location) return ''
  
  // Format WolfRPG : déjà au format parser_id, utiliser tel quel
  // "wolf_json:dump/mps/Map000.json#events[0].pages[0].list[0].stringArgs[0]"
  if (location.startsWith('wolf_json:')) {
    return location
  }
  
  // Format standard (RPG Maker) : "actor:1:name" → "actor_1_name"
  return location.replace(/:/g, '_')
}

// Valider l'injection avant de démarrer
const validateInjection = async (): Promise<boolean> => {
  const project = projectsStore.currentProject
  if (!project) {
    notifyError('Erreur', 'Aucun projet sélectionné')
    return false
  }

  if (translatedTexts.value.length === 0) {
    notifyWarning('Aucune traduction', 'Aucun texte traduit disponible pour l\'injection')
    return false
  }

  isValidating.value = true
  try {
    console.log('🔍 Validation de l\'injection:', {
      project_id: project.id,
      game_path: project.gamePath,
      total_translations: project.extractedTexts.length,
      translated_count: translatedTexts.value.length,
      untranslated_count: project.extractedTexts.length - translatedTexts.value.length
    })

    const validationResult = await invoke<{
      valid: boolean
      issues: Array<{ file_path: string; severity: string; message: string }>
      summary: {
        files_to_process: number
        entries_to_inject: number
        untranslated_entries: number
      }
    }>('validate_injection', {
      request: {
        project_id: project.id,
        game_path: project.gamePath,
        total_translations: project.extractedTexts.length,
        translated_count: translatedTexts.value.length,
        untranslated_count: project.extractedTexts.length - translatedTexts.value.length
      }
    })

    console.log('✅ Résultat de validation:', validationResult)

    if (!validationResult.valid) {
      const errors = validationResult.issues.filter(i => i.severity === 'error')
      const warnings = validationResult.issues.filter(i => i.severity === 'warning')

      console.log('❌ Erreurs de validation:', errors)
      console.log('⚠️ Avertissements de validation:', warnings)

      if (errors.length > 0) {
        const firstError = errors[0]
        const errorMessages = errors.map(e => e.message).join('; ')
        notifyError(
          'Validation échouée',
          `${errors.length} erreur(s) détectée(s): ${errorMessages}`
        )
        return false
      }

      if (warnings.length > 0) {
        // Afficher les avertissements mais continuer
        const warningMessages = warnings.map(w => w.message).join('; ')
        notifyWarning(
          'Avertissements de validation',
          warningMessages
        )
      }
    }

    return true
  } catch (error) {
    console.error('❌ Erreur lors de la validation:', error)
    const errorMessage = error instanceof Error ? error.message : String(error)
    notifyError(
      'Erreur de validation',
      `Impossible de valider l'injection: ${errorMessage}`
    )
    return false
  } finally {
    isValidating.value = false
  }
}

// Démarrer l'injection
const startInjection = async () => {
  const project = projectsStore.currentProject
  if (!project) {
    notifyError('Erreur', 'Aucun projet sélectionné')
    return
  }

  // Valider avant de démarrer
  const isValid = await validateInjection()
  if (!isValid) {
    return
  }

  isInjecting.value = true
  injectionProgress.value = null

  try {
    // Préparer les traductions pour l'injection
    // Reconstruire parser_id depuis location pour chaque texte traduit
    const translations = translatedTexts.value.map(text => {
      const parserId = reconstructParserId(text.location || '')
      if (!parserId) {
        console.warn(`⚠️ Impossible de reconstruire parser_id pour le texte ${text.id}, location: "${text.location}"`)
      }
      return {
        id: parserId || text.id, // Fallback sur l'ID si location vide
        translated_text: text.translated_text || ''
      }
    }).filter(t => t.id && t.translated_text.trim() !== '')

    if (translations.length === 0) {
      notifyError('Erreur', 'Aucune traduction valide pour l\'injection')
      return
    }

    // Démarrer l'injection
    // Retourne un tuple: (injection_id: String, total_files: usize, estimated_duration: u64)
    const result = await invoke<[string, number, number]>('start_injection', {
      request: {
        project_id: project.id,
        game_path: project.gamePath,
        translations,
        file_ids: null
      }
    })

    const [injectionId, totalFiles, estimatedDuration] = result

    // Attendre un peu pour que l'injection se termine (elle est synchrone)
    await new Promise(resolve => setTimeout(resolve, 500))

    // Récupérer le résultat de l'injection
    const injectionResult = await invoke<{
      injection_id: string
      status: string
      files_processed: number
      entries_injected: number
      errors: Array<{ file_path: string; error_message: string }>
      completed_at: string
    }>('get_injection_result', {
      injectionId: injectionId
    })

    if (injectionResult.status === 'completed') {
      notifySuccess(
        'Injection réussie',
        `${injectionResult.entries_injected} traduction(s) injectée(s) dans ${injectionResult.files_processed} fichier(s)`
      )
    } else if (injectionResult.status === 'partial') {
      notifyWarning(
        'Injection partielle',
        `${injectionResult.entries_injected} traduction(s) injectée(s) sur ${translations.length}. ${injectionResult.errors.length} erreur(s) rencontrée(s).`
      )
    } else {
      notifyError(
        'Injection échouée',
        `L'injection a échoué avec ${injectionResult.errors.length} erreur(s).`
      )
    }
  } catch (error) {
    notifyError(
      'Erreur d\'injection',
      error instanceof Error ? error.message : 'Erreur inconnue lors de l\'injection'
    )
    console.error('Erreur lors de l\'injection:', error)
  } finally {
    isInjecting.value = false
  }
}

// Computed pour savoir si l'injection est possible
const canInject = computed(() => {
  return translatedTexts.value.length > 0 && !isInjecting.value && !isValidating.value
})

// Computed pour les statistiques
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

// Fonction pour démarrer toutes les traductions
async function startAllTranslations() {
  const project = projectsStore.currentProject
  if (!project) return

  const untranslatedTexts = project.extractedTexts.filter(
    text => !text.translated_text || text.status === 'NotTranslated'
  )

  if (untranslatedTexts.length === 0) {
    notifyWarning('Aucun texte à traduire', 'Tous les textes sont déjà traduits')
    return
  }

  try {
    isStartingTranslation.value = true

    // Récupérer les settings utilisateur pour la traduction
    const userSettings = await settings.loadSettings()

    // Vérifier la connexion du provider AVANT de démarrer la traduction
    if (userSettings.provider === 'runpod') {
      const isRunpodReady = await checkRunpodBeforeTranslation()
      if (!isRunpodReady) {
        return
      }
    } else {
      const isOllamaReady = await checkOllamaBeforeTranslation()
      if (!isOllamaReady) {
        return
      }
    }

    // S'assurer que les textes sont chargés depuis la DB (avec IDs numériques)
    if (project.extractedTexts.length === 0) {
      await projectsStore.loadProjectTextsFromDB(project.id)
    }

    // Recharger les textes non traduits depuis le store (qui devrait avoir les IDs de la DB)
    const currentTexts = projectsStore.getProjectTexts(project.id)
    const untranslatedTextsFromDB = currentTexts.filter(
      text => !text.translated_text || text.status === 'NotTranslated'
    )

    // Map prompt_type to text_type for glossary filtering
    const promptTypeToTextType: Record<TextEntry['prompt_type'], string> = {
      'Character': 'character',
      'Dialogue': 'dialogue',
      'Item': 'item',
      'Skill': 'skill',
      'System': 'system',
      'General': 'general',
      'Other': 'other'
    }

    // Valider et filtrer les textes avec des IDs valides (numériques depuis la DB)
    const validTexts = untranslatedTextsFromDB
      .filter(text => {
        const id = parseInt(text.id, 10)
        if (isNaN(id) || id <= 0) {
          console.warn(`⚠️ Texte avec ID invalide ignoré: "${text.id}" (source: "${text.source_text.substring(0, 50)}...")`)
          return false
        }
        return true
      })
      .map(text => ({
        id: parseInt(text.id, 10),
        sourceText: text.source_text,
        context: text.location || undefined,
        textType: promptTypeToTextType[text.prompt_type] || undefined
      }))

    if (validTexts.length === 0) {
      console.warn('⚠️ Aucun texte valide trouvé. Textes totaux:', untranslatedTextsFromDB.length)
      notifyWarning('Aucun texte valide', `Aucun texte valide trouvé pour la traduction. ${untranslatedTextsFromDB.length > 0 ? 'Les textes ont peut-être des IDs invalides.' : 'Aucun texte non traduit trouvé.'}`)
      return
    }

    // Use model from settings based on provider
    const model = userSettings.provider === 'ollama' 
      ? userSettings.ollama.model 
      : (userSettings.provider === 'runpod' ? userSettings.runpod.model : undefined)

    await translationStore.startTranslation({
      projectId: project.id,
      texts: validTexts,
      sourceLanguage: userSettings.translation.sourceLanguage,
      targetLanguage: userSettings.translation.targetLanguage,
      model
    })
    
    notifySuccess('Traduction démarrée', `${validTexts.length} texte(s) en cours de traduction`)
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Erreur inconnue'
    notifyError('Erreur lors du démarrage', `Impossible de démarrer la traduction: ${errorMessage}`)
    console.error('Erreur lors du démarrage de la traduction:', error)
  } finally {
    isStartingTranslation.value = false
  }
}

// Fonction pour arrêter toutes les sessions actives
async function stopAllTranslations() {
  const project = projectsStore.currentProject
  if (!project) return

  try {
    // Arrêter toutes les sessions actives pour ce projet
    const runningSessions = translationStore.activeSessions.filter(
      s => s.status === 'running' && s.project_id === project.id
    )

    for (const session of runningSessions) {
      await translationStore.stopSession(session.session_id)
    }
  } catch (error) {
    console.error('Erreur lors de l\'arrêt des traductions:', error)
  }
}

// Fonction pour retraduire les textes sélectionnés
async function handleRetranslateSelected() {
  const project = projectsStore.currentProject
  if (!project) {
    notifyError('Erreur', 'Aucun projet sélectionné')
    return
  }

  const selectedTexts = selectedTextsForRetranslation.value
  
  if (selectedTexts.length < 2) {
    notifyWarning('Sélection insuffisante', 'Veuillez sélectionner au moins 2 textes')
    return
  }

  try {
    isRetranslatingSelected.value = true

    // Récupérer les settings utilisateur AVANT de vérifier la connexion
    const userSettings = await settings.loadSettings()

    // Vérifier la connexion du provider AVANT de démarrer la traduction
    if (userSettings.provider === 'runpod') {
      const isRunpodReady = await checkRunpodBeforeTranslation()
      if (!isRunpodReady) {
        return
      }
    } else {
      const isOllamaReady = await checkOllamaBeforeTranslation()
      if (!isOllamaReady) {
        return
      }
    }

    // Mettre les textes en statut "InProgress" avant de démarrer
    for (const text of selectedTexts) {
      const textId = parseInt(text.id, 10)
      if (!isNaN(textId) && textId > 0) {
        await translationStore.setTextInProgress(textId)
      }
    }

    // Préparer les textes pour la retraduction
    const promptTypeToTextType: Record<TextEntry['prompt_type'], string> = {
      'Character': 'character',
      'Dialogue': 'dialogue',
      'Item': 'item',
      'Skill': 'skill',
      'System': 'system',
      'General': 'general',
      'Other': 'other'
    }

    const textsToRetranslate = selectedTexts
      .filter(text => {
        const id = parseInt(text.id, 10)
        return !isNaN(id) && id > 0
      })
      .map(text => ({
        id: parseInt(text.id, 10),
        sourceText: text.source_text,
        context: text.location || undefined,
        textType: promptTypeToTextType[text.prompt_type as TextEntry['prompt_type']] || undefined
      }))

    if (textsToRetranslate.length === 0) {
      notifyWarning('Aucun texte valide', 'Aucun texte valide trouvé pour la retraduction')
      return
    }

    // Use model from settings based on provider
    const model = userSettings.provider === 'ollama' 
      ? userSettings.ollama.model 
      : (userSettings.provider === 'runpod' ? userSettings.runpod.model : undefined)

    // Démarrer la traduction
    await translationStore.startTranslation({
      projectId: project.id,
      texts: textsToRetranslate,
      sourceLanguage: userSettings.translation.sourceLanguage,
      targetLanguage: userSettings.translation.targetLanguage,
      model
    })

    // Réinitialiser la sélection
    translationStore.setSelectedTextsForRetranslation([])
    // Note: La sélection dans FinalTextsTable sera réinitialisée automatiquement
    // quand les textes passeront en statut InProgress et disparaîtront de la table

    notifySuccess(
      'Retraduction démarrée',
      `${textsToRetranslate.length} texte(s) en cours de retraduction. Les textes apparaîtront dans l'onglet "En cours".`
    )
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Erreur inconnue'
    notifyError('Erreur lors de la retraduction', errorMessage)
    console.error('Erreur lors de la retraduction:', error)
  } finally {
    isRetranslatingSelected.value = false
  }
}
</script>

<template>
  <div class="translation-controls flex gap-2 flex-wrap justify-center">
    <!-- Bouton Commencer la traduction -->
    <UButton
      v-if="!hasActiveSessions && stats.raw > 0"
      icon="i-heroicons-play-circle"
      color="primary"
      size="lg"
      :loading="isStartingTranslation"
      @click="startAllTranslations"
    >
      Commencer la traduction
    </UButton>

    <!-- Bouton Arrêter les traductions -->
    <UButton
      v-if="hasActiveSessions"
      icon="i-heroicons-stop-circle"
      color="error"
      size="lg"
      @click="stopAllTranslations"
    >
      Arrêter les traductions
    </UButton>

    <!-- Bouton Retraduire les sélectionnés -->
    <UButton
      v-if="selectedTextsCount && selectedTextsCount >= 2"
      icon="i-heroicons-arrow-path"
      color="warning"
      size="lg"
      :loading="isRetranslatingSelected"
      @click="handleRetranslateSelected"
    >
      Retraduire les {{ selectedTextsCount }} textes sélectionnés
    </UButton>

    <!-- Bouton Injecter les traductions -->
    <UButton
      v-if="stats.final > 0"
      icon="i-heroicons-arrow-down-tray"
      color="primary"
      size="lg"
      :loading="isInjecting || isValidating"
      :disabled="!canInject"
      @click="startInjection"
    >
      <span v-if="isValidating">Validation en cours...</span>
      <span v-else-if="isInjecting">Injection en cours...</span>
      <span v-else>
        Injecter les traductions
        <span v-if="translatedTexts.length > 0" class="ml-2 text-sm opacity-75">
          ({{ translatedTexts.length }} texte(s))
        </span>
      </span>
    </UButton>

    <!-- Message d'aide -->
    <div v-if="translatedTexts.length === 0 && stats.final === 0" class="mt-2 w-full text-center">
      <p class="text-sm text-gray-600 dark:text-gray-400">
        Aucune traduction disponible. Traduisez d'abord les textes avant de les injecter.
      </p>
    </div>
  </div>
</template>
