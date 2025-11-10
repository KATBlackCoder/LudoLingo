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

// Composables
const settingsComposable = useSettings()

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
const availableModels = ref<string[]>([])
const loadingModels = ref(false)

// Store original settings as simple string for comparison
let originalSettingsStr = ''

const hasChanges = computed(() => {
  return JSON.stringify(settings.value) !== originalSettingsStr
})

// Load settings on mount
onMounted(async () => {
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
  originalSettingsStr = JSON.stringify(settings.value)
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
    originalSettingsStr = JSON.stringify(settings.value)
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
