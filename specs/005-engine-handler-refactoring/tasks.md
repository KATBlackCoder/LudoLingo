# Task Breakdown: Refactorisation Architecture Handler Moteurs

## Vue d'Ensemble des Tâches

Cette spécification couvre la refactorisation majeure de l'architecture de détection et d'utilisation des moteurs de jeu. L'objectif est d'éliminer la duplication de code et de créer un système factory avec des handlers indépendants pour chaque moteur. Les tâches sont organisées par phase d'implémentation avec estimations réalistes.

---

## Phase 1: Création du Trait et Factory (1-2 jours - 12h)

### Tâche 1.1: Créer le Trait GameEngineHandler
**ID**: `refactor-1`
**Priorité**: P1
**Estimation**: 2h
**Assigné**: Backend Developer
**Dépendances**: Aucune

**Description**:
Créer le trait `GameEngineHandler` qui définit l'interface commune pour tous les moteurs de jeu.

**Étapes**:
1. Créer `src-tauri/src/parsers/handler.rs`
2. Définir le trait `GameEngineHandler` avec toutes les méthodes requises
3. Définir la structure `ValidationResult` pour les résultats de validation
4. Ajouter les imports nécessaires (`Path`, `PathBuf`, `TextEntry`, `TranslationEntry`)
5. Documenter chaque méthode du trait avec des commentaires Rust

**Critères d'acceptation**:
- ✅ Trait `GameEngineHandler` défini avec toutes les méthodes requises
- ✅ Structure `ValidationResult` définie avec tous les champs nécessaires
- ✅ Documentation complète avec exemples d'utilisation
- ✅ Code compile sans erreurs

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/handler.rs` (NOUVEAU)

### Tâche 1.2: Créer EngineFactory
**ID**: `refactor-2`
**Priorité**: P1
**Estimation**: 3h
**Assigné**: Backend Developer
**Dépendances**: `refactor-1`

**Description**:
Créer la factory `EngineFactory` qui détecte le moteur et crée le handler approprié.

**Étapes**:
1. Créer `src-tauri/src/parsers/factory.rs`
2. Implémenter `EngineFactory::create_handler()` avec logique de détection
3. Ordre de détection : WolfRPG (dump/), WolfRPG chiffré (Data.wolf), RPG Maker MZ, RPG Maker MV
4. Retourner des erreurs détaillées avec suggestions si aucun moteur détecté
5. Ajouter des tests unitaires pour chaque type de projet

**Critères d'acceptation**:
- ✅ Factory détecte correctement tous les types de projets
- ✅ Retourne le bon handler pour chaque type
- ✅ Messages d'erreur clairs avec suggestions
- ✅ Tests unitaires passent pour tous les cas

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/factory.rs` (NOUVEAU)

### Tâche 1.3: Créer RpgMakerHandler Basique
**ID**: `refactor-3`
**Priorité**: P1
**Estimation**: 3h
**Assigné**: Backend Developer
**Dépendances**: `refactor-1`

**Description**:
Créer `RpgMakerHandler` avec implémentation basique du trait (peut être incomplet à ce stade).

**Étapes**:
1. Créer `src-tauri/src/parsers/rpg_maker/handler.rs`
2. Définir la structure `RpgMakerHandler` avec champ pour version (MV/MZ)
3. Implémenter `new_mz()` et `new_mv()` pour création
4. Implémenter `engine_name()` pour retourner le nom approprié
5. Implémenter les autres méthodes avec stubs ou appels aux fonctions existantes
6. Mettre à jour `src-tauri/src/parsers/rpg_maker/mod.rs` pour exporter

**Critères d'acceptation**:
- ✅ `RpgMakerHandler` créé et compile
- ✅ Implémente le trait `GameEngineHandler`
- ✅ Distinction MV/MZ fonctionne
- ✅ Code compile sans erreurs

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/rpg_maker/handler.rs` (NOUVEAU)
- `src-tauri/src/parsers/rpg_maker/mod.rs` (MODIFIER)

### Tâche 1.4: Créer WolfRpgHandler Basique
**ID**: `refactor-4`
**Priorité**: P1
**Estimation**: 2h
**Assigné**: Backend Developer
**Dépendances**: `refactor-1`

**Description**:
Créer `WolfRpgHandler` avec implémentation basique du trait (peut être incomplet à ce stade).

**Étapes**:
1. Créer `src-tauri/src/parsers/wolfrpg/handler.rs`
2. Définir la structure `WolfRpgHandler`
3. Implémenter `new()` pour création
4. Implémenter `engine_name()` pour retourner "Wolf RPG Editor"
5. Implémenter les autres méthodes avec stubs ou appels aux fonctions existantes
6. Mettre à jour `src-tauri/src/parsers/wolfrpg/mod.rs` pour exporter

**Critères d'acceptation**:
- ✅ `WolfRpgHandler` créé et compile
- ✅ Implémente le trait `GameEngineHandler`
- ✅ Code compile sans erreurs

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/wolfrpg/handler.rs` (NOUVEAU)
- `src-tauri/src/parsers/wolfrpg/mod.rs` (MODIFIER)

