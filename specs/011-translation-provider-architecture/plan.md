# Plan d'Implémentation : Alignement Architecture Translation avec Pattern Parsers

**Branch**: `011-translation-provider-architecture` | **Date**: 2025-01-XX | **Spec**: [specs/011-translation-provider-architecture/spec.md](spec.md)
**Input**: Feature specification from `/specs/011-translation-provider-architecture/spec.md`

## Summary

Refactorisation complète de l'architecture du module `translation/` pour suivre exactement le même pattern architectural que `parsers/`. Cette refactorisation garantit la cohérence architecturale, la maintenabilité et l'extensibilité en alignant complètement les deux modules.

**🎯 Objectif**: Aligner complètement `translation/` avec `parsers/` pour que chaque provider soit aussi indépendant que `RpgMakerHandler` ou `WolfRpgHandler`.

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
- **Alignement architectural complet avec `parsers/`**

## Constitution Check

*GATE: Must pass before implementation*

### I. Architecture Tauri + Nuxt
- [x] Séparation frontend/backend respectée
- [x] Commands Tauri pour toute interaction système
- [x] Stores Pinia pour état partagé frontend

### II. Alignement Architectural avec Parsers
- [x] Structure identique à `parsers/`
- [x] Trait commun suivant le même pattern que `GameEngineHandler`
- [x] Factory suivant le même pattern que `EngineFactory`
- [x] Providers aussi indépendants que `RpgMakerHandler`

### III. Backward Compatibility
- [x] APIs publiques des commands inchangées
- [x] Types de données identiques
- [x] Aucun changement frontend requis

## Project Structure

### Documentation (this feature)

```text
specs/011-translation-provider-architecture/
├── spec.md              # Feature specification
├── plan.md              # This file
└── tasks.md             # Detailed task breakdown
```

### Source Code Changes

```text
src-tauri/src/translation/
├── mod.rs               # Exports mis à jour
├── provider.rs          # NOUVEAU - Trait TranslationProvider (comme handler.rs)
├── factory.rs           # NOUVEAU - TranslationProviderFactory (comme factory.rs)
├── common/
│   └── types.rs         # Existant (inchangé)
├── ollama/
│   ├── mod.rs           # Exports mis à jour
│   ├── provider.rs      # NOUVEAU - OllamaProvider (comme handler.rs)
│   ├── engine.rs        # NOUVEAU - OllamaEngine (comme engine.rs)
│   ├── client.rs        # Existant (inchangé)
│   ├── single.rs        # Existant (inchangé)
│   └── sequential.rs    # Existant (inchangé)
└── runpod/
    ├── mod.rs           # Exports mis à jour
    ├── provider.rs      # NOUVEAU - RunPodProvider (comme handler.rs)
    ├── engine.rs        # NOUVEAU - RunPodEngine (comme engine.rs)
    ├── client.rs        # Existant (inchangé)
    ├── single.rs        # Existant (inchangé)
    └── sequential.rs    # Existant (inchangé)

src-tauri/src/commands/
└── translation.rs       # Refactorisé pour utiliser factory (comme commands de parsing)
```

## Comparaison avec Parsers (Référence)

### Structure Parsers (Modèle)

```
parsers/
├── handler.rs          # Trait GameEngineHandler
├── factory.rs          # EngineFactory
├── rpg_maker/
│   ├── handler.rs     # RpgMakerHandler impl GameEngineHandler
│   └── engine.rs      # RpgMakerEngine (logique métier)
└── wolfrpg/
    ├── handler.rs     # WolfRpgHandler impl GameEngineHandler
    └── engine.rs      # WolfRpgEngine (logique métier)
```

### Structure Translation (Cible)

```
translation/
├── provider.rs         # Trait TranslationProvider (identique à handler.rs)
├── factory.rs          # TranslationProviderFactory (identique à factory.rs)
├── ollama/
│   ├── provider.rs    # OllamaProvider (identique à handler.rs)
│   └── engine.rs      # OllamaEngine (identique à engine.rs)
└── runpod/
    ├── provider.rs    # RunPodProvider (identique à handler.rs)
    └── engine.rs      # RunPodEngine (identique à engine.rs)
```

## Implementation Phases

### Phase 1: Création Trait TranslationProvider (Aligné avec handler.rs)

