# Tasks Breakdown: Alignement Architecture Translation avec Pattern Parsers

**Branch**: `011-translation-provider-architecture` | **Spec**: [specs/011-translation-provider-architecture/spec.md](spec.md)

## Overview

Breakdown détaillé des tâches pour aligner complètement l'architecture du module `translation/` avec celle de `parsers/`. Chaque tâche inclut une validation d'alignement avec le pattern des parsers. Les tâches sont organisées par phase et peuvent être exécutées en parallèle quand indiqué avec `[P]`.

## Phase 1: Création Trait TranslationProvider (Aligné avec handler.rs)

### Task 1.1: Examiner parsers/handler.rs pour comprendre le pattern
**File**: `src-tauri/src/parsers/handler.rs` (référence)
**Status**: Pending

- [ ] Lire et comprendre la structure de `GameEngineHandler`
- [ ] Noter le pattern de documentation
- [ ] Noter le pattern des méthodes
- [ ] Noter les derives (`Send + Sync`)
- [ ] Noter les exemples de code dans la documentation

### Task 1.2: Définir le trait TranslationProvider (identique à GameEngineHandler)
**File**: `src-tauri/src/translation/provider.rs` (nouveau fichier)
**Status**: Pending

- [ ] Créer le fichier `provider.rs`
- [ ] Définir le trait `TranslationProvider` avec la même structure que `GameEngineHandler` :
  - Documentation complète en haut du trait (comme `GameEngineHandler`)
  - `provider_name() -> &str` (comme `engine_name()`)
  - `check_status(config: ProviderConfig) -> Result<serde_json::Value, String>`
  - `start_sequential_translation(app: AppHandle, request: SequentialTranslationRequest) -> Result<String, String>`
  - `get_sequential_progress(session_id: &str) -> Result<Option<SequentialProgress>, String>`
  - `pause_sequential_session(session_id: &str) -> Result<(), String>`
  - `resume_sequential_session(session_id: &str) -> Result<(), String>`
  - `stop_sequential_session(session_id: &str) -> Result<(), String>`
  - `translate_single_text(app: AppHandle, request: SingleTranslationRequest) -> Result<SingleTranslationResult, String>`
  - `get_translation_suggestions(app: AppHandle, source_text: &str, context: Option<&str>, count: usize) -> Result<Vec<TranslationSuggestion>, String>`
- [ ] Ajouter `Send + Sync` au trait (comme `GameEngineHandler`)
- [ ] Documenter chaque méthode avec le même format que `GameEngineHandler` :
  - Description de la méthode
  - Arguments avec types
  - Returns avec types
  - Exemples de code si approprié
- [ ] **Validation d'alignement** : Comparer ligne par ligne avec `GameEngineHandler`

### Task 1.3: Définir les types communs
**File**: `src-tauri/src/translation/provider.rs`
**Status**: Pending

- [ ] Définir `ProviderConfig` struct avec :
  - `id: Option<String>` (pod_id pour RunPod, ou identifiant général pour autres providers)
  - `host: Option<String>`
  - `port: Option<u16>`
  - `model: Option<String>`
  - `source_language: Option<String>`
  - `target_language: Option<String>`
- [ ] Réutiliser les types existants depuis `ollama` et `runpod` :
  - `SequentialTranslationRequest`
  - `SingleTranslationRequest`
  - `SequentialProgress`
  - `SingleTranslationResult`
  - `TranslationSuggestion`
- [ ] Créer des alias de types pour faciliter l'utilisation

### Task 1.4: Exporter le trait
**File**: `src-tauri/src/translation/mod.rs`
**Status**: Pending

- [ ] Ajouter `pub mod provider;` dans `mod.rs`
- [ ] Exporter `TranslationProvider` et `ProviderConfig`
- [ ] **Validation d'alignement** : Vérifier que les exports suivent le même pattern que `parsers/mod.rs`

### Task 1.5: Tests et validation d'alignement
**File**: `src-tauri/src/translation/provider.rs`
**Status**: Pending

- [ ] Test compilation du trait
- [ ] Test que le trait peut être utilisé comme type de retour (`Box<dyn TranslationProvider>`)
- [ ] Comparaison structurelle avec `GameEngineHandler` :
  - [ ] Même nombre de méthodes
  - [ ] Même pattern de documentation
  - [ ] Même derives
  - [ ] Même structure générale