### Tâche 1.5: Mettre à Jour Exports Parsers
**ID**: `refactor-5`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-2`, `refactor-3`, `refactor-4`

**Description**:
Mettre à jour `parsers/mod.rs` pour exporter les nouveaux modules (handler, factory).

**Étapes**:
1. Ajouter `pub mod handler;` dans `parsers/mod.rs`
2. Ajouter `pub mod factory;` dans `parsers/mod.rs`
3. Exporter le trait et la factory : `pub use handler::*;` et `pub use factory::*;`
4. Vérifier que tous les imports fonctionnent

**Critères d'acceptation**:
- ✅ Tous les nouveaux modules exportés correctement
- ✅ Imports fonctionnent dans les autres fichiers
- ✅ Code compile sans erreurs

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/mod.rs` (MODIFIER)

### Tâche 1.6: Tests Factory
**ID**: `refactor-6`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-5`

**Description**:
Créer des tests unitaires pour la factory afin de valider la détection correcte.

**Étapes**:
1. Créer des structures de projet temporaires pour chaque moteur
2. Tester `EngineFactory::create_handler()` pour chaque type
3. Vérifier que le bon handler est retourné
4. Tester les cas d'erreur (projet non reconnu)

**Critères d'acceptation**:
- ✅ Tests passent pour tous les types de projets
- ✅ Tests d'erreur passent correctement
- ✅ Coverage > 80% pour la factory

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/factory.rs` (MODIFIER - ajout tests)

---

## Phase 2: Implémentation RpgMakerHandler (1 jour - 6h)

### Tâche 2.1: Implémenter validate_project_structure pour RPG Maker
**ID**: `refactor-7`
**Priorité**: P1
**Estimation**: 1.5h
**Assigné**: Backend Developer
**Dépendances**: `refactor-3`

**Description**:
Implémenter complètement `validate_project_structure()` en utilisant la logique existante de `RpgMakerEngine`.

**Étapes**:
1. Appeler `RpgMakerEngine::validate_project_structure()` avec la version appropriée
2. Convertir le résultat en `ValidationResult`
3. Extraire les erreurs et warnings
4. Retourner le résultat formaté

**Critères d'acceptation**:
- ✅ Validation fonctionne pour MV et MZ
- ✅ Retourne des erreurs/warnings détaillés
- ✅ Tests unitaires passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/rpg_maker/handler.rs` (MODIFIER)

### Tâche 2.2: Implémenter extract_all_texts pour RPG Maker
**ID**: `refactor-8`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-3`

**Description**:
Implémenter `extract_all_texts()` en utilisant `RpgMakerEngine::extract_all()`.

**Étapes**:
1. Appeler `RpgMakerEngine::extract_all()` avec le game_path et la version
2. Retourner le résultat directement (même type)

**Critères d'acceptation**:
- ✅ Extraction fonctionne pour MV et MZ
- ✅ Retourne les mêmes résultats qu'avant
- ✅ Tests unitaires passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/rpg_maker/handler.rs` (MODIFIER)

### Tâche 2.3: Implémenter inject_all_texts pour RPG Maker
**ID**: `refactor-9`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-3`

**Description**:
Implémenter `inject_all_texts()` en utilisant `RpgMakerEngine::inject_all()`.

**Étapes**:
1. Appeler `RpgMakerEngine::inject_all()` avec le game_path, translations et version
2. Retourner le résultat directement

**Critères d'acceptation**:
- ✅ Injection fonctionne pour MV et MZ
- ✅ Retourne les mêmes résultats qu'avant
- ✅ Tests unitaires passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/rpg_maker/handler.rs` (MODIFIER)

### Tâche 2.4: Implémenter count_files_to_process pour RPG Maker
**ID**: `refactor-10`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-3`

**Description**:
Implémenter `count_files_to_process()` avec la logique RPG Maker (Actors.json, Items.json, Maps, etc.).

