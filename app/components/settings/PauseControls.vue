<template>
  <UCard>
    <template #header>
      <div class="flex items-center gap-2">
        <UIcon name="i-heroicons-pause-circle" class="h-5 w-5 text-blue-600 dark:text-blue-400" />
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
          Contrôles de Pause
        </h3>
      </div>
      <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
        Configuration des pauses automatiques pendant la traduction
      </p>
    </template>

    <div class="space-y-6">
      <!-- Enable/Disable Pause -->
      <UFormField label="Activer les pauses automatiques">
        <UCheckbox
          :model-value="settings?.translation?.pause?.enabled ?? true"
          label="Activer les pauses après un certain nombre de traductions"
          @update:model-value="$emit('update:enabled', $event as boolean)"
        />
      </UFormField>

      <!-- Batch Size -->
      <UFormField
        label="Nombre de traductions avant pause"
        required
        :error="batchSizeError"
      >
        <UInput
          :model-value="settings?.translation?.pause?.batchSize ?? 150"
          type="number"
          placeholder="150"
          :min="1"
          :max="1000"
          :disabled="!settings?.translation?.pause?.enabled"
          @update:model-value="handleBatchSizeUpdate($event)"
        />
        <template #hint>
          <span class="text-xs text-gray-500 dark:text-gray-400">
            Nombre minimum: 1, maximum: 1000
          </span>
        </template>
      </UFormField>

      <!-- Pause Duration -->
      <UFormField
        label="Durée de pause (minutes)"
        required
        :error="pauseDurationError"
      >
        <UInput
          :model-value="settings?.translation?.pause?.pauseDurationMinutes ?? 5"
          type="number"
          placeholder="5"
          :min="1"
          :max="60"
          :disabled="!settings?.translation?.pause?.enabled"
          @update:model-value="handlePauseDurationUpdate($event)"
        />
        <template #hint>
          <span class="text-xs text-gray-500 dark:text-gray-400">
            Durée en minutes: 1-60 minutes
          </span>
        </template>
      </UFormField>

      <!-- Preview -->
      <div v-if="settings?.translation?.pause?.enabled" class="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
        <div class="flex items-center gap-2 mb-2">
          <UIcon name="i-heroicons-information-circle" class="h-4 w-4 text-blue-600 dark:text-blue-400" />
          <span class="text-sm font-medium text-blue-900 dark:text-blue-100">
            Aperçu de la configuration
          </span>
        </div>
        <p class="text-sm text-blue-800 dark:text-blue-200">
          Une pause de {{ settings.translation.pause.pauseDurationMinutes }} minute{{ settings.translation.pause.pauseDurationMinutes > 1 ? 's' : '' }}
          sera effectuée automatiquement après chaque {{ settings.translation.pause.batchSize }} traduction{{ settings.translation.pause.batchSize > 1 ? 's' : '' }}.
        </p>
      </div>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'

interface Settings {
  translation: {
    pause: {
      enabled: boolean
      batchSize: number
      pauseDurationMinutes: number
    }
  }
}

interface Props {
  settings: Settings
}

interface Emits {
  (e: 'update:enabled', value: boolean): void
  (e: 'update:batchSize', value: number): void
  (e: 'update:pauseDurationMinutes', value: number): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// Validation du nombre de traductions
const batchSizeError = computed(() => {
  const value = props.settings?.translation?.pause?.batchSize ?? 150
  if (value < 1) return 'Le nombre minimum est 1'
  if (value > 1000) return 'Le nombre maximum est 1000'
  return undefined
})

// Validation de la durée de pause
const pauseDurationError = computed(() => {
  const value = props.settings?.translation?.pause?.pauseDurationMinutes ?? 5
  if (value < 1) return 'La durée minimum est 1 minute'
  if (value > 60) return 'La durée maximum est 60 minutes'
  return undefined
})

// Gestionnaire pour batchSize avec correction automatique
const handleBatchSizeUpdate = (value: string | number) => {
  let numValue = typeof value === 'string' ? parseInt(value) || 150 : value

  // Correction automatique des valeurs invalides
  if (numValue < 1) numValue = 1
  if (numValue > 1000) numValue = 1000

  emit('update:batchSize', numValue)
}

// Gestionnaire pour pauseDuration avec correction automatique
const handlePauseDurationUpdate = (value: string | number) => {
  let numValue = typeof value === 'string' ? parseInt(value) || 5 : value

  // Correction automatique des valeurs invalides
  if (numValue < 1) numValue = 1
  if (numValue > 60) numValue = 60

  emit('update:pauseDurationMinutes', numValue)
}
</script>
