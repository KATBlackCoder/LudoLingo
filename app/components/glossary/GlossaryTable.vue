<script setup lang="ts">
import { h, resolveComponent } from 'vue'
import { getPaginationRowModel } from '@tanstack/vue-table'
import type { TableColumn } from '@nuxt/ui'
import type { GlossaryEntry } from '~/composables/db/glossary'

const UBadge = resolveComponent('UBadge')
const UButton = resolveComponent('UButton')
const UIcon = resolveComponent('UIcon')
const table = useTemplateRef('table')

interface Props {
  entries: GlossaryEntry[]
  loading?: boolean
}

interface Emits {
  (e: 'edit', entry: GlossaryEntry): void
  (e: 'delete', entry: GlossaryEntry): void
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
</script>

<template>
  <div class="glossary-table">
    <div class="flex flex-col flex-1 w-full">
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

