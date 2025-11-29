# Tasks Breakdown: Factory Pattern pour Providers de Traduction

**Branch**: `009-translation-factory` | **Spec**: [specs/009-translation-factory/spec.md](spec.md)

## Overview

Breakdown détaillé des tâches pour implémenter le factory pattern pour les providers de traduction. Les tâches sont organisées par phase et peuvent être exécutées en parallèle quand indiqué avec `[P]`.

## Phase 1: Création Trait TranslationProvider

### Task 1.1: Définir le trait TranslationProvider
**File**: `src-tauri/src/translation/provider.rs` (nouveau fichier)
**Status**: Pending

- [ ] Créer le fichier `provider.rs`
- [ ] Définir le trait `TranslationProvider` avec toutes les méthodes :
  - `provider_name() -> &str`
  - `check_status(config: ProviderConfig) -> Result<serde_json::Value, String>`
  - `start_sequential_translation(app: AppHandle, request: SequentialTranslationRequest) -> Result<String, String>`
  - `get_sequential_progress(session_id: &str) -> Result<SequentialProgress, String>`
  - `pause_sequential_session(session_id: &str) -> Result<(), String>`
  - `resume_sequential_session(session_id: &str) -> Result<(), String>`
  - `stop_sequential_session(session_id: &str) -> Result<(), String>`
  - `translate_single_text(app: AppHandle, request: SingleTranslationRequest) -> Result<SingleTranslationResult, String>`
  - `get_translation_suggestions(app: AppHandle, source_text: &str, context: Option<&str>, count: usize) -> Result<Vec<TranslationSuggestion>, String>`
- [ ] Ajouter `Send + Sync` au trait
- [ ] Documenter chaque méthode

### Task 1.2: Définir les types communs
**File**: `src-tauri/src/translation/provider.rs`
**Status**: Pending

- [ ] Définir `ProviderConfig` struct avec :
  - `pod_id: Option<String>`
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

### Task 1.3: Exporter le trait
**File**: `src-tauri/src/translation/mod.rs`
**Status**: Pending

- [ ] Ajouter `pub mod provider;` dans `mod.rs`
- [ ] Exporter `TranslationProvider` et `ProviderConfig`

## Phase 2: Création OllamaProvider

### Task 2.1: Créer OllamaProvider struct
**File**: `src-tauri/src/translation/ollama/provider.rs` (nouveau fichier)
**Status**: Pending

- [ ] Créer le fichier `provider.rs` dans `ollama/`
- [ ] Définir `OllamaProvider` struct qui **encapsule complètement** :
  - `Arc<OllamaSequentialManager>` (créé interne)
  - `Arc<OllamaSingleManager>` (créé interne)
  - `Arc<OllamaClient>` (créé interne)
  - Configuration (host, port, model)
- [ ] Implémenter `new(config: ProviderConfig) -> Result<Self, String>` qui :
  - Crée `OllamaClient` en interne (pas de dépendance externe)
  - Crée `OllamaSingleManager` en interne
  - Crée `OllamaSequentialManager` en interne
  - Aucune référence aux managers globaux
- [ ] **Principe d'indépendance** : Le provider peut être créé sans contexte externe

### Task 2.2: Implémenter TranslationProvider pour OllamaProvider
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

### Task 2.3: Exporter OllamaProvider
**File**: `src-tauri/src/translation/ollama/mod.rs`
**Status**: Pending

- [ ] Ajouter `pub mod provider;` dans `mod.rs`
- [ ] Exporter `OllamaProvider`

### Task 2.4: Tests OllamaProvider
**File**: `src-tauri/src/translation/ollama/provider.rs`
**Status**: Pending

- [ ] Test création `OllamaProvider` avec configuration valide **sans contexte externe**
- [ ] Test que plusieurs instances de `OllamaProvider` peuvent coexister
- [ ] Test `check_status()` avec Ollama démarré
- [ ] Test `translate_single_text()` avec texte valide
- [ ] Test `start_sequential_translation()` avec requête valide
- [ ] Test indépendance complète (pas de dépendances externes)

## Phase 3: Création RunPodProvider

### Task 3.1: Créer RunPodProvider struct
**File**: `src-tauri/src/translation/runpod/provider.rs` (nouveau fichier)
**Status**: Pending

- [ ] Créer le fichier `provider.rs` dans `runpod/`
- [ ] Définir `RunPodProvider` struct qui **encapsule complètement** :
  - Cache interne des managers par pod_id (`Arc<Mutex<HashMap<String, RunPodManagers>>>`)
  - Configuration (pod_id, model)
  - **Pas de dépendance vers `RUNPOD_MANAGERS_CACHE` global**
- [ ] Implémenter `new(config: ProviderConfig, pod_id: String) -> Result<Self, String>` qui :
  - Crée le cache interne (pas de référence au cache global)
  - Initialise la configuration
  - Aucune dépendance externe
- [ ] Implémenter méthode privée `get_managers()` qui gère le cache interne
- [ ] **Principe d'indépendance** : Le provider peut être créé avec juste pod_id et config

### Task 3.2: Implémenter TranslationProvider pour RunPodProvider
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

### Task 3.3: Exporter RunPodProvider
**File**: `src-tauri/src/translation/runpod/mod.rs`
**Status**: Pending

- [ ] Ajouter `pub mod provider;` dans `mod.rs`
- [ ] Exporter `RunPodProvider`

### Task 3.4: Tests RunPodProvider
**File**: `src-tauri/src/translation/runpod/provider.rs`
**Status**: Pending

