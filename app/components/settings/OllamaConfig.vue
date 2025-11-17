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

    <!-- Local Settings -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <UFormField label="Endpoint" required>
        <UInput
          :model-value="settings?.ollama?.endpoint"
          placeholder="http://localhost"
          @update:model-value="$emit('update:endpoint', $event)"
        />
      </UFormField>

      <UFormField label="Port" required>
        <UInput
          :model-value="settings?.ollama?.port"
          type="number"
          placeholder="11434"
          @update:model-value="$emit('update:port', Number($event))"
        />
      </UFormField>
    </div>

    <!-- Model Selection -->
    <UFormField label="Modèle de traduction">
      <div class="flex gap-2">
        <USelect
          :model-value="settings?.ollama?.model"
          :items="availableModels"
          placeholder="Sélectionner un modèle"
          :disabled="isCheckingConnection || availableModels.length === 0"
          class="flex-1"
          @update:model-value="$emit('update:model', $event)"
        />
        <UButton
          icon="i-heroicons-arrow-path"
          :loading="isCheckingConnection"
          :disabled="!isConfigValid"
          @click="ollamaStore.refreshModels()"
        />
      </div>
      <template #hint>
        <span v-if="availableModels.length === 0 && !isCheckingConnection" class="text-amber-600 dark:text-amber-400">
          Testez la connexion pour charger les modèles
        </span>
      </template>
    </UFormField>

    <!-- Connection Test -->
    <div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-900 rounded-lg">
      <div class="flex items-center gap-2">
        <UButton
          icon="i-heroicons-wifi"
          :loading="isCheckingConnection"
          :disabled="!isConfigValid"
          @click="ollamaStore.checkConnection()"
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
import { computed, watch } from 'vue'
import { useOllamaStore } from '~/stores/ollama'
import { storeToRefs } from 'pinia'

// Use toast for connection success notifications
const toast = useToast()

interface Settings {
  ollama: {
    endpoint: string
    port: number
    model: string
  }
}

interface Props {
  settings: Settings
}

interface Emits {
  (e: 'update:endpoint', value: string): void
  (e: 'update:port', value: number): void
  (e: 'update:model', value: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// Use Ollama store directly
const ollamaStore = useOllamaStore()
const { availableModels, isCheckingConnection } = storeToRefs(ollamaStore)

// Transform Ollama status to component expected format
const connectionStatus = computed(() => {
  if (!ollamaStore.status) return null
  return {
    success: ollamaStore.status.available,
    message: ollamaStore.status.available
      ? 'Connection successful!'
      : (ollamaStore.status.error || 'Connection failed')
  }
})

const isConfigValid = computed(() => {
  if (!props.settings?.ollama) return false

  const { endpoint, port } = props.settings.ollama
  return endpoint.trim() !== '' && port > 0
})

// Watch for connection success to show toast
watch(
  () => ollamaStore.isConnected,
  (isConnected, wasConnected) => {
    // Only show toast when connection becomes true (not on initial load)
    if (isConnected && !wasConnected) {
      toast.add({
        title: 'Connexion Ollama réussie',
        description: 'Le service de traduction est maintenant disponible.',
        icon: 'i-simple-icons-ollama',
        color: 'success'
      })
}
  }
)
</script>
