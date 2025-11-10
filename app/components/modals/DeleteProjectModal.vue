<template>
  <UModal
    v-model="isOpen"
    :title="tmReactive('projects', 'delete_project_confirm').value"
  >
    <template #body>
      <div class="space-y-4">
        <p>{{ tmReactive('projects', 'delete_project_message').value }}</p>
        <div class="bg-red-50 p-4 rounded-lg">
          <div class="font-semibold text-red-900">{{ project?.name }}</div>
          <div class="text-sm text-red-700 mt-1">{{ project?.game_path }}</div>
        </div>
        <p class="text-sm text-gray-600">{{ tmReactive('projects', 'delete_project_warning').value }}</p>
      </div>
    </template>

    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton
          color="neutral"
          variant="ghost"
          @click="handleCancel"
        >
          {{ tmReactive('common', 'cancel').value }}
        </UButton>
        <UButton
          color="error"
          :loading="isDeleting"
          @click="handleDelete"
        >
          {{ tmReactive('common', 'delete').value }}
        </UButton>
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { ProjectDB } from '~/composables/db/project'
import { useAppLocale } from '~/composables/useLocale'

interface Props {
  modelValue: boolean
  project?: ProjectDB | null
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
  (e: 'confirm', project: ProjectDB): void
  (e: 'cancel'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const { tmReactive } = useAppLocale()

// State
const isDeleting = ref(false)

// Computed
const isOpen = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

// Methods
async function handleDelete() {
  if (!props.project) return

  isDeleting.value = true
  try {
    emit('confirm', props.project)
  } finally {
    isDeleting.value = false
  }
}

function handleCancel() {
  emit('cancel')
}

// Expose isDeleting for parent component
defineExpose({
  isDeleting
})
</script>
