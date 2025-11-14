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

const { locale, lang, dir } = useAppLocale()

useHead({
  htmlAttrs: {
    lang,
    dir
  }
})

// Setup glossary bridge for backend-frontend communication
let unlistenGlossaryBridge: (() => void) | null = null

onMounted(async () => {
  try {
    unlistenGlossaryBridge = await setupGlossaryBridge()
    console.log('[App] Glossary bridge initialized')
  } catch (error) {
    console.error('[App] Failed to setup glossary bridge:', error)
  }
})

onUnmounted(() => {
  if (unlistenGlossaryBridge) {
    unlistenGlossaryBridge()
    console.log('[App] Glossary bridge cleaned up')
  }
})
</script>