# Spécification 003: Tauri Updater Plugin

## Vue d'ensemble

Cette spécification documente l'intégration du plugin Tauri Updater pour permettre les mises à jour automatiques de LudoLingo.

## Documents

- **[spec.md](spec.md)**: Spécification complète de la fonctionnalité avec user stories et requirements techniques
- **[plan.md](plan.md)**: Plan d'implémentation détaillé avec phases et structure de code
- **[tasks.md](tasks.md)**: Liste détaillée des tâches à accomplir par phase

## Objectifs

1. **Vérification automatique** des mises à jour disponibles au démarrage
2. **Téléchargement** des nouvelles versions en arrière-plan avec progression
3. **Installation** guidée avec possibilité de reporter
4. **Notifications** discrètes pour informer l'utilisateur des nouvelles versions
5. **Sécurité** via signing obligatoire pour toutes les mises à jour

## Phases d'implémentation

1. **Phase 1**: Setup et Configuration Backend ✅ TERMINÉE
2. **Phase 2**: Frontend - Composable et Store ✅ TERMINÉE
3. **Phase 3**: UI - Composants de Mise à Jour ✅ TERMINÉE
4. **Phase 4**: Configuration Serveur et Build ✅ TERMINÉE (documentation complète)
5. **Phase 5**: Intégration et Tests ✅ TERMINÉE (tests réels à faire lors du premier release)

**Progression**: 5/5 phases terminées (100%)

## Dependencies

### Backend
- `tauri-plugin-updater = "2.0"` (nouveau)

### Frontend
- `@tauri-apps/plugin-updater = "^2.0.0"` (nouveau)

## Configuration

### Signing des Mises à Jour

1. **Génération des clés**: `pnpm tauri signer generate -w ~/.tauri/ludolingo.key`
2. **Clé publique**: Stockée dans `tauri.conf.json` (peut être partagée)
3. **Clé privée**: Variable d'environnement `TAURI_SIGNING_PRIVATE_KEY` (jamais partagée)
4. **Obligatoire**: Toutes les mises à jour doivent être signées

### Configuration Serveur

Deux options disponibles:

1. **GitHub Releases**: Utiliser l'API GitHub Releases (gratuit, automatique)
2. **Serveur Custom**: Créer un endpoint API qui retourne JSON selon la plateforme

### Variables d'URL

Les endpoints peuvent utiliser des variables:
- `{{target}}`: Plateforme cible (windows, linux, macos)
- `{{arch}}`: Architecture (x86_64, arm64, etc.)
- `{{current_version}}`: Version actuelle de l'application

## Permissions

Permissions Tauri requises dans `capabilities/default.json`:
- `updater:default` (inclut check, download, install)

## Workflow Utilisateur

1. **Démarrage**: Application vérifie automatiquement les mises à jour (silencieusement)
2. **Mise à jour disponible**: Notification discrète affichée
3. **Utilisateur clique**: Dialog avec détails de la mise à jour
4. **Téléchargement**: Barre de progression affichée
5. **Installation**: Application se ferme et installateur démarre (Windows) ou installation guidée (Linux)

## Gestion d'Erreurs

- **Pas de connexion**: Ignorer silencieusement, pas de notification
- **Serveur indisponible**: Message d'erreur optionnel, possibilité de réessayer
- **Signature invalide**: Bloquer installation avec message clair
- **Téléchargement interrompu**: Permettre reprise ou nouveau téléchargement

## Statut

- **Créé**: 2025-01-XX
- **Statut**: ✅ Implémentation terminée (tests réels à faire lors du premier release)
- **Branch**: `003-tauri-updater`
- **Progression**: 5/5 phases terminées (100%)
  - ✅ Phase 1: Setup et Configuration Backend
  - ✅ Phase 2: Frontend - Composable et Store
  - ✅ Phase 3: UI - Composants de Mise à Jour
  - ✅ Phase 4: Configuration Serveur et Build (documentation complète dans `docs/UPDATER.md` et `docs/RELEASE.md`)
  - ✅ Phase 5: Intégration et Tests (vérification automatique, paramètres utilisateur, workflow complet)

## Références

- [Documentation Tauri Updater](https://tauri.app/plugin/updater/)
- [Tauri Signing Guide](https://tauri.app/plugin/updater/#signing-updates)

