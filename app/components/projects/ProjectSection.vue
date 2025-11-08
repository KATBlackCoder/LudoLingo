<template>
  <UEmpty
    icon="i-heroicons-folder"
    :title="tm('projects', 'title')"
    :description="tm('projects', 'emptyDescription')"
  >
    <template #actions>
        <UButton
          color="primary"
          variant="solid"
        icon="i-heroicons-folder-open"
          @click="$emit('scan-projects')"
        class="w-full sm:w-auto"
        >
        {{ tm('projects', 'open') }}
        </UButton>
    </template>
  </UEmpty>
</template>

<script setup lang="ts">
import { useMessages } from '~/composables/useMessages'

interface Emits {
  (e: 'scan-projects'): void
}

defineEmits<Emits>()

const { tm } = useMessages()
</script>