**Étapes**:
1. Déterminer le data_root selon la version (data/ ou www/data/)
2. Compter les fichiers JSON dans data/
3. Compter les fichiers dans Map/
4. Retourner le total

**Critères d'acceptation**:
- ✅ Compte correctement les fichiers pour MV et MZ
- ✅ Retourne le même nombre qu'avant
- ✅ Tests unitaires passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/rpg_maker/handler.rs` (MODIFIER)

### Tâche 2.5: Implémenter get_data_root pour RPG Maker
**ID**: `refactor-11`
**Priorité**: P1
**Estimation**: 0.5h
**Assigné**: Backend Developer
**Dépendances**: `refactor-3`

**Description**:
Implémenter `get_data_root()` en utilisant `RpgMakerEngine::get_data_root()`.

**Étapes**:
1. Appeler `RpgMakerEngine::get_data_root()` avec le game_path et la version
2. Retourner le résultat directement

**Critères d'acceptation**:
- ✅ Retourne le bon chemin pour MV et MZ
- ✅ Tests unitaires passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/rpg_maker/handler.rs` (MODIFIER)

### Tâche 2.6: Tests Complets RpgMakerHandler
**ID**: `refactor-12`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: Toutes les tâches Phase 2

**Description**:
Créer des tests unitaires complets pour `RpgMakerHandler` avec projets MV et MZ.

**Étapes**:
1. Créer des structures de projet temporaires MV et MZ
2. Tester toutes les méthodes du handler
3. Vérifier que les résultats sont identiques à ceux de `RpgMakerEngine`
4. Tests d'erreur pour projets invalides

**Critères d'acceptation**:
- ✅ Tous les tests passent
- ✅ Coverage > 80% pour RpgMakerHandler
- ✅ Résultats identiques à l'ancien code

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/rpg_maker/handler.rs` (MODIFIER - ajout tests)

---

## Phase 3: Implémentation WolfRpgHandler (1 jour - 6h)

### Tâche 3.1: Implémenter validate_project_structure pour WolfRPG
**ID**: `refactor-13`
**Priorité**: P1
**Estimation**: 1.5h
**Assigné**: Backend Developer
**Dépendances**: `refactor-4`

**Description**:
Implémenter complètement `validate_project_structure()` en utilisant la logique existante de `WolfRpgEngine`.

**Étapes**:
1. Appeler `WolfRpgEngine::validate_project_structure()`
2. Convertir le résultat en `ValidationResult`
3. Extraire les erreurs et warnings
4. Retourner le résultat formaté

**Critères d'acceptation**:
- ✅ Validation fonctionne pour WolfRPG
- ✅ Retourne des erreurs/warnings détaillés
- ✅ Tests unitaires passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/wolfrpg/handler.rs` (MODIFIER)

### Tâche 3.2: Implémenter extract_all_texts pour WolfRPG
**ID**: `refactor-14`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-4`

**Description**:
Implémenter `extract_all_texts()` en utilisant `WolfRpgEngine::extract_all()`.

**Étapes**:
1. Appeler `WolfRpgEngine::extract_all()` avec le game_path
2. Retourner le résultat directement

**Critères d'acceptation**:
- ✅ Extraction fonctionne pour WolfRPG
- ✅ Retourne les mêmes résultats qu'avant
- ✅ Tests unitaires passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/wolfrpg/handler.rs` (MODIFIER)

### Tâche 3.3: Implémenter inject_all_texts pour WolfRPG
**ID**: `refactor-15`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-4`

**Description**:
Implémenter `inject_all_texts()` en utilisant `WolfRpgEngine::inject_all()`.

**Étapes**:
1. Appeler `WolfRpgEngine::inject_all()` avec le game_path et translations
2. Retourner le résultat directement

**Critères d'acceptation**:
- ✅ Injection fonctionne pour WolfRPG
- ✅ Retourne les mêmes résultats qu'avant
- ✅ Tests unitaires passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/wolfrpg/handler.rs` (MODIFIER)

### Tâche 3.4: Implémenter count_files_to_process pour WolfRPG
**ID**: `refactor-16`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-4`

**Description**:
Implémenter `count_files_to_process()` avec la logique WolfRPG (db/, mps/, common/).

**Étapes**:
1. Compter les fichiers JSON dans dump/db/
2. Compter les fichiers JSON dans dump/mps/
3. Compter les fichiers JSON dans dump/common/
4. Retourner le total

**Critères d'acceptation**:
- ✅ Compte correctement les fichiers pour WolfRPG
- ✅ Retourne le même nombre qu'avant
- ✅ Tests unitaires passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/wolfrpg/handler.rs` (MODIFIER)

