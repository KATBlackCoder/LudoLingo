<template>
  <UContainer class="py-8">
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
        {{ tmReactive('settings', 'title').value }}
      </h1>
      <p class="text-gray-600 dark:text-gray-300">
        {{ tmReactive('settings', 'subtitle').value }}
      </p>
    </div>

    <!-- Settings Form -->
    <UCard class="max-w-4xl">
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
        <div class="flex justify-end gap-4 pt-6 border-t">
          <UButton
            color="neutral"
            variant="outline"
            @click="handleReset"
          >
            {{ tmReactive('settings', 'reset').value }}
          </UButton>

          <UButton
            color="primary"
            type="submit"
            :loading="saving"
            :disabled="!hasChanges"
          >
            {{ tmReactive('settings', 'save').value }}
          </UButton>
        </div>
      </form>
    </UCard>
  </UContainer>
</template>

<script setup lang="ts">
import { useSettings } from '~/composables/useTauriSetting'
import { useLocale } from '~/composables/useLocale'
import OllamaConfig from '~/components/settings/OllamaConfig.vue'

// Composables
const settingsComposable = useSettings()
const { tmReactive } = useLocale()

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
