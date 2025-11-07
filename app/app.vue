<template>
  <UApp :locale="currentLocale">

    <UMain>
      <NuxtLayout>
        <NuxtPage />
      </NuxtLayout>
    </UMain>

  </UApp>
</template>

<script setup lang="ts">
import * as locales from '@nuxt/ui/locale'
import { useSettingsStore } from '~/stores/settings'

const settingsStore = useSettingsStore()

// Locale réactive basée sur les paramètres utilisateur
const currentLocale = computed(() => {
  const userLocale = settingsStore.settings.ui.language
  // Type assertion pour éviter l'erreur de type
  return (locales as any)[userLocale] || locales.fr
})

// Synchronisation avec les attributs HTML
const lang = computed(() => currentLocale.value.code)
const dir = computed(() => currentLocale.value.dir)

useHead({
  htmlAttrs: {
    lang,
    dir
  }
})
</script>