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
        <!-- Badge de statut du provider de traduction -->
        <OllamaStatusBadge v-if="currentProvider === 'ollama'" />
        <RunPodStatusBadge v-if="currentProvider === 'runpod'" />
      </div>
    </template>
  </UHeader>
</template>

<script setup lang="ts">
import type { NavigationMenuItem } from "@nuxt/ui";
import LanguageSwitcher from "~/components/common/LanguageSwitcher.vue";
import { OllamaStatusBadge, RunPodStatusBadge } from "~/components/settings";
import { useAppLocale } from "~/composables/useLocale";
import { useSettings } from "~/composables/useTauriSetting";

const route = useRoute();
const { tmReactive } = useAppLocale();
const settings = useSettings();

// Get current provider from settings
const currentProvider = ref<'ollama' | 'runpod'>('ollama');

// Load provider on mount
onMounted(async () => {
  const userSettings = await settings.loadSettings();
  currentProvider.value = userSettings.provider;
});

// Refresh provider when navigating (user might have changed provider in settings)
watch(() => route.path, async () => {
  const userSettings = await settings.loadSettings();
  currentProvider.value = userSettings.provider;
});

// Navigation items
const navigationItems = computed<NavigationMenuItem[]>(() => [
  {
    label: tmReactive("nav", "home").value,
    to: "/",
    active: route.path === "/",
    icon: "i-heroicons-home",
  },
  {
    label: "Traduction",
    to: "/translation",
    active: route.path === "/translation",
    icon: "i-heroicons-language",
  },
  {
    label: "Glossaire",
    to: "/glossary",
    active: route.path === "/glossary",
    icon: "i-heroicons-book-open",
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
