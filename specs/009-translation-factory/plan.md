# Plan d'Implémentation : Factory Pattern pour Providers de Traduction

**Branch**: `009-translation-factory` | **Date**: 2025-01-XX | **Spec**: [specs/009-translation-factory/spec.md](spec.md)
**Input**: Feature specification from `/specs/009-translation-factory/spec.md`

## Summary

Refactorisation de l'architecture de traduction pour utiliser un pattern factory similaire à celui utilisé dans `parsers`. Cette refactorisation découple `commands/translation.rs` des implémentations spécifiques des providers (Ollama, RunPod) et permet une extension facile vers de nouveaux providers.

**🎯 Objectif**: Créer un système factory avec un trait `TranslationProvider` pour abstraire les providers de traduction.

## Technical Context

**Language/Version**: Rust 1.x (Tauri), TypeScript 5.x (Nuxt)
**Primary Dependencies**: 
- Backend: `ollama-rs` (existant), `reqwest` (existant)
- Frontend: Nuxt 4.x, Nuxt UI, Pinia
**Storage**: 
- Sessions en mémoire (comportement actuel préservé)
**Testing**: Cargo test (backend uniquement) - TDD obligatoire
**Target Platform**: Desktop (Windows & Linux uniquement)
**Project Type**: Desktop application (Tauri + Nuxt)
**Constraints**: 
- Architecture offline-first maintenue
- Backward compatibility avec APIs existantes
- Pas de régression sur fonctionnalités existantes
- Préservation du cache RunPod par pod_id

## Constitution Check

*GATE: Must pass before implementation*

### I. Architecture Tauri + Nuxt
- [x] Séparation frontend/backend respectée
- [x] Commands Tauri pour toute interaction système
- [x] Stores Pinia pour état partagé frontend

### II. Séparation des Responsabilités
- [x] Factory dans module `translation/` uniquement
- [x] Providers encapsulent leur logique spécifique
- [x] Commands délèguent au factory

### III. Backward Compatibility
- [x] APIs publiques des commands inchangées
- [x] Types de données identiques
- [x] Aucun changement frontend requis

## Project Structure

### Documentation (this feature)

```text
specs/009-translation-factory/
├── spec.md              # Feature specification
├── plan.md              # This file
└── tasks.md             # Detailed task breakdown
```

### Source Code Changes

```text
src-tauri/src/translation/
├── mod.rs               # Exports mis à jour
├── provider.rs           # NOUVEAU - Trait TranslationProvider
├── factory.rs            # NOUVEAU - TranslationFactory
├── ollama/
│   ├── mod.rs
│   ├── provider.rs       # NOUVEAU - OllamaProvider impl TranslationProvider
│   ├── client.rs         # Existant (inchangé)
│   ├── single.rs         # Existant (inchangé)
│   └── sequential.rs     # Existant (inchangé)
└── runpod/
    ├── mod.rs
    ├── provider.rs       # NOUVEAU - RunPodProvider impl TranslationProvider
    ├── client.rs         # Existant (inchangé)
    ├── single.rs         # Existant (inchangé)
    └── sequential.rs     # Existant (inchangé)

src-tauri/src/commands/
└── translation.rs        # Refactorisé pour utiliser factory
```

## Implementation Phases

### Phase 1: Création Trait TranslationProvider

**Objectif**: Définir l'interface commune pour tous les providers.

**Fichiers créés**:
- `src-tauri/src/translation/provider.rs`

**Tâches**:
1. Définir le trait `TranslationProvider` avec toutes les méthodes nécessaires
2. Définir les types communs (`ProviderConfig`, types de requêtes/résultats)
3. Documenter chaque méthode du trait
4. Ajouter les derives nécessaires (`Send + Sync`)

**Tests**:
- [ ] Test compilation du trait
- [ ] Test que le trait peut être utilisé comme type de retour

### Phase 2: Création OllamaProvider

**Objectif**: Implémenter le provider Ollama qui encapsule **toute** la logique existante de manière **indépendante**.

**Fichiers créés**:
- `src-tauri/src/translation/ollama/provider.rs`

**Fichiers modifiés**:
- `src-tauri/src/translation/ollama/mod.rs` (exports)

