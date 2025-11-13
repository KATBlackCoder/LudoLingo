<script setup lang="ts">
import { h, resolveComponent } from 'vue'
import { getPaginationRowModel } from '@tanstack/vue-table'
import { useTranslationStore } from '~/stores/translation'
import { useProjectsStore } from '~/stores/projects'
import type { TableColumn } from '@nuxt/ui'
import type { TextEntry } from '~/types/scanning-commands'

const UButton = resolveComponent('UButton')
const UBadge = resolveComponent('UBadge')
const table = useTemplateRef('table')

const translationStore = useTranslationStore()
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

// Textes en cours de traduction depuis le store
const inProgressTexts = computed(() => {
  const project = projectsStore.currentProject
  if (!project) return []
  
  // Filtrer les textes avec status InProgress OU dans textsBeingTranslated
  // Note: textsBeingTranslated utilise des numbers, donc on convertit text.id
  return project.extractedTexts.filter(text => {
    const textIdNum = parseInt(text.id, 10)
    const isNumericId = !isNaN(textIdNum)
    return text.status === 'InProgress' || 
           (isNumericId && translationStore.textsBeingTranslated.has(textIdNum))
  })
})

// Progression actuelle depuis les sessions actives
const currentProgress = computed(() => {
  const sessions = translationStore.activeSessions.filter(s => s.status === 'running')
  if (sessions.length === 0) return null
  
  const totalProcessed = sessions.reduce((sum, s) => sum + (s.processed_count || 0), 0)
  const totalEntries = sessions.reduce((sum, s) => sum + (s.total_count || 0), 0)
  
  return {
    processed: totalProcessed,
    total: totalEntries,
    percentage: totalEntries > 0 ? Math.round((totalProcessed / totalEntries) * 100) : 0
  }
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
    accessorKey: 'status',
    header: 'Statut',
    cell: ({ row }) => {
      return h('div', { class: 'flex items-center gap-2' }, [
        h('span', { class: 'animate-spin text-primary' }, '⏳'),
        h(UBadge, { color: 'warning', variant: 'soft' }, () => 'En cours...')
      ])
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
  }
]
</script>

<template>
  <div class="in-progress-table">
    <div class="mb-4">
      <h2 class="text-xl font-semibold mb-2">Textes en Cours de Traduction</h2>
      <div v-if="currentProgress" class="mb-2">
        <div class="flex items-center justify-between mb-1">
          <span class="text-sm text-gray-600 dark:text-gray-400">
            Progression globale
          </span>
          <span class="text-sm font-medium">
            {{ currentProgress.processed }} / {{ currentProgress.total }}
            ({{ currentProgress.percentage }}%)
          </span>
        </div>
        <UProgress 
          :value="currentProgress.percentage"
          color="primary"
        />
      </div>
      <p class="text-sm text-gray-600 dark:text-gray-400">
        {{ inProgressTexts.length }} texte(s) en cours
      </p>
    </div>

    <div class="flex flex-col flex-1 w-full">
      <div class="flex px-4 py-3.5 border-b border-accented">
        <UInput
          v-model="globalFilter"
          class="max-w-sm"
          placeholder="Rechercher dans les textes en cours..."
        />
      </div>

      <UTable
        ref="table"
        v-model:pagination="pagination"
        v-model:global-filter="globalFilter"
        v-model:sorting="sorting"
        :data="inProgressTexts"
        :columns="columns"
        :pagination-options="{
          getPaginationRowModel: getPaginationRowModel()
        }"
        :empty-state="{
          icon: 'i-heroicons-clock',
          label: 'Aucune traduction en cours',
          description: 'Aucun texte n\'est actuellement en cours de traduction'
        }"
        class="flex-1"
      />

      <div v-if="inProgressTexts.length > pagination.pageSize" class="flex justify-center border-t border-default pt-4">
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

