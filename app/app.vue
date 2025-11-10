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
import { useAppLocale } from '~/composables/useLocale'

const { locale, lang, dir } = useAppLocale()

useHead({
  htmlAttrs: {
    lang,
    dir
  }
})
</script>