**Tâches**:
1. Créer `OllamaProvider` struct qui **encapsule complètement** :
   - Création interne de `OllamaClient` (pas de dépendance externe)
   - Création interne de `OllamaSingleManager` et `OllamaSequentialManager`
   - Gestion interne de la configuration
2. **Supprimer** les managers globaux de `commands/translation.rs` (ils seront dans le provider)
3. Implémenter `TranslationProvider` pour `OllamaProvider`
4. Chaque instance de `OllamaProvider` gère ses propres managers (pas de singleton global)
5. Exporter `OllamaProvider` dans `mod.rs`

**Principe d'indépendance**:
- ✅ Le provider peut être créé sans contexte externe
- ✅ Aucune dépendance vers `commands/translation.rs`
- ✅ Toute la logique est encapsulée dans le provider
- ✅ Similaire à `RpgMakerHandler::new()` qui est complètement indépendant

**Tests**:
- [ ] Test création `OllamaProvider` sans contexte externe
- [ ] Test que plusieurs instances peuvent coexister
- [ ] Test `check_status()` avec Ollama
- [ ] Test `translate_single_text()` avec Ollama
- [ ] Test `start_sequential_translation()` avec Ollama

### Phase 3: Création RunPodProvider

**Objectif**: Implémenter le provider RunPod qui encapsule **toute** la logique existante de manière **indépendante**.

**Fichiers créés**:
- `src-tauri/src/translation/runpod/provider.rs`

**Fichiers modifiés**:
- `src-tauri/src/translation/runpod/mod.rs` (exports)

**Tâches**:
1. Créer `RunPodProvider` struct qui **encapsule complètement** :
   - Cache interne des managers par pod_id (déplacé depuis `commands/translation.rs`)
   - Création interne de `RunPodClient`, `RunPodSingleManager`, `RunPodSequentialManager`
   - Gestion interne de la configuration
2. **Supprimer** `RUNPOD_MANAGERS_CACHE` de `commands/translation.rs` (il sera dans le provider)
3. **Supprimer** `get_runpod_managers()` de `commands/translation.rs` (logique dans le provider)
4. Implémenter `TranslationProvider` pour `RunPodProvider`
5. Le cache est géré **interne au provider** (méthode privée `get_managers()`)
6. Exporter `RunPodProvider` dans `mod.rs`

**Principe d'indépendance**:
- ✅ Le provider peut être créé avec juste `pod_id` et `config`
- ✅ Aucune dépendance vers `commands/translation.rs`
- ✅ Le cache est géré **dans le provider**, pas globalement
- ✅ Similaire à `WolfRpgHandler::new()` qui est complètement indépendant

**Note sur le cache**:
- Le cache RunPod par pod_id est **préservé** mais **déplacé dans le provider**
- Chaque instance de `RunPodProvider` gère son propre cache interne
- Pour partager le cache entre plusieurs appels, utiliser la même instance de provider

**Tests**:
- [ ] Test création `RunPodProvider` avec pod_id sans contexte externe
- [ ] Test cache interne fonctionne correctement
- [ ] Test `check_status()` avec RunPod
- [ ] Test `translate_single_text()` avec RunPod
- [ ] Test `start_sequential_translation()` avec RunPod
- [ ] Test cache partagé pour même pod_id dans la même instance

### Phase 4: Création TranslationFactory

**Objectif**: Créer la factory qui instancie les providers appropriés.

**Fichiers créés**:
- `src-tauri/src/translation/factory.rs`

**Fichiers modifiés**:
- `src-tauri/src/translation/mod.rs` (exports)

**Tâches**:
1. Créer `TranslationFactory` struct
2. Implémenter `create_provider()` qui route selon le type
3. Gérer la validation de configuration (pod_id requis pour RunPod)
4. Retourner des erreurs claires pour configurations invalides
5. Exporter `TranslationFactory` dans `mod.rs`

**Tests**:
- [ ] Test création provider "ollama"
- [ ] Test création provider "runpod" avec pod_id
- [ ] Test création provider "runpod" sans pod_id (erreur attendue)
- [ ] Test création provider inconnu (erreur attendue)

### Phase 5: Refactorisation commands/translation.rs

**Objectif**: Refactoriser les commands pour utiliser la factory au lieu de la logique directe.