## Phase 2: Création OllamaProvider (Aligné avec RpgMakerHandler)

### Task 2.1: Examiner parsers/rpg_maker/handler.rs pour comprendre l'indépendance
**File**: `src-tauri/src/parsers/rpg_maker/handler.rs` (référence)
**Status**: Pending

- [ ] Lire et comprendre la structure de `RpgMakerHandler`
- [ ] Noter comment `RpgMakerHandler::new()` est complètement indépendant
- [ ] Noter l'encapsulation complète (pas de dépendances externes)
- [ ] Noter le pattern d'implémentation du trait `GameEngineHandler`

### Task 2.2: Créer OllamaProvider struct (identique à RpgMakerHandler)
**File**: `src-tauri/src/translation/ollama/provider.rs` (nouveau fichier)
**Status**: Pending

- [ ] Créer le fichier `provider.rs` dans `ollama/`
- [ ] Définir `OllamaProvider` struct qui **encapsule complètement** (comme `RpgMakerHandler`) :
  - `Arc<OllamaSequentialManager>` (créé interne)
  - `Arc<OllamaSingleManager>` (créé interne)
  - `Arc<OllamaClient>` (créé interne)
  - Configuration (host, port, model)
- [ ] Implémenter `new(config: ProviderConfig) -> Result<Self, String>` qui :
  - Crée `OllamaClient` en interne (pas de dépendance externe)
  - Crée `OllamaSingleManager` en interne
  - Crée `OllamaSequentialManager` en interne
  - Aucune référence aux managers globaux
  - **Même pattern que `RpgMakerHandler::new()`**
- [ ] **Principe d'indépendance** : Le provider peut être créé sans contexte externe
- [ ] **Validation d'alignement** : Comparer la structure avec `RpgMakerHandler`

### Task 2.3: Implémenter TranslationProvider pour OllamaProvider
**File**: `src-tauri/src/translation/ollama/provider.rs`
**Status**: Pending

- [ ] Implémenter `provider_name() -> "ollama"`
- [ ] Implémenter `check_status()` en utilisant `check_ollama_status()`
- [ ] Implémenter `start_sequential_translation()` en utilisant `OllamaSequentialManager`
- [ ] Implémenter `get_sequential_progress()` en utilisant `OllamaSequentialManager`
- [ ] Implémenter `pause_sequential_session()` en utilisant `OllamaSequentialManager`
- [ ] Implémenter `resume_sequential_session()` en utilisant `OllamaSequentialManager`
- [ ] Implémenter `stop_sequential_session()` en utilisant `OllamaSequentialManager`
- [ ] Implémenter `translate_single_text()` en utilisant `OllamaSingleManager`
- [ ] Implémenter `get_translation_suggestions()` en utilisant `OllamaSingleManager`
- [ ] **Validation d'alignement** : Comparer le pattern d'implémentation avec `RpgMakerHandler`

### Task 2.4: Exporter OllamaProvider
**File**: `src-tauri/src/translation/ollama/mod.rs`
**Status**: Pending

- [ ] Ajouter `pub mod provider;` dans `mod.rs`
- [ ] Exporter `OllamaProvider`
- [ ] **Validation d'alignement** : Vérifier que les exports suivent le même pattern que `parsers/rpg_maker/mod.rs`

### Task 2.5: Tests OllamaProvider et validation d'indépendance
**File**: `src-tauri/src/translation/ollama/provider.rs`
**Status**: Pending

- [ ] Test création `OllamaProvider` avec configuration valide **sans contexte externe**
- [ ] Test que plusieurs instances de `OllamaProvider` peuvent coexister
- [ ] Test `check_status()` avec Ollama démarré
- [ ] Test `translate_single_text()` avec texte valide
- [ ] Test `start_sequential_translation()` avec requête valide
- [ ] Test indépendance complète (pas de dépendances externes)
- [ ] **Validation d'alignement** : Comparer les tests avec ceux de `RpgMakerHandler`

## Phase 3: Création RunPodProvider (Aligné avec WolfRpgHandler)

### Task 3.1: Examiner parsers/wolfrpg/handler.rs pour comprendre l'indépendance
**File**: `src-tauri/src/parsers/wolfrpg/handler.rs` (référence)
**Status**: Pending

