# Feature Specification: Tauri Updater Plugin

**Feature Branch**: `003-tauri-updater`
**Created**: 2025-01-XX
**Status**: Draft
**Input**: Intégration du plugin Tauri Updater pour permettre les mises à jour automatiques de l'application

**🎯 Objectif**: Intégrer le système de mise à jour automatique de Tauri pour permettre aux utilisateurs de recevoir et installer les nouvelles versions de LudoLingo sans intervention manuelle.

## Contexte

LudoLingo est une application desktop qui nécessite des mises à jour régulières pour corriger des bugs, ajouter des fonctionnalités et améliorer les performances. Actuellement, les utilisateurs doivent télécharger et installer manuellement les nouvelles versions. Cette fonctionnalité permettra :

1. **Vérification automatique** des mises à jour disponibles
2. **Téléchargement** des nouvelles versions en arrière-plan
3. **Installation** guidée avec possibilité de reporter
4. **Notifications** pour informer l'utilisateur des nouvelles versions

## User Scenarios & Testing

### User Story 1 - Vérification Automatique des Mises à Jour (Priority: P1)

L'application vérifie automatiquement les mises à jour disponibles au démarrage ou à la demande.

**Why this priority**: Fonctionnalité de base pour le système de mise à jour.

**Independent Test**: Peut être testé en lançant l'application et vérifiant qu'une requête est envoyée au serveur de mises à jour.

**Acceptance Scenarios**:

1. **Given** l'application démarre, **When** une connexion internet est disponible, **Then** l'application vérifie automatiquement les mises à jour disponibles
2. **Given** une nouvelle version est disponible, **When** l'utilisateur démarre l'application, **Then** une notification discrète informe l'utilisateur de la disponibilité
3. **Given** aucune mise à jour n'est disponible, **When** l'application vérifie, **Then** aucune notification n'est affichée
4. **Given** aucune connexion internet, **When** l'application vérifie les mises à jour, **Then** l'erreur est gérée silencieusement sans perturber l'utilisateur

---

### User Story 2 - Téléchargement et Installation des Mises à Jour (Priority: P1)

L'utilisateur peut télécharger et installer les mises à jour disponibles via une interface simple.

**Why this priority**: Fonctionnalité essentielle pour compléter le système de mise à jour.

**Independent Test**: Peut être testé en déclenchant une mise à jour et vérifiant le téléchargement et l'installation.

**Acceptance Scenarios**:

1. **Given** une mise à jour est disponible, **When** l'utilisateur clique sur "Mettre à jour", **Then** le téléchargement commence avec une barre de progression
2. **Given** le téléchargement est en cours, **When** l'utilisateur continue à utiliser l'application, **Then** le téléchargement se poursuit en arrière-plan
3. **Given** le téléchargement est terminé, **When** l'utilisateur confirme l'installation, **Then** l'application se ferme et l'installateur démarre
4. **Given** l'utilisateur annule l'installation, **When** il relance l'application, **Then** la mise à jour reste disponible pour installation ultérieure
5. **Given** l'installation échoue, **When** l'utilisateur relance l'application, **Then** un message d'erreur clair est affiché avec possibilité de réessayer

---

### User Story 3 - Configuration Serveur de Mises à Jour (Priority: P2)

L'application peut être configurée pour utiliser un serveur de mises à jour statique (JSON) ou dynamique.

**Why this priority**: Flexibilité pour différents environnements de déploiement (GitHub Releases, serveur custom, etc.).

**Independent Test**: Peut être testé en configurant différents endpoints et vérifiant que les mises à jour sont récupérées correctement.

**Acceptance Scenarios**:

1. **Given** un fichier JSON statique est configuré, **When** l'application vérifie les mises à jour, **Then** elle récupère les informations depuis le fichier JSON
2. **Given** un serveur dynamique est configuré, **When** l'application vérifie les mises à jour, **Then** elle interroge l'API du serveur avec les paramètres appropriés
3. **Given** plusieurs endpoints sont configurés, **When** l'application vérifie les mises à jour, **Then** elle essaie chaque endpoint jusqu'à obtenir une réponse valide

---

### User Story 4 - Gestion des Versions et Downgrades (Priority: P3)

L'application gère correctement les versions et peut permettre les downgrades si nécessaire.

**Why this priority**: Cas avancés pour gestion flexible des versions.

**Independent Test**: Peut être testé en configurant différentes versions et vérifiant le comportement.

**Acceptance Scenarios**:

1. **Given** une version plus récente est disponible, **When** l'application vérifie, **Then** elle propose la mise à jour
2. **Given** le downgrade est autorisé, **When** une version plus ancienne est disponible, **Then** l'application permet l'installation de cette version
3. **Given** le downgrade est désactivé, **When** une version plus ancienne est disponible, **Then** l'application ignore cette version

---

## Technical Requirements

### Backend Architecture

#### Plugin Tauri Updater

- **Installation**: Ajout du plugin via `tauri add updater`
- **Configuration**: Configuration dans `tauri.conf.json` avec endpoints et clé publique
- **Signing**: Génération de clés de signature (publique/privée) pour vérifier l'intégrité des mises à jour
- **Commands**: Commandes Rust pour vérifier, télécharger et installer les mises à jour

#### Structure Backend

```rust
src-tauri/src/
├── commands/
│   └── updater.rs          # Commands pour gérer les mises à jour
└── lib.rs                  # Initialisation du plugin updater
```

