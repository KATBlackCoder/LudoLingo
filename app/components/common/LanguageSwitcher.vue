<template>
  <ULocaleSelect
    :model-value="currentLocaleCode"
    :locales="supportedLocales"
    class="w-48"
    @update:model-value="handleLocaleChange"
  />
</template>

<script setup lang="ts">
import * as locales from '@nuxt/ui/locale'
import { supportedLanguages, type SupportedLanguage } from '~/i18n/locales'
import { useAppLocale } from '~/composables/useLocale'

const { currentLocaleCode, setLocale } = useAppLocale()

// Filtrer seulement les locales supportées par notre app
const supportedLocales = Object.values(locales).filter(locale =>
  supportedLanguages.includes(locale.code as SupportedLanguage)
)

// Gérer le changement de locale
function handleLocaleChange(newLocale: string | undefined) {
  if (newLocale && newLocale !== currentLocaleCode.value) {
    setLocale(newLocale as SupportedLanguage)
  }
}
</script>