### Tâche 3.5: Implémenter get_data_root pour WolfRPG
**ID**: `refactor-17`
**Priorité**: P1
**Estimation**: 0.5h
**Assigné**: Backend Developer
**Dépendances**: `refactor-4`

**Description**:
Implémenter `get_data_root()` en utilisant `WolfRpgEngine::get_data_root()`.

**Étapes**:
1. Appeler `WolfRpgEngine::get_data_root()` avec le game_path
2. Retourner le résultat directement

**Critères d'acceptation**:
- ✅ Retourne le bon chemin (dump/)
- ✅ Tests unitaires passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/wolfrpg/handler.rs` (MODIFIER)

### Tâche 3.6: Tests Complets WolfRpgHandler
**ID**: `refactor-18`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: Toutes les tâches Phase 3

**Description**:
Créer des tests unitaires complets pour `WolfRpgHandler`.

**Étapes**:
1. Créer une structure de projet temporaire WolfRPG
2. Tester toutes les méthodes du handler
3. Vérifier que les résultats sont identiques à ceux de `WolfRpgEngine`
4. Tests d'erreur pour projets invalides

**Critères d'acceptation**:
- ✅ Tous les tests passent
- ✅ Coverage > 80% pour WolfRpgHandler
- ✅ Résultats identiques à l'ancien code

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/parsers/wolfrpg/handler.rs` (MODIFIER - ajout tests)

---

## Phase 4: Refactorisation projects.rs (1 jour - 4h)

### Tâche 4.1: Supprimer detect_game_engine
**ID**: `refactor-19`
**Priorité**: P1
**Estimation**: 0.5h
**Assigné**: Backend Developer
**Dépendances**: Phase 1, Phase 2, Phase 3

**Description**:
Supprimer la fonction `detect_game_engine()` de `projects.rs` car elle est dupliquée dans la factory.

**Étapes**:
1. Identifier tous les appels à `detect_game_engine()`
2. Vérifier qu'ils seront remplacés par la factory
3. Supprimer la fonction `detect_game_engine()`
4. Vérifier que le code compile

**Critères d'acceptation**:
- ✅ Fonction `detect_game_engine()` supprimée
- ✅ Code compile sans erreurs

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/projects.rs` (MODIFIER)

### Tâche 4.2: Remplacer par Factory dans validate_game_path
**ID**: `refactor-20`
**Priorité**: P1
**Estimation**: 2h
**Assigné**: Backend Developer
**Dépendances**: `refactor-19`

**Description**:
Refactoriser `validate_game_path()` pour utiliser la factory au lieu de `detect_game_engine()`.

**Étapes**:
1. Remplacer `detect_game_engine()` par `EngineFactory::create_handler()`
2. Utiliser `handler.validate_project_structure()` pour la validation
3. Utiliser `handler.engine_name()` pour `detected_engine`
4. Supprimer toute la logique de validation spécifique au moteur
5. Mettre à jour les imports

**Critères d'acceptation**:
- ✅ `validate_game_path()` utilise la factory
- ✅ Plus de logique spécifique au moteur dans `projects.rs`
- ✅ Tests existants passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/projects.rs` (MODIFIER)

### Tâche 4.3: Simplifier validate_game_path
**ID**: `refactor-21`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-20`

**Description**:
Simplifier `validate_game_path()` en supprimant toute la logique de validation spécifique qui est maintenant dans les handlers.

**Étapes**:
1. Supprimer les validations spécifiques RPG Maker MZ
2. Supprimer les validations spécifiques RPG Maker MV
3. Supprimer les validations spécifiques WolfRPG
4. Utiliser uniquement le résultat du handler
5. Convertir `ValidationResult` en `GamePathValidation`

**Critères d'acceptation**:
- ✅ Code simplifié et plus court
- ✅ Toute la logique spécifique déléguée aux handlers
- ✅ Tests existants passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/projects.rs` (MODIFIER)

### Tâche 4.4: Nettoyer Imports projects.rs
**ID**: `refactor-22`
**Priorité**: P1
**Estimation**: 0.5h
**Assigné**: Backend Developer
**Dépendances**: `refactor-21`

**Description**:
Supprimer les imports inutiles de `projects.rs`.

**Étapes**:
1. Identifier les imports non utilisés
2. Supprimer les imports spécifiques aux moteurs
3. Ajouter `use crate::parsers::factory::EngineFactory;`
4. Vérifier que le code compile