#### Configuration Tauri

```json
{
  "plugins": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://releases.example.com/{{target}}/{{arch}}/{{current_version}}"
      ],
      "pubkey": "YOUR_PUBLIC_KEY_HERE"
    }
  }
}
```

### Frontend Architecture

#### Composables

- **`app/composables/updater/useUpdater.ts`**: Composable principal pour gérer les mises à jour
  - `checkForUpdates()`: Vérifier les mises à jour disponibles
  - `downloadUpdate()`: Télécharger la mise à jour
  - `installUpdate()`: Installer la mise à jour téléchargée
  - `downloadAndInstall()`: Télécharger et installer en une seule opération

#### Components

- **`app/components/updater/UpdateNotification.vue`**: Notification discrète pour nouvelles versions
- **`app/components/updater/UpdateDialog.vue`**: Dialog pour afficher les détails de la mise à jour
- **`app/components/updater/UpdateProgress.vue`**: Barre de progression pour le téléchargement
- **`app/components/settings/UpdaterSettings.vue`**: Configuration des paramètres de mise à jour (optionnel)

#### Stores

- **`app/stores/updater.ts`**: Store Pinia pour gérer l'état des mises à jour
  - État: `availableUpdate`, `isChecking`, `isDownloading`, `downloadProgress`
  - Actions: `checkUpdates()`, `downloadUpdate()`, `installUpdate()`

### Dependencies

**`Cargo.toml`**
```toml
[dependencies]
tauri-plugin-updater = "2.0"  # Nouveau plugin
```

**`package.json`**
```json
{
  "dependencies": {
    "@tauri-apps/plugin-updater": "^2.0.0"  # Nouveau plugin
  }
}
```

### Configuration Serveur

#### Option 1: Fichier JSON Statique

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

#### Option 2: Serveur Dynamique

Endpoint API qui retourne les informations de mise à jour selon la plateforme et la version actuelle.

### Signing des Mises à Jour

1. **Génération des clés**: `pnpm tauri signer generate -w ~/.tauri/ludolingo.key`
2. **Clé publique**: Stockée dans `tauri.conf.json` (peut être partagée)
3. **Clé privée**: Stockée de manière sécurisée, utilisée uniquement lors du build
4. **Variables d'environnement**: 
   - `TAURI_SIGNING_PRIVATE_KEY`: Chemin ou contenu de la clé privée
   - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: Mot de passe optionnel

## Implementation Plan

### Phase 1: Setup et Configuration Backend

1. Installer le plugin Tauri Updater (`pnpm tauri add updater`)
2. Générer les clés de signature
3. Configurer `tauri.conf.json` avec endpoints et clé publique
4. Initialiser le plugin dans `lib.rs`
5. Créer les commands Rust pour gérer les mises à jour

### Phase 2: Frontend - Composable et Store

1. Créer le composable `useUpdater.ts`
2. Créer le store Pinia `updater.ts`
3. Implémenter la logique de vérification des mises à jour
4. Implémenter la logique de téléchargement avec progression

### Phase 3: UI - Composants de Mise à Jour

1. Créer `UpdateNotification.vue` pour notifications discrètes
2. Créer `UpdateDialog.vue` pour afficher les détails
3. Créer `UpdateProgress.vue` pour la barre de progression
4. Intégrer les composants dans le layout principal

### Phase 4: Configuration Serveur

1. Configurer le serveur de mises à jour (JSON statique ou API dynamique)
2. Générer les artefacts de mise à jour lors du build (`createUpdaterArtifacts: true`)
3. Tester la vérification des mises à jour avec différentes configurations

### Phase 5: Intégration et Tests

1. Intégrer la vérification automatique au démarrage
2. Tester le workflow complet: vérification → téléchargement → installation
3. Gérer les cas d'erreur (pas de connexion, serveur indisponible, etc.)
4. Tester sur différentes plateformes (Windows, Linux)

## Migration Strategy

### Backward Compatibility

- Les utilisateurs existants continueront à fonctionner normalement
- La première vérification de mise à jour se fera au prochain démarrage
- Pas de migration de données nécessaire

### Breaking Changes

- Aucun breaking change pour les utilisateurs existants
- Nouvelle dépendance backend (plugin updater)
- Nouvelle dépendance frontend (`@tauri-apps/plugin-updater`)

## Security Considerations

### Signing des Mises à Jour

- **Obligatoire**: Toutes les mises à jour doivent être signées
- **Clé privée**: Jamais partagée, stockée de manière sécurisée
- **Clé publique**: Incluse dans l'application pour vérification
- **Vérification**: Chaque mise à jour téléchargée est vérifiée avant installation

### Permissions

- Permissions Tauri requises pour le plugin updater:
  - `updater:allow-check`
  - `updater:allow-download`
  - `updater:allow-install`
  - `updater:allow-download-and-install`

## Success Criteria

- [ ] Le plugin updater est installé et configuré correctement
- [ ] Les clés de signature sont générées et configurées
- [ ] La vérification automatique fonctionne au démarrage
- [ ] Le téléchargement des mises à jour fonctionne avec progression
- [ ] L'installation des mises à jour fonctionne correctement
- [ ] Les notifications sont affichées de manière appropriée
- [ ] Les erreurs sont gérées proprement sans perturber l'utilisateur
- [ ] Le système fonctionne sur Windows et Linux
- [ ] La documentation est complète pour la configuration serveur

