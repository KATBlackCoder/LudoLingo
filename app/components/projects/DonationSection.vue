<template>
  <UCard class="mt-12 bg-gradient-to-r from-purple-500 to-pink-500 text-white">
    <div class="text-center">
      <UIcon name="i-heroicons-heart" class="h-12 w-12 mx-auto mb-4 opacity-80" />
      <h3 class="text-2xl font-bold mb-2">
        {{ tm('donations', 'title') }}
      </h3>
      <p class="mb-6 opacity-90">
        {{ tm('donations', 'description') }}
      </p>
      <NuxtLink to="/donation">
        <UButton
          color="neutral"
          variant="solid"
          icon="i-heroicons-heart"
        >
          {{ tm('donations', 'donate') }}
        </UButton>
      </NuxtLink>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { useMessages } from '~/composables/useMessages'

const { tm } = useMessages()
</script>