**Objectif**: Créer le trait `TranslationProvider` en suivant exactement le même pattern que `GameEngineHandler` dans `parsers/handler.rs`.

**Référence**: `src-tauri/src/parsers/handler.rs`

**Fichiers créés**:
- `src-tauri/src/translation/provider.rs`

**Tâches**:
1. Examiner `parsers/handler.rs` pour comprendre le pattern exact
2. Créer `translation/provider.rs` avec la même structure :
   - Documentation complète comme `GameEngineHandler`
   - Méthodes documentées avec exemples
   - Trait `Send + Sync` comme `GameEngineHandler`
3. Définir les types communs (`ProviderConfig`, types de requêtes/résultats)
4. S'assurer que la structure est identique à `GameEngineHandler`

**Validation d'alignement**:
- [ ] Comparer ligne par ligne avec `parsers/handler.rs`
- [ ] Vérifier que la documentation suit le même format
- [ ] Vérifier que les méthodes suivent le même pattern

**Tests**:
- [ ] Test compilation du trait
- [ ] Test que le trait peut être utilisé comme type de retour (`Box<dyn TranslationProvider>`)
- [ ] Comparaison structurelle avec `GameEngineHandler`

### Phase 2: Création OllamaProvider (Aligné avec RpgMakerHandler)

**Objectif**: Créer `OllamaProvider` qui suit exactement le même pattern que `RpgMakerHandler` dans `parsers/rpg_maker/handler.rs`.

**Référence**: `src-tauri/src/parsers/rpg_maker/handler.rs` et `src-tauri/src/parsers/rpg_maker/engine.rs`

**Fichiers créés**:
- `src-tauri/src/translation/ollama/provider.rs`
- `src-tauri/src/translation/ollama/engine.rs` (optionnel, pour logique métier)

**Fichiers modifiés**:
- `src-tauri/src/translation/ollama/mod.rs` (exports)

**Tâches**:
1. Examiner `RpgMakerHandler` pour comprendre le pattern d'indépendance :
   - Création interne de tous les composants
   - Aucune dépendance externe
   - Encapsulation complète
2. Créer `OllamaProvider` struct qui **encapsule complètement** :
   - Création interne de `OllamaClient` (pas de dépendance externe)
   - Création interne de `OllamaSingleManager` et `OllamaSequentialManager`
   - Gestion interne de la configuration
   - **Aucune référence aux managers globaux**
3. Implémenter `TranslationProvider` pour `OllamaProvider`
4. **Supprimer** les managers globaux de `commands/translation.rs` (ils seront dans le provider)
5. Exporter `OllamaProvider` dans `mod.rs`

**Principe d'indépendance (identique à RpgMakerHandler)**:
- ✅ Le provider peut être créé sans contexte externe
- ✅ Aucune dépendance vers `commands/translation.rs`
- ✅ Toute la logique est encapsulée dans le provider
- ✅ Similaire à `RpgMakerHandler::new()` qui est complètement indépendant

**Validation d'alignement**:
- [ ] Comparer la structure avec `RpgMakerHandler`
- [ ] Vérifier que l'indépendance est identique
- [ ] Vérifier qu'aucun singleton global n'est utilisé

**Tests**:
- [ ] Test création `OllamaProvider` sans contexte externe
- [ ] Test que plusieurs instances peuvent coexister
- [ ] Test `check_status()` avec Ollama
- [ ] Test `translate_single_text()` avec Ollama
- [ ] Test `start_sequential_translation()` avec Ollama
- [ ] Test indépendance complète (pas de dépendances externes)

### Phase 3: Création RunPodProvider (Aligné avec WolfRpgHandler)

**Objectif**: Créer `RunPodProvider` qui suit exactement le même pattern que `WolfRpgHandler` dans `parsers/wolfrpg/handler.rs`.

**Référence**: `src-tauri/src/parsers/wolfrpg/handler.rs`

**Fichiers créés**:
- `src-tauri/src/translation/runpod/provider.rs`
- `src-tauri/src/translation/runpod/engine.rs` (optionnel, pour logique métier)

**Fichiers modifiés**:
- `src-tauri/src/translation/runpod/mod.rs` (exports)

