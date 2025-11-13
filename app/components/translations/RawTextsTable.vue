<script setup lang="ts">
import { h, resolveComponent } from 'vue'
import { getPaginationRowModel } from '@tanstack/vue-table'
import { useProjectsStore } from '~/stores/projects'
import type { TableColumn } from '@nuxt/ui'
import type { TextEntry } from '~/types/scanning-commands'

const UButton = resolveComponent('UButton')
const table = useTemplateRef('table')

const projectsStore = useProjectsStore()

const pagination = ref({
  pageIndex: 0,
  pageSize: 10
})

const globalFilter = ref('')

const sorting = ref([
  {
    id: 'source_text',
    desc: false
  }
])

// Filtrer uniquement les textes non traduits
const rawTexts = computed(() => {
  const project = projectsStore.currentProject
  if (!project) return []
  return project.extractedTexts.filter(
    text => !text.translated_text || text.status === 'NotTranslated'
  )
})

const columns: TableColumn<TextEntry>[] = [
  {
    accessorKey: 'source_text',
    header: ({ column }) => {
      const isSorted = column.getIsSorted()
      return h(UButton, {
        color: 'neutral',
        variant: 'ghost',
        label: 'Texte Original',
        icon: isSorted
          ? isSorted === 'asc'
            ? 'i-lucide-arrow-up-narrow-wide'
            : 'i-lucide-arrow-down-wide-narrow'
          : 'i-lucide-arrow-up-down',
        class: '-mx-2.5',
        onClick: () => column.toggleSorting(column.getIsSorted() === 'asc')
      })
    },
    cell: ({ row }) => {
      const sourceText = row.getValue('source_text') as string
      return h('div', {
        class: 'max-w-lg truncate',
        title: sourceText
      }, sourceText)
    }
  },
  {
    accessorKey: 'context',
    header: 'Contexte',
    cell: ({ row }) => {
      const context = row.getValue('context') as string
      return h('div', {
        class: 'max-w-md truncate text-sm text-gray-600 dark:text-gray-400',
        title: context
      }, context || '-')
    }
  },
  {
    accessorKey: 'prompt_type',
    header: 'Type',
    cell: ({ row }) => {
      const promptType = row.getValue('prompt_type') as string
      return h('span', { class: 'text-sm' }, promptType || '-')
    }
  }
]
</script>

<template>
  <div class="raw-texts-table">
    <div class="mb-4">
      <h2 class="text-xl font-semibold mb-2">Textes Bruts Extraits</h2>
      <p class="text-sm text-gray-600 dark:text-gray-400">
        {{ rawTexts.length }} texte(s) en attente de traduction
      </p>
    </div>

    <div class="flex flex-col flex-1 w-full">
      <div class="flex px-4 py-3.5 border-b border-accented">
        <UInput
          v-model="globalFilter"
          class="max-w-sm"
          placeholder="Rechercher dans les textes bruts..."
        />
      </div>

      <UTable
        ref="table"
        v-model:pagination="pagination"
        v-model:global-filter="globalFilter"
        v-model:sorting="sorting"
        :data="rawTexts"
        :columns="columns"
        :pagination-options="{
          getPaginationRowModel: getPaginationRowModel()
        }"
        :empty-state="{
          icon: 'i-heroicons-document-text',
          label: 'Aucun texte brut',
          description: 'Tous les textes ont été traduits'
        }"
        class="flex-1"
      />

      <div v-if="rawTexts.length > pagination.pageSize" class="flex justify-center border-t border-default pt-4">
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

