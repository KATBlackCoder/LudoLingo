<template>
  <USelect
    v-model="currentLocale"
    :items="localeOptions"
    placeholder="Select language"
    class="w-48"
  />
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import * as locales from '@nuxt/ui/locale'
import type { SupportedLocale } from '~/stores/settings'
import { supportedLanguages, getLocaleFlag } from '~/i18n/locales'
import { useSettingsStore } from '~/stores/settings'

const settingsStore = useSettingsStore()

const currentLocale = ref<SupportedLocale>(settingsStore.settings.ui.language)

const localeOptions = computed(() =>
  Object.entries(locales)
    .filter(([code]) => supportedLanguages.includes(code as SupportedLocale))
    .map(([code, locale]: [string, any]) => ({
      label: `${getLocaleFlag(code)} ${locale.name}`,
      value: code as SupportedLocale
    }))
)


// Watcher pour synchroniser avec le store quand la valeur change
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
