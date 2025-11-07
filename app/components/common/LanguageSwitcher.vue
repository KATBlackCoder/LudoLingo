<template>
  <ULocaleSelect
    v-model="currentLocale"
    :locales="supportedLocales"
    class="w-48"
  />
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import * as locales from '@nuxt/ui/locale'
import type { SupportedLocale } from '~/stores/settings'
import { supportedLanguages } from '~/i18n/locales'
import { useSettingsStore } from '~/stores/settings'

const settingsStore = useSettingsStore()

// Filtrer seulement les locales supportées par notre app
const supportedLocales = Object.values(locales).filter(locale =>
  supportedLanguages.includes(locale.code as SupportedLocale)
)

const currentLocale = ref<SupportedLocale>(settingsStore.settings.ui.language)

// Synchroniser avec le store quand la langue change
watch(currentLocale, async (newLocale) => {
  if (newLocale && newLocale !== settingsStore.settings.ui.language) {
    try {
      await settingsStore.updateLanguage(newLocale)
    } catch (error) {
      console.error('Failed to change language:', error)
    }
  }
})

onMounted(() => {
  currentLocale.value = settingsStore.settings.ui.language
})
</script>
