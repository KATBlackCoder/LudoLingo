<template>
  <div class="border-t pt-8">
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-6 flex items-center gap-2">
      <UIcon name="i-simple-icons-ollama" class="h-5 w-5" />
      {{ tmReactive('settings', 'ollama').value }}
    </h3>

    <!-- Mode Selection -->
    <UFormGroup :label="tmReactive('settings', 'mode')" class="mb-6">
      <div class="grid grid-cols-2 gap-4">
        <UButton
          :variant="settings.ollama.mode === 'local' ? 'solid' : 'outline'"
          color="primary"
          icon="i-heroicons-home"
          @click="$emit('update:mode', 'local')"
        >
          {{ tmReactive('settings', 'local').value }}
        </UButton>
        <UButton
          :variant="settings.ollama.mode === 'online' ? 'solid' : 'outline'"
          color="primary"
          icon="i-heroicons-globe-alt"
          @click="$emit('update:mode', 'online')"
        >
          {{ tmReactive('settings', 'online').value }}
        </UButton>
      </div>
    </UFormGroup>

    <!-- Local Mode Settings -->
    <div v-if="settings.ollama.mode === 'local'" class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <UFormGroup :label="tmReactive('settings', 'endpoint')" required>
        <UInput
          :model-value="settings.ollama.endpoint"
          :placeholder="tmReactive('settings', 'endpointPlaceholder')"
          size="lg"
          @update:model-value="$emit('update:endpoint', $event)"
        />
      </UFormGroup>

      <UFormGroup :label="tmReactive('settings', 'port')" required>
        <UInput
          :model-value="settings.ollama.port"
          type="number"
          :placeholder="tmReactive('settings', 'portPlaceholder')"
          size="lg"
          @update:model-value="$emit('update:port', Number($event))"
        />
      </UFormGroup>
    </div>

    <!-- Online Mode Settings -->
    <div v-else class="mb-6">
      <UFormGroup :label="tmReactive('settings', 'endpoint')" required>
        <UInput
          :model-value="settings.ollama.endpoint"
          :placeholder="tmReactive('settings', 'onlineEndpointPlaceholder')"
          size="lg"
          @update:model-value="$emit('update:endpoint', $event)"
        />
      </UFormGroup>
    </div>

    <!-- Model Selection -->
    <UFormGroup :label="tmReactive('settings', 'model')" required>
      <div class="flex gap-2">
        <USelect
          :model-value="settings.ollama.model"
          :items="availableModels"
          :placeholder="loadingModels ? tmReactive('common', 'loading') : tmReactive('settings', 'selectModel')"
          :disabled="loadingModels || availableModels.length === 0"
          size="lg"
          class="flex-1"
          @update:model-value="$emit('update:model', $event)"
        />
        <UButton
          color="neutral"
          variant="outline"
          icon="i-heroicons-arrow-path"
          :loading="loadingModels"
          size="lg"
          :disabled="!isConfigValid"
          @click="$emit('refresh-models')"
        >
          {{ tmReactive('settings', 'refreshModels').value }}
        </UButton>
      </div>
      <p v-if="availableModels.length === 0 && !loadingModels" class="text-sm text-gray-500 mt-2">
        {{ tmReactive('settings', 'testConnectionFirst').value }}
      </p>
    </UFormGroup>

    <!-- Connection Test -->
    <div class="flex items-center gap-4">
      <UButton
        color="neutral"
        variant="outline"
        icon="i-heroicons-wifi"
        :loading="testingConnection"
        :disabled="!isConfigValid"
        @click="$emit('test-connection')"
      >
        {{ tmReactive('settings', 'test').value }}
      </UButton>

      <div v-if="connectionStatus" class="flex items-center gap-2">
        <UIcon
          :name="connectionStatus.success ? 'i-heroicons-check-circle' : 'i-heroicons-x-circle'"
          :class="connectionStatus.success ? 'text-green-500' : 'text-red-500'"
          class="h-5 w-5"
        />
        <span
          :class="connectionStatus.success ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'"
          class="text-sm"
        >
          {{ connectionStatus.message }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Settings interface is now defined in useTauriSetting composable
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
import { useLocale } from '~/composables/useLocale'

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
const emit = defineEmits<Emits>()

const { tmReactive } = useLocale()

const isConfigValid = computed(() => {
  const { mode, endpoint, port } = props.settings.ollama

  if (mode === 'local') {
    return endpoint.trim() && port && port > 0
  } else {
    return endpoint.trim() && endpoint.startsWith('http')
  }
})
</script>
