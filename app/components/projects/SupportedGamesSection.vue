<template>
  <UCard class="hover:shadow-lg transition-shadow">
    <template #header>
      <div class="flex items-center gap-3">
        <UIcon name="i-lucide-gamepad-2" class="h-6 w-6 text-primary" />
        <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
          {{ tmReactive('games', 'supported').value }}
        </h3>
      </div>
    </template>

    <div class="space-y-3">
      <div class="flex items-center gap-3 p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
        <div class="w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center">
          <span class="text-white text-sm font-bold">RM</span>
        </div>
        <div>
          <p class="font-medium text-gray-900 dark:text-white">{{ tmReactive('games', 'rpgMaker').value }}</p>
          <p class="text-sm text-gray-500 dark:text-gray-400">MV/MZ</p>
        </div>
      </div>

      <div class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg opacity-50">
        <div class="w-8 h-8 bg-gray-400 rounded-full flex items-center justify-center">
          <span class="text-white text-sm font-bold">W</span>
        </div>
        <div>
          <p class="font-medium text-gray-900 dark:text-white">{{ tmReactive('games', 'wolfRPG').value }}</p>
          <p class="text-sm text-gray-500 dark:text-gray-400">{{ tmReactive('games', 'comingSoon').value }}</p>
        </div>
      </div>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { useLocale } from '~/composables/useLocale'

const { tmReactive } = useLocale()
</script>
