<template>
  <UContainer class="py-8">
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
        {{ tm('settings', 'title') }}
      </h1>
      <p class="text-gray-600 dark:text-gray-300">
        {{ tm('settings', 'subtitle') }}
      </p>
    </div>

    <!-- Settings Form -->
    <UCard class="max-w-4xl">
      <form @submit.prevent="handleSave" class="space-y-8">
        <OllamaConfig
          :settings="settingsStore.settings"
          :available-models="availableModels"
          :loading-models="loadingModels"
          :testing-connection="testingConnection"
          :connection-status="connectionStatus"
          @update:mode="settingsStore.updateOllamaMode($event)"
          @update:endpoint="settingsStore.settings.ollama.endpoint = $event"
          @update:port="settingsStore.settings.ollama.port = $event"
          @update:model="settingsStore.settings.ollama.model = $event"
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
            {{ tm('settings', 'reset') }}
          </UButton>

          <UButton
            color="primary"
            type="submit"
            :loading="saving"
            :disabled="!hasChanges"
          >
            {{ tm('settings', 'save') }}
          </UButton>
        </div>
      </form>
    </UCard>
  </UContainer>
</template>

<script setup lang="ts">
import { useSettingsStore } from '~/stores/settings'
import { useMessages } from '~/composables/useMessages'
import OllamaConfig from '~/components/settings/OllamaConfig.vue'

// Reactive state
const settingsStore = useSettingsStore()
const { tm } = useMessages()

const saving = ref(false)
const testingConnection = ref(false)
const connectionStatus = ref<{ success: boolean; message: string } | null>(null)
const originalSettings = ref(JSON.parse(JSON.stringify(settingsStore.settings)))
const availableModels = ref<string[]>([])
const loadingModels = ref(false)


const hasChanges = computed(() => {
  return JSON.stringify(settingsStore.settings) !== JSON.stringify(originalSettings.value)
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
    await settingsStore.saveSettings()

    // Update original settings for change detection
    originalSettings.value = JSON.parse(JSON.stringify(settingsStore.settings))

    // Show success message
    console.log('Settings saved successfully!')
  } catch (error) {
    console.error('Failed to save settings:', error)
  } finally {
    saving.value = false
  }
}

function handleReset() {
  settingsStore.resetToDefaults()
}
</script>
