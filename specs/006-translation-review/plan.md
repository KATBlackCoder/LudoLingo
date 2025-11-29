# Implementation Plan: Review et Validation de Qualité des Traductions

**Branch**: `006-translation-review` | **Date**: 2025-01-XX | **Spec**: [specs/006-translation-review/spec.md](specs/006-translation-review/spec.md)
**Input**: Feature specification from `/specs/006-translation-review/spec.md`

## Summary

Ajout d'un système de review et validation de qualité pour les traductions générées, permettant aux utilisateurs de vérifier automatiquement la conformité et la qualité avant l'injection dans les fichiers de jeu. Le système analyse les traductions selon plusieurs critères (ratio longueur, utilisation glossaire, cohérence, anomalies) et fournit des scores de qualité avec suggestions d'amélioration.

**🎯 Version 1.0 Focus**: Implémentation des critères de base avec review global et individuel. Les fonctionnalités avancées (suggestions automatiques, comparaison avec références) seront ajoutées dans les versions futures.

## Technical Context

**Language/Version**: Rust 1.x (Tauri), TypeScript 5.x (Nuxt)
**Primary Dependencies**: Tauri 2.x, Nuxt 4.x, Nuxt UI, tauri-plugin-sql
**Storage**:
- SQLite via tauri-plugin-sql : Utilisation des tables existantes (texts, glossary)
- Optionnel : Nouveau champ `quality_score` dans la table `texts`
**Testing**: Cargo test (backend uniquement) - TDD obligatoire
**Target Platform**: Desktop (Windows & Linux)
**Performance Goals**: Review individuel < 100ms, review global (100 textes) < 5 secondes
**Constraints**: Architecture offline-first, pas de dépendances externes pour l'analyse

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Architecture Tauri + Nuxt
- [x] Logique métier en Rust côté backend
- [x] Interface utilisateur en Vue/Nuxt côté frontend
- [x] Séparation claire frontend/backend respectée

### II. Base de Données SQLite
- [x] Utilisation des tables existantes (texts, glossary)
- [x] Pas de nouvelles dépendances de stockage
- [x] Optionnel : Extension de la table texts avec quality_score

### III. Performance
- [x] Traitement efficace pour gros volumes
- [x] Calculs optimisés avec cache du glossaire
- [x] Traitement asynchrone non-bloquant

## Project Structure

### Documentation (this feature)

```text
specs/006-translation-review/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md           # Research on quality assessment techniques
├── contracts/           # TypeScript contracts
│   └── review-commands.ts
└── tasks.md              # Task breakdown (to be generated)
```

### Source Code (repository root)

```text
src-tauri/src/
├── translation/
│   ├── review/           # NEW: Review module
│   │   ├── mod.rs        # Module exports
│   │   ├── quality.rs    # Quality score calculation
│   │   ├── glossary.rs   # Glossary usage verification
│   │   ├── consistency.rs # Terminological consistency
│   │   └── anomalies.rs  # Anomaly detection
│   └── ...
├── commands/
│   ├── translation.rs    # MODIFY: Add review commands
│   └── ...
```

## Implementation Phases

### Phase 1: Backend - Module Review (Core)

**Objectif**: Créer le module backend avec les fonctions de calcul de qualité

**Fichiers à créer**:
- `src-tauri/src/translation/review/mod.rs`
- `src-tauri/src/translation/review/quality.rs`
- `src-tauri/src/translation/review/glossary.rs`
- `src-tauri/src/translation/review/consistency.rs`
- `src-tauri/src/translation/review/anomalies.rs`

**Fonctionnalités**:
1. Calcul du score de qualité composite
2. Vérification de l'utilisation du glossaire
3. Détection de cohérence terminologique
4. Détection d'anomalies (longueur, caractères non traduits)

**Tests**:
- Tests unitaires pour chaque critère
- Tests d'intégration pour le score composite
- Tests de performance pour le review global

### Phase 2: Backend - Commands Tauri

**Objectif**: Exposer les fonctions de review via des commands Tauri

**Fichiers à modifier**:
- `src-tauri/src/commands/translation.rs` - Ajouter `review_translations` et `review_single_translation`

**Fonctionnalités**:
1. Command `review_translations` pour review global
2. Command `review_single_translation` pour review individuel
3. Gestion d'erreurs et validation des inputs

**Tests**:
- Tests d'intégration pour les commands
- Tests avec données réelles de projets

### Phase 3: Frontend - Composables et Stores

**Objectif**: Créer les composables et stores pour gérer le review

**Fichiers à créer**:
- `app/composables/db/texts/review.ts` - Fonctions de review

**Fichiers à modifier**:
- `app/stores/translation.ts` - Ajouter état pour les résultats de review

**Fonctionnalités**:
1. Fonction `reviewTranslations()` pour review global
2. Fonction `reviewSingleTranslation()` pour review individuel
3. Gestion de l'état des résultats de review

### Phase 4: Frontend - Interface Utilisateur

**Objectif**: Ajouter les boutons et l'affichage des résultats dans l'UI

**Fichiers à modifier**:
- `app/components/translations/TranslationControls.vue` - Bouton review global
- `app/components/translations/FinalTextsTable.vue` - Bouton review par ligne
- `app/components/translations/EditTranslationModal.vue` - Bouton review dans modal

**Fonctionnalités**:
1. Bouton "Vérifier la qualité" dans TranslationControls
2. Bouton review dans chaque ligne de FinalTextsTable
3. Bouton review dans EditTranslationModal
4. Affichage des résultats avec badges et notifications
5. Indicateurs visuels pour les problèmes détectés

## Dependencies

### Backend
- Utilise les modules existants : `translation/glossary` pour lookup
- Utilise `tauri-plugin-sql` pour requêtes DB
- Pas de nouvelles dépendances externes

### Frontend
- Utilise les composables existants : `useNotifications`, `useSettings`
- Utilise les stores existants : `useTranslationStore`, `useProjectsStore`
- Pas de nouvelles dépendances externes

## Migration Strategy

### Base de Données (Optionnel)

Si on veut stocker les scores de qualité :

```sql
-- Migration optionnelle pour ajouter quality_score
ALTER TABLE texts ADD COLUMN quality_score REAL;
```

Cette migration est optionnelle car les scores peuvent être calculés à la volée.

## Testing Strategy

### Backend Tests

1. **Tests unitaires** pour chaque critère de qualité
2. **Tests d'intégration** pour le review complet
3. **Tests de performance** pour valider les objectifs (< 5s pour 100 textes)

### Frontend Tests

1. **Tests d'intégration** pour les composables
2. **Tests E2E** pour le workflow complet (traduction → review → affichage)

## Success Criteria

- ✅ Review individuel fonctionne en < 100ms
- ✅ Review global fonctionne en < 5s pour 100 textes
- ✅ Tous les critères de qualité sont correctement évalués
- ✅ Interface utilisateur claire avec indicateurs visuels
- ✅ Tests avec couverture > 80%

