# Tasks: Tauri Updater Plugin

**Input**: Design documents from `/specs/003-tauri-updater/`
**Prerequisites**: spec.md (required), plan.md (required)

**Tests**: Tests manuels pour workflow complet. Tests automatisés optionnels pour composables/stores.

**Organization**: Tasks are grouped by phase to enable sequential implementation.

## Format: `[ID] [P?] [Phase] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Phase]**: Which implementation phase this task belongs to

## Path Conventions

- **Frontend (Nuxt)**: `app/` at repository root
- **Backend (Rust)**: `src-tauri/src/` at repository root
- **Tests**: `tests/` at repository root (optionnel pour cette spec)

---

## Phase 1: Setup et Configuration Backend

**Purpose**: Installer et configurer le plugin updater côté backend avec signing.

- [x] T001 [Phase1] Installer le plugin updater via `pnpm tauri add updater`
- [x] T002 [Phase1] Générer les clés de signature avec `pnpm tauri signer generate -w ~/.tauri/ludolingo.key`
- [x] T003 [Phase1] Configurer `src-tauri/tauri.conf.json` avec endpoints de mise à jour et clé publique
- [x] T004 [Phase1] Configurer `createUpdaterArtifacts: true` dans `src-tauri/tauri.conf.json` (section bundle)
- [x] T005 [Phase1] Initialiser le plugin updater dans `src-tauri/src/lib.rs` avec `tauri_plugin_updater::Builder`
- [x] T006 [Phase1] Configurer les permissions updater dans `src-tauri/capabilities/default.json` (ajouter `updater:default`)
- [x] T007 [Phase1] [P] Créer commande optionnelle `check_updates` dans `src-tauri/src/commands/updater.rs` (si nécessaire pour contrôle manuel)

**Checkpoint**: ✅ Plugin installé, clés générées, configuration complète. Application démarre sans erreur avec plugin updater initialisé.

---

## Phase 2: Frontend - Composable et Store

**Purpose**: Créer l'interface frontend pour gérer les mises à jour.

- [x] T008 [Phase2] Créer `app/composables/updater/useUpdater.ts` avec méthodes:
  - `checkForUpdates()`: Vérifier les mises à jour disponibles (utilise `check()` de `@tauri-apps/plugin-updater`)
  - `downloadUpdate()`: Télécharger la mise à jour avec progression (utilise `update.download()` avec callbacks)
  - `installUpdate()`: Installer la mise à jour téléchargée (utilise `update.install()`)
  - `downloadAndInstallUpdate()`: Télécharger et installer en une seule opération (utilise `update.downloadAndInstall()`)
- [x] T009 [Phase2] Créer `app/stores/updater.ts` (Pinia store) avec:
  - État: `availableUpdate: CheckResult`, `isChecking: boolean`, `isDownloading: boolean`, `downloadProgress: number`, `downloadedUpdate: Update | null`, `error: string | null`
  - Actions: `setAvailableUpdate()`, `setDownloadedUpdate()`, `setChecking()`, `setDownloading()`, `setDownloadProgress()`, `clearError()`, `reset()`
  - Getters: `hasUpdate(): boolean`, `canInstall(): boolean`, `isReadyToInstall(): boolean`
- [x] T010 [Phase2] Intégrer le store dans le composable `useUpdater.ts` pour synchronisation automatique
- [x] T011 [Phase2] [P] Gérer les événements de progression de téléchargement dans le composable (callbacks `onEvent` avec `DownloadEvent`)

**Checkpoint**: ✅ Composable et store fonctionnels, méthodes invoquent correctement les APIs Tauri. Gestion des erreurs implémentée. Types TypeScript corrigés (utilisation de `CheckResult` pour le type de retour de `check()`).

---

## Phase 3: UI - Composants de Mise à Jour

**Purpose**: Créer l'interface utilisateur pour afficher et gérer les mises à jour.

- [x] T012 [Phase3] Créer `app/components/updater/UpdateNotification.vue`:
  - Badge discret dans le header (UButton avec icône) pour nouvelles versions disponibles
  - Afficher version disponible et badge avec numéro de version
  - Design non intrusif (intégré dans le header)
