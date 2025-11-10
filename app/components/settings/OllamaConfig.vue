<template>
  <div class="space-y-6">
    <!-- Header -->
    <div>
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white flex items-center gap-2 mb-1">
        <UIcon name="i-simple-icons-ollama" class="h-5 w-5 text-purple-600 dark:text-purple-400" />
        Configuration Ollama
      </h3>
      <p class="text-sm text-gray-600 dark:text-gray-400">
        Service de traduction automatique
      </p>
    </div>

    <!-- Mode Selection -->
    <UFormField label="Mode de connexion" required>
      <div class="grid grid-cols-2 gap-3">
        <UButton
          :variant="settings.ollama.mode === 'local' ? 'solid' : 'outline'"
          color="primary"
          size="lg"
          icon="i-heroicons-home"
          @click="$emit('update:mode', 'local')"
        >
          Local
        </UButton>
        <UButton
          :variant="settings.ollama.mode === 'online' ? 'solid' : 'outline'"
          color="primary"
          size="lg"
          icon="i-heroicons-globe-alt"
          @click="$emit('update:mode', 'online')"
        >
          En ligne
        </UButton>
      </div>
    </UFormField>

    <!-- Local Mode Settings -->
    <div v-if="settings.ollama.mode === 'local'" class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <UFormField label="Endpoint" required>
        <UInput
          :model-value="settings.ollama.endpoint"
          placeholder="http://localhost"
          @update:model-value="$emit('update:endpoint', $event)"
        />
      </UFormField>

      <UFormField label="Port" required>
        <UInput
          :model-value="settings.ollama.port"
          type="number"
          placeholder="11434"
          @update:model-value="$emit('update:port', Number($event))"
        />
      </UFormField>
    </div>

    <!-- Online Mode Settings -->
    <UFormField v-else label="URL du service" required>
      <UInput
        :model-value="settings.ollama.endpoint"
        placeholder="https://api.ollama.example.com"
        @update:model-value="$emit('update:endpoint', $event)"
      />
    </UFormField>

    <!-- Model Selection -->
    <UFormField label="Modèle de traduction">
      <div class="flex gap-2">
        <USelect
          :model-value="settings.ollama.model"
          :items="availableModels"
          placeholder="Sélectionner un modèle"
          :disabled="loadingModels || availableModels.length === 0"
          class="flex-1"
          @update:model-value="$emit('update:model', $event)"
        />
        <UButton
          icon="i-heroicons-arrow-path"
          :loading="loadingModels"
          :disabled="!isConfigValid"
          @click="$emit('refresh-models')"
        />
      </div>
      <template #hint>
        <span v-if="availableModels.length === 0 && !loadingModels" class="text-amber-600 dark:text-amber-400">
          Testez la connexion pour charger les modèles
        </span>
      </template>
    </UFormField>

    <!-- Connection Test -->
    <div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-900 rounded-lg">
      <div class="flex items-center gap-2">
        <UButton
          icon="i-heroicons-wifi"
          :loading="testingConnection"
          :disabled="!isConfigValid"
          @click="$emit('test-connection')"
        >
          Tester la connexion
        </UButton>
        <span v-if="isConfigValid" class="text-xs text-green-600 dark:text-green-400">
          Prêt
        </span>
      </div>

      <div v-if="connectionStatus" class="flex items-center gap-2">
        <UIcon
          :name="connectionStatus.success ? 'i-heroicons-check-circle' : 'i-heroicons-x-circle'"
          :class="connectionStatus.success ? 'text-green-600' : 'text-red-600'"
          class="h-5 w-5"
        />
        <span class="text-sm">
          {{ connectionStatus.message }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Settings {
  ollama: {
    mode: 'local' | 'online'
    endpoint: string
    port: number
    model: string
  }
  ui: {
    language: string
  }
}

interface Props {
  settings: Settings
  availableModels: string[]
  loadingModels: boolean
  testingConnection: boolean
  connectionStatus: { success: boolean; message: string } | null
}

interface Emits {
  (e: 'update:mode', value: 'local' | 'online'): void
  (e: 'update:endpoint', value: string): void
  (e: 'update:port', value: number): void
  (e: 'update:model', value: string): void
  (e: 'refresh-models'): void
  (e: 'test-connection'): void
}

const props = defineProps<Props>()
defineEmits<Emits>()

const isConfigValid = computed(() => {
  const { mode, endpoint, port } = props.settings.ollama
  if (mode === 'local') {
    return endpoint.trim() !== '' && port > 0
  }
  return endpoint.trim() !== '' && endpoint.startsWith('http')
})
</script>
