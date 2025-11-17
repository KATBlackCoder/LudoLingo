# Implementation Plan: Tauri Updater Plugin

**Branch**: `003-tauri-updater` | **Date**: 2025-01-XX | **Spec**: [specs/003-tauri-updater/spec.md](spec.md)
**Input**: Feature specification from `/specs/003-tauri-updater/spec.md`

## Summary

Intégration du plugin Tauri Updater pour permettre les mises à jour automatiques de LudoLingo. Cette fonctionnalité permettra aux utilisateurs de recevoir et installer les nouvelles versions sans intervention manuelle.

**🎯 Objectif**: Système de mise à jour automatique complet et sécurisé avec interface utilisateur intuitive.

## Technical Context

**Language/Version**: Rust 1.77.2+ (Tauri), TypeScript 5.x (Nuxt)
**Primary Dependencies**: 
- Backend: `tauri-plugin-updater` (nouveau)
- Frontend: `@tauri-apps/plugin-updater` (nouveau)
**Storage**: 
- Configuration dans `tauri.conf.json`
- Clés de signature stockées de manière sécurisée
**Testing**: Tests manuels pour workflow complet (vérification, téléchargement, installation)
**Target Platform**: Desktop (Windows & Linux uniquement)
**Project Type**: Desktop application (Tauri + Nuxt)
**Constraints**: 
- Signing obligatoire pour toutes les mises à jour
- Compatibilité avec architecture Tauri + Nuxt existante
- Gestion d'erreurs robuste pour ne pas perturber l'utilisateur

## Constitution Check

*GATE: Must pass before implementation*

### I. Architecture Tauri + Nuxt
- [x] Séparation frontend/backend respectée
- [x] Commands Tauri pour toute interaction système
- [x] Stores Pinia pour état partagé frontend
- [x] Plugin updater initialisé côté backend uniquement

### II. Sécurité
- [x] Signing obligatoire pour toutes les mises à jour
- [x] Clé privée stockée de manière sécurisée
- [x] Vérification d'intégrité avant installation

### III. Expérience Utilisateur
- [x] Vérification automatique non intrusive
- [x] Notifications discrètes pour nouvelles versions
- [x] Possibilité de reporter l'installation
- [x] Feedback visuel pendant téléchargement

## Project Structure

### Documentation (this feature)

```text
specs/003-tauri-updater/
├── spec.md              # Feature specification
├── plan.md              # This file
└── tasks.md             # Detailed task breakdown
```

### Source Code Changes

```text
src-tauri/
├── Cargo.toml           # Ajout tauri-plugin-updater
├── tauri.conf.json      # Configuration updater (endpoints, pubkey)
└── src/
    ├── lib.rs           # Initialisation plugin updater
    └── commands/
        └── updater.rs   # Commands pour gérer mises à jour (optionnel)

app/
├── composables/
│   └── updater/
│       └── useUpdater.ts        # Composable principal
├── stores/
│   └── updater.ts               # Store Pinia pour état
└── components/
    └── updater/
        ├── UpdateNotification.vue   # Notification discrète
        ├── UpdateDialog.vue         # Dialog détails mise à jour
        └── UpdateProgress.vue       # Barre progression téléchargement
```

## Implementation Phases

### Phase 1: Setup et Configuration Backend

**Objectif**: Installer et configurer le plugin updater côté backend.

**Fichiers modifiés**:
- `src-tauri/Cargo.toml` (ajout plugin)
- `src-tauri/tauri.conf.json` (configuration updater)
- `src-tauri/src/lib.rs` (initialisation plugin)

**Tâches**:
1. Installer le plugin via `pnpm tauri add updater`
2. Générer les clés de signature (`pnpm tauri signer generate`)
3. Configurer `tauri.conf.json` avec endpoints et clé publique
4. Initialiser le plugin dans `lib.rs`
5. Configurer les permissions dans `capabilities/default.json`

**Configuration**:
- Endpoints: URLs pour vérifier les mises à jour (support variables `{{target}}`, `{{arch}}`, `{{current_version}}`)
- Clé publique: Stockée dans `tauri.conf.json`
- Clé privée: Variable d'environnement `TAURI_SIGNING_PRIVATE_KEY`

