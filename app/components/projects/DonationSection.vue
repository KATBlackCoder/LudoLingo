<template>
  <UCard class="mt-12 bg-gradient-to-r from-purple-500 to-pink-500 text-white">
    <div class="text-center">
      <UIcon name="i-heroicons-heart" class="h-12 w-12 mx-auto mb-4 opacity-80" />
      <h3 class="text-2xl font-bold mb-2">
        {{ tmReactive('donations', 'title').value }}
      </h3>
      <p class="mb-6 opacity-90">
        {{ tmReactive('donations', 'description').value }}
      </p>
      <NuxtLink to="/donation">
        <UButton
          color="neutral"
          variant="solid"
          icon="i-heroicons-heart"
        >
          {{ tmReactive('donations', 'donate').value }}
        </UButton>
      </NuxtLink>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { useLocale } from '~/composables/useLocale'

const { tmReactive } = useLocale()
</script>
