# Feature Specification: Intégration des Outils WolfRPG

**Feature Branch**: `004-wolfrpg-tools-integration`
**Created**: 2025-11-18
**Status**: Draft
**Input**: Intégration des outils UberWolf et WolfTL pour l'extraction et l'injection des données WolfRPG

**🎯 Objectif**: Permettre à LudoLingo de traiter automatiquement les projets WolfRPG (chiffrés ou natifs) en intégrant UberWolf et WolfTL de manière transparente, avec compatibilité cross-platform via Wine sur Linux.

## Contexte

WolfRPG Editor utilise deux types de formats pour stocker les données de jeu :

1. **Formats chiffrés** (.wolf, .data, .pak, .bin, .assets, .content, .res, .resource) - nécessitent UberWolf pour déchiffrement
2. **Formats natifs** (.dat, .mps) - peuvent être traités directement par WolfTL

Actuellement, LudoLingo ne peut travailler qu'avec des données déjà déchiffrées (dossier dump/ existant). Cette fonctionnalité permettra un **workflow entièrement automatique** :

1. **Détection automatique** du type de projet WolfRPG
2. **Traitement automatique** : UberWolf (si chiffré) → WolfTL (extraction)
3. **Traduction normale** avec l'interface existante
4. **Injection automatique** des traductions avec WolfTL
5. **Compatibilité cross-platform** transparente (Wine sur Linux)

## User Scenarios & Testing

### User Story 1 - Détection Automatique des Projets WolfRPG (Priority: P1)

L'application détecte automatiquement si un projet WolfRPG nécessite l'utilisation d'UberWolf ou si les données sont déjà extraites.

**Why this priority**: Fonctionnalité de base pour déterminer le workflow approprié.

**Independent Test**: Peut être testé en scannant différents types de projets WolfRPG et vérifiant la détection.

**Acceptance Scenarios**:

1. **Given** un projet avec Game.exe et fichiers .wolf/.data/etc., **When** l'utilisateur le sélectionne, **Then** l'application détecte qu'UberWolf est nécessaire
2. **Given** un projet avec Game.exe et dossier dump/ existant, **When** l'utilisateur le sélectionne, **Then** l'application détecte que les données sont déjà extraites
3. **Given** un projet invalide (pas de Game.exe), **When** l'utilisateur le sélectionne, **Then** un message d'erreur clair est affiché
4. **Given** un projet avec données partielles, **When** l'application analyse, **Then** elle propose les actions correctives appropriées

---

### User Story 2 - Traitement Automatique des Projets (Priority: P1)

L'application traite automatiquement les projets WolfRPG lors de leur sélection, gérant UberWolf et WolfTL de manière transparente.

**Why this priority**: Fonctionnalité essentielle pour l'accessibilité des données WolfRPG sans intervention manuelle.

**Independent Test**: Peut être testé avec différents types de projets et vérifier le workflow automatique.

**Acceptance Scenarios**:

1. **Given** un projet avec fichiers chiffrés, **When** l'utilisateur le sélectionne, **Then** l'application exécute automatiquement UberWolf puis WolfTL et présente les textes extraits
2. **Given** un projet avec fichiers natifs, **When** l'utilisateur le sélectionne, **Then** l'application exécute WolfTL directement et présente les textes extraits
3. **Given** Wine n'est pas installé sur Linux, **When** l'utilisateur sélectionne un projet chiffré, **Then** l'application installe Wine automatiquement puis traite le projet
4. **Given** le traitement automatique échoue, **When** une erreur survient, **Then** un message d'erreur détaillé explique le problème et propose des solutions
5. **Given** le traitement réussit, **When** il se termine, **Then** l'utilisateur voit directement l'interface de traduction avec les textes prêts

---

### User Story 3 - Injection Automatique Finale (Priority: P1)

L'application injecte automatiquement les traductions dans les fichiers binaires WolfRPG via WolfTL lors de l'export final.