- [ ] Lire et comprendre la structure de `WolfRpgHandler`
- [ ] Noter comment `WolfRpgHandler::new()` est complètement indépendant
- [ ] Noter l'encapsulation complète (pas de dépendances externes)
- [ ] Noter le pattern d'implémentation du trait `GameEngineHandler`

### Task 3.2: Créer RunPodProvider struct (identique à WolfRpgHandler)
**File**: `src-tauri/src/translation/runpod/provider.rs` (nouveau fichier)
**Status**: Pending

- [ ] Créer le fichier `provider.rs` dans `runpod/`
- [ ] Définir `RunPodProvider` struct qui **encapsule complètement** (comme `WolfRpgHandler`) :
  - Cache interne des managers par pod_id (`Arc<Mutex<HashMap<String, RunPodManagers>>>`)
  - Configuration (pod_id, model)
  - **Pas de dépendance vers `RUNPOD_MANAGERS_CACHE` global**
- [ ] Implémenter `new(config: ProviderConfig, pod_id: String) -> Result<Self, String>` qui :
  - Crée le cache interne (pas de référence au cache global)
  - Initialise la configuration
  - Aucune dépendance externe
  - **Même pattern que `WolfRpgHandler::new()`**
- [ ] Implémenter méthode privée `get_managers()` qui gère le cache interne
- [ ] **Principe d'indépendance** : Le provider peut être créé avec juste pod_id et config
- [ ] **Validation d'alignement** : Comparer la structure avec `WolfRpgHandler`

### Task 3.3: Implémenter TranslationProvider pour RunPodProvider
**File**: `src-tauri/src/translation/runpod/provider.rs`
**Status**: Pending

- [ ] Implémenter `provider_name() -> "runpod"`
- [ ] Implémenter `check_status()` en utilisant `check_runpod_status()`
- [ ] Implémenter `start_sequential_translation()` en utilisant le manager du cache
- [ ] Implémenter `get_sequential_progress()` en utilisant le manager du cache
- [ ] Implémenter `pause_sequential_session()` en utilisant le manager du cache
- [ ] Implémenter `resume_sequential_session()` en utilisant le manager du cache
- [ ] Implémenter `stop_sequential_session()` en utilisant le manager du cache
- [ ] Implémenter `translate_single_text()` en utilisant le manager du cache
- [ ] Implémenter `get_translation_suggestions()` en utilisant le manager du cache
- [ ] **Validation d'alignement** : Comparer le pattern d'implémentation avec `WolfRpgHandler`

### Task 3.4: Exporter RunPodProvider
**File**: `src-tauri/src/translation/runpod/mod.rs`
**Status**: Pending

- [ ] Ajouter `pub mod provider;` dans `mod.rs`
- [ ] Exporter `RunPodProvider`
- [ ] **Validation d'alignement** : Vérifier que les exports suivent le même pattern que `parsers/wolfrpg/mod.rs`

### Task 3.5: Tests RunPodProvider et validation d'indépendance
**File**: `src-tauri/src/translation/runpod/provider.rs`
**Status**: Pending

- [ ] Test création `RunPodProvider` avec pod_id valide **sans contexte externe**
- [ ] Test cache interne fonctionne correctement (pas de cache global)
- [ ] Test `check_status()` avec RunPod disponible
- [ ] Test `translate_single_text()` avec texte valide
- [ ] Test `start_sequential_translation()` avec requête valide
- [ ] Test cache partagé pour même pod_id dans la même instance de provider
- [ ] Test indépendance complète (pas de dépendances externes)
- [ ] **Validation d'alignement** : Comparer les tests avec ceux de `WolfRpgHandler`

## Phase 4: Création TranslationProviderFactory (Aligné avec EngineFactory)

### Task 4.1: Examiner parsers/factory.rs pour comprendre le pattern
**File**: `src-tauri/src/parsers/factory.rs` (référence)
**Status**: Pending

- [ ] Lire et comprendre la structure de `EngineFactory`
- [ ] Noter le pattern de création (`create_handler()`)
- [ ] Noter la gestion d'erreurs
- [ ] Noter la documentation
- [ ] Noter les exemples de code

### Task 4.2: Créer TranslationProviderFactory (identique à EngineFactory)
**File**: `src-tauri/src/translation/factory.rs` (nouveau fichier)
**Status**: Pending

