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
import { useLocale } from '~/composables/useLocale'

const { locale, lang, dir } = useLocale()

useHead({
  htmlAttrs: {
    lang,
    dir
  }
})
</script>