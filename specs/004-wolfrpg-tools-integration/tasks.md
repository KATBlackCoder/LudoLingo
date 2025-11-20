# Task Breakdown: Intégration Automatique des Outils WolfRPG

## Vue d'Ensemble des Tâches

Cette spécification couvre l'intégration automatique et transparente des outils UberWolf et WolfTL dans LudoLingo. Le focus est sur un backend robuste qui gère tout automatiquement tandis que l'interface utilisateur reste simple et familière. Les tâches sont organisées par phase d'implémentation avec estimations réalistes.

---

## Phase 1: Backend Core Automatique (2 semaines - 25h)

### Tâche 1.1: Configuration Plugins Tauri
**ID**: `wolfrpg-core-1`
**Priorité**: P1
**Estimation**: 3h
**Assigné**: Backend Developer
**Dépendances**: Aucune

**Description**:
Ajouter et configurer `tauri-plugin-shell` et `tauri-plugin-os` pour l'exécution des outils externes et la détection OS.

**Étapes**:
1. Ajouter `tauri-plugin-shell = "2"` et `tauri-plugin-os = "2"` dans Cargo.toml
2. Configurer les plugins dans `tauri.conf.json`
3. Initialiser les plugins dans `lib.rs`
4. Vérifier les permissions de sécurité (shell:open, os:default)

**Critères d'acceptation**:
- ✅ Les deux plugins installés et configurés correctement
- ✅ Permissions shell:open et os:default activées
- ✅ Build réussi sans erreurs de compilation

### Tâche 1.2: Détection du Système d'Exploitation
**ID**: `wolfrpg-core-2`
**Priorité**: P1
**Estimation**: 2h
**Assigné**: Backend Developer
**Dépendances**: `wolfrpg-core-1`

**Description**:
Implémenter `detect_os_platform()` pour déterminer automatiquement Windows ou Linux.

**Étapes**:
1. Utiliser `tauri_plugin_os::platform()` pour détecter l'OS
2. Retourner "windows" ou "linux" (ou autres si nécessaire)
3. Gérer les erreurs de détection

**Critères d'acceptation**:
- ✅ Détection correcte de Windows et Linux
- ✅ Gestion d'erreurs appropriée pour OS non supportés
- ✅ Tests sur les deux plateformes cibles

### Tâche 1.3: Gestion Wine avec Confirmation Utilisateur
**ID**: `wolfrpg-core-3`
**Priorité**: P1
**Estimation**: 4h
**Assigné**: Backend Developer
**Dépendances**: `wolfrpg-core-2`

**Description**:
Implémenter la logique de vérification Wine et demande de confirmation utilisateur sur Linux.

**Étapes**:
1. `check_wine_installed()` : Vérifier présence de Wine via `which wine`
2. Si Wine absent : `request_wine_installation_permission()` → message au frontend
3. Attendre réponse utilisateur via channel/événement
4. Si approuvé : `setup_wine_environment()` pour installation
5. Gestion des erreurs et messages détaillés

**Critères d'acceptation**:
- ✅ Vérification correcte de la présence de Wine
- ✅ Communication backend→frontend pour confirmation
- ✅ Installation automatique si approuvée
- ✅ Messages d'erreur appropriés si refusée

### Tâche 1.4: Détection de Type de Projet
**ID**: `wolfrpg-core-4`
**Priorité**: P1
**Estimation**: 4h
**Assigné**: Backend Developer
**Dépendances**: `wolfrpg-core-3`

**Description**:
Implémenter `detect_wolfrpg_project_type` pour classifier automatiquement les projets WolfRPG.

**Étapes**:
1. Vérifier l'existence de `Game.exe`
2. Scanner les fichiers chiffrés (.wolf, .data, .pak, .bin, .assets, .content, .res, .resource)
3. Vérifier l'existence et validité du dossier `dump/`
4. Retourner le type approprié (encrypted, native, extracted, invalid)

**Critères d'acceptation**:
- ✅ Détection correcte des projets chiffrés (nécessitent UberWolf)
- ✅ Détection correcte des projets avec dump/ existant
- ✅ Gestion appropriée des projets invalides
- ✅ Performance < 2 secondes pour analyse complète

### Tâche 1.5: Validation des Outils Externes
**ID**: `wolfrpg-core-5`
**Priorité**: P1
**Estimation**: 3h
**Assigné**: Backend Developer
**Dépendances**: `wolfrpg-core-4`

**Description**:
Créer `validate_wolfrpg_tools` pour vérifier la disponibilité et fonctionnalité d'UberWolf et WolfTL.

**Étapes**:
1. Vérifier l'existence des fichiers exécutables
2. Tester l'exécution avec --help ou --version
3. Valider les codes de retour
4. Support Wine sur Linux

**Critères d'acceptation**:
- ✅ Vérification de l'existence d'UberWolfCli.exe et WolfTL.exe
- ✅ Test d'exécution réussi sur Windows et via Wine sur Linux
- ✅ Gestion d'erreurs appropriée pour outils manquants/corrompus

