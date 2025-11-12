<script setup lang="ts">
import { h, resolveComponent } from 'vue'
import { getPaginationRowModel } from '@tanstack/vue-table'
import { useAppLocale } from '~/composables/useLocale'
import type { TableColumn, TableRow } from '@nuxt/ui'
import type { TextEntry } from '~/types/scanning-commands'

const UBadge = resolveComponent('UBadge')
const UCheckbox = resolveComponent('UCheckbox')
const UButton = resolveComponent('UButton')
const table = useTemplateRef('table')

interface Props {
  texts: TextEntry[]
}

defineProps<Props>()

const { tmReactive } = useAppLocale()

const pagination = ref({
  pageIndex: 0,
  pageSize: 10
})

const rowSelection = ref<Record<string, boolean>>({})

const globalFilter = ref('')

const sorting = ref([
  {
    id: 'status',
    desc: false
  }
])

const columns: TableColumn<TextEntry>[] = [
  {
    id: 'select',
    header: ({ table }) =>
      h(UCheckbox, {
        modelValue: table.getIsSomePageRowsSelected()
          ? 'indeterminate'
          : table.getIsAllPageRowsSelected(),
        'onUpdate:modelValue': (value: boolean | 'indeterminate') =>
          table.toggleAllPageRowsSelected(!!value),
        'aria-label': 'Sélectionner tout'
      }),
    cell: ({ row }) =>
      h(UCheckbox, {
        modelValue: row.getIsSelected(),
        'onUpdate:modelValue': (value: boolean | 'indeterminate') => row.toggleSelected(!!value),
        'aria-label': 'Sélectionner la ligne'
      })
  },
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
        class: 'max-w-xs truncate',
        title: sourceText
      }, sourceText)
    }
  },
  {
    accessorKey: 'translated_text',
    header: ({ column }) => {
      const isSorted = column.getIsSorted()
      return h(UButton, {
        color: 'neutral',
        variant: 'ghost',
        label: 'Traduction',
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
      const translatedText = row.getValue('translated_text') as string
      return h('div', {
        class: 'max-w-xs truncate',
        title: translatedText || '-'
      }, translatedText || '-')
    }
  },
  {
    accessorKey: 'status',
    header: ({ column }) => {
      const isSorted = column.getIsSorted()
      return h(UButton, {
        color: 'neutral',
        variant: 'ghost',
        label: 'Statut',
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
      const status = row.getValue('status') as string
      const color = getStatusColor(status)

      return h(UBadge, {
        color,
        variant: 'soft'
      }, () => status)
    }
  },
  {
    accessorKey: 'prompt_type',
    header: 'Type',
    cell: ({ row }) => {
      const promptType = row.getValue('prompt_type') as string
      return h('span', { class: 'text-sm' }, promptType)
    }
  },
  {
    accessorKey: 'context',
    header: 'Contexte',
    cell: ({ row }) => {
      const context = row.getValue('context') as string
      return h('div', {
        class: 'max-w-sm truncate text-sm text-gray-600',
        title: context
      }, context)
    }
  }
]

function getStatusColor(status: string): "primary" | "success" | "warning" | "error" | "neutral" {
  switch (status) {
    case 'Translated': return 'success'
    case 'InProgress': return 'warning'
    case 'Ignored': return 'neutral'
    case 'NotTranslated': return 'error'
    default: return 'neutral'
  }
}

function onSelect(e: Event, row: TableRow<TextEntry>) {
  row.toggleSelected(!row.getIsSelected())
}
</script>

<template>
  <div class="texts-table">
    <div class="flex flex-col flex-1 w-full">
      <div class="flex px-4 py-3.5 border-b border-accented">
        <UInput
          v-model="globalFilter"
          class="max-w-sm"
          placeholder="Rechercher dans les textes..."
        />
      </div>

      <UTable
        ref="table"
        v-model:pagination="pagination"
        v-model:row-selection="rowSelection"
        v-model:global-filter="globalFilter"
        v-model:sorting="sorting"
        :data="texts"
        :columns="columns"
        :pagination-options="{
          getPaginationRowModel: getPaginationRowModel()
        }"
        :empty-state="{
          icon: 'i-heroicons-document-text',
          label: tmReactive('texts', 'no_texts_found').value,
          description: tmReactive('texts', 'no_texts_description').value
        }"
        @select="onSelect"
        class="flex-1"
      />

      <div class="px-4 py-3.5 border-t border-accented text-sm text-muted">
        {{ table?.tableApi?.getFilteredSelectedRowModel().rows.length || 0 }} sur
        {{ table?.tableApi?.getFilteredRowModel().rows.length || 0 }} ligne(s) sélectionnée(s).
      </div>

      <div v-if="texts.length > pagination.pageSize" class="flex justify-center border-t border-default pt-4">
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
