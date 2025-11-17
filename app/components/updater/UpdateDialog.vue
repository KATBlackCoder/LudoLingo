<template>
  <UModal
    :open="open"
    title="Mise à jour disponible"
    description="Une nouvelle version de LudoLingo est disponible"
    :ui="{
      wrapper: 'w-full sm:max-w-lg'
    }"
    @update:open="(value: boolean) => $emit('update:open', value)"
  >
    <template #body>
      <div class="space-y-4">
        <!-- Informations de version -->
        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
              Version actuelle
            </span>
            <span class="text-sm text-gray-600 dark:text-gray-400">
              {{ currentVersion }}
            </span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
              Nouvelle version
            </span>
            <UBadge color="success" variant="soft">
              {{ availableUpdate?.version }}
            </UBadge>
          </div>
        </div>

        <!-- Notes de version -->
        <div v-if="availableUpdate?.body" class="space-y-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
            Notes de version
          </label>
          <div class="p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
            <p class="text-sm text-gray-700 dark:text-gray-300 whitespace-pre-wrap">
              {{ availableUpdate.body }}
            </p>
          </div>
        </div>

        <!-- Progression du téléchargement -->
        <UpdateProgress
          v-if="isDownloading || isReadyToInstall"
          :download-progress="downloadProgress"
          :is-downloading="isDownloading"
          :is-ready-to-install="isReadyToInstall"
          @cancel="$emit('cancel-download')"
        />
      </div>
    </template>

    <template #footer="{ close }">
      <div class="flex justify-end gap-2">
        <UButton
          v-if="!isDownloading && !isReadyToInstall"
          variant="ghost"
          color="neutral"
          @click="close"
        >
          Plus tard
        </UButton>
        <UButton
          v-if="isReadyToInstall"
          color="success"
          @click="$emit('install')"
        >
          Installer maintenant
        </UButton>
        <UButton
          v-else-if="!isDownloading"
          color="primary"
          :loading="isChecking"
          @click="$emit('download')"
        >
          Mettre à jour maintenant
        </UButton>
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import UpdateProgress from './UpdateProgress.vue'
import { check } from '@tauri-apps/plugin-updater'

// Type helper pour le type de retour de check()
type CheckResult = Awaited<ReturnType<typeof check>>

interface Props {
  open: boolean
  availableUpdate: CheckResult | null
  currentVersion: string
  isChecking: boolean
  isDownloading: boolean
  isReadyToInstall: boolean
  downloadProgress: number
}

defineProps<Props>()

defineEmits<{
  'update:open': [value: boolean]
  download: []
  install: []
  'cancel-download': []
}>()
</script>

