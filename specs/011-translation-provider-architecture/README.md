# Feature 011: Alignement Architecture Translation avec Pattern Parsers

## Vue d'Ensemble

Cette feature aligne complètement l'architecture du module `translation/` avec celle de `parsers/` pour garantir cohérence architecturale, maintenabilité et extensibilité. Chaque provider doit être aussi indépendant que `RpgMakerHandler` ou `WolfRpgHandler`.

## Objectifs

- ✅ Créer un trait `TranslationProvider` suivant le même pattern que `GameEngineHandler`
- ✅ Créer `TranslationProviderFactory` suivant le même pattern que `EngineFactory`
- ✅ Refactoriser `OllamaProvider` et `RunPodProvider` pour être aussi indépendants que `RpgMakerHandler`
- ✅ Simplifier `commands/translation.rs` pour utiliser la factory comme les commands de parsing
- ✅ Éliminer tous les managers globaux et singletons
- ✅ Garantir l'alignement architectural complet avec `parsers/`

## Structure

```
specs/011-translation-provider-architecture/
├── README.md          # Ce fichier
├── spec.md            # Spécification détaillée
├── plan.md            # Plan d'implémentation
└── tasks.md           # Breakdown des tâches
```

## Architecture

### Comparaison Avant/Après

#### Avant (État Actuel)

```
translation/
├── common/types.rs
├── ollama/            # Implémentation couplée
└── runpod/            # Implémentation couplée

commands/translation.rs
├── Managers globaux (OLLAMA_SEQUENTIAL_MANAGER, etc.)
├── Cache global (RUNPOD_MANAGERS_CACHE)
└── Routage manuel avec match provider
```

#### Après (État Cible - Aligné avec Parsers)

```
translation/
├── provider.rs        # Trait TranslationProvider (comme handler.rs)
├── factory.rs         # TranslationProviderFactory (comme factory.rs)
├── ollama/
│   ├── provider.rs    # OllamaProvider (comme handler.rs)
│   └── engine.rs      # OllamaEngine (comme engine.rs)
└── runpod/
    ├── provider.rs    # RunPodProvider (comme handler.rs)
    └── engine.rs      # RunPodEngine (comme engine.rs)

commands/translation.rs
└── Utilise TranslationProviderFactory::create_provider()
```

### Alignement Architectural

| Aspect | Parsers | Translation (Cible) |
|--------|---------|-------------------|
| **Trait** | `GameEngineHandler` | `TranslationProvider` |
| **Factory** | `EngineFactory` | `TranslationProviderFactory` |
| **Handler/Provider** | `RpgMakerHandler` | `OllamaProvider` |
| **Engine** | `RpgMakerEngine` | `OllamaEngine` |
| **Indépendance** | ✅ Complète | ✅ Complète |
| **Managers globaux** | ❌ Aucun | ❌ Aucun |
| **Création** | `EngineFactory::create_handler()` | `TranslationProviderFactory::create_provider()` |

## Principes d'Alignement

### 1. Structure Identique
- Même organisation de fichiers et modules
- Même séparation entre trait, factory, handlers et engines
- Même pattern de nommage

### 2. Indépendance Complète
- Chaque provider est auto-suffisant comme `RpgMakerHandler`
- Aucune dépendance vers `commands/translation.rs`
- Aucun singleton global ou manager partagé

### 3. Factory Pattern Identique
- Même structure de factory
- Même pattern de création
- Même gestion d'erreurs

### 4. Commands Simplifiées
- Même pattern de délégation que les commands de parsing
- Utilisation de la factory au lieu de routage manuel
- Pas de logique spécifique aux providers dans les commands

## Avantages

1. **Cohérence Architecturale** : Même pattern partout dans le codebase
2. **Maintenabilité** : Code plus facile à comprendre et maintenir
3. **Extensibilité** : Ajouter un nouveau provider suit exactement le même pattern qu'ajouter un nouveau parser
4. **Testabilité** : Chaque provider peut être testé indépendamment
5. **Réduction de Complexité** : Élimination des managers globaux et du routage manuel

## Backward Compatibility

✅ **Aucun changement** dans les APIs publiques des commands Tauri  
✅ **Aucun changement** requis côté frontend  
✅ **Aucun changement** dans les types de données  
✅ Tous les tests existants doivent continuer à passer

## Implémentation

Voir [plan.md](plan.md) pour le plan d'implémentation détaillé avec comparaison systématique avec `parsers/` et [tasks.md](tasks.md) pour le breakdown des tâches.

## Validation d'Alignement

À chaque étape, valider que :
- ✅ La structure suit exactement le pattern de `parsers/`
- ✅ L'indépendance est aussi complète que `RpgMakerHandler`
- ✅ La factory suit le même pattern que `EngineFactory`
- ✅ Les commands suivent le même pattern de délégation

## Statut

**Status**: Draft  
**Branch**: `011-translation-provider-architecture`  
**Created**: 2025-01-XX

