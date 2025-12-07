# Implementation Plan - Translation Architecture Refactoring

## Overview

Ce plan décrit l'implémentation étape par étape de la refactorisation de l'architecture de traduction pour éliminer la duplication entre les modules `ollama` et `runpod`.

## Phase 1: Foundation & Analysis (1-2 jours)

### 1.1 Audit Complet de la Duplication
**Objectif**: Identifier précisément toute la duplication existante
**Tâches**:
- Analyser tous les fichiers `single.rs` et `sequential.rs`
- Lister toutes les structures identiques
- Documenter toutes les méthodes dupliquées
- Identifier les différences mineures (logs, noms, etc.)

**Critères de succès**:
- Rapport détaillé de duplication avec métriques
- Liste complète des éléments à factoriser

### 1.2 Création du Module Common
**Objectif**: Poser les bases de l'architecture commune
**Tâches**:
- Créer `src-tauri/src/translation/common/mod.rs`
- Créer `src-tauri/src/translation/common/types.rs`
- Migrer toutes les structures communes depuis `ollama/`

**Fichiers créés**:
```rust
src-tauri/src/translation/common/
├── mod.rs
└── types.rs  // SingleTranslationRequest, SequentialTranslationRequest, etc.
```

## Phase 2: Interface Générique (2-3 jours)

### 2.1 Définition du Trait TranslationApiTrait
**Objectif**: Créer les fonctions communes free-standing
**Tâches**:
- Créer `src-tauri/src/translation/common/functions.rs`
- Implémenter `translate_single_common()` et autres fonctions de base
- Fonctions qui prennent les clients spécifiques en paramètre

**Contenu du trait**:
```rust
// Using manual futures - no external dependency
pub trait TranslationApiTrait: Send + Sync {
    async fn translate_single(&self, prompt: &str, model: Option<String>) -> Result<String, String>;
    async fn list_available_models(&self) -> Result<Vec<String>, String>;
    async fn test_connection(&self) -> Result<(), String>;
    fn get_provider_name(&self) -> &'static str;
}
```

### 2.2 Implémentation du Trait
**Objectif**: Faire implémenter le trait par les clients existants
**Tâches**:
- Modifier `OllamaClient` pour implémenter `TranslationApiTrait`
- Modifier `RunpodClient` pour implémenter `TranslationApiTrait`
- Adapter les signatures de méthodes existantes si nécessaire

## Phase 3: Logique Commune (3-4 jours)

### 3.1 Création du CommonSequentialManager
**Objectif**: Implémenter les fonctions séquentielles communes
**Tâches**:
- Dans `src-tauri/src/translation/common/functions.rs`
- Implémenter `get_progress_common()`, `pause_session_common()`, etc.
- Fonctions qui manipulent directement les HashMap de sessions

**Méthodes à migrer**:
- `new()`
- `start_session()`
- `get_progress()`
- `pause_session()`
- `resume_session()`
- `stop_session()`
- `get_active_sessions()`
- `generate_session_id()` (privée)
- `process_session()` (privée)

### 3.2 Création du CommonSingleManager
**Objectif**: Factoriser la logique de traduction unique (~250 lignes communes)
**Tâches**:
- Ajouter `CommonSingleManager<T: TranslationApiTrait>` dans `logic.rs`
- Migrer la méthode `translate()` et ses helpers (~80 lignes)
- Garder la logique métier commune (validation, glossary, parsing)
- Déléguer les appels API spécifiques via le trait

## Phase 4: Refactorisation des Modules (2-3 jours)

### 4.1 Mise à Jour des Exports
**Objectif**: Créer les type aliases appropriés
**Tâches**:
- Dans `ollama/mod.rs` : `pub type OllamaSequentialManager = CommonSequentialManager<OllamaClient>;`
- Dans `runpod/mod.rs` : `pub type RunpodSequentialManager = CommonSequentialManager<RunpodClient>;`
- Faire de même pour les Single managers

