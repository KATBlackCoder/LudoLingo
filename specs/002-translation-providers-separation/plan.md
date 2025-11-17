# Implementation Plan: Séparation des Providers de Traduction

**Branch**: `002-translation-providers-separation` | **Date**: 2025-01-XX | **Spec**: [specs/002-translation-providers-separation/spec.md](spec.md)
**Input**: Feature specification from `/specs/002-translation-providers-separation/spec.md`

## Summary

Migration de l'architecture de traduction pour séparer complètement Ollama (local) et RunPod (online). Cette migration nettoie le code existant et ajoute un nouveau module pour RunPod, permettant un choix clair entre traduction locale et en ligne.

**🎯 Objectif**: Séparation complète des providers avec architecture claire et maintenable.

## Technical Context

**Language/Version**: Rust 1.x (Tauri), TypeScript 5.x (Nuxt)
**Primary Dependencies**: 
- Backend: `ollama-rs` (existant), `reqwest` (nouveau)
- Frontend: Nuxt 3.x, Nuxt UI, Pinia
**Storage**: 
- Settings via `tauri-plugin-store` : Configuration des providers
**Testing**: Cargo test (backend uniquement) - TDD obligatoire
**Target Platform**: Desktop (Windows & Linux uniquement)
**Project Type**: Desktop application (Tauri + Nuxt)
**Constraints**: 
- Architecture offline-first maintenue
- Backward compatibility avec settings existants
- Pas de régression sur fonctionnalités existantes

## Constitution Check

*GATE: Must pass before implementation*

### I. Architecture Tauri + Nuxt
- [x] Séparation frontend/backend respectée
- [x] Commands Tauri pour toute interaction système
- [x] Stores Pinia pour état partagé frontend

### II. Séparation des Responsabilités
- [x] Ollama = local uniquement
- [x] RunPod = online uniquement
- [x] Pas de mélange des modes dans un même module

### III. Backward Compatibility
- [x] Migration automatique des settings existants
- [x] Pas de breaking changes pour l'utilisateur final

## Project Structure

### Documentation (this feature)

```text
specs/002-translation-providers-separation/
├── spec.md              # Feature specification
├── plan.md              # This file
└── tasks.md             # Detailed task breakdown
```

### Source Code Changes

```text
src-tauri/src/translation/
├── ollama/              # Local uniquement (nettoyé)
│   ├── client.rs        # OllamaClient local uniquement
│   ├── single.rs        # Traduction individuelle (local)
│   ├── sequential.rs    # Traduction séquentielle (local)
│   ├── common.rs        # Utilitaires partagés
│   └── mod.rs           # Exports
├── runpod/              # Online uniquement (nouveau)
│   ├── client.rs        # RunPodClient avec reqwest
│   ├── single.rs        # Traduction individuelle (online)
│   ├── sequential.rs    # Traduction séquentielle (online)
│   ├── common.rs        # Réutilise ollama::common
│   └── mod.rs           # Exports
├── service.rs           # Coordinateur (mis à jour)
└── mod.rs               # Exports mis à jour

src-tauri/src/commands/
└── translation.rs       # Commands mis à jour avec provider

app/composables/
└── useTauriSetting.ts   # Settings mis à jour

app/components/settings/
├── OllamaConfig.vue     # Configuration Ollama (nettoyé)
└── RunPodConfig.vue     # Configuration RunPod (nouveau)

app/stores/
├── ollama.ts            # Store Ollama (adapté)
└── runpod.ts            # Store RunPod (nouveau) ou intégré dans translation.ts
```

## Implementation Phases

### Phase 1: Backend - Nettoyage Ollama (Local uniquement)

**Objectif**: Nettoyer le module Ollama pour être 100% local.

**Fichiers modifiés**:
- `src-tauri/src/translation/ollama/client.rs`
- `src-tauri/src/translation/ollama/mod.rs`

**Tâches**:
1. Supprimer `OllamaMode::Online`
2. Simplifier `OllamaConfig` (supprimer `mode`, garder `endpoint` + `port`)
3. Nettoyer `check_ollama_status` pour local uniquement
4. Mettre à jour les exports

**Tests**:
- [ ] Test connexion Ollama local
- [ ] Test liste modèles Ollama local
- [ ] Test traduction avec Ollama local

### Phase 2: Backend - Création RunPod

**Objectif**: Créer le module RunPod pour gérer les connexions online.

**Fichiers créés**:
- `src-tauri/src/translation/runpod/mod.rs`
- `src-tauri/src/translation/runpod/client.rs`
- `src-tauri/src/translation/runpod/single.rs`
- `src-tauri/src/translation/runpod/sequential.rs`
- `src-tauri/src/translation/runpod/common.rs`

**Fichiers modifiés**:
- `src-tauri/Cargo.toml` (ajouter `reqwest`)

