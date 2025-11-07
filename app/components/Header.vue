<template>
  <UHeader>
    <template #title>
      <NuxtLink to="/" class="flex items-center gap-2">
        <UIcon name="i-lucide-gamepad-2" class="h-6 w-6" />
        <span class="font-bold text-xl">LudoLingo</span>
      </NuxtLink>
    </template>

    <UNavigationMenu :items="navigationItems" />

    <template #right>
      <div class="flex items-center gap-2">
        <LanguageSwitcher />
        <UColorModeButton />
      </div>
    </template>
  </UHeader>
</template>

<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'
import LanguageSwitcher from '~/components/common/LanguageSwitcher.vue'
import { useMessages } from '~/composables/useMessages'

const route = useRoute()
const { tm } = useMessages()

// Navigation items
const navigationItems = computed<NavigationMenuItem[]>(() => [
  {
    label: tm('nav', 'home'),
    to: '/',
    active: route.path === '/',
    icon: 'i-heroicons-home'
  },
  {
    label: tm('nav', 'projects'),
    to: '/projects',
    active: route.path.startsWith('/projects'),
    icon: 'i-heroicons-folder'
  },
  {
    label: tm('nav', 'donation'),
    to: '/donation',
    active: route.path === '/donation',
    icon: 'i-heroicons-heart'
  },
  {
    label: tm('nav', 'settings'),
    to: '/settings',
    active: route.path === '/settings',
    icon: 'i-heroicons-cog-6-tooth'
  }
])
</script>
