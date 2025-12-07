<script setup lang="ts">
import { h, resolveComponent, computed } from 'vue'
import { getPaginationRowModel } from '@tanstack/vue-table'
import type { TableColumn } from '@nuxt/ui'
import type { GlossaryEntry } from '~/composables/db/glossary'
import { useProjectsStore } from '~/stores/projects'

const UBadge = resolveComponent('UBadge')
const UButton = resolveComponent('UButton')
const UCheckbox = resolveComponent('UCheckbox')
const UIcon = resolveComponent('UIcon')
const UInput = resolveComponent('UInput')
const table = useTemplateRef('table')

const projectsStore = useProjectsStore()

interface Props {
  entries: GlossaryEntry[]
  loading?: boolean
}

interface Emits {
  (e: 'edit', entry: GlossaryEntry): void
  (e: 'delete', entry: GlossaryEntry): void
  (e: 'delete-multiple', entries: GlossaryEntry[]): void
}

const props = withDefaults(defineProps<Props>(), {
  loading: false
})

const emit = defineEmits<Emits>()

const pagination = ref({
  pageIndex: 0,
  pageSize: 10
})

const globalFilter = ref('')

const sorting = ref([
  {
    id: 'source_term',
    desc: false
  }
])

// Selection state for bulk operations
const selectedEntries = ref<Set<string>>(new Set())
const isAllSelected = computed(() => {
  const visibleEntries = table.value?.tableApi?.getFilteredRowModel().rows || []
  return visibleEntries.length > 0 && visibleEntries.every(row => selectedEntries.value.has(row.original.id.toString()))
})
const isIndeterminate = computed(() => {
  const visibleEntries = table.value?.tableApi?.getFilteredRowModel().rows || []
  const selectedCount = visibleEntries.filter(row => selectedEntries.value.has(row.original.id.toString())).length
  return selectedCount > 0 && selectedCount < visibleEntries.length
})

// Category badge colors (using Nuxt UI valid colors)
const categoryColors: Record<string, 'primary' | 'success' | 'warning' | 'error' | 'neutral' | 'info' | 'secondary'> = {
  general: 'neutral',
  character: 'primary',
  item: 'success',
  location: 'info',
  system: 'warning',
  skill: 'error'
}

const categoryLabels: Record<string, string> = {
  general: 'Général',
  character: 'Personnage',
  item: 'Objet',
  location: 'Lieu',
  system: 'Système',
  skill: 'Compétence'
}

const columns: TableColumn<GlossaryEntry>[] = [
  {
    id: 'select',
    header: ({ table: tableApi }) => h(UCheckbox, {
      modelValue: isAllSelected.value,
      indeterminate: isIndeterminate.value,
      onChange: toggleAllSelection,
      disabled: props.loading
    }),
    cell: ({ row }) => h(UCheckbox, {
      modelValue: selectedEntries.value.has(row.original.id.toString()),
      onChange: () => toggleEntrySelection(row.original.id.toString()),
      disabled: props.loading
    })
  },
  {
    id: 'source_term',
    accessorKey: 'source_term',
    header: 'Terme source',
    cell: ({ row }) => h('div', { class: 'font-medium' }, row.original.source_term)
  },
  {
    id: 'translated_term',
    accessorKey: 'translated_term',
    header: 'Traduction',
    cell: ({ row }) => h('div', { class: 'text-muted' }, row.original.translated_term)
  },
  {
    id: 'source_language',
    accessorKey: 'source_language',
    header: 'Langue source',
    cell: ({ row }) => h('div', { class: 'text-sm uppercase text-muted' }, row.original.source_language)
  },
  {
    id: 'target_language',
    accessorKey: 'target_language',
    header: 'Langue cible',
    cell: ({ row }) => h('div', { class: 'text-sm uppercase text-muted' }, row.original.target_language)
  },
  {
    id: 'category',
    accessorKey: 'category',
    header: 'Catégorie',
    cell: ({ row }) =>
      h(UBadge, {
        color: categoryColors[row.original.category] || 'neutral',
        variant: 'subtle'
      }, () => categoryLabels[row.original.category] || row.original.category)
  },
  {
    id: 'project_scope',
    accessorKey: 'project_id',
    header: 'Portée',
    cell: ({ row }) => {
      const entry = row.original
      if (entry.project_id === null || entry.project_id === undefined) {
        // Global term
        return h(UBadge, {
          color: 'info',
          variant: 'subtle'
        }, () => '🌍 Global')
      } else {
        // Project-specific term - try to get project name
        const project = projectsStore.projects.find(p => p.id === entry.project_id)
        const projectName = project ? project.name : `Projet #${entry.project_id}`
        return h(UBadge, {
          color: 'primary',
          variant: 'subtle'
        }, () => `📁 ${projectName}`)
      }
    }
  },
  {
    id: 'actions',
    header: 'Actions',
    cell: ({ row }) =>
      h('div', { class: 'flex gap-2' }, [
        h(UButton, {
          size: 'xs',
          variant: 'ghost',
          color: 'primary',
          onClick: () => emit('edit', row.original)
        }, () => [
          h(UIcon, { name: 'i-heroicons-pencil', class: 'w-4 h-4' })
        ]),
        h(UButton, {
          size: 'xs',
          variant: 'ghost',
          color: 'red',
          onClick: () => emit('delete', row.original)
        }, () => [
          h(UIcon, { name: 'i-heroicons-trash', class: 'w-4 h-4' })
        ])
      ])
  }
]

