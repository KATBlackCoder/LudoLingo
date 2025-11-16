<template>
  <div class="space-y-6">
    <!-- Header -->
    <div>
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white flex items-center gap-2 mb-1">
        <UIcon name="i-heroicons-language" class="h-5 w-5 text-blue-600 dark:text-blue-400" />
        Langues de Traduction
      </h3>
      <p class="text-sm text-gray-600 dark:text-gray-400">
        Configuration des langues source et cible par défaut pour Ollama
      </p>
    </div>

    <!-- Language Settings -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- Source Language -->
      <UCard>
        <template #header>
          <h4 class="text-md font-medium text-gray-900 dark:text-white">
            Langue Source
          </h4>
        </template>

        <div class="space-y-4">
          <UFormField label="Langue par défaut" required>
            <USelect
              :model-value="props.settings?.translation?.sourceLanguage"
              :items="availableLanguages"
              value-key="value"
              :icon="sourceLanguageIcon"
              placeholder="Sélectionner une langue"
              @update:model-value="emitSourceLanguage($event)"
            />
          </UFormField>

          <div class="text-xs text-gray-500 dark:text-gray-400">
            Cette langue sera utilisée comme source par défaut pour toutes les traductions automatiques.
          </div>
        </div>
      </UCard>

      <!-- Target Language -->
      <UCard>
        <template #header>
          <h4 class="text-md font-medium text-gray-900 dark:text-white">
            Langue Cible
          </h4>
        </template>

        <div class="space-y-4">
          <UFormField label="Langue par défaut" required>
            <USelect
              :model-value="props.settings?.translation?.targetLanguage"
              :items="availableLanguages"
              value-key="value"
              :icon="targetLanguageIcon"
              placeholder="Sélectionner une langue"
              @update:model-value="emitTargetLanguage($event)"
            />
          </UFormField>

          <div class="text-xs text-gray-500 dark:text-gray-400">
            Cette langue sera utilisée comme cible par défaut pour toutes les traductions automatiques.
          </div>
        </div>
      </UCard>
    </div>

  </div>
</template>

<script setup lang="ts">
// Available languages for translation
const availableLanguages = [
  { label: 'Français', value: 'fr', icon: 'i-flagpack:fr' },
  { label: 'Anglais', value: 'en', icon: 'i-flagpack:gb' },
  { label: 'Japonais', value: 'ja', icon: 'i-flagpack:jp' },
  { label: 'Allemand', value: 'de', icon: 'i-flagpack:de' },
  { label: 'Espagnol', value: 'es', icon: 'i-flagpack:es' },
  { label: 'Italien', value: 'it', icon: 'i-flagpack:it' },
  { label: 'Portugais', value: 'pt', icon: 'i-flagpack:pt' },
  { label: 'Russe', value: 'ru', icon: 'i-flagpack:ru' },
  { label: 'Chinois', value: 'zh', icon: 'i-flagpack:cn' },
  { label: 'Coréen', value: 'ko', icon: 'i-flagpack:kr' },
  { label: 'Arabe', value: 'ar', icon: 'i-flagpack:sa' },
  { label: 'Hindi', value: 'hi', icon: 'i-flagpack:in' }
]

interface Settings {
  translation: {
    sourceLanguage: string
    targetLanguage: string
  }
}

interface Props {
  settings: Settings
}

const props = defineProps<Props>()

const emit = defineEmits<{
  sourceLanguage: [value: string]
  targetLanguage: [value: string]
}>()

// Wrapper functions to avoid TypeScript overload issues with proper type handling
const emitSourceLanguage = (value: any) => {
  if (typeof value === 'string') emit('sourceLanguage', value)
}
const emitTargetLanguage = (value: any) => {
  if (typeof value === 'string') emit('targetLanguage', value)
}

// Computed properties for selected language icons
const sourceLanguageIcon = computed(() =>
  props.settings?.translation?.sourceLanguage
    ? availableLanguages.find(lang => lang.value === props.settings.translation.sourceLanguage)?.icon
    : undefined
)
const targetLanguageIcon = computed(() =>
  props.settings?.translation?.targetLanguage
    ? availableLanguages.find(lang => lang.value === props.settings.translation.targetLanguage)?.icon
    : undefined
)

// Helper function to get language name from code
function getLanguageName(code: string): string {
  const language = availableLanguages.find(lang => lang.value === code)
  return language ? language.label : code.toUpperCase()
}
</script>
