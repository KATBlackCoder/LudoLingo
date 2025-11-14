<script setup lang="ts">
import { ref, computed } from 'vue'
import { useProjectsStore } from '~/stores/projects'
import { useNotifications } from '~/composables/useNotifications'
import { invoke } from '@tauri-apps/api/core'
import type { TextEntry } from '~/types/scanning-commands'

const projectsStore = useProjectsStore()
const { notifySuccess, notifyError, notifyWarning } = useNotifications()

// État de l'injection
const isInjecting = ref(false)
const isValidating = ref(false)
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
  // Reconstruire parser_id : "actor:1:name" → "actor_1_name"
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
</script>

<template>
  <div class="injection-button">
    <UButton
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
    <div v-if="translatedTexts.length === 0" class="mt-2">
      <p class="text-sm text-gray-600 dark:text-gray-400">
        Aucune traduction disponible. Traduisez d'abord les textes avant de les injecter.
      </p>
    </div>
  </div>
</template>

