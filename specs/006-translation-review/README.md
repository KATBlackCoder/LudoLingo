# Feature Specification: Review et Validation de Qualité des Traductions

**Feature Branch**: `006-translation-review`  
**Status**: Draft  
**Created**: 2025-01-XX

## Vue d'ensemble

Cette spécification définit l'ajout d'un système de review et validation de qualité pour les traductions générées par LudoLingo. Le système permet aux utilisateurs de vérifier automatiquement la conformité et la qualité des traductions avant leur injection dans les fichiers de jeu.

## Objectifs Principaux

1. **Review automatique** avec analyse multi-critères de qualité
2. **Validation de conformité** avec le glossaire et les termes techniques
3. **Indicateurs visuels** dans l'interface pour identifier les problèmes
4. **Suggestions d'amélioration** pour les traductions problématiques

## Structure de la Spécification

- **`spec.md`** - Spécification complète avec user stories et requirements
- **`plan.md`** - Plan d'implémentation détaillé
- **`research.md`** - Recherche sur les techniques d'évaluation de qualité
- **`tasks.md`** - Liste détaillée des tâches d'implémentation
- **`contracts/review-commands.ts`** - Contrats TypeScript pour les commands Tauri

## Critères de Qualité

Le système évalue les traductions selon 4 critères principaux :

1. **Ratio Longueur** (30%) - Vérifie que la longueur de la traduction est appropriée
2. **Utilisation Glossaire** (30%) - Vérifie l'utilisation correcte des termes du glossaire
3. **Cohérence Terminologique** (25%) - Détecte les incohérences dans les traductions
4. **Détection d'Anomalies** (15%) - Identifie les problèmes évidents (caractères non traduits, etc.)

## Fonctionnalités

### Review Global
- Analyse de toutes les traductions d'un projet
- Score moyen de qualité
- Distribution des scores (excellent, bon, moyen, faible)
- Liste des problèmes par sévérité

### Review Individuel
- Analyse d'une traduction spécifique
- Score de qualité détaillé
- Liste des problèmes spécifiques
- Suggestions d'amélioration

## Interface Utilisateur

### Boutons de Review
- **TranslationControls.vue** - Bouton "Vérifier la qualité" pour review global
- **FinalTextsTable.vue** - Bouton review dans chaque ligne pour review individuel
- **EditTranslationModal.vue** - Bouton "Vérifier qualité" dans le modal d'édition

### Affichage des Résultats
- Badges colorés selon le score (vert > 0.9, jaune 0.7-0.9, orange 0.5-0.7, rouge < 0.5)
- Notifications avec détails des problèmes
- Liste des suggestions d'amélioration

## Performance

- **Review individuel**: < 100ms
- **Review global** (100 textes): < 5 secondes

## Implémentation

Voir `tasks.md` pour la liste détaillée des tâches. L'implémentation suit 4 phases :

1. **Phase 1**: Backend - Module Review (Core)
2. **Phase 2**: Backend - Commands Tauri
3. **Phase 3**: Frontend - Composables et Stores
4. **Phase 4**: Frontend - Interface Utilisateur

## Tests

- Tests unitaires pour chaque critère de qualité
- Tests d'intégration pour le review complet
- Tests de performance pour valider les objectifs
- Tests E2E pour le workflow utilisateur

## Références

- [Spec Kit](https://github.com/github/spec-kit) - Toolkit pour Spec-Driven Development
- `specs/001-game-localization/` - Spécification principale de LudoLingo
- `specs/002-translation-providers-separation/` - Spécification des providers de traduction

