# Composants Settings

## OllamaConfig.vue

Composant complet pour configurer Ollama et les langues de traduction.

### Utilisation

```vue
<template>
  <OllamaConfig
    :settings="settings"
    :available-models="availableModels"
    :loading-models="loadingModels"
    :testing-connection="testingConnection"
    :connection-status="connectionStatus"
    @update:mode="(value) => settings.ollama.mode = value"
    @update:endpoint="(value) => settings.ollama.endpoint = value"
    @update:port="(value) => settings.ollama.port = value"
    @update:model="(value) => settings.ollama.model = value"
    @update:sourceLanguage="(value) => settings.translation.sourceLanguage = value"
    @update:targetLanguage="(value) => settings.translation.targetLanguage = value"
    @refresh-models="handleRefreshModels"
    @test-connection="handleTestConnection"
  />
</template>

<script setup lang="ts">
import { OllamaConfig } from '~/components/settings'

const settings = ref({
  ollama: {
    mode: 'local' as 'local' | 'online',
    endpoint: 'http://localhost',
    port: 11434,
    model: 'llama3.2:3b'
  },
  translation: {
    sourceLanguage: 'ja',
    targetLanguage: 'fr'
  }
})

// Props additionnelles pour OllamaConfig
const availableModels = ref<string[]>([])
const loadingModels = ref(false)
const testingConnection = ref(false)
const connectionStatus = ref<{ success: boolean; message: string } | null>(null)
</script>
```

### Props

- `settings`: Configuration complète (Ollama + langues de traduction)
- `availableModels`: Liste des modèles disponibles
- `loadingModels`: Indicateur de chargement des modèles
- `testingConnection`: Indicateur de test de connexion
- `connectionStatus`: Statut de la connexion

### Events

#### Configuration Ollama
- `update:mode`: Changement du mode (local/online)
- `update:endpoint`: Changement de l'endpoint
- `update:port`: Changement du port
- `update:model`: Changement du modèle
- `refresh-models`: Actualisation de la liste des modèles
- `test-connection`: Test de connexion à Ollama

#### Configuration Langues
- `update:sourceLanguage`: Changement de la langue source
- `update:targetLanguage`: Changement de la langue cible

### Langues supportées

Chaque langue affiche son drapeau national :

- 🇫🇷 Français (fr)
- 🇬🇧 Anglais (en)
- 🇯🇵 Japonais (ja)
- 🇩🇪 Allemand (de)
- 🇪🇸 Espagnol (es)
- 🇮🇹 Italien (it)
- 🇵🇹 Portugais (pt)
- 🇷🇺 Russe (ru)
- 🇨🇳 Chinois (zh)
- 🇰🇷 Coréen (ko)
- 🇸🇦 Arabe (ar)
- 🇮🇳 Hindi (hi)

## TranslationLanguages.vue

Composant autonome pour configurer les langues source et cible de traduction.

### Intégration dans settings.vue

Le composant est intégré directement dans la page settings pour une configuration modulaire :

```vue
<TranslationLanguages
  :settings="settings"
  @sourceLanguage="(value) => settings.translation.sourceLanguage = value"
  @targetLanguage="(value) => settings.translation.targetLanguage = value"
/>
```

### Utilisation isolée

Pour une utilisation indépendante :

```vue
<template>
  <TranslationLanguages
    :settings="settings"
    @sourceLanguage="(value) => handleSourceLanguageChange(value)"
    @targetLanguage="(value) => handleTargetLanguageChange(value)"
  />
</template>

<script setup lang="ts">
import { TranslationLanguages } from '~/components/settings'

const settings = ref({
  translation: {
    sourceLanguage: 'ja',
    targetLanguage: 'fr'
  }
})

function handleSourceLanguageChange(value: string) {
  // Handle source language change
}

function handleTargetLanguageChange(value: string) {
  // Handle target language change
}
</script>
```