- [ ] Test création `RunPodProvider` avec pod_id valide **sans contexte externe**
- [ ] Test cache interne fonctionne correctement (pas de cache global)
- [ ] Test `check_status()` avec RunPod disponible
- [ ] Test `translate_single_text()` avec texte valide
- [ ] Test `start_sequential_translation()` avec requête valide
- [ ] Test cache partagé pour même pod_id dans la même instance de provider
- [ ] Test indépendance complète (pas de dépendances externes)

## Phase 4: Création TranslationFactory

### Task 4.1: Créer TranslationFactory
**File**: `src-tauri/src/translation/factory.rs` (nouveau fichier)
**Status**: Pending

- [ ] Créer le fichier `factory.rs`
- [ ] Définir `TranslationFactory` struct (unit struct)
- [ ] Implémenter `create_provider(provider_type: &str, config: ProviderConfig) -> Result<Box<dyn TranslationProvider>, String>`
- [ ] Gérer le routing selon `provider_type` :
  - `"ollama"` → créer `OllamaProvider`
  - `"runpod"` → créer `RunPodProvider` (valider que pod_id est présent)
  - Autre → retourner erreur
- [ ] Valider la configuration (pod_id requis pour RunPod)

### Task 4.2: Exporter TranslationFactory
**File**: `src-tauri/src/translation/mod.rs`
**Status**: Pending

- [ ] Ajouter `pub mod factory;` dans `mod.rs`
- [ ] Exporter `TranslationFactory`

### Task 4.3: Tests TranslationFactory
**File**: `src-tauri/src/translation/factory.rs`
**Status**: Pending

- [ ] Test création provider "ollama" avec configuration valide
- [ ] Test création provider "runpod" avec pod_id valide
- [ ] Test création provider "runpod" sans pod_id (erreur attendue)
- [ ] Test création provider inconnu (erreur attendue)

## Phase 5: Refactorisation commands/translation.rs

### Task 5.1: Supprimer managers globaux
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] **Supprimer complètement** `OLLAMA_SEQUENTIAL_MANAGER` (Lazy static) → maintenant dans `OllamaProvider`
- [ ] **Supprimer complètement** `OLLAMA_SINGLE_MANAGER` (Lazy static) → maintenant dans `OllamaProvider`
- [ ] **Supprimer complètement** `RUNPOD_MANAGERS_CACHE` (Lazy static) → maintenant dans `RunPodProvider`
- [ ] **Supprimer complètement** la fonction `get_runpod_managers()` → logique dans `RunPodProvider`
- [ ] Supprimer les fonctions de conversion si plus nécessaires
- [ ] Vérifier qu'aucune référence aux managers globaux ne subsiste

### Task 5.2: Refactoriser check_ollama_status
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Utiliser `TranslationFactory::create_provider("ollama", config)`
- [ ] Appeler `provider.check_status(config)`
- [ ] Préserver la signature de la fonction publique

### Task 5.3: Refactoriser check_runpod_status
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Utiliser `TranslationFactory::create_provider("runpod", config)` avec pod_id
- [ ] Appeler `provider.check_status(config)`
- [ ] Préserver la signature de la fonction publique

### Task 5.4: Refactoriser start_sequential_translation
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Utiliser `TranslationFactory::create_provider()` selon le provider
- [ ] Convertir les paramètres en `ProviderConfig`
- [ ] Appeler `provider.start_sequential_translation()`
- [ ] Préserver la signature de la fonction publique

### Task 5.5: Refactoriser get_sequential_progress
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Utiliser `TranslationFactory::create_provider()` selon le provider
- [ ] Appeler `provider.get_sequential_progress()`
- [ ] Convertir le résultat en format JSON attendu
- [ ] Préserver la signature de la fonction publique

### Task 5.6: Refactoriser pause/resume/stop_sequential_session
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Refactoriser `pause_sequential_session()` pour utiliser factory
- [ ] Refactoriser `resume_sequential_session()` pour utiliser factory
- [ ] Refactoriser `stop_sequential_session()` pour utiliser factory
- [ ] Préserver les signatures des fonctions publiques

### Task 5.7: Refactoriser get_translation_suggestions
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Utiliser `TranslationFactory::create_provider()` selon le provider
- [ ] Appeler `provider.get_translation_suggestions()`
- [ ] Préserver la signature de la fonction publique

### Task 5.8: Refactoriser translate_single_text
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Utiliser `TranslationFactory::create_provider()` selon le provider
- [ ] Appeler `provider.translate_single_text()`
- [ ] Préserver la signature de la fonction publique

### Task 5.9: Tests commands refactorisés
**File**: `src-tauri/src/commands/translation.rs`
**Status**: Pending

- [ ] Vérifier que tous les tests existants passent
- [ ] Tester chaque command avec Ollama
- [ ] Tester chaque command avec RunPod
- [ ] Tester gestion d'erreurs

## Phase 6: Nettoyage et Tests Finaux

### Task 6.1: Nettoyage code
**Status**: Pending

- [ ] Supprimer code mort (imports non utilisés)
- [ ] Vérifier tous les imports
- [ ] Vérifier tous les exports
- [ ] Formater le code

### Task 6.2: Tests de régression
**Status**: Pending

- [ ] Exécuter tous les tests existants
- [ ] Vérifier qu'aucune régression n'est introduite
- [ ] Ajouter tests manquants si nécessaire

### Task 6.3: Documentation
**Status**: Pending

- [ ] Documenter le trait `TranslationProvider`
- [ ] Documenter `TranslationFactory`
- [ ] Documenter `OllamaProvider`
- [ ] Documenter `RunPodProvider`
- [ ] Mettre à jour les commentaires dans `commands/translation.rs`

## Validation Finale

- [ ] Tous les tests passent
- [ ] Code compile sans warnings
- [ ] Documentation complète
- [ ] Backward compatibility vérifiée
- [ ] Pas de régression fonctionnelle
- [ ] Code review effectué

