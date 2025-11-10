<template>
  <UContainer class="py-8">
    <!-- Header -->
    <div class="mb-8">
      <div class="flex items-center gap-4 mb-4">
        <div class="p-3 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl">
          <UIcon name="i-heroicons-cog-6-tooth" class="h-8 w-8 text-white" />
        </div>
        <div>
          <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
            {{ tmReactive('settings', 'title').value }}
          </h1>
          <p class="text-gray-600 dark:text-gray-400 mt-1">
            {{ tmReactive('settings', 'subtitle').value }}
          </p>
        </div>
      </div>

      <!-- Status Banner -->
      <div v-if="!hasChanges && originalSettings" class="p-4 bg-green-50 dark:bg-green-950/20 border border-green-200 dark:border-green-800 rounded-lg mb-6">
        <div class="flex items-center gap-2">
          <UIcon name="i-heroicons-check-circle" class="h-5 w-5 text-green-600 dark:text-green-400" />
          <span class="text-sm text-green-800 dark:text-green-200 font-medium">
            Configuration chargée et synchronisée
          </span>
        </div>
      </div>
    </div>

    <!-- Settings Form -->
    <UCard class="max-w-4xl shadow-sm">
      <form @submit.prevent="handleSave" class="space-y-8">
        <OllamaConfig
          :settings="settings"
          :available-models="availableModels"
          :loading-models="loadingModels"
          :testing-connection="testingConnection"
          :connection-status="connectionStatus"
          @update:mode="settings.ollama.mode = $event"
          @update:endpoint="settings.ollama.endpoint = $event"
          @update:port="settings.ollama.port = $event"
          @update:model="settings.ollama.model = $event"
          @refresh-models="loadAvailableModels"
          @test-connection="testConnection"
        />

        <!-- Action Buttons -->
        <div class="flex flex-col sm:flex-row sm:justify-between gap-4 pt-6 border-t border-gray-200 dark:border-gray-700">
          <div class="flex items-center gap-2 text-sm">
            <div v-if="hasChanges" class="flex items-center gap-2 text-amber-600 dark:text-amber-400">
              <UIcon name="i-heroicons-exclamation-triangle" class="h-4 w-4" />
              <span>Modifications non sauvegardées</span>
            </div>
            <div v-else class="flex items-center gap-2 text-green-600 dark:text-green-400">
              <UIcon name="i-heroicons-check-circle" class="h-4 w-4" />
              <span>Configuration à jour</span>
            </div>
          </div>

          <div class="flex gap-3">
            <UButton
              color="neutral"
              variant="outline"
              icon="i-heroicons-arrow-path"
              @click="handleReset"
              :disabled="saving"
            >
              {{ tmReactive('settings', 'reset').value }}
            </UButton>

            <UButton
              color="primary"
              type="submit"
              icon="i-heroicons-check"
              :loading="saving"
              :disabled="!hasChanges"
            >
              {{ tmReactive('settings', 'save').value }}
            </UButton>
          </div>
        </div>
      </form>
    </UCard>
  </UContainer>
</template>

<script setup lang="ts">
import { useSettings } from '~/composables/useTauriSetting'
import { useAppLocale } from '~/composables/useLocale'
import OllamaConfig from '~/components/settings/OllamaConfig.vue'

// Composables
const settingsComposable = useSettings()
const { tmReactive } = useAppLocale()

// Reactive state
const settings = ref({
  ollama: {
    mode: 'local' as 'local' | 'online',
    endpoint: 'http://localhost',
    port: 11434,
    model: 'llama2:13b'
  },
  ui: {
    language: 'fr'
  }
})

const saving = ref(false)
const testingConnection = ref(false)
const connectionStatus = ref<{ success: boolean; message: string } | null>(null)
const originalSettings = ref(JSON.parse(JSON.stringify(settings.value)))
const availableModels = ref<string[]>([])
const loadingModels = ref(false)

const hasChanges = computed(() => {
  return JSON.stringify(settings.value) !== JSON.stringify(originalSettings.value)
})

// Charger les settings au montage
onMounted(async () => {
  try {
    const loadedSettings = await settingsComposable.loadSettings()
    settings.value = {
      ollama: {
        mode: loadedSettings.ollama?.mode || 'local',
        endpoint: loadedSettings.ollama?.endpoint || 'http://localhost',
        port: loadedSettings.ollama?.port || 11434,
        model: loadedSettings.ollama?.model || 'llama2:13b'
      },
      ui: {
        language: loadedSettings.ui?.language || 'fr'
      }
    }
    originalSettings.value = JSON.parse(JSON.stringify(settings.value))
  } catch (error) {
    console.error('Failed to load settings:', error)
  }
})

// Methods
async function loadAvailableModels() {
  loadingModels.value = true

  try {
    // TODO: Replace with actual Ollama API call
    // For now, simulate fetching models from Ollama API
    await new Promise(resolve => setTimeout(resolve, 1500))

    // Simulate getting models from Ollama (this would be replaced by actual API call)
    availableModels.value = [
      'llama2:latest',
      'llama2:13b',
      'llama2:70b',
      'mistral:latest',
      'codellama:latest',
      'qwen2.5:32b',
      'gemma2:27b',
      'llama3:8b',
      'llama3:70b',
      'phi3:medium'
    ]
  } catch (error) {
    console.error('Failed to load models:', error)
    availableModels.value = []
  } finally {
    loadingModels.value = false
  }
}

async function testConnection() {
  testingConnection.value = true
  connectionStatus.value = null

  try {
    // TODO: Implement actual Ollama connection test
    // For now, just simulate a successful connection
    await new Promise(resolve => setTimeout(resolve, 2000))
    connectionStatus.value = {
      success: true,
      message: 'Connection successful!'
    }

    // Load models after successful connection test
    await loadAvailableModels()
  } catch (error) {
    connectionStatus.value = {
      success: false,
      message: 'Connection failed. Please check your settings.'
    }
  } finally {
    testingConnection.value = false
  }
}

async function handleSave() {
  saving.value = true

  try {
    await settingsComposable.saveSettings(settings.value)

    // Update original settings for change detection
    originalSettings.value = JSON.parse(JSON.stringify(settings.value))

    // Show success message
    console.log('Settings saved successfully!')
  } catch (error) {
    console.error('Failed to save settings:', error)
  } finally {
    saving.value = false
  }
}

function handleReset() {
  settings.value = {
    ollama: {
      mode: 'local',
      endpoint: 'http://localhost',
      port: 11434,
      model: 'llama2:13b'
    },
    ui: {
      language: 'fr'
    }
  }
}
</script>
