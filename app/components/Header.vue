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
        <!-- Badge de statut Ollama -->
        <OllamaStatusBadge />
      </div>
    </template>
  </UHeader>
</template>

<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";
import LanguageSwitcher from "~/components/common/LanguageSwitcher.vue";
import { OllamaStatusBadge } from "~/components/settings";
import { useAppLocale } from "~/composables/useLocale";

const route = useRoute();
const { tmReactive } = useAppLocale();

// Navigation items
const navigationItems = computed<NavigationMenuItem[]>(() => [
  {
    label: tmReactive("nav", "home").value,
    to: "/",
    active: route.path === "/",
    icon: "i-heroicons-home",
  },
  {
    label: tmReactive("nav", "projects").value,
    to: "/projects",
    active: route.path.startsWith("/projects"),
    icon: "i-heroicons-folder",
  },
  {
    label: tmReactive("nav", "donation").value,
    to: "/donation",
    active: route.path === "/donation",
    icon: "i-heroicons-heart",
  },
  {
    label: tmReactive("nav", "settings").value,
    to: "/settings",
    active: route.path === "/settings",
    icon: "i-heroicons-cog-6-tooth",
  },
]);
</script>