**Tâches**:
1. Examiner `WolfRpgHandler` pour comprendre le pattern d'indépendance
2. Créer `RunPodProvider` struct qui **encapsule complètement** :
   - Cache interne des managers par pod_id (déplacé depuis `commands/translation.rs`)
   - Création interne de `RunPodClient`, `RunPodSingleManager`, `RunPodSequentialManager`
   - Gestion interne de la configuration
   - **Pas de dépendance vers `RUNPOD_MANAGERS_CACHE` global**
3. **Supprimer** `RUNPOD_MANAGERS_CACHE` de `commands/translation.rs` (il sera dans le provider)
4. **Supprimer** `get_runpod_managers()` de `commands/translation.rs` (logique dans le provider)
5. Implémenter `TranslationProvider` pour `RunPodProvider`
6. Le cache est géré **interne au provider** (méthode privée `get_managers()`)
7. Exporter `RunPodProvider` dans `mod.rs`

**Principe d'indépendance (identique à WolfRpgHandler)**:
- ✅ Le provider peut être créé avec juste `pod_id` et `config`
- ✅ Aucune dépendance vers `commands/translation.rs`
- ✅ Le cache est géré **dans le provider**, pas globalement
- ✅ Similaire à `WolfRpgHandler::new()` qui est complètement indépendant

**Note sur le cache**:
- Le cache RunPod par pod_id est **préservé** mais **déplacé dans le provider**
- Chaque instance de `RunPodProvider` gère son propre cache interne
- Pour partager le cache entre plusieurs appels, utiliser la même instance de provider

**Validation d'alignement**:
- [ ] Comparer la structure avec `WolfRpgHandler`
- [ ] Vérifier que l'indépendance est identique
- [ ] Vérifier que le cache est géré de manière similaire

**Tests**:
- [ ] Test création `RunPodProvider` avec pod_id sans contexte externe
- [ ] Test cache interne fonctionne correctement (pas de cache global)
- [ ] Test `check_status()` avec RunPod
- [ ] Test `translate_single_text()` avec RunPod
- [ ] Test `start_sequential_translation()` avec RunPod
- [ ] Test cache partagé pour même pod_id dans la même instance
- [ ] Test indépendance complète (pas de dépendances externes)

### Phase 4: Création TranslationProviderFactory (Aligné avec EngineFactory)

**Objectif**: Créer `TranslationProviderFactory` qui suit exactement le même pattern que `EngineFactory` dans `parsers/factory.rs`.

**Référence**: `src-tauri/src/parsers/factory.rs`

**Fichiers créés**:
- `src-tauri/src/translation/factory.rs`

**Fichiers modifiés**:
- `src-tauri/src/translation/mod.rs` (exports)

**Tâches**:
1. Examiner `EngineFactory` pour comprendre le pattern exact :
   - Structure de la factory
   - Pattern de création
   - Gestion d'erreurs
   - Documentation
2. Créer `TranslationProviderFactory` struct (unit struct comme `EngineFactory`)
3. Implémenter `create_provider()` qui route selon le type :
   - Même structure que `EngineFactory::create_handler()`
   - Même pattern de gestion d'erreurs
   - Même documentation
4. Gérer la validation de configuration (pod_id requis pour RunPod)
5. Retourner des erreurs claires pour configurations invalides (comme `EngineFactory`)
6. Exporter `TranslationProviderFactory` dans `mod.rs`

**Validation d'alignement**:
- [ ] Comparer ligne par ligne avec `parsers/factory.rs`
- [ ] Vérifier que la structure est identique
- [ ] Vérifier que la gestion d'erreurs est identique
- [ ] Vérifier que la documentation suit le même format

**Tests**:
- [ ] Test création provider "ollama"
- [ ] Test création provider "runpod" avec pod_id
- [ ] Test création provider "runpod" sans pod_id (erreur attendue)
- [ ] Test création provider inconnu (erreur attendue)
- [ ] Comparaison avec tests de `EngineFactory`

### Phase 5: Refactorisation commands/translation.rs (Aligné avec Commands Parsing)

**Objectif**: Refactoriser `commands/translation.rs` pour utiliser la factory au lieu de la logique directe, suivant le même pattern que les commands de parsing.

**Référence**: Examiner les commands de parsing pour comprendre le pattern de délégation

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
   - Créer le provider via `TranslationProviderFactory::create_provider()`
   - Appeler les méthodes du trait `TranslationProvider`
   - Préserver les signatures des fonctions publiques
   - **Suivre le même pattern que les commands de parsing**
