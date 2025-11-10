<template>
  <div class="border-t border-gray-200 dark:border-gray-700 pt-8">
    <div class="mb-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white flex items-center gap-3 mb-2">
        <UIcon name="i-simple-icons-ollama" class="h-6 w-6 text-purple-600 dark:text-purple-400" />
        {{ tmReactive('settings', 'ollama').value }}
      </h3>
      <p class="text-sm text-gray-600 dark:text-gray-400">
        Configuration du service de traduction automatique
      </p>
    </div>

    <!-- Mode Selection -->
    <div class="mb-8">
      <label class="block text-sm font-medium text-gray-900 dark:text-white mb-3">
        {{ tmReactive('settings', 'mode').value }}
      </label>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <div
          class="relative p-4 rounded-lg border-2 cursor-pointer transition-all duration-200"
          :class="settings.ollama.mode === 'local'
            ? 'border-blue-500 bg-blue-50 dark:bg-blue-950/20'
            : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'"
          @click="$emit('update:mode', 'local')"
        >
          <div class="flex items-start gap-3">
            <UIcon
              name="i-heroicons-home"
              class="h-5 w-5 mt-0.5 flex-shrink-0"
              :class="settings.ollama.mode === 'local' ? 'text-blue-600 dark:text-blue-400' : 'text-gray-400'"
            />
            <div>
              <div class="font-medium text-gray-900 dark:text-white mb-1">
                {{ tmReactive('settings', 'local').value }}
              </div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Exécution locale sur votre machine
              </div>
            </div>
          </div>
          <div v-if="settings.ollama.mode === 'local'" class="absolute top-2 right-2">
            <UIcon name="i-heroicons-check-circle" class="h-5 w-5 text-blue-600 dark:text-blue-400" />
          </div>
        </div>

        <div
          class="relative p-4 rounded-lg border-2 cursor-pointer transition-all duration-200"
          :class="settings.ollama.mode === 'online'
            ? 'border-green-500 bg-green-50 dark:bg-green-950/20'
            : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'"
          @click="$emit('update:mode', 'online')"
        >
          <div class="flex items-start gap-3">
            <UIcon
              name="i-heroicons-globe-alt"
              class="h-5 w-5 mt-0.5 flex-shrink-0"
              :class="settings.ollama.mode === 'online' ? 'text-green-600 dark:text-green-400' : 'text-gray-400'"
            />
            <div>
              <div class="font-medium text-gray-900 dark:text-white mb-1">
                {{ tmReactive('settings', 'online').value }}
              </div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Service cloud distant
              </div>
            </div>
          </div>
          <div v-if="settings.ollama.mode === 'online'" class="absolute top-2 right-2">
            <UIcon name="i-heroicons-check-circle" class="h-5 w-5 text-green-600 dark:text-green-400" />
          </div>
        </div>
      </div>
    </div>

    <!-- Local Mode Settings -->
    <div v-if="settings.ollama.mode === 'local'" class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <UFormField :label="tmReactive('settings', 'endpoint').value" required>
        <UInput
          :model-value="settings.ollama.endpoint"
          :placeholder="tmReactive('settings', 'endpointPlaceholder').value"
          size="lg"
          @update:model-value="$emit('update:endpoint', $event)"
        />
      </UFormField>

      <UFormField :label="tmReactive('settings', 'port').value" required>
        <UInput
          :model-value="settings.ollama.port"
          type="number"
          :placeholder="tmReactive('settings', 'portPlaceholder').value"
          size="lg"
          @update:model-value="$emit('update:port', Number($event))"
        />
      </UFormField>
    </div>

    <!-- Online Mode Settings -->
    <div v-else class="mb-6">
      <UFormField :label="tmReactive('settings', 'endpoint').value" required>
        <UInput
          :model-value="settings.ollama.endpoint"
          :placeholder="tmReactive('settings', 'onlineEndpointPlaceholder').value"
          size="lg"
          @update:model-value="$emit('update:endpoint', $event)"
        />
      </UFormField>
    </div>

    <!-- Model Selection -->
    <div class="mb-8">
      <label class="block text-sm font-medium text-gray-900 dark:text-white mb-3">
        {{ tmReactive('settings', 'model').value }}
      </label>
      <div class="flex gap-3">
        <div class="flex-1">
          <USelect
            :model-value="settings.ollama.model"
            :items="availableModels"
            :placeholder="loadingModels ? tmReactive('common', 'loading').value : tmReactive('settings', 'selectModel').value"
            :disabled="loadingModels || availableModels.length === 0"
            size="lg"
            @update:model-value="$emit('update:model', $event)"
          />
        </div>
        <UButton
          color="neutral"
          variant="outline"
          icon="i-heroicons-arrow-path"
          :loading="loadingModels"
          size="lg"
          :disabled="!isConfigValid"
          @click="$emit('refresh-models')"
          class="px-4"
        >
          <span class="hidden sm:inline">{{ tmReactive('settings', 'refreshModels').value }}</span>
        </UButton>
      </div>
      <div v-if="availableModels.length === 0 && !loadingModels" class="mt-3 p-3 bg-amber-50 dark:bg-amber-950/20 border border-amber-200 dark:border-amber-800 rounded-lg">
        <div class="flex items-start gap-2">
          <UIcon name="i-heroicons-exclamation-triangle" class="h-4 w-4 text-amber-600 dark:text-amber-400 mt-0.5 flex-shrink-0" />
          <p class="text-sm text-amber-800 dark:text-amber-200">
            {{ tmReactive('settings', 'testConnectionFirst').value }}
          </p>
        </div>
      </div>
    </div>

    <!-- Connection Test -->
    <div class="mb-6">
      <div class="flex items-center justify-between mb-3">
        <label class="text-sm font-medium text-gray-900 dark:text-white">
          Test de connexion
        </label>
        <div v-if="connectionStatus" class="flex items-center gap-2">
          <UIcon
            :name="connectionStatus.success ? 'i-heroicons-check-circle' : 'i-heroicons-x-circle'"
            :class="connectionStatus.success ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'"
            class="h-5 w-5"
          />
          <span
            :class="connectionStatus.success ? 'text-green-700 dark:text-green-300' : 'text-red-700 dark:text-red-300'"
            class="text-sm font-medium"
          >
            {{ connectionStatus.message }}
          </span>
        </div>
      </div>

      <div class="flex gap-3">
        <UButton
          color="neutral"
          variant="outline"
          icon="i-heroicons-wifi"
          :loading="testingConnection"
          :disabled="!isConfigValid"
          @click="$emit('test-connection')"
          class="flex-1 sm:flex-none"
        >
          <span class="hidden sm:inline">{{ tmReactive('settings', 'test').value }}</span>
          <span class="sm:hidden">Tester</span>
        </UButton>

        <div v-if="isConfigValid" class="flex items-center gap-2 text-sm text-green-600 dark:text-green-400">
          <UIcon name="i-heroicons-check-circle" class="h-4 w-4" />
          Configuration valide
        </div>
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
import { useAppLocale } from '~/composables/useLocale'

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

const { tmReactive } = useAppLocale()

const isConfigValid = computed(() => {
  const { mode, endpoint, port } = props.settings.ollama

  if (mode === 'local') {
    return endpoint.trim() && port && port > 0
  } else {
    return endpoint.trim() && endpoint.startsWith('http')
  }
})
</script>
