<template>
  <UModal
    v-model="isOpen"
    :title="editingProject ? tmReactive('projects', 'edit_project').value : tmReactive('projects', 'create_project').value"
  >
    <template #body>
      <form @submit.prevent="handleSubmit" class="space-y-4">
        <UFormField
          :label="tmReactive('projects', 'project_name').value"
          required
        >
          <UInput
            v-model="formData.name"
            :placeholder="tmReactive('projects', 'project_name_placeholder').value"
            required
          />
        </UFormField>

        <UFormField :label="tmReactive('projects', 'project_description').value">
          <UTextarea
            v-model="formData.description"
            :placeholder="tmReactive('projects', 'project_description_placeholder').value"
            :rows="3"
          />
        </UFormField>

        <UFormField :label="tmReactive('projects', 'game_path').value" required>
          <div class="flex gap-2">
            <UInput
              v-model="formData.game_path"
              :placeholder="tmReactive('projects', 'game_path_placeholder').value"
              class="flex-1"
              required
              readonly
            />
            <UButton
              icon="i-heroicons-folder-open"
              color="neutral"
              variant="outline"
              @click="selectGameFolder"
            >
              {{ tmReactive('common', 'browse').value }}
            </UButton>
          </div>
        </UFormField>

        <UFormField :label="tmReactive('projects', 'game_engine').value">
          <USelect
            v-model="formData.game_engine"
            :items="engineSelectOptions"
            value-key="value"
            :placeholder="tmReactive('projects', 'auto_detect').value"
          />
        </UFormField>

        <div class="flex justify-end gap-2 pt-4">
          <UButton
            type="button"
            color="neutral"
            variant="ghost"
            @click="handleCancel"
          >
            {{ tmReactive('common', 'cancel').value }}
          </UButton>
          <UButton
            type="submit"
            :loading="isSaving"
          >
            {{ tmReactive('common', 'save').value }}
          </UButton>
        </div>
      </form>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import type { ProjectDB, CreateProjectData } from '~/composables/db/project'
import { useAppLocale } from '~/composables/useLocale'

interface Props {
  modelValue: boolean
  editingProject?: ProjectDB | null
}

interface Emits {
  (e: 'update:modelValue', value: boolean): void
  (e: 'submit', data: CreateProjectData, isEditing: boolean, projectId?: number): void
  (e: 'cancel'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const { tmReactive } = useAppLocale()

// State
const isSaving = ref(false)

// Form data
const formData = ref<CreateProjectData>({
  name: '',
  description: '',
  game_path: '',
  game_engine: undefined
})

// Computed
const isOpen = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})

const engineSelectOptions = computed(() => [
  { label: 'RPG Maker MZ', value: 'RPG Maker MZ' },
  { label: 'RPG Maker MV', value: 'RPG Maker MV' }
])

// Watchers
watch(() => props.editingProject, (newProject) => {
  if (newProject) {
    // Editing mode
    formData.value = {
      name: newProject.name,
      description: newProject.description || '',
      game_path: newProject.game_path,
      game_engine: newProject.game_engine
    }
  } else {
    // Creating mode
    formData.value = {
      name: '',
      description: '',
      game_path: '',
      game_engine: undefined
    }
  }
}, { immediate: true })

// Methods
async function selectGameFolder() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: tmReactive('projects', 'select_game_folder').value
    })

    if (selected && typeof selected === 'string') {
      formData.value.game_path = selected
      // Could auto-detect engine here
    }
  } catch (err) {
    console.error('Error selecting folder:', err)
  }
}

async function handleSubmit() {
  isSaving.value = true
  try {
    const isEditing = !!props.editingProject
    const projectId = props.editingProject?.id

    emit('submit', { ...formData.value }, isEditing, projectId)
  } finally {
    isSaving.value = false
  }
}

function handleCancel() {
  emit('cancel')
}

// Expose isSaving for parent component
defineExpose({
  isSaving
})
</script>
