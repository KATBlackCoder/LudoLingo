# Feature Specification: Review et Validation de Qualité des Traductions

**Feature Branch**: `006-translation-review`
**Created**: 2025-01-XX
**Status**: Draft
**Input**: Ajout d'un système de review et validation de qualité pour vérifier la conformité et la qualité des traductions après leur génération

**🎯 Objectif**: Permettre aux utilisateurs de vérifier automatiquement la qualité et la conformité des traductions générées, avec des indicateurs visuels et des suggestions d'amélioration.

## Contexte

Après avoir traduit des textes via Ollama ou RunPod, les utilisateurs ont besoin de vérifier la qualité des traductions avant de les injecter dans les fichiers de jeu. Actuellement, il n'existe qu'une estimation basique de confiance basée sur le ratio de longueur. Cette fonctionnalité ajoute :

1. **Review automatique** avec analyse multi-critères de qualité
2. **Validation de conformité** avec le glossaire et les termes techniques
3. **Indicateurs visuels** dans l'interface pour identifier les problèmes
4. **Suggestions d'amélioration** pour les traductions problématiques

## User Scenarios & Testing

### User Story 1 - Review Global des Traductions (Priority: P1)

Utilisateur lance un review global de toutes les traductions terminées pour obtenir un score de qualité global et identifier les problèmes.

**Why this priority**: C'est la fonctionnalité principale qui permet de vérifier rapidement l'ensemble des traductions avant l'injection.

**Independent Test**: Peut être testé en traduisant plusieurs textes, lançant le review global, et vérifiant que le score et les problèmes sont correctement identifiés.

**Acceptance Scenarios**:

1. **Given** des textes traduits dans un projet, **When** l'utilisateur clique sur "Vérifier la qualité", **Then** un score global de qualité est calculé et affiché avec la liste des problèmes détectés
2. **Given** des traductions de bonne qualité, **When** le review est lancé, **Then** le score est élevé (>80%) et peu ou pas de problèmes sont détectés
3. **Given** des traductions avec problèmes, **When** le review est lancé, **Then** les problèmes sont catégorisés par sévérité (low, medium, high) avec messages explicites

---

### User Story 2 - Review Individuel d'une Traduction (Priority: P1)

Utilisateur vérifie la qualité d'une traduction spécifique depuis la table des textes finaux ou le modal d'édition.

**Why this priority**: Permet de vérifier rapidement une traduction suspecte sans relancer le review global.

**Independent Test**: Peut être testé en sélectionnant un texte traduit, cliquant sur le bouton review, et vérifiant que le score et les problèmes sont affichés.

**Acceptance Scenarios**:

1. **Given** un texte traduit dans la table, **When** l'utilisateur clique sur le bouton review, **Then** le score de qualité et les problèmes spécifiques sont affichés dans une notification
2. **Given** un texte en cours d'édition dans le modal, **When** l'utilisateur clique sur "Vérifier qualité", **Then** le score et les suggestions sont affichés dans le modal
3. **Given** une traduction modifiée manuellement, **When** le review est relancé, **Then** le nouveau score reflète les modifications

---

### User Story 3 - Critères de Qualité Multiples (Priority: P1)

Le système analyse les traductions selon plusieurs critères : ratio longueur, utilisation du glossaire, cohérence terminologique, détection d'anomalies.

**Why this priority**: Des critères multiples permettent une évaluation plus précise que le simple ratio de longueur.

**Independent Test**: Peut être testé en créant des traductions avec différents problèmes (trop courtes, termes non utilisés, etc.) et vérifiant que chaque critère est correctement évalué.

**Acceptance Scenarios**:

1. **Given** une traduction avec ratio longueur anormal, **When** le review est lancé, **Then** un problème de type "too_short" ou "too_long" est détecté avec sévérité appropriée
2. **Given** une traduction qui n'utilise pas les termes du glossaire, **When** le review est lancé, **Then** un problème "missing_glossary" est détecté si des termes du glossaire sont présents dans le texte source
3. **Given** une traduction avec incohérences terminologiques, **When** le review est lancé, **Then** un problème "inconsistent" est détecté avec détails sur les incohérences

---

### User Story 4 - Affichage des Résultats de Review (Priority: P2)

Les résultats du review sont affichés de manière claire avec indicateurs visuels (badges, couleurs) et suggestions d'amélioration.

**Why this priority**: Une présentation claire des résultats permet à l'utilisateur de comprendre rapidement les problèmes et d'agir en conséquence.

**Independent Test**: Peut être testé en lançant un review et vérifiant que les résultats sont affichés avec les bons indicateurs visuels et messages.

**Acceptance Scenarios**:

1. **Given** un review avec score élevé, **When** les résultats sont affichés, **Then** un badge vert avec le score est affiché et les notifications sont positives
2. **Given** un review avec problèmes critiques, **When** les résultats sont affichés, **Then** les problèmes sont affichés en rouge avec messages explicites
3. **Given** un review avec suggestions d'amélioration, **When** les résultats sont affichés, **Then** les suggestions sont listées avec possibilité d'agir (retraduire, modifier, etc.)

---

## Technical Requirements

### Backend (Rust)

- Nouvelle commande Tauri `review_translations` pour review global
- Nouvelle commande Tauri `review_single_translation` pour review individuel
- Module `translation/review` avec logique d'analyse de qualité
- Critères d'évaluation :
  - Ratio longueur source/traduction (0.5-2.0 acceptable)
  - Utilisation des termes du glossaire
  - Cohérence terminologique (détection de variations)
  - Détection de traductions trop courtes/longues
  - Vérification de la présence de caractères spéciaux non traduits

### Frontend (Vue/Nuxt)

- Bouton "Vérifier la qualité" dans `TranslationControls.vue`
- Bouton review dans chaque ligne de `FinalTextsTable.vue`
- Bouton review dans `EditTranslationModal.vue`
- Affichage des résultats avec badges et indicateurs visuels
- Intégration avec le système de notifications existant

### Base de Données

- Pas de nouvelles tables nécessaires
- Utilisation des données existantes (texts, glossary)
- Stockage optionnel des scores de qualité dans la table `texts` (nouveau champ `quality_score`)

## Non-Goals

- Review automatique en temps réel pendant la traduction (sera ajouté plus tard)
- Suggestions automatiques de correction (sera ajouté plus tard)
- Comparaison avec d'autres traductions similaires (sera ajouté plus tard)
- Export de rapports de qualité (sera ajouté plus tard)

## Success Metrics

- Temps de review < 5 secondes pour 100 traductions
- Précision de détection des problèmes > 85%
- Utilisation du review par > 70% des utilisateurs avant injection

