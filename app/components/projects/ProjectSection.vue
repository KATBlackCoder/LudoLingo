<template>
  <UCard class="hover:shadow-lg transition-shadow">
    <template #header>
      <div class="flex items-center gap-3">
        <UIcon name="i-heroicons-folder" class="h-6 w-6 text-primary" />
        <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
          {{ tm('projects', 'title') }}
        </h3>
      </div>
    </template>

    <div class="space-y-4">
      <p class="text-gray-600 dark:text-gray-300">
        {{ tm('projects', 'emptyDescription') }}
      </p>

      <div class="flex gap-2">
        <UButton
          color="primary"
          variant="solid"
          icon="i-heroicons-plus"
          @click="$emit('create-project')"
        >
          {{ tm('projects', 'create') }}
        </UButton>

        <UButton
          color="neutral"
          variant="outline"
          icon="i-heroicons-magnifying-glass"
          @click="$emit('scan-projects')"
        >
          {{ tm('projects', 'scan') }}
        </UButton>
      </div>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { useMessages } from '~/composables/useMessages'

interface Emits {
  (e: 'create-project'): void
  (e: 'scan-projects'): void
}

defineEmits<Emits>()

const { tm } = useMessages()
</script>