**Critères d'acceptation**:
- ✅ Plus d'imports inutiles
- ✅ Imports nécessaires présents
- ✅ Code compile sans erreurs

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/projects.rs` (MODIFIER)

### Tâche 4.5: Tests Régression projects.rs
**ID**: `refactor-23`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-22`

**Description**:
Exécuter tous les tests existants pour `projects.rs` et vérifier qu'il n'y a pas de régression.

**Étapes**:
1. Exécuter `cargo test` pour tous les tests de projects
2. Vérifier que tous les tests passent
3. Comparer les résultats avec l'ancien code (baseline)
4. Documenter tout changement de comportement

**Critères d'acceptation**:
- ✅ Tous les tests existants passent
- ✅ Aucune régression fonctionnelle
- ✅ Résultats identiques à l'ancien code

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- Tests existants (VALIDATION)

---

## Phase 5: Refactorisation scanning.rs (1 jour - 6h)

### Tâche 5.1: Remplacer detect_engine par Factory
**ID**: `refactor-24`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: Phase 1, Phase 2, Phase 3

**Description**:
Remplacer les appels à `detect_engine()` par `EngineFactory::create_handler()` dans `scanning.rs`.

**Étapes**:
1. Remplacer `detect_engine(path)` par `EngineFactory::create_handler(path)?`
2. Stocker le handler dans une variable
3. Utiliser le handler pour les opérations suivantes
4. Mettre à jour les imports

**Critères d'acceptation**:
- ✅ Plus d'appel direct à `detect_engine()` dans `scanning.rs`
- ✅ Utilise la factory pour obtenir le handler
- ✅ Code compile sans erreurs

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/scanning.rs` (MODIFIER)

### Tâche 5.2: Remplacer Match Explicite par Handler
**ID**: `refactor-25`
**Priorité**: P1
**Estimation**: 2h
**Assigné**: Backend Developer
**Dépendances**: `refactor-24`

**Description**:
Remplacer les match explicites sur `GameEngine` par des appels au handler dans `extract_texts_from_folder()`.

**Étapes**:
1. Supprimer le match sur `GameEngine` dans `extract_texts_from_folder()`
2. Appeler `handler.extract_all_texts(path)` directement
3. Utiliser `handler.engine_name()` pour les messages d'erreur
4. Mettre à jour les messages d'erreur pour utiliser le handler

**Critères d'acceptation**:
- ✅ Plus de match explicite sur `GameEngine`
- ✅ Utilise le handler pour l'extraction
- ✅ Messages d'erreur utilisent `handler.engine_name()`
- ✅ Tests existants passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/scanning.rs` (MODIFIER)

### Tâche 5.3: Refactoriser perform_scan
**ID**: `refactor-26`
**Priorité**: P1
**Estimation**: 2h
**Assigné**: Backend Developer
**Dépendances**: `refactor-25`

**Description**:
Refactoriser `perform_scan()` pour utiliser le handler au lieu des match explicites.

**Étapes**:
1. Modifier `perform_scan()` pour accepter un handler au lieu d'un `GameEngine`
2. Appeler `handler.extract_all_texts()` au lieu des appels spécifiques
3. Mettre à jour `scan_folder()` pour passer le handler à `perform_scan()`
4. Supprimer les match explicites

**Critères d'acceptation**:
- ✅ `perform_scan()` utilise le handler
- ✅ Plus de match explicite sur `GameEngine`
- ✅ Tests existants passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/scanning.rs` (MODIFIER)

### Tâche 5.4: Nettoyer Imports scanning.rs
**ID**: `refactor-27`
**Priorité**: P1
**Estimation**: 0.5h
**Assigné**: Backend Developer
**Dépendances**: `refactor-26`

**Description**:
Supprimer les imports inutiles (`RpgMakerEngine`, `WolfRpgEngine`) de `scanning.rs`.

**Étapes**:
1. Identifier les imports non utilisés
2. Supprimer `use crate::parsers::rpg_maker::engine::RpgMakerEngine;`
3. Supprimer `use crate::parsers::wolfrpg::engine::WolfRpgEngine;`
4. Ajouter `use crate::parsers::factory::EngineFactory;`
5. Vérifier que le code compile

**Critères d'acceptation**:
- ✅ Plus d'imports inutiles
- ✅ Imports nécessaires présents
- ✅ Code compile sans erreurs

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/scanning.rs` (MODIFIER)

