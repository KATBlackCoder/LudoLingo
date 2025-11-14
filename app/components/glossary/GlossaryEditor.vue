<script setup lang="ts">
import { ref, watch } from 'vue'
import { useGlossaryStore } from '~/stores/glossary'
import { useNotifications } from '~/composables/useNotifications'
import type { GlossaryEntry, CreateGlossaryEntry } from '~/composables/db/glossary'

interface Props {
  open: boolean
  entry?: GlossaryEntry | null
}

interface Emits {
  (e: 'update:open', value: boolean): void
  (e: 'saved'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const glossaryStore = useGlossaryStore()
const { notifySuccess, notifyError } = useNotifications()

const form = ref<CreateGlossaryEntry>({
  source_term: '',
  translated_term: '',
  source_language: 'ja',
  target_language: 'fr',
  category: 'general'
})

const isSaving = ref(false)

const categoryOptions = [
  { label: 'Général', value: 'general' },
  { label: 'Personnage', value: 'character' },
  { label: 'Objet', value: 'item' },
  { label: 'Lieu', value: 'location' },
  { label: 'Système', value: 'system' },
  { label: 'Compétence', value: 'skill' }
]

const languageOptions = [
  { label: 'Japonais', value: 'ja' },
  { label: 'Français', value: 'fr' },
  { label: 'Anglais', value: 'en' },
  { label: 'Espagnol', value: 'es' },
  { label: 'Allemand', value: 'de' },
  { label: 'Italien', value: 'it' },
  { label: 'Portugais', value: 'pt' },
  { label: 'Chinois', value: 'zh' },
  { label: 'Coréen', value: 'ko' }
]

// Synchroniser le formulaire avec l'entrée fournie (mode édition)
watch(() => props.entry, (entry) => {
  if (entry) {
    form.value = {
      source_term: entry.source_term,
      translated_term: entry.translated_term,
      source_language: entry.source_language,
      target_language: entry.target_language,
      category: entry.category
    }
  } else {
    // Réinitialiser le formulaire pour création
    form.value = {
      source_term: '',
      translated_term: '',
      source_language: 'ja',
      target_language: 'fr',
      category: 'general'
    }
  }
}, { immediate: true })

// Réinitialiser le formulaire quand le modal se ferme
watch(() => props.open, (open) => {
  if (!open) {
    form.value = {
      source_term: '',
      translated_term: '',
      source_language: 'ja',
      target_language: 'fr',
      category: 'general'
    }
  }
})

const isEditMode = computed(() => !!props.entry)

const handleSave = async () => {
  if (!form.value.source_term.trim() || !form.value.translated_term.trim()) {
    notifyError('Erreur de validation', 'Le terme source et la traduction sont requis')
    return
  }

  isSaving.value = true
  try {
    if (isEditMode.value && props.entry) {
      // Mode édition
      await glossaryStore.updateEntry(props.entry.id, {
        source_term: form.value.source_term.trim(),
        translated_term: form.value.translated_term.trim(),
        source_language: form.value.source_language,
        target_language: form.value.target_language,
        category: form.value.category
      })
      notifySuccess('Entrée mise à jour', 'L\'entrée du glossaire a été mise à jour avec succès')
    } else {
      // Mode création
      await glossaryStore.createEntry({
        source_term: form.value.source_term.trim(),
        translated_term: form.value.translated_term.trim(),
        source_language: form.value.source_language,
        target_language: form.value.target_language,
        category: form.value.category
      })
      notifySuccess('Entrée créée', 'L\'entrée du glossaire a été créée avec succès')
    }
    emit('saved')
    closeModal()
  } catch (error) {
    notifyError(
      'Erreur de sauvegarde',
      error instanceof Error ? error.message : 'Une erreur est survenue lors de la sauvegarde'
    )
  } finally {
    isSaving.value = false
  }
}

const closeModal = () => {
  emit('update:open', false)
}
</script>

<template>
  <UModal
    :open="props.open"
    :title="isEditMode ? 'Modifier l\'entrée du glossaire' : 'Nouvelle entrée du glossaire'"
    :description="isEditMode ? 'Modifiez les informations de l\'entrée du glossaire' : 'Ajoutez une nouvelle entrée au glossaire pour enrichir vos traductions'"
    @update:open="(value: boolean) => emit('update:open', value)"
  >
    <template #body>
      <div class="space-y-4">
        <UFormField label="Terme source" required>
          <UInput
            v-model="form.source_term"
            placeholder="Ex: 勇者"
            :disabled="isSaving"
          />
        </UFormField>

        <UFormField label="Traduction" required>
          <UInput
            v-model="form.translated_term"
            placeholder="Ex: Héros"
            :disabled="isSaving"
          />
        </UFormField>

        <div class="grid grid-cols-2 gap-4">
          <UFormField label="Langue source">
            <USelect
              v-model="form.source_language"
              :items="languageOptions"
              value-key="value"
              :disabled="isSaving"
            />
          </UFormField>

          <UFormField label="Langue cible">
            <USelect
              v-model="form.target_language"
              :items="languageOptions"
              value-key="value"
              :disabled="isSaving"
            />
          </UFormField>
        </div>

        <UFormField label="Catégorie">
          <USelect
            v-model="form.category"
            :items="categoryOptions"
            value-key="value"
            :disabled="isSaving"
          />
        </UFormField>
      </div>
    </template>

    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton
          variant="ghost"
          color="neutral"
          @click="closeModal"
          :disabled="isSaving"
        >
          Annuler
        </UButton>
        <UButton
          color="primary"
          @click="handleSave"
          :loading="isSaving"
        >
          {{ isEditMode ? 'Mettre à jour' : 'Créer' }}
        </UButton>
      </div>
    </template>
  </UModal>
</template>