### Tâche 1.6: Workflow Automatique Principal
**ID**: `wolfrpg-core-6`
**Priorité**: P1
**Estimation**: 8h
**Assigné**: Backend Developer
**Dépendances**: `wolfrpg-core-5`

**Description**:
Implémenter `process_wolfrpg_project` avec logique conditionnelle UberWolf → WolfTL et gestion OS.

**Étapes**:
1. Détecter l'OS (Windows/Linux) via plugin OS
2. Sur Linux : Vérifier Wine et demander confirmation si nécessaire
3. Si fichiers chiffrés détectés : Exécuter UberWolf (direct/Windows, via Wine/Linux)
4. Toujours exécuter WolfTL pour extraction vers dump/
5. Gestion des erreurs et récupération automatique

**Critères d'acceptation**:
- ✅ Workflow UberWolf → WolfTL pour projets chiffrés
- ✅ Workflow WolfTL seul pour projets natifs
- ✅ Gestion OS automatique (Windows direct, Linux via Wine)
- ✅ Création correcte du dossier dump/ avec JSON valides
- ✅ Gestion d'erreur complète avec messages détaillés
- ✅ Support cross-platform (Windows/Linux avec Wine)

### Tâche 1.7: Tests Backend Complets
**ID**: `wolfrpg-core-7`
**Priorité**: P1
**Estimation**: 5h
**Assigné**: Backend Developer
**Dépendances**: Toutes les tâches Phase 1

**Description**:
Créer une suite complète de tests unitaires et d'intégration pour le backend.

**Critères d'acceptation**:
- ✅ Tests unitaires pour toutes les fonctions (>80% coverage)
- ✅ Tests d'intégration avec les vrais outils externes
- ✅ Tests cross-platform (Windows/Linux)
- ✅ Tests de performance et de charge

---

## Phase 2: Interface Simplifiée (1 semaine - 12h)

### Tâche 2.1: Composable de Validation et Wine
**ID**: `wolfrpg-ui-1`
**Priorité**: P1
**Estimation**: 4h
**Assigné**: Frontend Developer
**Dépendances**: Phase 1 complète

**Description**:
Créer `useWolfRpgTools` pour la validation des outils et gestion Wine avec confirmation utilisateur.

**Étapes**:
1. `validateTools()` : Validation des outils externes
2. `handleWineInstallation()` : Gérer l'installation Wine avec dialog
3. `showWineInstallationDialog()` : Afficher dialog de confirmation
4. Gestion des événements backend pour demandes de confirmation

**Critères d'acceptation**:
- ✅ Validation des outils externes fonctionnelle
- ✅ Dialog de confirmation Wine avec UX appropriée
- ✅ Gestion des réponses utilisateur (oui/non)
- ✅ Intégration propre avec le store Pinia

### Tâche 2.2: Dialog de Confirmation Wine
**ID**: `wolfrpg-ui-2`
**Priorité**: P1
**Estimation**: 3h
**Assigné**: Frontend Developer
**Dépendances**: `wolfrpg-ui-1`

**Description**:
Créer `WineInstallationDialog.vue` pour demander confirmation utilisateur avant installation Wine.

**Étapes**:
1. Dialog modal avec explication claire du besoin de Wine
2. Boutons "Installer Wine" / "Annuler"
3. Gestion des réponses et communication avec le backend
4. Messages d'information sur Wine et compatibilité

**Critères d'acceptation**:
- ✅ Dialog responsive et accessible
- ✅ Explication claire du besoin de Wine
- ✅ Gestion appropriée des choix utilisateur
- ✅ Intégration avec le workflow d'installation

### Tâche 2.3: Extension du Project Scanner
**ID**: `wolfrpg-ui-3`
**Priorité**: P1
**Estimation**: 4h
**Assigné**: Frontend Developer
**Dépendances**: `wolfrpg-ui-2`

**Description**:
Étendre `ProjectScanner.vue` pour détecter automatiquement les projets WolfRPG.

**Critères d'acceptation**:
- ✅ Détection automatique lors du scan de projet
- ✅ Affichage du type de projet détecté
- ✅ Intégration transparente dans le workflow existant

### Tâche 2.4: Store Pinia avec Gestion Wine
**ID**: `wolfrpg-ui-4`
**Priorité**: P1
**Estimation**: 3h
**Assigné**: Frontend Developer
**Dépendances**: `wolfrpg-ui-3`

**Description**:
Implémenter le store `wolfrpg.ts` pour gérer l'état de validation des outils et Wine.

**Critères d'acceptation**:
- ✅ État synchronisé avec les opérations backend
- ✅ Gestion des demandes de confirmation Wine
- ✅ Persistance appropriée des paramètres
- ✅ Actions pour validation, configuration et Wine

### Tâche 2.5: Component de Configuration
**ID**: `wolfrpg-ui-5`
**Priorité**: P1
**Estimation**: 3h
**Assigné**: Frontend Developer
**Dépendances**: `wolfrpg-ui-4`

**Description**:
Créer `WolfRpgToolsConfig.vue` pour la configuration des chemins d'outils.

**Critères d'acceptation**:
- ✅ Interface intuitive de configuration
- ✅ Validation des chemins d'outils
- ✅ Auto-détection si possible
- ✅ Intégration dans les paramètres