- [ ] Créer le fichier `factory.rs`
- [ ] Définir `TranslationProviderFactory` struct (unit struct comme `EngineFactory`)
- [ ] Implémenter `create_provider(provider_type: &str, config: ProviderConfig) -> Result<Box<dyn TranslationProvider>, String>` avec :
  - Même structure que `EngineFactory::create_handler()`
  - Même pattern de documentation
  - Même gestion d'erreurs
  - Gérer le routing selon `provider_type` :
    - `"ollama"` → créer `OllamaProvider`
    - `"runpod"` → créer `RunPodProvider` (valider que config.id est présent)
    - Autre → retourner erreur avec message détaillé (comme `EngineFactory`)
- [ ] Valider la configuration (id requis pour RunPod)
- [ ] **Validation d'alignement** : Comparer ligne par ligne avec `EngineFactory`

### Task 4.3: Exporter TranslationProviderFactory
**File**: `src-tauri/src/translation/mod.rs`
**Status**: Pending

- [ ] Ajouter `pub mod factory;` dans `mod.rs`
- [ ] Exporter `TranslationProviderFactory`
- [ ] **Validation d'alignement** : Vérifier que les exports suivent le même pattern que `parsers/mod.rs`

### Task 4.4: Tests TranslationProviderFactory et validation d'alignement
**File**: `src-tauri/src/translation/factory.rs`
**Status**: Pending

- [ ] Test création provider "ollama" avec configuration valide
- [ ] Test création provider "runpod" avec config.id valide
- [ ] Test création provider "runpod" sans config.id (erreur attendue)
- [ ] Test création provider inconnu (erreur attendue)
- [ ] **Validation d'alignement** : Comparer les tests avec ceux de `EngineFactory`

## Phase 5: Refactorisation commands/translation.rs (Aligné avec Commands Parsing)

### Task 5.1: Examiner les commands de parsing pour comprendre le pattern
**File**: Examiner les commands de parsing (référence)
**Status**: Pending

- [ ] Lire et comprendre comment les commands de parsing utilisent `EngineFactory`
- [ ] Noter le pattern de délégation
- [ ] Noter l'absence de routage manuel
- [ ] Noter l'utilisation du trait commun

### Task 5.2: Supprimer managers globaux
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] **Supprimer complètement** `OLLAMA_SEQUENTIAL_MANAGER` (Lazy static) → maintenant dans `OllamaProvider`
- [ ] **Supprimer complètement** `OLLAMA_SINGLE_MANAGER` (Lazy static) → maintenant dans `OllamaProvider`
- [ ] **Supprimer complètement** `RUNPOD_MANAGERS_CACHE` (Lazy static) → maintenant dans `RunPodProvider`
- [ ] **Supprimer complètement** la fonction `get_runpod_managers()` → logique dans `RunPodProvider`
- [ ] Supprimer les fonctions de conversion si plus nécessaires
- [ ] Vérifier qu'aucune référence aux managers globaux ne subsiste
- [ ] **Validation d'alignement** : Vérifier qu'aucun singleton global n'existe (comme dans les commands de parsing)

### Task 5.3: Refactoriser check_ollama_status
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Utiliser `TranslationProviderFactory::create_provider("ollama", config)`
- [ ] Appeler `provider.check_status(config)`
- [ ] Préserver la signature de la fonction publique
- [ ] **Validation d'alignement** : Comparer avec le pattern des commands de parsing

### Task 5.4: Refactoriser check_runpod_status
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Utiliser `TranslationProviderFactory::create_provider("runpod", config)` avec pod_id
- [ ] Appeler `provider.check_status(config)`
- [ ] Préserver la signature de la fonction publique
- [ ] **Validation d'alignement** : Comparer avec le pattern des commands de parsing

### Task 5.5: Refactoriser start_sequential_translation
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Utiliser `TranslationProviderFactory::create_provider()` selon le provider
- [ ] Convertir les paramètres en `ProviderConfig`
- [ ] Appeler `provider.start_sequential_translation()`
- [ ] Préserver la signature de la fonction publique
- [ ] **Validation d'alignement** : Comparer avec le pattern des commands de parsing

