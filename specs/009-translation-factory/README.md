# Feature 009: Factory Pattern pour Providers de Traduction

## Vue d'Ensemble

Cette feature implémente un pattern factory pour les providers de traduction, similaire à celui utilisé dans le module `parsers`. L'objectif est de découpler `commands/translation.rs` des implémentations spécifiques des providers (Ollama, RunPod) et permettre une extension facile vers de nouveaux providers.

## Objectifs

- ✅ Créer un trait `TranslationProvider` pour abstraire les providers
- ✅ Créer `TranslationFactory` pour instancier les providers appropriés
- ✅ Refactoriser `commands/translation.rs` pour utiliser le factory
- ✅ Préserver la backward compatibility (APIs inchangées)
- ✅ Faciliter l'ajout de nouveaux providers

## Structure

```
specs/009-translation-factory/
├── README.md          # Ce fichier
├── spec.md            # Spécification détaillée
├── plan.md            # Plan d'implémentation
└── tasks.md           # Breakdown des tâches
```

## Architecture

### Avant (État Actuel)

```
commands/translation.rs
├── Managers globaux (OLLAMA_SEQUENTIAL_MANAGER, etc.)
├── Match explicite sur provider: String
└── Appels directs aux managers spécifiques
```

### Après (État Cible)

```
translation/
├── provider.rs        # Trait TranslationProvider
├── factory.rs         # TranslationFactory
├── ollama/
│   └── provider.rs    # OllamaProvider impl TranslationProvider
└── runpod/
    └── provider.rs    # RunPodProvider impl TranslationProvider

commands/translation.rs
└── Utilise TranslationFactory::create_provider()
```

## Avantages

1. **Découplage** : Les commands ne dépendent plus des implémentations spécifiques
2. **Indépendance complète** : Chaque provider est auto-suffisant, similaire à `RpgMakerHandler` et `WolfRpgHandler`
   - Encapsulation complète de toute la logique interne
   - Aucune dépendance vers `commands/translation.rs` ou managers globaux
   - Peut être créé et utilisé indépendamment
3. **Extensibilité** : Ajouter un nouveau provider ne nécessite que :
   - Créer un nouveau provider implémentant le trait
   - Ajouter la création dans la factory
   - Aucune modification des commands
4. **Maintenabilité** : Code plus clair et organisé
5. **Testabilité** : Chaque provider peut être testé indépendamment sans contexte externe

## Backward Compatibility

✅ **Aucun changement** dans les APIs publiques des commands Tauri
✅ **Aucun changement** requis côté frontend
✅ **Aucun changement** dans les types de données
✅ Tous les tests existants doivent continuer à passer

## Implémentation

Voir [plan.md](plan.md) pour le plan d'implémentation détaillé et [tasks.md](tasks.md) pour le breakdown des tâches.

## Tests

- Tests unitaires pour chaque provider
- Tests du factory
- Tests de régression pour les commands
- Validation de la backward compatibility

## Statut

**Status**: Draft  
**Branch**: `009-translation-factory`  
**Created**: 2025-01-XX

