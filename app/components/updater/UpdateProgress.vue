<template>
  <div class="space-y-3 transition-all duration-300">
    <!-- Message d'état -->
    <div class="flex items-center justify-between">
      <span class="text-sm font-medium text-gray-700 dark:text-gray-300 transition-colors">
        {{ statusMessage }}
      </span>
      <span v-if="showPercentage" class="text-sm text-gray-600 dark:text-gray-400 font-mono">
        {{ downloadProgress }}%
      </span>
    </div>

    <!-- Barre de progression -->
    <UProgress
      :value="downloadProgress"
      :color="progressColor"
      :size="size"
      :animated="isDownloading"
      class="transition-all duration-300"
    />

    <!-- Bouton Annuler (si téléchargement en cours) -->
    <div v-if="isDownloading && showCancel" class="flex justify-end animate-fade-in">
      <UButton
        variant="ghost"
        color="neutral"
        size="sm"
        @click="$emit('cancel')"
      >
        Annuler
      </UButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  downloadProgress: number
  isDownloading: boolean
  isReadyToInstall?: boolean
  showPercentage?: boolean
  showCancel?: boolean
  size?: 'xs' | 'sm' | 'md' | 'lg'
}

const props = withDefaults(defineProps<Props>(), {
  isReadyToInstall: false,
  showPercentage: true,
  showCancel: true,
  size: 'md'
})

defineEmits<{
  cancel: []
}>()

const statusMessage = computed(() => {
  if (props.isReadyToInstall) {
    return 'Prêt à installer'
  }
  if (props.isDownloading) {
    return 'Téléchargement en cours...'
  }
  return 'En attente...'
})

const progressColor = computed(() => {
  if (props.isReadyToInstall) {
    return 'success'
  }
  if (props.isDownloading) {
    return 'primary'
  }
  return 'neutral'
})
</script>