### Task 5.6: Refactoriser get_sequential_progress
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Utiliser `TranslationProviderFactory::create_provider()` selon le provider
- [ ] Appeler `provider.get_sequential_progress()`
- [ ] Convertir le résultat en format JSON attendu
- [ ] Préserver la signature de la fonction publique
- [ ] **Validation d'alignement** : Comparer avec le pattern des commands de parsing

### Task 5.7: Refactoriser pause/resume/stop_sequential_session
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Refactoriser `pause_sequential_session()` pour utiliser factory
- [ ] Refactoriser `resume_sequential_session()` pour utiliser factory
- [ ] Refactoriser `stop_sequential_session()` pour utiliser factory
- [ ] Préserver les signatures des fonctions publiques
- [ ] **Validation d'alignement** : Comparer avec le pattern des commands de parsing

### Task 5.8: Refactoriser get_translation_suggestions
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Utiliser `TranslationProviderFactory::create_provider()` selon le provider
- [ ] Appeler `provider.get_translation_suggestions()`
- [ ] Préserver la signature de la fonction publique
- [ ] **Validation d'alignement** : Comparer avec le pattern des commands de parsing

### Task 5.9: Refactoriser translate_single_text
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Utiliser `TranslationProviderFactory::create_provider()` selon le provider
- [ ] Appeler `provider.translate_single_text()`
- [ ] Préserver la signature de la fonction publique
- [ ] **Validation d'alignement** : Comparer avec le pattern des commands de parsing

### Task 5.10: Tests commands refactorisés et validation d'alignement
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Vérifier que tous les tests existants passent
- [ ] Tester chaque command avec Ollama
- [ ] Tester chaque command avec RunPod
- [ ] Tester gestion d'erreurs
- [ ] **Validation d'alignement** : Comparer avec les tests des commands de parsing

## Phase 6: Validation d'Alignement et Tests Finaux

### Task 6.1: Comparaison systématique avec parsers/
**Status**: Pending

- [ ] Comparer structure de fichiers :
  - [ ] `translation/provider.rs` vs `parsers/handler.rs`
  - [ ] `translation/factory.rs` vs `parsers/factory.rs`
  - [ ] `translation/ollama/provider.rs` vs `parsers/rpg_maker/handler.rs`
  - [ ] `translation/runpod/provider.rs` vs `parsers/wolfrpg/handler.rs`
- [ ] Comparer structure des traits :
  - [ ] `TranslationProvider` vs `GameEngineHandler`
- [ ] Comparer structure des factories :
  - [ ] `TranslationProviderFactory` vs `EngineFactory`
- [ ] Comparer structure des handlers/providers :
  - [ ] `OllamaProvider` vs `RpgMakerHandler`
  - [ ] `RunPodProvider` vs `WolfRpgHandler`
- [ ] Comparer pattern des commands :
  - [ ] `commands/translation.rs` vs commands de parsing

### Task 6.2: Nettoyage code
**Status**: Pending

- [ ] Supprimer code mort (imports non utilisés)
- [ ] Vérifier tous les imports
- [ ] Vérifier tous les exports
- [ ] Formater le code

### Task 6.3: Tests de régression
**Status**: Pending

- [ ] Exécuter tous les tests existants
- [ ] Vérifier qu'aucune régression n'est introduite
- [ ] Ajouter tests manquants si nécessaire
- [ ] Tests d'alignement architectural

### Task 6.4: Documentation
**Status**: Pending

- [ ] Documenter le trait `TranslationProvider`
- [ ] Documenter `TranslationProviderFactory`
- [ ] Documenter `OllamaProvider`
- [ ] Documenter `RunPodProvider`
- [ ] Mettre à jour les commentaires dans `commands/translation.rs`
- [ ] Créer un document de comparaison architecturelle

## Validation Finale

### Alignement Architectural
- [ ] Structure identique à `parsers/`
- [ ] Trait identique en structure à `GameEngineHandler`
- [ ] Factory identique en structure à `EngineFactory`
- [ ] Providers aussi indépendants que `RpgMakerHandler`
- [ ] Commands suivent le même pattern de délégation

### Validation Fonctionnelle
- [ ] Tous les tests passent
- [ ] Code compile sans warnings
- [ ] Documentation complète
- [ ] Backward compatibility vérifiée
- [ ] Pas de régression fonctionnelle
- [ ] Code review effectué

