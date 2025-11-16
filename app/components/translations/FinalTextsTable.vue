<script setup lang="ts">
import { h, resolveComponent, nextTick, computed, watch, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { getPaginationRowModel } from '@tanstack/vue-table'
import { useClipboard } from '@vueuse/core'
import { useProjectsStore } from '~/stores/projects'
import { useTranslationStore } from '~/stores/translation'
import { useNotifications } from '~/composables/useNotifications'
import { useSettings } from '~/composables/useTauriSetting'
import { translateSingleText } from '~/composables/db/texts/translation'
import { extractToGlossary } from '~/composables/db/glossary'
import { useGlossaryStore } from '~/stores/glossary'
import EditTranslationModal from '~/components/translations/EditTranslationModal.vue'
import type { TableColumn } from '@nuxt/ui'
import type { TextEntry } from '~/types/scanning-commands'

const UBadge = resolveComponent('UBadge')
const UButton = resolveComponent('UButton')
const table = useTemplateRef('table')

const projectsStore = useProjectsStore()
const translationStore = useTranslationStore()
const glossaryStore = useGlossaryStore()

// Vérifier si une traduction est en cours
const { hasActiveSessions } = storeToRefs(translationStore)

// Get current project ID for glossary extraction
const currentProjectId = computed(() => projectsStore.currentProject?.id ?? null)

// Filtrer uniquement les textes traduits (défini en premier pour être utilisé par selectedTexts)
const finalTexts = computed(() => {
  const project = projectsStore.currentProject
  if (!project) return []
  return project.extractedTexts.filter(
    text => text.translated_text && text.status === 'Translated'
  )
})

// État de sélection des lignes (utilise les IDs des textes comme clés)
const rowSelection = ref<Record<string, boolean>>({})

// Computed pour les textes sélectionnés
const selectedTexts = computed(() => {
  return finalTexts.value.filter(text => rowSelection.value[text.id])
})

// Exposer la sélection au parent via emit
const emit = defineEmits<{
  selectionChange: [count: number]
}>()

// Surveiller les changements de sélection
watch(rowSelection, () => {
  try {
    const newSelected = selectedTexts.value || []
    translationStore.setSelectedTextsForRetranslation(newSelected.map(text => ({
      id: text.id,
      source_text: text.source_text,
      location: text.location,
      prompt_type: text.prompt_type
    })))
    emit('selectionChange', newSelected.length)
  } catch (error) {
    console.warn('Error in selection watch:', error)
  }
}, { deep: true })

// Exposer une méthode pour réinitialiser la sélection
const clearSelection = () => {
  rowSelection.value = {}
  translationStore.setSelectedTextsForRetranslation([])
  emit('selectionChange', 0)
}

defineExpose({
  clearSelection
})

// Réinitialiser la sélection quand les textes changent (après retraduction)
watch(finalTexts, () => {
  // Si les textes sélectionnés ne sont plus dans finalTexts, réinitialiser
  const selectedIds = Object.keys(rowSelection.value).filter(id => rowSelection.value[id])
  const stillPresent = selectedIds.some(id => finalTexts.value.some(text => text.id === id))
  if (!stillPresent && selectedIds.length > 0) {
    clearSelection()
  }
})

const { copy, copied } = useClipboard()
const { notifySuccess, notifyError } = useNotifications()
const settings = useSettings()

// Modal state
const isModalOpen = ref(false)
const editingText = ref<TextEntry | null>(null)

// État de chargement pour les retraductions individuelles
const retranslatingTextIds = ref<Set<number>>(new Set())

// État de chargement pour les extractions vers le glossaire
const extractingTextIds = ref<Set<number>>(new Set())

// Fonction pour copier le texte
const handleCopyText = async (text: string, type: 'source' | 'translated') => {
  await copy(text)
  if (copied.value) {
    await notifySuccess('Texte copié dans le presse-papiers', 'Copie réussie')
  }
}

// Ouvrir le modal d'édition
const openEditModal = (text: TextEntry) => {
  editingText.value = text
  isModalOpen.value = true
}

// Fermer le modal
const closeModal = () => {
  isModalOpen.value = false
  editingText.value = null
}

// Callback quand la traduction est sauvegardée
const handleTranslationSaved = () => {
  // Forcer le rafraîchissement de la table en recalculant les données
  // La réactivité Vue devrait déjà mettre à jour, mais on force un refresh
  nextTick(() => {
    // Les computed se mettront à jour automatiquement grâce à la réactivité
  })
}

// Extraire vers le glossaire
const handleExtractToGlossary = async (text: TextEntry) => {
  const textId = parseInt(text.id, 10)
  if (isNaN(textId)) {
    notifyError('Erreur', 'ID de texte invalide')
    return
  }

  // Empêcher les clics multiples
  if (extractingTextIds.value.has(textId)) {
    return
  }

  // Validation : vérifier que le texte est traduit
  if (!text.translated_text || !text.source_text) {
    notifyError('Erreur', 'Le texte doit être traduit pour être ajouté au glossaire')
    return
  }

  extractingTextIds.value.add(textId)

  try {
    // Récupérer les settings utilisateur pour obtenir les langues
    const userSettings = await settings.loadSettings()
    
    // Extraire vers le glossaire avec le project_id du projet actuel (ou null pour global)
    const result = await extractToGlossary(
      text.source_text,
      text.translated_text,
      userSettings.translation.sourceLanguage,
      userSettings.translation.targetLanguage,
      'general', // Catégorie par défaut, peut être améliorée plus tard
      currentProjectId.value // Utilise le projet actuel par défaut, ou null pour global
    )

    if (result.success && result.data) {
      // Recharger les entrées du glossaire pour mettre à jour le store
      await glossaryStore.loadEntries()
      
      notifySuccess(
        'Terme ajouté au glossaire',
        `"${text.source_text}" → "${text.translated_text}" a été ajouté au glossaire`
      )
    } else {
      throw new Error(result.error || 'Échec de l\'extraction vers le glossaire')
    }
  } catch (error) {
    notifyError(
      'Erreur d\'extraction',
      error instanceof Error ? error.message : 'Une erreur est survenue lors de l\'extraction vers le glossaire'
    )
  } finally {
    extractingTextIds.value.delete(textId)
  }
}

// Retraduire directement (sans ouvrir le modal)
const handleQuickRetranslate = async (text: TextEntry) => {
  const textId = parseInt(text.id, 10)
  if (isNaN(textId)) {
    notifyError('Erreur', 'ID de texte invalide')
    return
  }

  // Empêcher les clics multiples
  if (retranslatingTextIds.value.has(textId)) {
    return
  }

  retranslatingTextIds.value.add(textId)

  try {
    // Récupérer les settings utilisateur pour utiliser le bon modèle
    const userSettings = await settings.loadSettings()
    
    const result = await translateSingleText(
      text.source_text,
      userSettings.translation.sourceLanguage,
      userSettings.translation.targetLanguage,
      text.location || undefined,
      userSettings.ollama.model
    )

    if (result.success && result.data) {
      await translationStore.applyTranslation(textId, result.data.translated_text, 'ollama')
      
      // Forcer le rafraîchissement de la table
      await nextTick()
      
      notifySuccess('Retraduction réussie', 'Le texte a été retraduit avec succès')
    } else {
      throw new Error(result.error || 'Échec de la retraduction')
    }
  } catch (error) {
    notifyError(
      'Erreur de retraduction',
      error instanceof Error ? error.message : 'Une erreur est survenue lors de la retraduction'
    )
  } finally {
    retranslatingTextIds.value.delete(textId)
  }
}

const pagination = ref({
  pageIndex: 0,
  pageSize: 10
})

const globalFilter = ref('')

const sorting = ref([
  {
    id: 'translated_text',
    desc: false
  }
])

const columns: TableColumn<TextEntry>[] = [
  {
    id: 'select',
    header: ({ table }) => {
      return h('input', {
        type: 'checkbox',
        checked: table.getIsAllRowsSelected(),
        indeterminate: table.getIsSomeRowsSelected(),
        onChange: table.getToggleAllRowsSelectedHandler(),
        class: 'w-4 h-4 text-primary bg-gray-100 border-gray-300 rounded focus:ring-primary dark:bg-gray-700 dark:border-gray-600'
      })
    },
    cell: ({ row }) => {
      return h('input', {
        type: 'checkbox',
        checked: row.getIsSelected(),
        onChange: row.getToggleSelectedHandler(),
        class: 'w-4 h-4 text-primary bg-gray-100 border-gray-300 rounded focus:ring-primary dark:bg-gray-700 dark:border-gray-600'
      })
    },
    enableSorting: false,
    enableHiding: false
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
      const text = row.getValue('source_text') as string
      return h('div', {
        class: 'flex items-center gap-2 group max-w-md'
      }, [
        h('div', {
          class: 'flex-1 truncate text-gray-600 dark:text-gray-400 cursor-pointer max-w-lg',
          title: text,
          onClick: () => handleCopyText(text, 'source')
        }, text),
        h('button', {
          class: 'opacity-0 group-hover:opacity-100 transition-opacity p-1 hover:bg-gray-100 dark:hover:bg-gray-800 rounded',
          title: 'Copier le texte original',
          onClick: () => handleCopyText(text, 'source')
        }, [
          h('span', {
            class: 'i-heroicons-clipboard-document w-4 h-4 text-gray-500 dark:text-gray-400'
          })
        ])
      ])
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
      const text = row.getValue('translated_text') as string
      return h('div', {
        class: 'flex items-center gap-2 group max-w-md'
      }, [
        h('div', {
          class: 'flex-1 truncate font-medium cursor-pointer max-w-lg',
          title: text,
          onClick: () => handleCopyText(text, 'translated')
        }, text),
        h('button', {
          class: 'opacity-0 group-hover:opacity-100 transition-opacity p-1 hover:bg-gray-100 dark:hover:bg-gray-800 rounded',
          title: 'Copier la traduction',
          onClick: () => handleCopyText(text, 'translated')
        }, [
          h('span', {
            class: 'i-heroicons-clipboard-document w-4 h-4 text-gray-500 dark:text-gray-400'
          })
        ])
      ])
    }
  },
  {
    accessorKey: 'status',
    header: 'Statut',
    cell: ({ row }) => {
      return h(UBadge, { color: 'success', variant: 'soft' }, () => 'Traduit')
    }
  },
  {
    accessorKey: 'context',
    header: 'Contexte',
    cell: ({ row }) => {
      const context = row.getValue('context') as string
      if (!context || context === '-') {
        return h('div', {
          class: 'text-sm text-gray-400 dark:text-gray-500'
        }, '-')
      }
      return h('div', {
        class: 'flex items-center gap-2 group max-w-md'
      }, [
        h('div', {
          class: 'flex-1 truncate text-sm text-gray-600 dark:text-gray-400 cursor-pointer',
          title: context,
          onClick: () => handleCopyText(context, 'source')
        }, context),
        h('button', {
          class: 'opacity-0 group-hover:opacity-100 transition-opacity p-1 hover:bg-gray-100 dark:hover:bg-gray-800 rounded',
          title: 'Copier le contexte',
          onClick: () => handleCopyText(context, 'source')
        }, [
          h('span', {
            class: 'i-heroicons-clipboard-document w-3 h-3 text-gray-500 dark:text-gray-400'
          })
        ])
      ])
    }
  },
  {
    id: 'actions',
    header: 'Actions',
    cell: ({ row }) => {
      const text = row.original as TextEntry
      const textId = parseInt(text.id, 10)
      const isRetranslating = !isNaN(textId) && retranslatingTextIds.value.has(textId)
      const isExtracting = !isNaN(textId) && extractingTextIds.value.has(textId)
      const isProcessing = isRetranslating || isExtracting
      
      return h('div', {
        class: 'flex items-center gap-2'
      }, [
        h(UButton, {
          color: 'primary',
          variant: 'ghost',
          size: 'xs',
          icon: 'i-heroicons-arrow-path',
          title: hasActiveSessions.value ? 'Une traduction est en cours' : isRetranslating ? 'Traduction en cours...' : 'Retraduire avec AI',
          loading: isRetranslating,
          disabled: isProcessing || hasActiveSessions.value,
          onClick: () => handleQuickRetranslate(text)
        }),
        h(UButton, {
          color: 'success',
          variant: 'ghost',
          size: 'xs',
          icon: 'i-heroicons-book-open',
          title: isExtracting ? 'Extraction en cours...' : 'Ajouter au glossaire',
          loading: isExtracting,
          disabled: isProcessing,
          onClick: () => handleExtractToGlossary(text)
        }),
        h(UButton, {
          color: 'gray',
          variant: 'ghost',
          size: 'xs',
          icon: 'i-heroicons-pencil-square',
          title: 'Modifier manuellement',
          disabled: isProcessing,
          onClick: () => openEditModal(text)
        })
      ])
    }
  }
]
</script>

