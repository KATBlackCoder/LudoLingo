<template>
  <div class="space-y-6">
    <!-- Header -->
    <div>
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white flex items-center gap-2 mb-1">
        <UIcon name="i-heroicons-cloud" class="h-5 w-5 text-blue-600 dark:text-blue-400" />
        Configuration RunPod
      </h3>
      <p class="text-sm text-gray-600 dark:text-gray-400">
        Service de traduction en ligne via RunPod
      </p>
    </div>

    <!-- Pod ID Input -->
    <UFormField label="Pod ID ou URL RunPod" required>
      <UInput
        :model-value="settings?.runpod?.pod_id"
        placeholder="1jdab9q2l1ya6l ou https://1jdab9q2l1ya6l-11434.proxy.runpod.net"
        @update:model-value="handlePodIdChange($event)"
      />
      <template #hint>
        <span class="text-xs text-gray-500 dark:text-gray-400">
          Entrez le Pod ID (ex: 1jdab9q2l1ya6l) ou l'URL complète. L'URL sera automatiquement construite si seul le Pod ID est fourni.
        </span>
      </template>
    </UFormField>

    <!-- Model Selection -->
    <UFormField label="Modèle de traduction">
      <div class="flex gap-2">
        <USelect
          :model-value="settings?.runpod?.model"
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
          @click="runpodStore.refreshModels()"
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
          @click="checkConnection"
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
import { computed, ref } from 'vue'
import { useRunPodStore } from '~/stores/runpod'
import { storeToRefs } from 'pinia'

interface Settings {
  runpod: {
    pod_id: string
    model: string
  }
}

interface Props {
  settings: Settings
}

interface Emits {
  (e: 'update:podId', value: string): void
  (e: 'update:model', value: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// Use RunPod store directly
const runpodStore = useRunPodStore()
const { isCheckingConnection, status, availableModels: storeModels } = storeToRefs(runpodStore)
const toast = useToast()

// Format models for USelect component
const availableModels = computed(() => {
  return storeModels.value.map(model => ({
    label: model,
    value: model
  }))
})

// Transform RunPod status to component expected format
const connectionStatus = computed(() => {
  if (!runpodStore.status) return null
  return {
    success: runpodStore.status.available,
    message: runpodStore.status.available
      ? `Connexion réussie${runpodStore.status.models_available ? ` (${runpodStore.status.models_available.length} modèles)` : ''}`
      : (runpodStore.status.error || 'Connexion échouée')
  }
})

/**
 * Extract pod_id from URL or return as-is if it's already a pod_id
 */
function extractPodId(input: string): string {
  if (!input || input.trim() === '') return ''
  
  const trimmed = input.trim()
  
  // If it's a URL, extract the pod_id
  const urlMatch = trimmed.match(/https?:\/\/([a-z0-9]+)-11434\.proxy\.runpod\.net/)
  if (urlMatch && urlMatch[1]) {
    return urlMatch[1]
  }
  
  // If it's already a pod_id (alphanumeric, no special chars except -)
  if (/^[a-z0-9-]+$/i.test(trimmed)) {
    return trimmed
  }
  
  // Return as-is if we can't determine
  return trimmed
}

const isConfigValid = computed(() => {
  const podId = extractPodId(props.settings?.runpod?.pod_id || '')
  return podId.trim() !== ''
})

function handlePodIdChange(value: string) {
  // Extract pod_id and emit it
  const extractedPodId = extractPodId(value)
  emit('update:podId', extractedPodId)
}

async function checkConnection() {
  if (!isConfigValid.value) return

  // Extract pod_id from input (handles both URL and pod_id format)
  const podId = extractPodId(props.settings.runpod.pod_id || '')
  
  if (!podId || podId.trim() === '') {
    return
  }

  await runpodStore.checkConnection(podId.trim())
  
  if (runpodStore.isConnected) {
    toast.add({
      title: 'Connexion RunPod réussie',
      description: 'Le service de traduction est maintenant disponible.',
      icon: 'i-heroicons-cloud',
      color: 'success'
    })
  }
}
</script>

