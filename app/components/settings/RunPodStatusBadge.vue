<template>
  <UIcon
    :name="isConnected ? 'i-heroicons-check-circle' : 'i-heroicons-x-circle'"
    :class="isConnected ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'"
    class="h-5 w-5"
    :title="isConnected ? 'RunPod connecté' : 'RunPod déconnecté'"
  />
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useRunPodStore } from '~/stores/runpod'

const runpodStore = useRunPodStore()
const { isConnected } = storeToRefs(runpodStore)
</script>