<template>
  <div class="final-texts-table">
    <div class="mb-4">
      <h2 class="text-xl font-semibold mb-2">Résultats Finaux</h2>
      <p class="text-sm text-gray-600 dark:text-gray-400">
        {{ finalTexts.length }} texte(s) traduit(s) avec succès
      </p>
    </div>

    <div class="flex flex-col flex-1 w-full">
      <div class="flex px-4 py-3.5 border-b border-accented">
        <UInput
          v-model="globalFilter"
          class="max-w-sm"
          placeholder="Rechercher dans les traductions..."
        />
      </div>

      <UTable
        ref="table"
        v-model:pagination="pagination"
        v-model:global-filter="globalFilter"
        v-model:sorting="sorting"
        v-model:row-selection="rowSelection"
        :data="finalTexts"
        :columns="columns"
        :enable-row-selection="true"
        :get-row-id="(row) => row.id"
        :pagination-options="{
          getPaginationRowModel: getPaginationRowModel()
        }"
        :empty-state="{
          icon: 'i-heroicons-check-circle',
          label: 'Aucune traduction terminée',
          description: 'Les traductions terminées apparaîtront ici'
        }"
        class="flex-1"
      />

      <!-- Message de sélection -->
      <div v-if="selectedTexts.length > 0" class="px-4 py-2 bg-primary/10 border-t border-default">
        <p class="text-sm text-primary dark:text-primary-400">
          {{ selectedTexts.length }} texte(s) sélectionné(s)
        </p>
      </div>

      <div v-if="finalTexts.length > pagination.pageSize" class="flex justify-center border-t border-default pt-4">
        <UPagination
          :default-page="(table?.tableApi?.getState().pagination.pageIndex || 0) + 1"
          :items-per-page="table?.tableApi?.getState().pagination.pageSize"
          :total="table?.tableApi?.getFilteredRowModel().rows.length"
          @update:page="(p) => table?.tableApi?.setPageIndex(p - 1)"
        />
      </div>
    </div>

    <!-- Modal d'édition/retraduction -->
    <EditTranslationModal
      v-model:open="isModalOpen"
      :text="editingText"
      @saved="handleTranslationSaved"
    />
  </div>
</template>