**Why this priority**: Fonctionnalité essentielle pour finaliser la localisation et produire un jeu jouable.

**Independent Test**: Peut être testé en vérifiant que les fichiers binaires sont correctement modifiés après traduction.

**Acceptance Scenarios**:

1. **Given** les traductions sont terminées, **When** l'utilisateur clique "Exporter vers WolfRPG", **Then** WolfTL injecte automatiquement les traductions dans les fichiers binaires
2. **Given** l'injection automatique échoue, **When** une erreur survient, **Then** l'application affiche les erreurs spécifiques et permet de corriger les problèmes
3. **Given** l'injection réussit, **When** elle se termine, **Then** une confirmation est affichée avec instructions pour tester le jeu localisé
4. **Given** les fichiers originaux sont préservés, **When** l'injection s'exécute, **Then** les modifications sont appliquées de manière sécurisée

---

### User Story 4 - Gestion des Outils Externes (Priority: P2)

L'application gère automatiquement la disponibilité et la validation des outils UberWolf et WolfTL.

**Why this priority**: Assure la fiabilité du système et une bonne expérience utilisateur.

**Independent Test**: Peut être testé en configurant différents états des outils et vérifiant les messages appropriés.

**Acceptance Scenarios**:

1. **Given** les outils sont absents, **When** l'application démarre, **Then** des messages d'avertissement sont affichés
2. **Given** les outils sont présents mais corrompus, **When** l'utilisateur teste la validation, **Then** des erreurs spécifiques sont détectées
3. **Given** les outils sont à jour, **When** l'application vérifie, **Then** une confirmation de fonctionnalité est affichée
4. **Given** Wine est requis sur Linux, **When** il n'est pas installé, **Then** l'application propose l'installation automatique

---

### User Story 5 - Compatibilité Cross-Platform (Priority: P2)

L'application fonctionne correctement sur Windows et Linux en gérant les différences de plateforme.

**Why this priority**: LudoLingo doit fonctionner sur les plateformes cibles des utilisateurs.

**Independent Test**: Peut être testé sur Windows et Linux avec les mêmes projets.

**Acceptance Scenarios**:

1. **Given** plateforme Windows, **When** les outils sont utilisés, **Then** ils s'exécutent nativement sans Wine
2. **Given** plateforme Linux, **When** les outils sont utilisés, **Then** ils s'exécutent via Wine de manière transparente
3. **Given** Wine n'est pas disponible sur Linux, **When** l'utilisateur tente une opération, **Then** des instructions claires sont fournies
4. **Given** les chemins d'outils diffèrent, **When** l'application configure, **Then** elle détecte automatiquement les bons chemins

---

## Technical Requirements

### Backend Architecture

#### Plugin Shell Tauri

- **Installation**: `tauri-plugin-shell = "2"` dans Cargo.toml
- **Configuration**: Plugin activé avec permissions `shell:open`
- **Sécurité**: Exécution limitée aux outils connus et validés
- **Cross-platform**: Gestion automatique de Wine sur Linux

#### Structure Backend

```rust
src-tauri/src/
├── commands/
│   ├── mod.rs
│   └── wolfrpg_tools.rs          # Commands pour UberWolf et WolfTL
└── lib.rs                        # Initialisation du plugin shell
```

#### Commands Rust

```rust
// Détection système d'exploitation
detect_os_platform() -> Result<String, String>

// Gestion Wine (Linux uniquement)
check_wine_installed() -> Result<bool, String>
request_wine_installation_permission() -> Result<bool, String> // Envoie message au frontend
setup_wine_environment() -> Result<String, String>

// Détection de projet
detect_wolfrpg_project_type(game_path: String) -> Result<String, String>

// Gestion des outils
validate_wolfrpg_tools(tools_dir: String) -> Result<(), String>

// Workflow unifié
process_wolfrpg_project(game_path: String, tools_dir: String) -> Result<String, String>

// Injection finale
inject_wolfrpg_translations(game_path: String, tools_dir: String, dump_path: String) -> Result<String, String>
```

