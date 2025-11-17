<template>
  <UApp :locale="locale">

    <UMain>
      <NuxtLayout>
        <NuxtPage />
      </NuxtLayout>
    </UMain>

  </UApp>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { useAppLocale } from '~/composables/useLocale'
import { setupGlossaryBridge } from '~/composables/db/glossary/glossaryBridge'
import { useAutoUpdate } from '~/composables/updater/useAutoUpdate'

const { locale, lang, dir } = useAppLocale()

useHead({
  htmlAttrs: {
    lang,
    dir
  }
})

// Setup glossary bridge for backend-frontend communication
let unlistenGlossaryBridge: (() => void) | null = null

// Setup auto-update checking
const { initializeAutoCheck, stopAutoCheck } = useAutoUpdate()

onMounted(async () => {
  try {
    unlistenGlossaryBridge = await setupGlossaryBridge()
    console.log('[App] Glossary bridge initialized')
  } catch (error) {
    console.error('[App] Failed to setup glossary bridge:', error)
  }

  // Initialize automatic update checking
  try {
    await initializeAutoCheck()
    console.log('[App] Auto-update checking initialized')
  } catch (error) {
    console.error('[App] Failed to initialize auto-update:', error)
  }
})

onUnmounted(() => {
  if (unlistenGlossaryBridge) {
    unlistenGlossaryBridge()
    console.log('[App] Glossary bridge cleaned up')
  }

  // Stop auto-update checking
  stopAutoCheck()
})
</script>