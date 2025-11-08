<template>
  <UEmpty
    icon="i-heroicons-folder"
    :title="tmReactive('projects', 'title').value"
    :description="tmReactive('projects', 'emptyDescription').value"
  >
    <template #actions>
        <UButton
          color="primary"
          variant="solid"
        icon="i-heroicons-folder-open"
          @click="$emit('scan-projects')"
        class="w-full sm:w-auto"
        >
        {{ tmReactive('projects', 'open').value }}
        </UButton>
    </template>
  </UEmpty>
</template>

<script setup lang="ts">
import { useLocale } from '~/composables/useLocale'

interface Emits {
  (e: 'scan-projects'): void
}

defineEmits<Emits>()

const { tmReactive } = useLocale()
</script>