**Tests**:
- [ ] Plugin installé correctement
- [ ] Configuration valide dans `tauri.conf.json`
- [ ] Plugin initialisé sans erreur au démarrage

### Phase 2: Frontend - Composable et Store

**Objectif**: Créer l'interface frontend pour gérer les mises à jour.

**Fichiers créés**:
- `app/composables/updater/useUpdater.ts`
- `app/stores/updater.ts`

**Tâches**:
1. Créer le composable `useUpdater.ts` avec méthodes:
   - `checkForUpdates()`: Vérifier les mises à jour disponibles
   - `downloadUpdate()`: Télécharger la mise à jour avec progression
   - `installUpdate()`: Installer la mise à jour téléchargée
   - `downloadAndInstall()`: Télécharger et installer en une seule opération
2. Créer le store Pinia `updater.ts` avec:
   - État: `availableUpdate`, `isChecking`, `isDownloading`, `downloadProgress`, `error`
   - Actions: `checkUpdates()`, `downloadUpdate()`, `installUpdate()`
   - Getters: `hasUpdate()`, `canInstall()`

**Tests**:
- [ ] Composable fonctionne correctement
- [ ] Store gère l'état correctement
- [ ] Méthodes invoquent les commands Tauri correctement

### Phase 3: UI - Composants de Mise à Jour

**Objectif**: Créer l'interface utilisateur pour afficher et gérer les mises à jour.

**Fichiers créés**:
- `app/components/updater/UpdateNotification.vue`
- `app/components/updater/UpdateDialog.vue`
- `app/components/updater/UpdateProgress.vue`

**Tâches**:
1. Créer `UpdateNotification.vue`:
   - Notification discrète (toast ou badge) pour nouvelles versions
   - Bouton "Voir les détails" et "Ignorer"
   - Design non intrusif
2. Créer `UpdateDialog.vue`:
   - Afficher version disponible, notes de version
   - Boutons "Mettre à jour maintenant" et "Plus tard"
   - Afficher la progression du téléchargement
3. Créer `UpdateProgress.vue`:
   - Barre de progression pour téléchargement
   - Afficher pourcentage et vitesse
   - Bouton "Annuler" pendant téléchargement
4. Intégrer dans `app/layouts/default.vue` ou `app/components/Header.vue`

**Tests**:
- [ ] Notification s'affiche quand mise à jour disponible
- [ ] Dialog affiche correctement les détails
- [ ] Progression s'affiche pendant téléchargement
- [ ] Intégration dans layout fonctionne

### Phase 4: Configuration Serveur et Build

**Objectif**: Configurer le serveur de mises à jour et générer les artefacts.

**Fichiers modifiés**:
- `src-tauri/tauri.conf.json` (configuration build)

**Tâches**:
1. Configurer `createUpdaterArtifacts: true` dans `tauri.conf.json`
2. Configurer le serveur de mises à jour:
   - Option 1: Fichier JSON statique (GitHub Releases, serveur web)
   - Option 2: API dynamique (serveur custom)
3. Générer les artefacts de mise à jour lors du build:
   - Configurer variable d'environnement `TAURI_SIGNING_PRIVATE_KEY`
   - Exécuter `pnpm tauri build`
   - Vérifier génération des fichiers `.sig` et bundles
4. Déployer les artefacts sur le serveur de mises à jour

**Configuration Serveur**:
- Format JSON avec version, notes, signatures, URLs
- Support multi-plateforme (Windows, Linux)
- Variables d'URL: `{{target}}`, `{{arch}}`, `{{current_version}}`

**Tests**:
- [ ] Artefacts générés correctement lors du build
- [ ] Serveur de mises à jour accessible
- [ ] Format JSON valide et lisible par le plugin

### Phase 5: Intégration et Tests

**Objectif**: Intégrer la vérification automatique et tester le workflow complet.

**Fichiers modifiés**:
- `app/layouts/default.vue` ou composant d'initialisation
- `app/pages/settings.vue` (optionnel: paramètres updater)

**Tâches**:
1. Intégrer vérification automatique au démarrage:
   - Appeler `checkForUpdates()` dans composant d'initialisation
   - Gérer silencieusement les erreurs (pas de connexion, etc.)
   - Afficher notification seulement si mise à jour disponible