**Fichiers modifiés**:
- `src-tauri/src/commands/translation.rs`

**Tâches**:
1. **Supprimer complètement** :
   - `OLLAMA_SEQUENTIAL_MANAGER` (Lazy static) → maintenant dans `OllamaProvider`
   - `OLLAMA_SINGLE_MANAGER` (Lazy static) → maintenant dans `OllamaProvider`
   - `RUNPOD_MANAGERS_CACHE` (Lazy static) → maintenant dans `RunPodProvider`
   - `get_runpod_managers()` → logique dans `RunPodProvider`
   - Fonctions de conversion si plus nécessaires
2. Refactoriser chaque command pour :
   - Créer le provider via `TranslationFactory::create_provider()`
   - Appeler les méthodes du trait `TranslationProvider`
   - Préserver les signatures des fonctions publiques
3. Les commands ne connaissent plus les détails internes des providers
4. Les commands délèguent entièrement au provider via le trait

**Principe**:
- ✅ Les commands sont **découplées** des implémentations spécifiques
- ✅ Toute la logique spécifique est dans les providers
- ✅ Les commands utilisent uniquement le trait `TranslationProvider`

**Commands à refactoriser**:
- `check_ollama_status()` → utilise factory avec provider "ollama"
- `check_runpod_status()` → utilise factory avec provider "runpod"
- `start_sequential_translation()` → utilise factory selon provider
- `get_sequential_progress()` → utilise factory selon provider
- `pause_sequential_session()` → utilise factory selon provider
- `resume_sequential_session()` → utilise factory selon provider
- `stop_sequential_session()` → utilise factory selon provider
- `get_translation_suggestions()` → utilise factory selon provider
- `translate_single_text()` → utilise factory selon provider

**Tests**:
- [ ] Test que toutes les commands fonctionnent avec Ollama
- [ ] Test que toutes les commands fonctionnent avec RunPod
- [ ] Test que les erreurs sont gérées correctement
- [ ] Test que les APIs publiques sont inchangées

### Phase 6: Nettoyage et Tests Finaux

**Objectif**: Nettoyer le code et valider que tout fonctionne.

**Tâches**:
1. Supprimer le code mort (managers globaux non utilisés)
2. Vérifier que tous les imports sont corrects
3. Vérifier que tous les exports sont corrects
4. Exécuter tous les tests existants
5. Ajouter des tests pour le factory
6. Documenter les nouveaux modules

**Tests**:
- [ ] Tous les tests existants passent
- [ ] Tests du factory passent
- [ ] Tests des providers passent
- [ ] Pas de régression fonctionnelle

## Migration Checklist

### Backend
- [ ] Phase 1: Trait TranslationProvider
- [ ] Phase 2: OllamaProvider
- [ ] Phase 3: RunPodProvider
- [ ] Phase 4: TranslationFactory
- [ ] Phase 5: Refactorisation commands
- [ ] Phase 6: Nettoyage et Tests

### Validation
- [ ] Tous les tests passent
- [ ] Documentation complète
- [ ] Backward compatibility vérifiée
- [ ] Pas de régression fonctionnelle

## Dependencies

### Backend (Cargo.toml)

Aucune nouvelle dépendance nécessaire (utilise déjà `ollama-rs` et `reqwest`).

### Frontend

Aucune modification nécessaire (APIs inchangées).

## Risk Assessment

### Risques identifiés

1. **Breaking changes**: Modification de la structure interne
   - **Mitigation**: APIs publiques inchangées, tests de régression complets
   
2. **Gestion cache RunPod**: Préservation du comportement existant
   - **Mitigation**: Tests spécifiques pour le cache, préservation de la logique existante
   
3. **Performance**: Overhead de la factory
   - **Mitigation**: Factory simple, pas d'allocation inutile

### Contingency Plan

Si la refactorisation pose problème:
- Garder l'ancien code en commentaire temporairement
- Rollback possible via git
- Tests de régression pour détecter les problèmes rapidement

## Success Metrics

- [ ] Factory fonctionne pour Ollama et RunPod
- [ ] Commands utilisent le factory
- [ ] Pas de régression fonctionnelle
- [ ] Tests passent à 100%
- [ ] Documentation complète
- [ ] Code plus maintenable et extensible

