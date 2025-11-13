<script setup lang="ts">
import { ref, watch } from 'vue'
import { useTranslationStore } from '~/stores/translation'
import { useNotifications } from '~/composables/useNotifications'
import { useSettings } from '~/composables/useTauriSetting'
import { translateSingleText } from '~/composables/db/texts/translation'
import type { TextEntry } from '~/types/scanning-commands'

interface Props {
  open: boolean
  text?: TextEntry | null
}

interface Emits {
  (e: 'update:open', value: boolean): void
  (e: 'saved'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const translationStore = useTranslationStore()
const { notifySuccess, notifyError } = useNotifications()
const settings = useSettings()

const editedTranslation = ref('')
const isRetranslating = ref(false)

// Synchroniser editedTranslation avec le texte fourni
watch(() => props.text, (text) => {
  if (text) {
    editedTranslation.value = text.translated_text || ''
  } else {
    editedTranslation.value = ''
  }
}, { immediate: true })

// Retraduire avec AI
const handleRetranslate = async () => {
  if (!props.text) return

  isRetranslating.value = true
  try {
    // Récupérer les settings utilisateur pour utiliser le bon modèle
    const userSettings = await settings.loadSettings()
    
    const result = await translateSingleText(
      props.text.source_text,
      userSettings.translation.sourceLanguage,
      userSettings.translation.targetLanguage,
      props.text.context || undefined,
      userSettings.ollama.model
    )

    if (result.success && result.data) {
      editedTranslation.value = result.data.translated_text
      notifySuccess('Traduction générée avec succès', 'Retraduction réussie')
    } else {
      throw new Error(result.error || 'Échec de la retraduction')
    }
  } catch (error) {
    notifyError(
      'Erreur de retraduction',
      error instanceof Error ? error.message : 'Une erreur est survenue lors de la retraduction'
    )
  } finally {
    isRetranslating.value = false
  }
}

// Sauvegarder la traduction (manuelle ou AI)
const handleSaveTranslation = async () => {
  if (!props.text || !editedTranslation.value.trim()) {
    notifyError('Erreur', 'La traduction ne peut pas être vide')
    return
  }

  try {
    const textId = parseInt(props.text.id, 10)
    if (isNaN(textId)) {
      throw new Error('ID de texte invalide')
    }

    await translationStore.applyTranslation(textId, editedTranslation.value.trim(), 'manual')
    notifySuccess('Traduction sauvegardée', 'La traduction a été mise à jour avec succès')
    emit('saved')
    closeModal()
  } catch (error) {
    notifyError(
      'Erreur de sauvegarde',
      error instanceof Error ? error.message : 'Une erreur est survenue lors de la sauvegarde'
    )
  }
}

// Fermer le modal
const closeModal = () => {
  emit('update:open', false)
  editedTranslation.value = ''
}
</script>

<template>
  <UModal
    :open="props.open"
    title="Modifier la traduction"
    description="Modifiez la traduction manuellement ou utilisez l'IA pour retraduire le texte."
    :ui="{
      wrapper: 'w-full sm:max-w-2xl'
    }"
    @update:open="(value: boolean) => emit('update:open', value)"
  >
    <template #body>
      <div class="space-y-4">
        <!-- Texte original -->
        <div>
          <label class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 block">
            Texte original
          </label>
          <div class="p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
            <p class="text-sm text-gray-700 dark:text-gray-300 whitespace-pre-wrap">
              {{ text?.source_text }}
            </p>
          </div>
        </div>

        <!-- Contexte (si disponible) -->
        <div v-if="text?.context && text.context !== '-'">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 block">
            Contexte
          </label>
          <div class="p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {{ text.context }}
            </p>
          </div>
        </div>

        <!-- Traduction -->
        <div>
          <label class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 block">
            Traduction
          </label>
          <UTextarea
            v-model="editedTranslation"
            :rows="6"
            placeholder="Entrez la traduction..."
            class="w-full"
          />
        </div>
      </div>
    </template>

    <template #footer="{ close }">
      <div class="flex items-center justify-between">
        <UButton
          color="primary"
          variant="outline"
          icon="i-heroicons-arrow-path"
          :loading="isRetranslating"
          @click="handleRetranslate"
        >
          Retraduire avec AI
        </UButton>
        <div class="flex gap-2">
          <UButton
            color="neutral"
            variant="ghost"
            @click="close"
          >
            Annuler
          </UButton>
          <UButton
            color="primary"
            @click="handleSaveTranslation"
          >
            Sauvegarder
          </UButton>
        </div>
      </div>
    </template>
  </UModal>
</template>