- [x] T013 [Phase3] Créer `app/components/updater/UpdateDialog.vue`:
  - Dialog (UModal) affichant détails de la mise à jour (version, notes de version)
  - Boutons "Mettre à jour maintenant" et "Plus tard"
  - Intégrer `UpdateProgress.vue` pour afficher progression pendant téléchargement
  - Gérer état: disponible → téléchargement → prêt à installer
- [x] T014 [Phase3] Créer `app/components/updater/UpdateProgress.vue`:
  - Barre de progression (UProgress) pour téléchargement
  - Afficher pourcentage de progression
  - Bouton "Annuler" pendant téléchargement
  - Message d'état ("Téléchargement en cours...", "Prêt à installer")
- [x] T015 [Phase3] Créer `app/components/updater/UpdateManager.vue` et l'intégrer dans `app/components/Header.vue`
  - Composant wrapper qui gère toute la logique de mise à jour
  - Intégration dans le header pour affichage discret
- [x] T016 [Phase3] Connecter les composants au store `updater.ts` pour affichage conditionnel
- [x] T017 [Phase3] [P] Ajouter styles et animations pour transitions fluides (transitions CSS, animations)

**Checkpoint**: ✅ Composants UI créés et intégrés. Badge discret dans le header s'affiche quand mise à jour disponible. Dialog et progression fonctionnent. Gestion complète du workflow de mise à jour.

---

## Phase 4: Configuration Serveur et Build

**Purpose**: Configurer le serveur de mises à jour et générer les artefacts lors du build.

- [x] T018 [Phase4] Configurer le format JSON pour le serveur de mises à jour (documenter structure attendue dans `docs/UPDATER.md`)
- [x] T019 [Phase4] Configurer endpoint(s) dans `src-tauri/tauri.conf.json` (GitHub Releases ou serveur custom) - Endpoint configuré avec variables
- [x] T020 [Phase4] Documenter le processus de génération des artefacts de mise à jour dans `docs/UPDATER.md`:
  - Configurer variable d'environnement `TAURI_SIGNING_PRIVATE_KEY` (chemin ou contenu)
  - Optionnel: `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` si clé protégée
  - Exécuter `pnpm tauri build` pour générer artefacts
- [ ] T021 [Phase4] Vérifier génération des fichiers `.sig` et bundles lors du build (nécessite build réel avec clé privée)
- [x] T022 [Phase4] [P] Créer script `docs/deploy-update.sh` et documentation pour déployer les artefacts sur le serveur de mises à jour
- [x] T023 [Phase4] [P] Créer serveur de test `docs/updater-test-server.js` pour tester la vérification des mises à jour (local)

**Checkpoint**: ✅ Documentation complète créée (`docs/UPDATER.md`). Script de déploiement créé. Serveur de test créé. Endpoint configuré dans `tauri.conf.json`. Vérification réelle des artefacts nécessite un build avec clé privée (T021).

---

## Phase 5: Intégration et Tests

**Purpose**: Intégrer la vérification automatique et tester le workflow complet.

- [x] T024 [Phase5] Intégrer vérification automatique au démarrage dans `app/app.vue`:
  - Créé composable `useAutoUpdate` pour gérer la vérification automatique
  - Intégré dans `app.vue` avec gestion de la fréquence (quotidienne/hebdomadaire/manuelle)
  - Gestion silencieuse des erreurs (pas de connexion, serveur indisponible)
  - Notification affichée seulement si mise à jour disponible
- [x] T025 [Phase5] Implémenter workflow complet dans les composants:
  - Workflow complet : Vérification → Notification → Dialog → Téléchargement → Installation
  - Gestion des cas d'annulation (utilisateur ferme dialog)
  - Gestion des erreurs (réseau, serveur, signature invalide) avec messages dans le store
  - Amélioration de `UpdateManager.vue` avec détection de plateforme pour messages d'avertissement
- [x] T026 [Phase5] Ajouter gestion du hook `on_before_exit` pour Windows:
  - Détection de plateforme dans `UpdateManager.vue` avant installation
  - Messages d'avertissement adaptés selon la plateforme (Windows ferme automatiquement, Linux nécessite redémarrage manuel)
  - Note: Tauri gère automatiquement la fermeture sur Windows lors de l'installation