2. Implémenter workflow complet:
   - Vérification → Notification → Dialog → Téléchargement → Installation
   - Gérer les cas d'annulation
   - Gérer les erreurs (réseau, serveur, signature)
3. Ajouter paramètres optionnels dans settings:
   - Activer/désactiver vérification automatique
   - Fréquence de vérification (quotidienne, hebdomadaire, manuelle)
4. Tester sur différentes plateformes:
   - Windows: Installation via `.exe`
   - Linux: Installation via `.AppImage`

**Gestion d'erreurs**:
- Pas de connexion: Ignorer silencieusement
- Serveur indisponible: Afficher message d'erreur optionnel
- Signature invalide: Bloquer installation avec message clair
- Téléchargement interrompu: Permettre reprise

**Tests**:
- [ ] Vérification automatique au démarrage fonctionne
- [ ] Workflow complet fonctionne de bout en bout
- [ ] Gestion d'erreurs appropriée
- [ ] Fonctionne sur Windows et Linux

## Migration Checklist

### Backend
- [ ] Phase 1: Setup et Configuration Backend
- [ ] Plugin installé et configuré
- [ ] Clés de signature générées
- [ ] Permissions configurées

### Frontend
- [ ] Phase 2: Composable et Store
- [ ] Phase 3: Composants UI
- [ ] Intégration dans layout

### Configuration
- [ ] Phase 4: Configuration Serveur et Build
- [ ] Serveur de mises à jour configuré
- [ ] Artefacts générés correctement

### Tests
- [ ] Phase 5: Intégration et Tests
- [ ] Workflow complet testé
- [ ] Fonctionne sur toutes les plateformes

## Dependencies

### Backend (Cargo.toml)

```toml
[dependencies]
tauri-plugin-updater = "2.0"  # Nouveau plugin
```

### Frontend (package.json)

```json
{
  "dependencies": {
    "@tauri-apps/plugin-updater": "^2.0.0"  # Nouveau plugin
  }
}
```

## Risk Assessment

### Risques identifiés

1. **Signing des mises à jour**: Complexité de gestion des clés
   - **Mitigation**: Documentation claire, génération guidée, stockage sécurisé
   
2. **Configuration serveur**: Nécessité d'un serveur pour héberger les mises à jour
   - **Mitigation**: Support GitHub Releases (gratuit), documentation pour serveur custom
   
3. **Compatibilité plateformes**: Différences entre Windows et Linux
   - **Mitigation**: Tests sur les deux plateformes, gestion spécifique si nécessaire
   
4. **Expérience utilisateur**: Perturbation avec notifications trop fréquentes
   - **Mitigation**: Notifications discrètes, possibilité de désactiver, vérification intelligente

### Contingency Plan

Si l'intégration pose problème:
- Désactiver temporairement la vérification automatique
- Garder fonctionnalité manuelle uniquement
- Rollback possible via git

## Success Metrics

- [ ] Plugin installé et configuré correctement
- [ ] Vérification automatique fonctionne au démarrage
- [ ] Téléchargement et installation fonctionnent correctement
- [ ] Interface utilisateur intuitive et non intrusive
- [ ] Gestion d'erreurs robuste
- [ ] Fonctionne sur Windows et Linux
- [ ] Documentation complète pour configuration serveur

## Notes Additionnelles

### Configuration Recommandée

Pour GitHub Releases:
- Utiliser l'API GitHub Releases pour servir les mises à jour
- Format JSON automatique via GitHub API
- URLs: `https://api.github.com/repos/{owner}/{repo}/releases/latest`

Pour Serveur Custom:
- Endpoint API qui retourne JSON selon plateforme
- Format JSON standard Tauri Updater
- Support variables d'URL pour flexibilité

### Variables d'URL Disponibles

- `{{target}}`: Plateforme cible (windows, linux, macos)
- `{{arch}}`: Architecture (x86_64, arm64, etc.)
- `{{current_version}}`: Version actuelle de l'application

### Exemple Configuration

```json
{
  "plugins": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://releases.ludolingo.app/{{target}}/{{arch}}/{{current_version}}",
        "https://api.github.com/repos/ludolingo/app/releases/latest"
      ],
      "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25pbmcgY2hhaW4gdGVzdA..."
    }
  },
  "bundle": {
    "createUpdaterArtifacts": true
  }
}
```

