<template>
  <UContainer class="py-8">
    <!-- Header -->
    <div class="mb-8">
      <div class="flex items-center gap-3 mb-2">
        <UIcon name="i-heroicons-cog-6-tooth" class="h-7 w-7 text-primary" />
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
          Paramètres
        </h1>
      </div>
      <p class="text-gray-600 dark:text-gray-400">
        Configuration de l'application et des services
      </p>
    </div>

    <!-- Settings Form -->
    <UCard>
      <form @submit.prevent="handleSave" class="space-y-6">
        <!-- Provider Selection -->
        <UFormField label="Provider de traduction" required>
          <div class="grid grid-cols-2 gap-3">
            <UButton
              :variant="settings.provider === 'ollama' ? 'solid' : 'outline'"
              color="primary"
              size="lg"
              icon="i-heroicons-home"
              @click="settings.provider = 'ollama'"
            >
              Ollama (Local)
            </UButton>
            <UButton
              :variant="settings.provider === 'runpod' ? 'solid' : 'outline'"
              color="primary"
              size="lg"
              icon="i-heroicons-cloud"
              @click="settings.provider = 'runpod'"
            >
              RunPod (Online)
            </UButton>
          </div>
        </UFormField>

        <!-- Provider-specific Configuration -->
        <OllamaConfig
          v-if="settings.provider === 'ollama'"
          :settings="settings"
          @update:endpoint="settings.ollama.endpoint = $event"
          @update:port="settings.ollama.port = $event"
          @update:model="settings.ollama.model = $event"
        />

            <RunPodConfig
              v-if="settings.provider === 'runpod'"
              :settings="settings"
              @update:podId="settings.runpod.pod_id = $event"
              @update:model="settings.runpod.model = $event"
            />

        <TranslationLanguages
          :settings="settings"
          @sourceLanguage="settings.translation.sourceLanguage = $event"
          @targetLanguage="settings.translation.targetLanguage = $event"
        />

        <!-- Action Buttons -->
        <div class="flex justify-end gap-3 pt-6 border-t">
          <UButton
            variant="outline"
            icon="i-heroicons-arrow-path"
            @click="handleReset"
            :disabled="saving"
          >
            Réinitialiser
          </UButton>

          <UButton
            type="submit"
            icon="i-heroicons-check"
            :loading="saving"
            :disabled="!hasChanges"
          >
            Enregistrer
          </UButton>
        </div>
      </form>
    </UCard>
  </UContainer>
</template>

<script setup lang="ts">
import { useSettings } from '~/composables/useTauriSetting'
import OllamaConfig from '~/components/settings/OllamaConfig.vue'
import RunPodConfig from '~/components/settings/RunPodConfig.vue'
import TranslationLanguages from '~/components/settings/TranslationLanguages.vue'

// Composables
const settingsComposable = useSettings()

// Reactive state
const settings = ref({
  provider: 'ollama' as 'ollama' | 'runpod',
  ollama: {
    endpoint: 'http://localhost',
    port: 11434,
    model: 'llama2:13b'
  },
  runpod: {
    pod_id: '',
    model: ''
  },
  translation: {
    sourceLanguage: 'ja',
    targetLanguage: 'fr'
  }
})

const saving = ref(false)


// Store original settings as simple string for comparison
let originalSettingsStr = ''

const hasChanges = computed(() => {
  return JSON.stringify(settings.value) !== originalSettingsStr
})

// Load settings on mount
onMounted(async () => {
  const loadedSettings = await settingsComposable.loadSettings()
  settings.value = {
    provider: loadedSettings.provider || 'ollama',
    ollama: {
      endpoint: loadedSettings.ollama?.endpoint || 'http://localhost',
      port: loadedSettings.ollama?.port || 11434,
      model: loadedSettings.ollama?.model || 'llama2:13b'
    },
        runpod: {
          pod_id: loadedSettings.runpod?.pod_id || '',
          model: loadedSettings.runpod?.model || ''
        },
    translation: {
      sourceLanguage: loadedSettings.translation?.sourceLanguage || 'ja',
      targetLanguage: loadedSettings.translation?.targetLanguage || 'fr'
    }
  }
  originalSettingsStr = JSON.stringify(settings.value)
})

// Methods

async function handleSave() {
  saving.value = true
  try {
    await settingsComposable.saveSettings(settings.value)
    originalSettingsStr = JSON.stringify(settings.value)
  } finally {
    saving.value = false
  }
}

function handleReset() {
  settings.value = {
    provider: 'ollama',
    ollama: {
      endpoint: 'http://localhost',
      port: 11434,
      model: 'llama2:13b'
    },
    runpod: {
      pod_id: ''
    },
    translation: {
      sourceLanguage: 'ja',
      targetLanguage: 'fr'
    }
  }
}
</script>