---

## Phase 3: Injection Finale (1 semaine - 10h)

### Tâche 3.1: Command d'Injection
**ID**: `wolfrpg-inject-1`
**Priorité**: P1
**Estimation**: 5h
**Assigné**: Backend Developer
**Dépendances**: Phase 1 complète

**Description**:
Implémenter `inject_wolfrpg_translations` pour injecter les traductions avec WolfTL.

**Critères d'acceptation**:
- ✅ Injection réussie des traductions dans les binaires
- ✅ Validation de l'injection
- ✅ Gestion d'erreurs appropriée
- ✅ Support cross-platform

### Tâche 3.2: Extension Translation Controls
**ID**: `wolfrpg-inject-2`
**Priorité**: P1
**Estimation**: 3h
**Assigné**: Frontend Developer
**Dépendances**: `wolfrpg-inject-1`

**Description**:
Ajouter le bouton "Exporter vers WolfRPG" dans `TranslationControls.vue`.

**Critères d'acceptation**:
- ✅ Bouton contextuel pour les projets WolfRPG
- ✅ Workflow fluide export → injection automatique
- ✅ Confirmation et feedback appropriés

### Tâche 3.3: Tests d'Injection
**ID**: `wolfrpg-inject-3`
**Priorité**: P1
**Estimation**: 2h
**Assigné**: Backend Developer
**Dépendances**: `wolfrpg-inject-1`

**Description**:
Créer des tests pour l'injection et validation des résultats.

**Critères d'acceptation**:
- ✅ Tests d'injection avec fichiers réels
- ✅ Validation de l'intégrité des fichiers modifiés
- ✅ Tests de performance pour gros projets

---

## Phase 4: Tests et Documentation (1 semaine - 12h)

### Tâche 4.1: Tests End-to-End
**ID**: `wolfrpg-tests-1`
**Priorité**: P1
**Estimation**: 4h
**Assigné**: QA Engineer
**Dépendances**: Toutes les phases

**Description**:
Tests complets du workflow automatique sur Windows et Linux.

**Critères d'acceptation**:
- ✅ Tests avec projets chiffrés et natifs
- ✅ Tests cross-platform complets
- ✅ Validation du workflow de bout en bout

### Tâche 4.2: Tests de Performance
**ID**: `wolfrpg-tests-2`
**Priorité**: P1
**Estimation**: 3h
**Assigné**: QA Engineer
**Dépendances**: Toutes les phases

**Description**:
Tests de performance avec différents types et tailles de projets.

**Critères d'acceptation**:
- ✅ Mesure des temps de traitement
- ✅ Tests avec gros projets (>1GB)
- ✅ Validation des performances Wine sur Linux

### Tâche 4.3: Documentation Utilisateur
**ID**: `wolfrpg-docs-1`
**Priorité**: P1
**Estimation**: 3h
**Assigné**: Technical Writer
**Dépendances**: Toutes les phases

**Description**:
Créer la documentation utilisateur pour l'installation et l'usage.

**Critères d'acceptation**:
- ✅ Guide d'installation des outils
- ✅ Instructions d'usage simples
- ✅ Guides de dépannage courants

### Tâche 4.4: Documentation Développeur
**ID**: `wolfrpg-docs-2`
**Priorité**: P1
**Estimation**: 2h
**Assigné**: Technical Writer
**Dépendances**: Toutes les phases

**Description**:
Documentation technique pour les développeurs.

**Critères d'acceptation**:
- ✅ Architecture des commands Tauri
- ✅ Guide d'extension des fonctionnalités
- ✅ API Reference complète

---

## Métriques de Suivi

### Tâches par Phase
- **Phase 1 (Backend)**: 7 tâches (27h) - Core automatique + OS + Wine
- **Phase 2 (Frontend)**: 5 tâches (17h) - Interface simplifiée + Wine dialog
- **Phase 3 (Injection)**: 3 tâches (10h) - Export final
- **Phase 4 (Tests/Docs)**: 4 tâches (12h) - Finalisation

**Total estimé**: 66 heures de développement (+7h pour OS/Wine vs approche initiale simplifiée)

### Points de Contrôle
- **Fin Phase 1**: Backend complet avec tests (>80% coverage)
- **Fin Phase 2**: Interface fonctionnelle, workflow transparent
- **Fin Phase 3**: Injection finale opérationnelle
- **Fin Phase 4**: Produit prêt pour la production

### Critères de Qualité
- **Code Coverage**: > 80% pour le nouveau code backend
- **Performance**: Traitement automatique < 10 minutes pour projets moyens
- **Fiabilité**: > 95% de succès pour les projets valides
- **Transparence**: Workflow invisible pour l'utilisateur final

### Risques et Mitigations
- **Risque**: Outils externes non fiables → **Mitigation**: Validation stricte et tests complets
- **Risque**: Performance Wine sur Linux → **Mitigation**: Tests de performance et optimisations
- **Risque**: Complexité d'intégration → **Mitigation**: Développement incrémental avec tests réguliers
- **Risque**: Sécurité de l'exécution externe → **Mitigation**: Sandboxing Tauri et validation stricte