const handleEdit = (entry: GlossaryEntry) => {
  emit('edit', entry)
}

const handleDelete = (entry: GlossaryEntry) => {
  emit('delete', entry)
}

// Bulk selection handlers
const toggleEntrySelection = (entryId: string) => {
  if (selectedEntries.value.has(entryId)) {
    selectedEntries.value.delete(entryId)
  } else {
    selectedEntries.value.add(entryId)
  }
}

const toggleAllSelection = () => {
  const visibleEntries = table.value?.tableApi?.getFilteredRowModel().rows || []
  const allSelected = isAllSelected.value

  if (allSelected) {
    // Deselect all visible entries
    visibleEntries.forEach(row => {
      selectedEntries.value.delete(row.original.id.toString())
    })
  } else {
    // Select all visible entries
    visibleEntries.forEach(row => {
      selectedEntries.value.add(row.original.id.toString())
    })
  }
}

const clearSelection = () => {
  selectedEntries.value.clear()
}

const handleDeleteMultiple = () => {
  const entriesToDelete = props.entries.filter(entry =>
    selectedEntries.value.has(entry.id.toString())
  )
  if (entriesToDelete.length > 0) {
    emit('delete-multiple', entriesToDelete)
  }
}

const selectedCount = computed(() => selectedEntries.value.size)
</script>

<template>
  <div class="glossary-table">
    <div class="flex flex-col flex-1 w-full">
      <!-- Bulk actions toolbar -->
      <div v-if="selectedCount > 0" class="flex items-center justify-between px-4 py-2 bg-primary-50 dark:bg-primary-950 border-b border-accented">
        <div class="flex items-center gap-3">
          <span class="text-sm font-medium text-primary-700 dark:text-primary-300">
            {{ selectedCount }} entrée{{ selectedCount > 1 ? 's' : '' }} sélectionnée{{ selectedCount > 1 ? 's' : '' }}
          </span>
          <UButton
            size="xs"
            variant="ghost"
            color="gray"
            @click="clearSelection"
          >
            Désélectionner
          </UButton>
        </div>
        <UButton
          size="xs"
          color="red"
          variant="solid"
          @click="handleDeleteMultiple"
          :loading="loading"
        >
          <UIcon name="i-heroicons-trash" class="w-4 h-4 mr-1" />
          Supprimer sélection
        </UButton>
      </div>

      <div class="flex px-4 py-3.5 border-b border-accented">
        <UInput
          v-model="globalFilter"
          class="max-w-sm"
          placeholder="Rechercher dans le glossaire..."
          :disabled="loading"
        />
      </div>


      <UTable
        ref="table"
        v-model:pagination="pagination"
        v-model:global-filter="globalFilter"
        v-model:sorting="sorting"
        :data="entries"
        :columns="columns"
        :pagination-options="{
          getPaginationRowModel: getPaginationRowModel()
        }"
        :empty-state="{
          icon: 'i-heroicons-book-open',
          label: 'Aucune entrée dans le glossaire',
          description: 'Ajoutez des termes pour enrichir vos traductions'
        }"
        :loading="loading"
        class="flex-1"
      />

      <div v-if="entries.length > pagination.pageSize" class="flex justify-center border-t border-default pt-4">
        <UPagination
          :default-page="(table?.tableApi?.getState().pagination.pageIndex || 0) + 1"
          :items-per-page="table?.tableApi?.getState().pagination.pageSize"
          :total="table?.tableApi?.getFilteredRowModel().rows.length"
          @update:page="(p) => table?.tableApi?.setPageIndex(p - 1)"
        />
      </div>
    </div>
  </div>
</template>

