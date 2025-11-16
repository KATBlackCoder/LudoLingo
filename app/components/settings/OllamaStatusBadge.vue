<template>
  <UButton
    :variant="isConnected ? 'solid' : 'outline'"
    :color="isConnected ? 'success' : 'error'"
    size="sm"
    leading-icon="i-simple-icons-ollama"
    @click="checkConnection"
    :loading="isCheckingConnection"
    :title="isCheckingConnection ? 'Vérification en cours...' : 'Vérifier la connexion Ollama'"
  >
    <span class="hidden sm:inline">
      {{ isConnected ? "Connecté" : "Déconnecté" }}
    </span>
    <span class="sm:hidden">
      {{ isConnected ? "On" : "Off" }}
    </span>
  </UButton>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useOllamaStore } from '~/stores/ollama'

const ollamaStore = useOllamaStore()
const { isConnected, isCheckingConnection } = storeToRefs(ollamaStore)

// Function to check Ollama connection
const checkConnection = async () => {
  await ollamaStore.checkConnection()
}
</script>