### Frontend Architecture

#### Composables

- **`app/composables/wolfrpg-tools.ts`**: Composable pour la gestion WolfRPG
  - `validateTools(toolsDir)`: Valider la disponibilité des outils
  - `handleWineInstallation()`: Gérer l'installation de Wine avec confirmation utilisateur
  - `showWineInstallationDialog()`: Afficher dialog de confirmation Wine

#### Components

- **`app/components/settings/WolfRpgToolsConfig.vue`**: Configuration des chemins d'outils
- **`app/components/wolfrpg/WineInstallationDialog.vue`**: Dialog de confirmation d'installation Wine
- Extension de **`app/components/projects/ProjectScanner.vue`**: Détection automatique des projets WolfRPG

#### Stores

- **`app/stores/wolfrpg.ts`**: Store Pinia pour l'état WolfRPG
  - État: `toolsValidated`, `wineInstalled`, `wineInstallationRequested`
  - Actions: `validateTools()`, `setupWine()`, `handleWineInstallationRequest()`
  - Événements: Gestion des messages backend pour confirmation Wine

### Dependencies

**`Cargo.toml`**
```toml
[dependencies]
tauri-plugin-shell = "2"    # Plugin pour exécuter les outils externes
tauri-plugin-os = "2"       # Plugin pour détecter le système d'exploitation
```

**`tauri.conf.json`**
```json
{
  "plugins": {
    "shell": {
      "open": true
    },
    "os": {
      "default": true
    }
  }
}
```

### Détection Automatique du Système d'Exploitation

#### Plugin OS Info Tauri
- **Utilisation** : `tauri-plugin-os = "2"` pour détecter automatiquement Windows/Linux
- **Logique Cross-Platform** :
  - **Windows** : Exécution directe des `.exe` (UberWolfCli.exe, WolfTL.exe)
  - **Linux** : Vérification de Wine installé, sinon demande de confirmation utilisateur

#### Gestion Wine sur Linux
- **Détection** : Vérification de l'existence de Wine via `which wine`
- **Installation** : Si Wine absent, message de confirmation au frontend
- **Confirmation Utilisateur** : Dialog demandant l'autorisation de télécharger/installer Wine
- **Workflow** : Installation automatique si approuvée, sinon message d'erreur explicite

### Gestion des Outils Externes

#### Structure des Outils
```
src-tauri/Tools/wolfrpg/
├── UberWolfCli.exe    # Déchiffrement des formats chiffrés
├── WolfTL.exe         # Extraction/injection des données
└── README.md          # Documentation des outils
```

#### Validation des Outils
- Vérification de l'existence des fichiers
- Test d'exécution avec `--help` ou `--version`
- Vérification des codes de retour
- Gestion des erreurs spécifiques

#### Wine sur Linux
- Détection automatique de Wine
- Installation automatique si absent
- Gestion transparente dans les commands
- Messages d'erreur appropriés

### Workflow d'Intégration Automatique

#### Phase 1: Sélection du Projet
1. Utilisateur sélectionne un dossier de projet WolfRPG
2. Détection automatique du type (chiffré, natif, ou déjà extrait)
3. Validation des outils externes (UberWolf, WolfTL)

#### Phase 2: Traitement Automatique (Backend Only)
1. **Si fichiers chiffrés présents** : Exécution automatique d'UberWolf pour déchiffrement
2. **Toujours** : Exécution de WolfTL pour extraction vers dossier `dump/`
3. **Gestion Wine** : Installation/configuration automatique sur Linux
4. **Gestion d'erreurs** : Retry automatique et messages détaillés

#### Phase 3: Traduction (Interface Existante)
1. Parsing automatique des JSON du dossier `dump/`
2. Workflow de traduction standard LudoLingo
3. Interface utilisateur inchangée pour l'utilisateur

#### Phase 4: Export Final Automatique
1. Sauvegarde des traductions dans les JSON modifiés
2. Exécution automatique de WolfTL pour injection dans les binaires
3. Validation de l'injection et confirmation finale