### Tâche 5.5: Tests Régression scanning.rs
**ID**: `refactor-28`
**Priorité**: P1
**Estimation**: 0.5h
**Assigné**: Backend Developer
**Dépendances**: `refactor-27`

**Description**:
Exécuter tous les tests existants pour `scanning.rs` et vérifier qu'il n'y a pas de régression.

**Étapes**:
1. Exécuter `cargo test` pour tous les tests de scanning
2. Vérifier que tous les tests passent
3. Comparer les résultats avec l'ancien code (baseline)
4. Documenter tout changement de comportement

**Critères d'acceptation**:
- ✅ Tous les tests existants passent
- ✅ Aucune régression fonctionnelle
- ✅ Résultats identiques à l'ancien code

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- Tests existants (VALIDATION)

---

## Phase 6: Refactorisation injection.rs (1 jour - 6h)

### Tâche 6.1: Remplacer detect_engine par Factory
**ID**: `refactor-29`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: Phase 1, Phase 2, Phase 3

**Description**:
Remplacer les appels à `detect_engine()` par `EngineFactory::create_handler()` dans `injection.rs`.

**Étapes**:
1. Remplacer `detect_engine(game_path)` par `EngineFactory::create_handler(game_path)?` dans `start_injection()`
2. Remplacer dans `validate_injection()` également
3. Stocker le handler dans des variables appropriées
4. Mettre à jour les imports

**Critères d'acceptation**:
- ✅ Plus d'appel direct à `detect_engine()` dans `injection.rs`
- ✅ Utilise la factory pour obtenir le handler
- ✅ Code compile sans erreurs

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/injection.rs` (MODIFIER)

### Tâche 6.2: Remplacer Match Explicite par Handler dans start_injection
**ID**: `refactor-30`
**Priorité**: P1
**Estimation**: 2h
**Assigné**: Backend Developer
**Dépendances**: `refactor-29`

**Description**:
Remplacer les match explicites sur `GameEngine` par des appels au handler dans `start_injection()`.

**Étapes**:
1. Supprimer le match sur `GameEngine` dans `start_injection()`
2. Utiliser `handler.count_files_to_process()` au lieu de `count_files_to_process()`
3. Passer le handler à `perform_injection_sync()` au lieu de `GameEngine`
4. Mettre à jour les messages d'erreur

**Critères d'acceptation**:
- ✅ Plus de match explicite sur `GameEngine` dans `start_injection()`
- ✅ Utilise le handler pour compter les fichiers
- ✅ Tests existants passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/injection.rs` (MODIFIER)

### Tâche 6.3: Refactoriser perform_injection_sync
**ID**: `refactor-31`
**Priorité**: P1
**Estimation**: 2h
**Assigné**: Backend Developer
**Dépendances**: `refactor-30`

**Description**:
Refactoriser `perform_injection_sync()` pour utiliser le handler au lieu des match explicites.

**Étapes**:
1. Modifier `perform_injection_sync()` pour accepter un handler au lieu d'un `GameEngine`
2. Appeler `handler.inject_all_texts()` au lieu des appels spécifiques
3. Supprimer les match explicites
4. Mettre à jour les appels à `perform_injection_sync()`

**Critères d'acceptation**:
- ✅ `perform_injection_sync()` utilise le handler
- ✅ Plus de match explicite sur `GameEngine`
- ✅ Tests existants passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/injection.rs` (MODIFIER)

### Tâche 6.4: Refactoriser validate_injection
**ID**: `refactor-32`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-29`

**Description**:
Refactoriser `validate_injection()` pour utiliser le handler au lieu de la détection manuelle.

**Étapes**:
1. Utiliser le handler obtenu de la factory
2. Appeler `handler.validate_project_structure()` au lieu de la validation manuelle
3. Utiliser `handler.count_files_to_process()` au lieu de `count_files_to_process()`
4. Utiliser `handler.engine_name()` pour `detected_engine`

**Critères d'acceptation**:
- ✅ `validate_injection()` utilise le handler
- ✅ Plus de validation manuelle spécifique au moteur
- ✅ Tests existants passent

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/injection.rs` (MODIFIER)

### Tâche 6.5: Supprimer count_files_to_process
**ID**: `refactor-33`
**Priorité**: P1
**Estimation**: 0.5h
**Assigné**: Backend Developer
**Dépendances**: `refactor-32`

**Description**:
Supprimer la fonction `count_files_to_process()` de `injection.rs` car elle est maintenant dans les handlers.

**Étapes**:
1. Identifier tous les appels à `count_files_to_process()`
2. Vérifier qu'ils sont tous remplacés par `handler.count_files_to_process()`
3. Supprimer la fonction `count_files_to_process()`
4. Vérifier que le code compile

**Critères d'acceptation**:
- ✅ Fonction `count_files_to_process()` supprimée
- ✅ Tous les appels utilisent le handler
- ✅ Code compile sans erreurs

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/injection.rs` (MODIFIER)

