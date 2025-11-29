# Research: Review et Validation de Qualité des Traductions

**Feature**: `006-translation-review` | **Date**: 2025-01-XX

## Objectif de la Recherche

Identifier les meilleures pratiques et techniques pour évaluer automatiquement la qualité des traductions générées par des modèles LLM, avec un focus sur la localisation de jeux vidéo.

## Techniques d'Évaluation de Qualité

### 1. Métriques Basées sur la Longueur

**Ratio Longueur Source/Traduction**
- **Acceptable**: 0.5 - 2.0 (selon les langues)
- **Idéal**: 0.8 - 1.5 pour la plupart des paires de langues
- **Implémentation**: Simple à calculer, déjà partiellement implémenté dans `estimate_confidence()`

**Référence**: 
- Études sur les ratios de longueur dans les traductions automatiques
- Variations selon les paires de langues (JA→FR peut avoir des ratios différents de EN→FR)

### 2. Utilisation du Glossaire

**Détection de Termes du Glossaire**
- Vérifier si les termes du glossaire présents dans le texte source sont utilisés dans la traduction
- Détecter les variations (majuscules, pluriels) des termes du glossaire
- Score basé sur le pourcentage de termes du glossaire correctement utilisés

**Implémentation**:
- Requête au glossaire pour récupérer les termes pertinents (déjà disponible via `lookup_glossary_terms`)
- Matching flexible (insensible à la casse, gestion des pluriels)
- Calcul du score : `(termes_utilisés / termes_disponibles) * 100`

### 3. Cohérence Terminologique

**Détection d'Incohérences**
- Identifier les variations de traduction pour le même terme source
- Détecter les traductions différentes pour des termes identiques dans le même contexte
- Comparer avec les traductions précédentes du même projet

**Implémentation**:
- Requête SQL pour trouver les occurrences du même terme source dans le projet
- Comparaison des traductions pour détecter les variations
- Score basé sur le pourcentage de cohérence

### 4. Détection d'Anomalies

**Traductions Trop Courtes/Longues**
- Détecter les traductions anormalement courtes (possiblement incomplètes)
- Détecter les traductions anormalement longues (possiblement verbeuses ou avec erreurs)

**Caractères Non Traduits**
- Détecter la présence de caractères japonais/chinois dans les traductions FR/EN
- Détecter les caractères spéciaux non traduits

**Implémentation**:
- Seuils configurables pour longueur minimale/maximale
- Regex pour détecter les caractères non-ASCII dans les traductions cibles

## Architecture Technique

### Module Backend Rust

**Structure proposée**:
```
src-tauri/src/translation/review/
├── mod.rs           # Exports du module
├── quality.rs       # Calcul des scores de qualité
├── glossary.rs      # Vérification utilisation glossaire
├── consistency.rs   # Vérification cohérence terminologique
└── anomalies.rs     # Détection d'anomalies
```

### Critères de Qualité

**Score Global**:
- Poids pour chaque critère :
  - Ratio longueur: 30%
  - Utilisation glossaire: 30%
  - Cohérence: 25%
  - Anomalies: 15%

**Sévérité des Problèmes**:
- **High**: Problèmes critiques (traduction incomplète, caractères non traduits)
- **Medium**: Problèmes modérés (non-utilisation glossaire, incohérences)
- **Low**: Problèmes mineurs (ratio légèrement hors norme)

### Performance

**Optimisations**:
- Cache des termes du glossaire pour éviter les requêtes répétées
- Traitement par lots pour le review global
- Calculs parallèles pour les critères indépendants

**Temps estimé**:
- Review individuel: < 100ms
- Review global (100 textes): < 5 secondes

## Références

- [BLEU Score](https://en.wikipedia.org/wiki/BLEU) - Métrique standard pour évaluation de traduction (trop complexe pour notre cas)
- [Translation Quality Assessment](https://www.aclweb.org/anthology/P19-1020/) - Recherche académique sur l'évaluation de qualité
- Pratiques de localisation de jeux vidéo - Standards de l'industrie

## Décisions Techniques

1. **Pas de BLEU Score**: Trop complexe et nécessite des références, pas adapté pour notre cas d'usage
2. **Score composite personnalisé**: Plus adapté à notre contexte de localisation de jeux
3. **Vérification glossaire obligatoire**: Critique pour la cohérence terminologique
4. **Détection d'anomalies simple**: Focus sur les cas évidents plutôt que l'analyse linguistique complexe