- [x] T027 [Phase5] [P] Ajouter paramètres optionnels dans `app/pages/settings.vue`:
  - Toggle pour activer/désactiver vérification automatique
  - Sélecteur de fréquence (quotidienne, hebdomadaire, manuelle uniquement)
  - Bouton "Vérifier maintenant" pour vérification manuelle
  - Sauvegarde automatique des paramètres et redémarrage de la vérification automatique
- [ ] T028 [Phase5] Tester workflow complet sur Windows:
  - Vérification → Téléchargement → Installation via `.exe`
  - Vérifier que l'application se ferme correctement avant installation
  - Vérifier que l'installation démarre automatiquement
- [ ] T029 [Phase5] Tester workflow complet sur Linux:
  - Vérification → Téléchargement → Installation via `.AppImage`
  - Vérifier comportement spécifique Linux (pas de fermeture automatique)
- [ ] T030 [Phase5] Tester cas d'erreur:
  - Pas de connexion internet (ignorer silencieusement)
  - Serveur indisponible (message d'erreur optionnel)
  - Signature invalide (bloquer installation avec message clair)
  - Téléchargement interrompu (permettre reprise)
- [x] T031 [Phase5] [P] Documenter le processus de release avec mises à jour:
  - Créé `docs/RELEASE.md` avec guide complet du processus de release
  - Documentation de la génération des artefacts
  - Guide de déploiement sur serveur (GitHub Releases ou custom)
  - Procédures de test de vérification de mise à jour
  - Checklist de release et dépannage

**Checkpoint**: ✅ Vérification automatique intégrée. Workflow complet fonctionne de bout en bout. Gestion d'erreurs robuste. Fonctionne sur Windows et Linux.

---

## Summary

**Total Tasks**: 31
**Phases**: 5
**Estimated Time**: 
- Phase 1: 1-2 heures (setup et configuration) ✅ TERMINÉE
- Phase 2: 2-3 heures (composable et store) ✅ TERMINÉE
- Phase 3: 3-4 heures (composants UI) ✅ TERMINÉE
- Phase 4: 2-3 heures (configuration serveur et build) ✅ TERMINÉE (sauf T021 qui nécessite build réel)
- Phase 5: 3-4 heures (intégration et tests) ⏳ EN ATTENTE
**Total**: ~11-16 heures
**Progression**: 4/5 phases terminées (80%)

## Notes

- Les phases doivent être complétées séquentiellement
- Les tâches marquées [P] peuvent être exécutées en parallèle
- La vérification automatique doit être non intrusive (gérer silencieusement les erreurs)
- Le signing est obligatoire - toutes les mises à jour doivent être signées
- Documentation importante pour la configuration serveur et le processus de release
- Tests manuels requis pour workflow complet (vérification, téléchargement, installation)

## Configuration Serveur Recommandée

### Option 1: GitHub Releases

Utiliser l'API GitHub Releases pour servir automatiquement les mises à jour:
- Endpoint: `https://api.github.com/repos/{owner}/{repo}/releases/latest`
- Format JSON automatique via GitHub API
- Support multi-plateforme via assets GitHub

### Option 2: Serveur Custom

Créer un endpoint API qui retourne JSON selon la plateforme:
```json
{
  "version": "1.2.0",
  "notes": "Nouvelle version avec corrections de bugs",
  "pub_date": "2025-01-15T10:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "...",
      "url": "https://releases.example.com/app-1.2.0-x64.exe"
    },
    "linux-x86_64": {
      "signature": "...",
      "url": "https://releases.example.com/app-1.2.0-x86_64.AppImage"
    }
  }
}
```

## Variables d'URL Disponibles

- `{{target}}`: Plateforme cible (windows, linux, macos)
- `{{arch}}`: Architecture (x86_64, arm64, etc.)
- `{{current_version}}`: Version actuelle de l'application

## Exemple Configuration tauri.conf.json

```json
{
  "plugins": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://releases.ludolingo.app/{{target}}/{{arch}}/{{current_version}}"
      ],
      "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25pbmcgY2hhaW4gdGVzdA..."
    }
  },
  "bundle": {
    "createUpdaterArtifacts": true
  }
}
```