### Tâche 6.6: Nettoyer Imports injection.rs
**ID**: `refactor-34`
**Priorité**: P1
**Estimation**: 0.5h
**Assigné**: Backend Developer
**Dépendances**: `refactor-33`

**Description**:
Supprimer les imports inutiles de `injection.rs`.

**Étapes**:
1. Identifier les imports non utilisés
2. Supprimer les imports spécifiques aux moteurs
3. Ajouter `use crate::parsers::factory::EngineFactory;`
4. Vérifier que le code compile

**Critères d'acceptation**:
- ✅ Plus d'imports inutiles
- ✅ Imports nécessaires présents
- ✅ Code compile sans erreurs

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- `src-tauri/src/commands/injection.rs` (MODIFIER)

### Tâche 6.7: Tests Régression injection.rs
**ID**: `refactor-35`
**Priorité**: P1
**Estimation**: 0.5h
**Assigné**: Backend Developer
**Dépendances**: `refactor-34`

**Description**:
Exécuter tous les tests existants pour `injection.rs` et vérifier qu'il n'y a pas de régression.

**Étapes**:
1. Exécuter `cargo test` pour tous les tests d'injection
2. Vérifier que tous les tests passent
3. Comparer les résultats avec l'ancien code (baseline)
4. Documenter tout changement de comportement

**Critères d'acceptation**:
- ✅ Tous les tests existants passent
- ✅ Aucune régression fonctionnelle
- ✅ Résultats identiques à l'ancien code

**Statut**: ✅ COMPLÉTÉ

**Fichiers à créer/modifier**:
- Tests existants (VALIDATION)

---

## Phase 7: Tests et Validation (1 jour - 6h)

### Tâche 7.1: Tests Unitaires Complets
**ID**: `refactor-36`
**Priorité**: P1
**Estimation**: 2h
**Assigné**: Backend Developer
**Dépendances**: Toutes les phases précédentes

**Description**:
Exécuter tous les tests unitaires et vérifier qu'ils passent tous.

**Étapes**:
1. Exécuter `cargo test` pour tous les tests
2. Identifier les tests qui échouent
3. Corriger les problèmes identifiés
4. Vérifier que tous les tests passent

**Critères d'acceptation**:
- ✅ Tous les tests unitaires passent
- ✅ Coverage > 80% pour les nouveaux modules
- ✅ Aucune régression détectée

**Fichiers à créer/modifier**:
- Tests existants (VALIDATION)

### Tâche 7.2: Tests d'Intégration
**ID**: `refactor-37`
**Priorité**: P1
**Estimation**: 2h
**Assigné**: Backend Developer
**Dépendances**: `refactor-36`

**Description**:
Exécuter tous les tests d'intégration et vérifier qu'ils passent tous.

**Étapes**:
1. Exécuter les tests d'intégration existants
2. Tester avec des projets réels (RPG Maker MV, MZ, WolfRPG)
3. Vérifier que les résultats sont identiques à l'ancien code
4. Documenter tout changement de comportement

**Critères d'acceptation**:
- ✅ Tous les tests d'intégration passent
- ✅ Résultats identiques à l'ancien code
- ✅ Aucune régression fonctionnelle

**Fichiers à créer/modifier**:
- Tests existants (VALIDATION)

### Tâche 7.3: Benchmarks Performance
**ID**: `refactor-38`
**Priorité**: P2
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-37`

**Description**:
Comparer les performances avant/après refactorisation pour s'assurer qu'il n'y a pas de dégradation.

**Étapes**:
1. Créer des benchmarks pour extraction, injection, validation
2. Exécuter les benchmarks avec l'ancien code (baseline)
3. Exécuter les benchmarks avec le nouveau code
4. Comparer les résultats
5. Documenter les différences

**Critères d'acceptation**:
- ✅ Performances identiques ou meilleures
- ✅ Pas d'overhead significatif introduit
- ✅ Détection reste < 2 secondes

**Fichiers à créer/modifier**:
- Benchmarks (CRÉER)

### Tâche 7.4: Validation APIs Publiques
**ID**: `refactor-39`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-37`

