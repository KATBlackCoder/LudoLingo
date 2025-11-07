<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <div class="container mx-auto px-4 py-8">
      <!-- Header avec sélecteur de langue -->
      <div class="flex justify-between items-center mb-8">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
          LudoLingo
        </h1>
        <div class="w-48">
          <LanguageSwitcher />
        </div>
      </div>

      <!-- Section principale -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <!-- Projets -->
        <UCard class="hover:shadow-lg transition-shadow">
          <template #header>
            <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
              {{ tm('projects', 'title') }}
            </h3>
          </template>

          <div class="space-y-4">
            <p class="text-gray-600 dark:text-gray-300">
              {{ tm('projects', 'emptyDescription') }}
            </p>

            <div class="flex gap-2">
              <UButton
                color="primary"
                variant="solid"
                icon="i-heroicons-plus"
              >
                {{ tm('projects', 'create') }}
              </UButton>

              <UButton
                color="neutral"
                variant="outline"
                icon="i-heroicons-magnifying-glass"
              >
                {{ tm('projects', 'scan') }}
              </UButton>
            </div>
          </div>
        </UCard>

        <!-- Jeux supportés -->
        <UCard class="hover:shadow-lg transition-shadow">
          <template #header>
            <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
              {{ tm('games', 'supported') }}
            </h3>
          </template>

          <div class="space-y-3">
            <div class="flex items-center gap-3 p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
              <div class="w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center">
                <span class="text-white text-sm font-bold">RM</span>
              </div>
              <div>
                <p class="font-medium text-gray-900 dark:text-white">{{ tm('games', 'rpgMaker') }}</p>
                <p class="text-sm text-gray-500 dark:text-gray-400">MV/MZ</p>
              </div>
            </div>

            <div class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg opacity-50">
              <div class="w-8 h-8 bg-gray-400 rounded-full flex items-center justify-center">
                <span class="text-white text-sm font-bold">W</span>
              </div>
              <div>
                <p class="font-medium text-gray-900 dark:text-white">{{ tm('games', 'wolfRPG') }}</p>
                <p class="text-sm text-gray-500 dark:text-gray-400">{{ tm('games', 'comingSoon') }}</p>
              </div>
            </div>
          </div>
        </UCard>

        <!-- Paramètres -->
        <UCard class="hover:shadow-lg transition-shadow">
          <template #header>
            <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
              {{ tm('settings', 'title') }}
            </h3>
          </template>

          <div class="space-y-4">
            <div class="space-y-2">
              <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
                {{ tm('settings', 'ollama') }}
              </label>
              <div class="grid grid-cols-2 gap-2">
                <UButton
                  :variant="settingsStore.settings.ollama.mode === 'local' ? 'solid' : 'outline'"
                  color="primary"
                  size="sm"
                  @click="settingsStore.updateOllamaMode('local')"
                >
                  {{ tm('settings', 'local') }}
                </UButton>
                <UButton
                  :variant="settingsStore.settings.ollama.mode === 'online' ? 'solid' : 'outline'"
                  color="primary"
                  size="sm"
                  @click="settingsStore.updateOllamaMode('online')"
                >
                  {{ tm('settings', 'online') }}
                </UButton>
              </div>
            </div>

            <div v-if="settingsStore.settings.ollama.mode === 'local'" class="space-y-2">
              <UInput
                v-model="settingsStore.settings.ollama.endpoint"
                :placeholder="tm('settings', 'endpoint')"
                size="sm"
              />
              <UInput
                v-model.number="settingsStore.settings.ollama.port"
                :placeholder="tm('settings', 'port')"
                type="number"
                size="sm"
              />
            </div>

            <div v-else class="space-y-2">
              <UInput
                v-model="settingsStore.settings.ollama.endpoint"
                :placeholder="tm('settings', 'endpoint')"
                size="sm"
              />
            </div>
          </div>
        </UCard>
      </div>

      <!-- Section dons -->
      <UCard class="mt-8 bg-gradient-to-r from-purple-500 to-pink-500 text-white">
        <div class="text-center">
          <h3 class="text-2xl font-bold mb-2">
            {{ tm('donations', 'title') }}
          </h3>
          <p class="mb-4 opacity-90">
            {{ tm('donations', 'description') }}
          </p>
          <UButton
            color="neutral"
            variant="solid"
            icon="i-heroicons-heart"
            @click="handleDonate"
          >
            {{ tm('donations', 'donate') }}
          </UButton>
        </div>
      </UCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSettingsStore } from '~/stores/settings'
import { useMessages } from '~/composables/useMessages'
import LanguageSwitcher from '~/components/common/LanguageSwitcher.vue'

const settingsStore = useSettingsStore()
const { tm } = useMessages()

function handleDonate() {
  // TODO: Implement donation logic
  console.log('Opening donation page...')
}
</script>