### 4.2 Simplification des Implémentations
**Objectif**: Réduire drastiquement la taille des fichiers
**Tâches**:
- `ollama/single.rs` : ~281 lignes → ~50 lignes (wrapper appelant fonctions communes)
- `ollama/sequential.rs` : ~524 lignes → ~200 lignes (manager avec délégation)
- `runpod/single.rs` : ~321 lignes → ~50 lignes (wrapper appelant fonctions communes)
- `runpod/sequential.rs` : ~432 lignes → ~200 lignes (manager avec délégation)

### 4.3 Mise à Jour des Imports
**Objectif**: Adapter tous les imports dans le projet
**Tâches**:
- Mettre à jour `src-tauri/src/commands/translation.rs`
- Vérifier tous les fichiers qui importent depuis `translation::*`
- Compiler et corriger les erreurs d'import

## Phase 5: Tests & Validation (2-3 jours)

### 5.1 Tests de Non-Regression
**Objectif**: S'assurer que rien ne casse
**Tâches**:
- Compiler le projet à chaque étape majeure
- Tester manuellement les fonctionnalités de traduction
- Vérifier les APIs Tauri exposées

### 5.2 Tests des Nouvelles Structures
**Objectif**: Valider la nouvelle architecture
**Tâches**:
- Tests unitaires pour `CommonSequentialManager`
- Tests unitaires pour `CommonSingleManager`
- Tests d'intégration pour chaque provider

### 5.3 Performance Validation
**Objectif**: S'assurer qu'il n'y a pas de regression
**Tâches**:
- Benchmarks des traductions avant/après
- Vérification de l'utilisation mémoire
- Tests de charge si possible

## Phase 6: Nettoyage & Documentation (1 jour)

### 6.1 Suppression du Code Mort
**Objectif**: Nettoyer les duplications restantes
**Tâches**:
- Supprimer les structures dupliquées dans `ollama/` et `runpod/`
- Supprimer les méthodes dupliquées
- Vérifier qu'aucun code mort ne reste

### 6.2 Mise à Jour de la Documentation
**Objectif**: Documenter la nouvelle architecture
**Tâches**:
- Mettre à jour les commentaires dans le code
- Ajouter des exemples d'utilisation des traits
- Documenter comment ajouter un nouveau provider

## Timeline Estimation

| Phase | Durée | Dépendances |
|-------|-------|-------------|
| 1. Foundation | 1-2 jours | Aucune |
| 2. Interface | 2-3 jours | Phase 1 |
| 3. Logique Commune | 3-4 jours | Phase 2 |
| 4. Refactorisation | 2-3 jours | Phase 3 |
| 5. Tests | 2-3 jours | Phase 4 |
| 6. Nettoyage | 1 jour | Phase 5 |

**Total estimé**: 11-16 jours de développement

## Risk Mitigation

### Risques Techniques
- **Breaking changes**: Tests automatisés + compilation fréquente
- **Performance**: Benchmarks avant/après + profiling
- **Complexité**: Revue de code systématique

### Risques Fonctionnels
- **API publique**: Tests d'intégration complets
- **Fonctionnalités**: Tests manuels de toutes les features
- **Compatibilité**: Validation avec l'existant

## Success Metrics

- ✅ **Code duplication**: <5% des structures et méthodes (67% supprimé)
- ✅ **Lignes supprimées**: 670+ lignes de code dupliqué supprimées
- ✅ **Files reduced**: 4 fichiers réduits de ~95% (sequential.rs: 500→10 lignes)
- ✅ **Tests**: Tous les tests passent
- ✅ **Performance**: Pas de regression >2%
- ✅ **Maintenance**: Changement dans un seul endroit pour la logique commune

## Rollback Plan

Si nécessaire, rollback complet possible :
1. Revert des commits de refactorisation
2. Les fichiers originaux sont préservés dans git history
3. API publique reste inchangée pendant la refactorisation

## Communication Plan

- **Daily standups**: Suivi de progression
- **Weekly reviews**: Validation des phases
- **Documentation**: Mise à jour en continu
- **Testing**: Validation à chaque phase majeure