**Description**:
Vérifier que les APIs publiques des commands Tauri sont inchangées.

**Étapes**:
1. Comparer les signatures des commands avant/après
2. Vérifier que les types de retour sont identiques
3. Vérifier que les types de paramètres sont identiques
4. Documenter tout changement (ne devrait pas y en avoir)

**Critères d'acceptation**:
- ✅ Toutes les APIs publiques inchangées
- ✅ Aucun changement breaking
- ✅ Documentation à jour

**Fichiers à créer/modifier**:
- Documentation (VALIDATION)

---

## Phase 8: Nettoyage et Documentation (0.5 jour - 3h)

### Tâche 8.1: Supprimer Code Mort
**ID**: `refactor-40`
**Priorité**: P2
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: Toutes les phases précédentes

**Description**:
Identifier et supprimer le code mort (fonctions/structures non utilisées).

**Étapes**:
1. Identifier les fonctions non utilisées avec `cargo clippy`
2. Vérifier si `RpgMakerEngine` et `WolfRpgEngine` sont encore utilisés directement
3. Si non utilisés directement, les marquer comme dépréciés ou les supprimer
4. Nettoyer les imports inutiles

**Critères d'acceptation**:
- ✅ Code mort supprimé
- ✅ Aucun warning `cargo clippy` pour code non utilisé
- ✅ Code propre et maintenable

**Fichiers à créer/modifier**:
- Tous les fichiers modifiés (NETTOYAGE)

### Tâche 8.2: Documentation Code
**ID**: `refactor-41`
**Priorité**: P1
**Estimation**: 1h
**Assigné**: Backend Developer
**Dépendances**: `refactor-40`

**Description**:
Ajouter de la documentation complète sur le trait, la factory et les handlers.

**Étapes**:
1. Documenter le trait `GameEngineHandler` avec exemples
2. Documenter la factory `EngineFactory` avec exemples
3. Documenter chaque handler avec exemples d'utilisation
4. Ajouter des commentaires dans les commands refactorisées

**Critères d'acceptation**:
- ✅ Documentation complète pour tous les nouveaux modules
- ✅ Exemples d'utilisation fournis
- ✅ Documentation générée avec `cargo doc`

**Fichiers à créer/modifier**:
- Tous les fichiers modifiés (DOCUMENTATION)

### Tâche 8.3: Mettre à Jour progress.md
**ID**: `refactor-42`
**Priorité**: P1
**Estimation**: 0.5h
**Assigné**: Backend Developer
**Dépendances**: `refactor-41`

**Description**:
Mettre à jour `progress.md` pour documenter la refactorisation complétée.

**Étapes**:
1. Ajouter une section sur la refactorisation complétée
2. Documenter les changements architecturaux
3. Mettre à jour les métriques de code
4. Documenter les bénéfices obtenus

**Critères d'acceptation**:
- ✅ `progress.md` mis à jour
- ✅ Refactorisation documentée
- ✅ Bénéfices clairement expliqués

**Fichiers à créer/modifier**:
- `progress.md` (MODIFIER)

### Tâche 8.4: Review Finale
**ID**: `refactor-43`
**Priorité**: P1
**Estimation**: 0.5h
**Assigné**: Backend Developer
**Dépendances**: `refactor-42`

**Description**:
Effectuer une review finale du code pour s'assurer de la qualité et de la cohérence.

**Étapes**:
1. Vérifier la cohérence des noms et de l'organisation
2. Vérifier qu'il n'y a pas de duplication restante
3. Vérifier que tous les tests passent
4. Vérifier que la documentation est complète

**Critères d'acceptation**:
- ✅ Code de qualité professionnelle
- ✅ Architecture claire et maintenable
- ✅ Prêt pour merge

**Fichiers à créer/modifier**:
- Tous les fichiers modifiés (REVIEW)

---

## Résumé des Estimations

**Total estimé** : 6-7 jours de développement (45-50h)

- Phase 1 : 1-2 jours (12h)
- Phase 2 : 1 jour (6h)
- Phase 3 : 1 jour (6h)
- Phase 4 : 1 jour (4h) - Refactorisation projects.rs
- Phase 5 : 1 jour (6h) - Refactorisation scanning.rs
- Phase 6 : 1 jour (6h) - Refactorisation injection.rs
- Phase 7 : 1 jour (6h)
- Phase 8 : 0.5 jour (3h)

**Complexité** : Moyenne à élevée (refactorisation majeure nécessitant une attention aux détails)

**Risques** : Régression fonctionnelle (mitigé par tests de régression complets)

