# Spécification 005 : Refactorisation Architecture Handler Moteurs de Jeu

**Feature Branch**: `005-engine-handler-refactoring`  
**Created**: 2025-01-XX  
**Status**: Draft  
**Priority**: P1 - Refactorisation Critique

## Vue d'Ensemble

Cette spécification décrit la refactorisation majeure de l'architecture de détection et d'utilisation des moteurs de jeu dans LudoLingo. L'objectif est d'éliminer la duplication de code et la logique spécifique aux moteurs dans les fichiers `scanning.rs`, `injection.rs` et `projects.rs`, en créant un système factory avec des handlers indépendants pour chaque moteur.

## Problème Actuel

Actuellement, les fichiers suivants contiennent tous de la logique spécifique aux moteurs :

- **`scanning.rs`** : Détection manuelle des moteurs, appels directs à `RpgMakerEngine` et `WolfRpgEngine`
- **`injection.rs`** : Logique conditionnelle basée sur `GameEngine` enum avec match explicite
- **`projects.rs`** : Validation spécifique par moteur avec détection manuelle

Cette approche crée plusieurs problèmes :

1. **Duplication de code** : La détection de moteur est répétée dans plusieurs fichiers
2. **Couplage fort** : Les commands Tauri connaissent directement les implémentations des moteurs
3. **Maintenance difficile** : Ajouter un nouveau moteur nécessite de modifier plusieurs fichiers
4. **Violation du principe DRY** : La logique de validation et de détection est dupliquée

## Solution Proposée

Créer un système factory avec un trait `GameEngineHandler` qui encapsule toute la logique spécifique à un moteur :

1. **Trait `GameEngineHandler`** : Interface commune pour tous les moteurs
2. **Factory Pattern** : Détection automatique et création du bon handler
3. **Handlers Indépendants** : Chaque moteur gère sa propre logique (validation, extraction, injection)
4. **Abstraction Complète** : Les commands Tauri utilisent uniquement le trait, sans connaître les implémentations

## Architecture Cible

```
parsers/
├── mod.rs
├── engine.rs (types communs uniquement)
├── factory.rs (NOUVEAU - détection et création handlers)
├── handler.rs (NOUVEAU - trait GameEngineHandler)
├── rpg_maker/
│   └── handler.rs (NOUVEAU - implémentation trait)
└── wolfrpg/
    └── handler.rs (NOUVEAU - implémentation trait)
```

## Bénéfices

- ✅ **Séparation des responsabilités** : Chaque moteur gère sa propre logique
- ✅ **Extensibilité** : Ajouter un nouveau moteur = créer un nouveau handler
- ✅ **Maintenabilité** : Code organisé et modulaire
- ✅ **Testabilité** : Handlers testables indépendamment
- ✅ **DRY** : Plus de duplication de code

## Fichiers Impactés

### Backend (Rust)
- `src-tauri/src/parsers/` - Nouvelle architecture factory/handler
- `src-tauri/src/commands/scanning.rs` - Utilisation factory au lieu de détection manuelle
- `src-tauri/src/commands/injection.rs` - Utilisation factory au lieu de match explicite
- `src-tauri/src/commands/projects.rs` - Utilisation factory au lieu de détection manuelle

### Tests
- Tests unitaires pour chaque handler
- Tests d'intégration pour la factory
- Tests de régression pour les commands existantes

## Migration

Cette refactorisation est **backward compatible** :
- Les APIs publiques des commands Tauri restent inchangées
- Les types de données (`TextEntry`, `TranslationEntry`) restent identiques
- Aucun changement requis côté frontend

## Documentation

- **spec.md** : Spécification détaillée avec user stories
- **plan.md** : Plan d'implémentation avec architecture technique
- **tasks.md** : Breakdown détaillé des tâches par phase

