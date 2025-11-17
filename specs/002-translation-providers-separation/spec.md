# Feature Specification: Séparation des Providers de Traduction

**Feature Branch**: `002-translation-providers-separation`
**Created**: 2025-01-XX
**Status**: Draft
**Input**: Migration de l'architecture de traduction pour séparer complètement Ollama (local) et RunPod (online)

**🎯 Objectif**: Séparer complètement les providers de traduction pour permettre un choix clair entre traduction locale (Ollama) et traduction en ligne (RunPod).

## Contexte

Actuellement, le module `translation/ollama` gère à la fois les modes local et online, ce qui crée de la confusion et de la complexité. Cette migration vise à :

1. **Nettoyer `translation/ollama`** pour être 100% local uniquement avec `ollama-rs`
2. **Créer `translation/runpod`** pour gérer uniquement les connexions RunPod avec `reqwest`
3. **Mettre à jour le frontend** pour permettre un choix clair entre les deux providers

## User Scenarios & Testing

### User Story 1 - Configuration Provider Local (Ollama) (Priority: P1)

Utilisateur configure Ollama en mode local pour traduire sans connexion internet.

**Why this priority**: C'est le mode par défaut et le plus utilisé pour la confidentialité des données.

**Independent Test**: Peut être testé en configurant Ollama local, vérifiant la connexion, et lançant une traduction.

**Acceptance Scenarios**:

1. **Given** Ollama installé localement, **When** l'utilisateur configure le provider "Ollama Local", **Then** la connexion est testée et les modèles disponibles sont listés
2. **Given** Ollama non démarré, **When** l'utilisateur teste la connexion, **Then** un message d'erreur clair est affiché
3. **Given** une configuration Ollama valide, **When** l'utilisateur lance une traduction, **Then** la traduction utilise uniquement le client local

---

### User Story 2 - Configuration Provider Online (RunPod) (Priority: P1)

Utilisateur configure RunPod pour traduire via un service cloud.

**Why this priority**: Permet d'utiliser des modèles plus puissants sans installation locale.

**Independent Test**: Peut être testé en configurant RunPod avec une URL valide, vérifiant la connexion, et lançant une traduction.

**Acceptance Scenarios**:

1. **Given** un POD_ID RunPod valide, **When** l'utilisateur configure le provider "RunPod", **Then** la connexion est testée et les modèles disponibles sont listés
2. **Given** un POD_ID RunPod invalide, **When** l'utilisateur teste la connexion, **Then** un message d'erreur clair est affiché
3. **Given** une configuration RunPod valide (POD_ID), **When** l'utilisateur lance une traduction, **Then** la traduction utilise uniquement le client HTTP avec l'URL construite automatiquement

---

### User Story 3 - Switch entre Providers (Priority: P2)

Utilisateur peut basculer entre Ollama local et RunPod online selon ses besoins.

**Why this priority**: Flexibilité pour utiliser le meilleur provider selon le contexte.

**Independent Test**: Peut être testé en basculant entre les deux providers et vérifiant que les traductions utilisent le bon provider.

**Acceptance Scenarios**:

1. **Given** deux providers configurés, **When** l'utilisateur bascule de Ollama à RunPod, **Then** les nouvelles traductions utilisent RunPod
2. **Given** un provider actif, **When** l'utilisateur change de provider, **Then** les sessions en cours continuent avec l'ancien provider
3. **Given** un changement de provider, **When** l'utilisateur sauvegarde, **Then** le choix est persisté et restauré au prochain démarrage

---

## Technical Requirements

### Backend Architecture

#### Module `translation/ollama/` (Local uniquement)

- **`client.rs`**: Client Ollama local avec `ollama-rs` uniquement
  - Supprimer toute logique "online"
  - `OllamaMode::Local` uniquement
  - Configuration simplifiée: `endpoint` (localhost) + `port` (11434)
  
- **`single.rs`**: Traduction individuelle (local)
  - Utilise `ollama-rs` pour les appels Chat API
  - Pas de changement majeur, juste nettoyage
  
- **`sequential.rs`**: Traduction séquentielle (local)
  - Utilise `SingleTranslationManager` avec client Ollama local
  - Pas de changement majeur, juste nettoyage
  
- **`common.rs`**: Utilitaires partagés
  - Prompts, parsing, validation
  - Réutilisable par RunPod

#### Module `translation/runpod/` (Nouveau, Online uniquement)

- **`client.rs`**: Client HTTP avec `reqwest` pour l'API Ollama RunPod
  - Implémente les mêmes méthodes que `OllamaClient` mais avec HTTP
  - Endpoints: `/api/tags`, `/api/generate`, `/api/chat`
  - Configuration: `pod_id` (POD_ID uniquement)
  - URL construite automatiquement: `https://{pod_id}-11434.proxy.runpod.net`
  
