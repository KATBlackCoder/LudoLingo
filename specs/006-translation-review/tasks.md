# Task Breakdown: Review et Validation de Qualité des Traductions

**Feature**: `006-translation-review` | **Date**: 2025-01-XX

## Phase 1: Backend - Module Review (Core)

### T001 [P1] Créer le module review avec structure de base
- **File**: `src-tauri/src/translation/review/mod.rs`
- **Description**: Créer le module avec exports de base et structures de données
- **Dependencies**: None
- **Tests**: Tests de structure de base

### T002 [P1] Implémenter le calcul du score de qualité composite
- **File**: `src-tauri/src/translation/review/quality.rs`
- **Description**: Calculer le score global basé sur les critères (ratio longueur, glossaire, cohérence, anomalies)
- **Dependencies**: T001
- **Tests**: Tests unitaires pour chaque critère et score composite

### T003 [P1] Implémenter la vérification d'utilisation du glossaire
- **File**: `src-tauri/src/translation/review/glossary.rs`
- **Description**: Vérifier si les termes du glossaire sont utilisés dans la traduction
- **Dependencies**: T001, `translation/glossary` module
- **Tests**: Tests avec termes du glossaire présents/absents

### T004 [P1] Implémenter la détection de cohérence terminologique
- **File**: `src-tauri/src/translation/review/consistency.rs`
- **Description**: Détecter les incohérences dans les traductions du même projet
- **Dependencies**: T001, `tauri-plugin-sql` pour requêtes DB
- **Tests**: Tests avec traductions cohérentes/incohérentes

### T005 [P1] Implémenter la détection d'anomalies
- **File**: `src-tauri/src/translation/review/anomalies.rs`
- **Description**: Détecter traductions trop courtes/longues et caractères non traduits
- **Dependencies**: T001
- **Tests**: Tests avec différentes anomalies

## Phase 2: Backend - Commands Tauri

### T006 [P1] Ajouter command review_translations pour review global
- **File**: `src-tauri/src/commands/translation.rs`
- **Description**: Exposer la fonctionnalité de review global via command Tauri
- **Dependencies**: T002, T003, T004, T005
- **Tests**: Tests d'intégration avec données réelles

### T007 [P1] Ajouter command review_single_translation pour review individuel
- **File**: `src-tauri/src/commands/translation.rs`
- **Description**: Exposer la fonctionnalité de review individuel via command Tauri
- **Dependencies**: T002, T003, T004, T005
- **Tests**: Tests d'intégration pour review individuel

## Phase 3: Frontend - Composables et Stores

### T008 [P1] Créer composable review.ts pour les fonctions de review
- **File**: `app/composables/db/texts/review.ts`
- **Description**: Créer les fonctions TypeScript pour appeler les commands de review
- **Dependencies**: T006, T007
- **Tests**: Tests d'intégration des composables

### T009 [P2] Ajouter état pour résultats de review dans translation store
- **File**: `app/stores/translation.ts`
- **Description**: Ajouter état réactif pour stocker les résultats de review
- **Dependencies**: T008
- **Tests**: Tests du store

## Phase 4: Frontend - Interface Utilisateur

### T010 [P1] Ajouter bouton review global dans TranslationControls
- **File**: `app/components/translations/TranslationControls.vue`
- **Description**: Ajouter bouton "Vérifier la qualité" avec fonction de review global
- **Dependencies**: T008, T009
- **Tests**: Tests E2E du bouton

### T011 [P1] Ajouter bouton review dans chaque ligne de FinalTextsTable
- **File**: `app/components/translations/FinalTextsTable.vue`
- **Description**: Ajouter bouton review dans la colonne actions pour review individuel
- **Dependencies**: T008, T009
- **Tests**: Tests E2E du bouton par ligne

### T012 [P1] Ajouter bouton review dans EditTranslationModal
- **File**: `app/components/translations/EditTranslationModal.vue`
- **Description**: Ajouter bouton "Vérifier qualité" dans le modal d'édition
- **Dependencies**: T008, T009
- **Tests**: Tests E2E du modal avec review

### T013 [P2] Implémenter affichage des résultats avec badges et indicateurs
- **Files**: `app/components/translations/TranslationControls.vue`, `FinalTextsTable.vue`, `EditTranslationModal.vue`
- **Description**: Afficher les résultats de review avec badges colorés et messages
- **Dependencies**: T010, T011, T012
- **Tests**: Tests visuels de l'affichage

## Phase 5: Tests et Optimisation

### T014 [P2] Tests de performance pour review global
- **Description**: Valider que le review global fonctionne en < 5s pour 100 textes
- **Dependencies**: T006
- **Tests**: Tests de performance avec volumes variés

### T015 [P2] Tests de performance pour review individuel
- **Description**: Valider que le review individuel fonctionne en < 100ms
- **Dependencies**: T007
- **Tests**: Tests de performance pour review individuel

### T016 [P3] Optimisations (cache glossaire, traitement parallèle)
- **Description**: Optimiser les performances avec cache et parallélisation
- **Dependencies**: T014, T015
- **Tests**: Tests de performance après optimisations

## Notes

- Les tâches sont ordonnées par dépendances
- Les tâches marquées [P1] sont critiques pour la fonctionnalité de base
- Les tâches marquées [P2] sont importantes mais peuvent être ajoutées après
- Les tâches marquées [P3] sont des améliorations optionnelles