**Tâches**:
1. Créer `RunPodClient` avec `reqwest`
2. Implémenter construction automatique URL: `https://{pod_id}-11434.proxy.runpod.net`
3. Implémenter méthodes: `list_models()`, `generate()`, `chat()`
4. Créer `SingleTranslationManager` pour RunPod
5. Créer `SequentialTranslationManager` pour RunPod
6. Réutiliser `common.rs` depuis `ollama`

**Tests**:
- [ ] Test connexion RunPod
- [ ] Test liste modèles RunPod
- [ ] Test traduction avec RunPod

### Phase 3: Backend - Coordination

**Objectif**: Créer un système de routing pour utiliser le bon provider.

**Fichiers modifiés**:
- `src-tauri/src/translation/service.rs`
- `src-tauri/src/translation/mod.rs`
- `src-tauri/src/commands/translation.rs`

**Tâches**:
1. Mettre à jour `TranslationService` pour router selon provider
2. Ajouter paramètre `provider` aux commands
3. Créer managers globaux pour chaque provider
4. Router les requêtes vers le bon manager

**Tests**:
- [ ] Test routing Ollama
- [ ] Test routing RunPod
- [ ] Test changement de provider

### Phase 4: Frontend - Settings

**Objectif**: Mettre à jour les settings pour gérer les deux providers.

**Fichiers modifiés**:
- `app/composables/useTauriSetting.ts`
- `app/pages/settings.vue`
- `app/components/settings/OllamaConfig.vue`

**Fichiers créés**:
- `app/components/settings/RunPodConfig.vue`

**Tâches**:
1. Mettre à jour `AppSettings` avec nouvelle structure (`podId` au lieu de `endpoint` pour RunPod)
2. Créer composant `RunPodConfig.vue` avec champ POD_ID uniquement
3. Mettre à jour `settings.vue` pour sélecteur de provider
4. Migration automatique des settings existants (extraire POD_ID depuis URL si présente)

**Tests**:
- [ ] Test sauvegarde settings Ollama
- [ ] Test sauvegarde settings RunPod
- [ ] Test migration settings existants
- [ ] Test restauration settings

### Phase 5: Frontend - Stores et Composants

**Objectif**: Mettre à jour les stores et composants pour utiliser le bon provider.

**Fichiers modifiés**:
- `app/stores/ollama.ts`
- `app/stores/translation.ts`
- Composants de traduction

**Fichiers créés**:
- `app/stores/runpod.ts` (ou intégré dans `translation.ts`)

**Tâches**:
1. Adapter `ollama.ts` pour local uniquement
2. Créer store RunPod
3. Mettre à jour `translation.ts` pour gérer provider actif
4. Mettre à jour composants pour utiliser le bon provider

**Tests**:
- [ ] Test traduction avec Ollama depuis UI
- [ ] Test traduction avec RunPod depuis UI
- [ ] Test switch provider depuis UI

### Phase 6: Tests et Validation

**Objectif**: Valider que tout fonctionne correctement.

**Tâches**:
1. Tests unitaires backend
2. Tests d'intégration backend
3. Tests e2e frontend
4. Validation backward compatibility
5. Documentation mise à jour

**Tests**:
- [ ] Tous les tests passent
- [ ] Pas de régression
- [ ] Documentation complète

## Migration Checklist

### Backend
- [ ] Phase 1: Nettoyage Ollama
- [ ] Phase 2: Création RunPod
- [ ] Phase 3: Coordination
- [ ] Tests backend complets

### Frontend
- [ ] Phase 4: Settings
- [ ] Phase 5: Stores et Composants
- [ ] Tests frontend complets

### Validation
- [ ] Phase 6: Tests et Validation
- [ ] Documentation mise à jour
- [ ] Backward compatibility vérifiée

## Dependencies

### Backend (Cargo.toml)

```toml
[dependencies]
ollama-rs = "0.3.2"  # Existant
reqwest = { version = "0.11", features = ["json"] }  # Nouveau
```

### Frontend

Aucune nouvelle dépendance nécessaire (utilise déjà Nuxt UI et Pinia).

## Risk Assessment

### Risques identifiés

1. **Breaking changes**: Migration des settings existants
   - **Mitigation**: Migration automatique au chargement
   
2. **Complexité RunPod**: Implémentation HTTP manuelle
   - **Mitigation**: Réutiliser la logique Ollama, adapter pour HTTP
   
3. **Tests**: Couverture complète des deux providers
   - **Mitigation**: Tests unitaires et d'intégration pour chaque provider

### Contingency Plan

Si la migration pose problème:
- Garder l'ancien code en commentaire
- Rollback possible via git
- Feature flag pour activer/désactiver RunPod

## Success Metrics

- [ ] Ollama fonctionne uniquement en local
- [ ] RunPod fonctionne uniquement en online
- [ ] Frontend permet choix clair entre providers
- [ ] Pas de régression fonctionnelle
- [ ] Tests passent à 100%
- [ ] Documentation complète