- **`single.rs`**: Traduction individuelle (online)
  - Copie de `ollama/single.rs` adaptée pour `RunPodClient`
  - Réutilise `common.rs` pour prompts/parsing
  
- **`sequential.rs`**: Traduction séquentielle (online)
  - Copie de `ollama/sequential.rs` adaptée pour `RunPodClient`
  - Réutilise `common.rs` pour prompts/parsing
  
- **`common.rs`**: Utilitaires (réutilise ceux de `ollama/common.rs`)
  - Import depuis `ollama::common` pour éviter duplication
  
- **`mod.rs`**: Exports du module

#### Module `translation/service.rs` (Coordinateur)

- Détecte le provider actif (local/online)
- Route les requêtes vers le bon manager
- Interface unifiée pour les deux providers

#### Commands `commands/translation.rs`

- Ajouter paramètre `provider: 'ollama' | 'runpod'` aux commands
- Router vers le bon manager selon le provider
- Utiliser les settings pour déterminer le provider par défaut

### Frontend Architecture

#### Settings Store (`app/composables/useTauriSetting.ts`)

```typescript
export interface AppSettings {
  translation: {
    provider: 'ollama' | 'runpod'
    ollama: {
      endpoint: string
      port: number
      model: string
    }
    runpod: {
      podId: string  // POD_ID uniquement (ex: "xedezhzb9la3ye")
      model: string
    }
    sourceLanguage: string
    targetLanguage: string
  }
}
```

#### Components

- **`app/components/settings/OllamaConfig.vue`**: Configuration Ollama local uniquement
- **`app/components/settings/RunPodConfig.vue`**: Nouveau composant pour RunPod
- **`app/pages/settings.vue`**: Afficher le bon composant selon le provider sélectionné

#### Stores

- **`app/stores/ollama.ts`**: Renommer ou adapter pour gérer uniquement Ollama local
- **`app/stores/runpod.ts`**: Nouveau store pour RunPod (ou intégrer dans `translation.ts`)
- **`app/stores/translation.ts`**: Gérer le provider actif et router vers le bon store

### Dependencies

**`Cargo.toml`**
```toml
[dependencies]
ollama-rs = "0.3.2"  # Déjà présent
reqwest = { version = "0.11", features = ["json"] }  # Nouveau pour RunPod
```

## Implementation Plan

### Phase 1: Backend - Nettoyage Ollama (Local uniquement)

1. Supprimer `OllamaMode::Online` de `ollama/client.rs`
2. Simplifier `OllamaConfig` pour local uniquement
3. Nettoyer `check_ollama_status` pour local uniquement
4. Tester que tout fonctionne en local

### Phase 2: Backend - Création RunPod

1. Créer `translation/runpod/mod.rs`
2. Créer `translation/runpod/client.rs` avec `reqwest`
3. Créer `translation/runpod/single.rs` (copie de `ollama/single.rs` adaptée)
4. Créer `translation/runpod/sequential.rs` (copie de `ollama/sequential.rs` adaptée)
5. Créer `translation/runpod/common.rs` (réutilise les prompts)
6. Ajouter `reqwest` à `Cargo.toml`

### Phase 3: Backend - Coordination

1. Mettre à jour `translation/service.rs` pour router selon le provider
2. Mettre à jour `commands/translation.rs` pour accepter `provider`
3. Créer des commands séparés ou un paramètre de routing

### Phase 4: Frontend

1. Mettre à jour `useTauriSetting.ts` avec la nouvelle structure
2. Créer `RunPodConfig.vue`
3. Mettre à jour `settings.vue` pour le sélecteur de provider
4. Mettre à jour les stores pour gérer les deux providers
5. Mettre à jour les composants de traduction pour utiliser le bon provider

### Phase 5: Tests

1. Tester Ollama local
2. Tester RunPod online
3. Tester le switch entre providers
4. Vérifier la persistance des settings

## Migration Strategy

### Backward Compatibility

- Les settings existants avec `mode: 'local'` seront migrés vers `provider: 'ollama'`
- Les settings existants avec `mode: 'online'` seront migrés vers `provider: 'runpod'`
- Migration automatique au chargement des settings

### Breaking Changes

- `OllamaMode::Online` supprimé (remplacé par `RunPodClient`)
- `OllamaConfig.mode` supprimé (remplacé par `provider` dans settings)
- Commands de traduction nécessitent maintenant un `provider` explicite

## Success Criteria

- [ ] Ollama fonctionne uniquement en local avec `ollama-rs`
- [ ] RunPod fonctionne uniquement en online avec `reqwest`
- [ ] Le frontend permet de choisir entre les deux providers
- [ ] Les traductions utilisent le bon provider selon la configuration
- [ ] Les settings sont persistés et restaurés correctement
- [ ] Pas de régression sur les fonctionnalités existantes