## Implementation Plan

### Phase 1: Backend Core (2 semaines)

1. **Plugin Shell Tauri** : Installation et configuration
2. **Commands de Base** : `detect_wolfrpg_project_type`, `validate_wolfrpg_tools`
3. **Workflow Automatique** : `process_wolfrpg_project` avec logique UberWolf → WolfTL
4. **Gestion Wine** : Détection et installation automatique sur Linux
5. **Gestion d'Erreurs** : Messages détaillés et récupération automatique

### Phase 2: Intégration Frontend (1 semaine)

1. **Composable Minimal** : `useWolfRpgTools` pour validation des outils
2. **Extension ProjectScanner** : Détection automatique des projets WolfRPG
3. **Store Pinia** : État minimal pour la validation des outils
4. **Configuration** : Interface de configuration des chemins d'outils

### Phase 3: Injection Finale (1 semaine)

1. **Command Injection** : `inject_wolfrpg_translations` avec WolfTL
2. **Extension TranslationControls** : Bouton "Exporter vers WolfRPG"
3. **Workflow d'Export** : Intégration dans le processus d'export final
4. **Validation** : Vérification de l'injection réussie

### Phase 4: Tests et Documentation (1 semaine)

1. **Tests Cross-Platform** : Windows (natif) et Linux (Wine)
2. **Tests d'Intégration** : Workflow complet avec projets réels
3. **Documentation** : Guide d'installation et dépannage
4. **Performance** : Optimisation et mesures

## Migration Strategy

### Backward Compatibility

- Les projets existants avec dump/ continueront à fonctionner normalement
- Aucun changement pour les projets déjà extraits
- Les nouveaux projets WolfRPG bénéficieront automatiquement des outils

### Breaking Changes

- Nouvelle dépendance: `tauri-plugin-shell`
- Configuration supplémentaire dans `tauri.conf.json`
- Extension du scanner de projets pour la détection WolfRPG automatique

## Security Considerations

### Exécution d'Outils Externes

- **Validation stricte**: Seuls UberWolfCli.exe et WolfTL.exe sont autorisés
- **Chemins contrôlés**: Les outils doivent être dans un dossier dédié et validé
- **Sandboxing**: Utilisation du plugin shell Tauri pour l'isolation
- **Wine sécurisé**: Exécution via Wine sur Linux avec restrictions appropriées

### Permissions Tauri

Permissions requises pour les plugins utilisés:

**Plugin Shell:**
- `shell:open`: Nécessaire pour exécuter les outils externes (UberWolf, WolfTL)

**Plugin OS Info:**
- `os:allow-platform`: Nécessaire pour détecter Windows/Linux
- `os:allow-family`: Utilisé pour identifier la famille d'OS
- `os:default`: Permission par défaut pour les informations OS de base

### Validation des Entrées

- **Chemins absolus**: Tous les chemins sont validés et absolus
- **Existence des fichiers**: Vérification avant exécution
- **Formats supportés**: Limitation aux formats WolfRPG connus
- **Taille des fichiers**: Protection contre les fichiers trop volumineux

## Success Criteria

- [ ] Le plugin OS Info détecte correctement Windows/Linux
- [ ] La gestion Wine fonctionne automatiquement avec confirmation utilisateur
- [ ] Le plugin shell est installé et configuré correctement
- [ ] La détection automatique des projets WolfRPG fonctionne
- [ ] UberWolf s'exécute directement sur Windows, via Wine sur Linux
- [ ] WolfTL extrait et injecte correctement les données JSON/binaires
- [ ] Le workflow automatique est transparent pour l'utilisateur
- [ ] Les erreurs sont gérées avec messages détaillés et solutions
- [ ] Le système fonctionne de manière transparente sur Windows et Linux
- [ ] La documentation inclut les instructions d'installation des outils
- [ ] Les tests couvrent tous les scénarios (chiffré/extrait, plateformes, Wine)
