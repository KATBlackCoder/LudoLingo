# Spec 007: Translation Architecture Refactoring

## Overview

Cette spécification décrit la refactorisation majeure de l'architecture de traduction pour éliminer **~670 lignes de code dupliqué** (67% du code translation) entre les modules `ollama` et `runpod`.

## Problem Statement

L'analyse du code révèle une duplication massive :
- **9 structures identiques** : `SingleTranslationRequest`, `SequentialTranslationRequest`, etc.
- **13+ méthodes dupliquées** : `get_progress()`, `pause_session()`, `translate()`, etc.
- **Logique métier commune** : Validation, parsing, gestion des glossaires
- **Impact** : 670+ lignes dupliquées (67% du code), maintenance ×2, risque d'incohérence

## Solution

Créer une architecture commune avec :
- Module `translation/common` pour les types et fonctions partagés
- Fonctions communes free-standing qui marchent avec n'importe quel client
- Implémentations spécialisées pour Ollama et RunPod
- Réduction de 67% de la duplication (670+ lignes supprimées)

## Portée : Back-end uniquement

Cette refactorisation est **strictement back-end** (Rust/Tauri) :
- ✅ **Aucune modification front-end** requise
- ✅ **API Tauri inchangée** (mêmes commandes, mêmes signatures)
- ✅ **Interface utilisateur** identique
- ✅ **Compatibilité totale** avec le code existant

## Contraintes Importantes

- **Aucune dépendance externe** : Utilisation des futures Rust standard uniquement
- **Pas de trait générique** : Approche plus simple avec fonctions communes
- **Backward compatibility** : API publique inchangée
- **Performance** : Overhead minimal (<2%)

## Files

- `spec.md` - Spécification détaillée des changements
- `plan.md` - Plan d'implémentation étape par étape
- `tasks.md` - Liste des tâches concrètes
- `research.md` - Analyse technique et décisions
- `quickstart.md` - Guide d'utilisation post-refactorisation
- `checklists/` - Points de validation

## Related Specs

- **002-translation-providers-separation** - Base de la séparation actuelle
- **009-translation-factory** - Évolution future de l'architecture