3. Les commands ne connaissent plus les détails internes des providers
4. Les commands délèguent entièrement au provider via le trait

**Principe (identique aux commands de parsing)**:
- ✅ Les commands sont **découplées** des implémentations spécifiques
- ✅ Toute la logique spécifique est dans les providers
- ✅ Les commands utilisent uniquement le trait `TranslationProvider`
- ✅ Même pattern de délégation que les commands de parsing

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

**Validation d'alignement**:
- [ ] Comparer avec les commands de parsing
- [ ] Vérifier que le pattern de délégation est identique
- [ ] Vérifier qu'aucun routage manuel n'existe

**Tests**:
- [ ] Test que toutes les commands fonctionnent avec Ollama
- [ ] Test que toutes les commands fonctionnent avec RunPod
- [ ] Test que les erreurs sont gérées correctement
- [ ] Test que les APIs publiques sont inchangées
- [ ] Comparaison avec tests des commands de parsing

### Phase 6: Validation d'Alignement et Tests Finaux

**Objectif**: Valider que l'alignement architectural est complet et que tout fonctionne.

**Tâches**:
1. Comparaison systématique avec `parsers/` :
   - Structure de fichiers
   - Structure des traits
   - Structure des factories
   - Structure des handlers/providers
   - Pattern des commands
2. Supprimer le code mort (managers globaux non utilisés)
3. Vérifier que tous les imports sont corrects
4. Vérifier que tous les exports sont corrects
5. Exécuter tous les tests existants
6. Ajouter des tests pour le factory
7. Documenter les nouveaux modules
8. Créer un document de comparaison architecturelle

**Validation d'alignement**:
- [ ] Structure identique à `parsers/`
- [ ] Trait identique en structure à `GameEngineHandler`
- [ ] Factory identique en structure à `EngineFactory`
- [ ] Providers aussi indépendants que `RpgMakerHandler`
- [ ] Commands suivent le même pattern de délégation

**Tests**:
- [ ] Tous les tests existants passent
- [ ] Tests du factory passent
- [ ] Tests des providers passent
- [ ] Pas de régression fonctionnelle
- [ ] Tests d'alignement architectural

## Migration Checklist

### Backend
- [ ] Phase 1: Trait TranslationProvider (aligné avec handler.rs)
- [ ] Phase 2: OllamaProvider (aligné avec RpgMakerHandler)
- [ ] Phase 3: RunPodProvider (aligné avec WolfRpgHandler)
- [ ] Phase 4: TranslationProviderFactory (aligné avec EngineFactory)
- [ ] Phase 5: Refactorisation commands (aligné avec commands parsing)
- [ ] Phase 6: Validation d'alignement et Tests

### Validation d'Alignement
- [ ] Structure identique à `parsers/`
- [ ] Trait identique en structure à `GameEngineHandler`
- [ ] Factory identique en structure à `EngineFactory`
- [ ] Providers aussi indépendants que `RpgMakerHandler`
- [ ] Commands suivent le même pattern de délégation

### Validation Fonctionnelle
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

1. **Non-alignement architectural**: Structure différente de `parsers/`
   - **Mitigation**: Comparaison systématique à chaque phase, validation d'alignement
   
2. **Perte d'indépendance**: Providers dépendent encore de contextes externes
   - **Mitigation**: Tests d'indépendance à chaque phase, comparaison avec `RpgMakerHandler`
   
3. **Breaking changes**: Modification de la structure interne
   - **Mitigation**: APIs publiques inchangées, tests de régression complets
   
4. **Gestion cache RunPod**: Préservation du comportement existant
   - **Mitigation**: Tests spécifiques pour le cache, préservation de la logique existante

### Contingency Plan

Si la refactorisation pose problème:
- Garder l'ancien code en commentaire temporairement
- Rollback possible via git
- Tests de régression pour détecter les problèmes rapidement
- Comparaison continue avec `parsers/` pour garantir l'alignement

## Success Metrics

- [ ] Factory fonctionne pour Ollama et RunPod
- [ ] Commands utilisent le factory
- [ ] Pas de régression fonctionnelle
- [ ] Tests passent à 100%
- [ ] Documentation complète
- [ ] **Alignement architectural complet avec `parsers/`**
- [ ] **Structure identique à `parsers/`**
- [ ] **Providers aussi indépendants que `RpgMakerHandler`**

