<template>
  <UContainer class="py-8 relative">
    <!-- Version badge (coin en bas à droite) -->
    <div class="fixed bottom-4 right-4 z-10">
      <UBadge color="neutral" variant="subtle" size="sm">
        v{{ appVersion }}
      </UBadge>
    </div>

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

        <!-- Updater Settings -->
        <div class="space-y-4 pt-6 border-t">
          <UFormField label="Mises à jour automatiques">
            <div class="space-y-4">
              <div class="flex items-center justify-between">
                <div>
                  <p class="text-sm font-medium text-gray-900 dark:text-white">
                    Vérification automatique
                  </p>
                  <p class="text-xs text-gray-500 dark:text-gray-400">
                    Vérifier automatiquement les mises à jour disponibles
                  </p>
                </div>
                <USwitch
                  v-model="settings.updater.autoCheck"
                  @update:model-value="handleUpdaterSettingChange"
                />
              </div>

              <div v-if="settings.updater.autoCheck" class="space-y-2">
                <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
                  Fréquence de vérification
                </label>
                <div class="grid grid-cols-3 gap-2">
                  <UButton
                    :variant="settings.updater.checkFrequency === 'daily' ? 'solid' : 'outline'"
                    color="primary"
                    size="sm"
                    @click="settings.updater.checkFrequency = 'daily'; handleUpdaterSettingChange()"
                  >
                    Quotidienne
                  </UButton>
                  <UButton
                    :variant="settings.updater.checkFrequency === 'weekly' ? 'solid' : 'outline'"
                    color="primary"
                    size="sm"
                    @click="settings.updater.checkFrequency = 'weekly'; handleUpdaterSettingChange()"
                  >
                    Hebdomadaire
                  </UButton>
                  <UButton
                    :variant="settings.updater.checkFrequency === 'manual' ? 'solid' : 'outline'"
                    color="primary"
                    size="sm"
                    @click="settings.updater.checkFrequency = 'manual'; handleUpdaterSettingChange()"
                  >
                    Manuelle
                  </UButton>
                </div>
              </div>

              <div class="flex justify-end">
                <UButton
                  variant="outline"
                  color="primary"
                  size="sm"
                  icon="i-heroicons-arrow-path"
                  :loading="isCheckingUpdates"
                  @click="handleManualCheck"
                >
                  Vérifier maintenant
                </UButton>
              </div>
            </div>
          </UFormField>
        </div>

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
import { useUpdater } from '~/composables/updater/useUpdater'
import { useAutoUpdate } from '~/composables/updater/useAutoUpdate'
import OllamaConfig from '~/components/settings/OllamaConfig.vue'
import RunPodConfig from '~/components/settings/RunPodConfig.vue'
import TranslationLanguages from '~/components/settings/TranslationLanguages.vue'
import { getVersion } from '@tauri-apps/api/app'

// Composables
const settingsComposable = useSettings()
const updater = useUpdater()
const { restartAutoCheck } = useAutoUpdate()

// Version de l'application
const appVersion = ref('0.1.0')

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
  },
  updater: {
    autoCheck: true,
    checkFrequency: 'daily' as 'daily' | 'weekly' | 'manual',
    lastCheckDate: undefined as string | undefined
  }
})

const saving = ref(false)
const isCheckingUpdates = ref(false)


// Store original settings as simple string for comparison
let originalSettingsStr = ''

const hasChanges = computed(() => {
  return JSON.stringify(settings.value) !== originalSettingsStr
})

// Load settings on mount
onMounted(async () => {
  // Charger la version de l'application
  try {
    appVersion.value = await getVersion()
  } catch (error) {
    console.warn('Failed to get app version:', error)
  }

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
    },
    updater: {
      autoCheck: loadedSettings.updater?.autoCheck ?? true,
      checkFrequency: loadedSettings.updater?.checkFrequency || 'daily',
      lastCheckDate: loadedSettings.updater?.lastCheckDate
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
      pod_id: '',
      model: ''
    },
    translation: {
      sourceLanguage: 'ja',
      targetLanguage: 'fr'
    },
    updater: {
      autoCheck: true,
      checkFrequency: 'daily',
      lastCheckDate: undefined
    }
  }
}

// Gérer le changement des paramètres de l'updater
async function handleUpdaterSettingChange() {
  // Sauvegarder immédiatement les paramètres de l'updater
  await settingsComposable.saveSetting('updater', settings.value.updater)
  
  // Redémarrer la vérification automatique avec les nouveaux paramètres
  await restartAutoCheck()
}

// Vérification manuelle des mises à jour
async function handleManualCheck() {
  isCheckingUpdates.value = true
  try {
    await updater.checkForUpdates()
    // La notification sera affichée automatiquement si une mise à jour est disponible
  } catch (error) {
    console.error('Error checking for updates:', error)
  } finally {
    isCheckingUpdates.value = false
  }
}
</script>
