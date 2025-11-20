# Implementation Plan: Intégration Automatique des Outils WolfRPG

## Vue d'Ensemble

Cette spécification décrit l'intégration automatique des outils externes UberWolf et WolfTL dans LudoLingo. L'objectif est de permettre aux utilisateurs de travailler avec n'importe quel projet WolfRPG (chiffré ou natif) de manière entièrement transparente - l'application gère automatiquement tous les outils externes en backend tandis que l'utilisateur voit seulement l'interface de traduction familière.

## Architecture Cible

### Backend (Rust/Tauri) - Logique Métier Complète

#### Commands Tauri Principales
- `detect_wolfrpg_project_type`: Détecte automatiquement le type de projet
- `validate_wolfrpg_tools`: Valide la disponibilité des outils externes
- `setup_wine_environment`: Installe Wine sur Linux
- `process_wolfrpg_project`: **Workflow unifié automatique** - gère UberWolf → WolfTL
- `inject_wolfrpg_translations`: Injection finale avec WolfTL

#### Intégration dans le Workflow Existants
- Extension des commands de scanning pour détecter les projets WolfRPG
- Modification des parsers pour gérer automatiquement les données extraites
- Intégration transparente dans l'extraction/injection existante

### Frontend (Vue.js/Nuxt) - Interface Simplifiée

#### Nouveaux Composables
- `useWolfRpgTools`: Validation des outils et configuration Wine

#### Extension des Components Existants
- `ProjectScanner`: Détection automatique des projets WolfRPG
- `TranslationControls`: Bouton "Exporter vers WolfRPG" pour l'injection finale

#### Store Minimal
```typescript
interface WolfRpgState {
  toolsValidated: boolean
  wineInstalled: boolean  // Linux uniquement
}
```

### Workflow Automatique

#### Processus Transparent
1. **Sélection Projet** → Détection automatique du type WolfRPG
2. **Traitement Backend** → UberWolf (si chiffré) → WolfTL (toujours) → JSON
3. **Interface Standard** → Traduction normale avec les textes extraits
4. **Export Final** → WolfTL injecte automatiquement les traductions

## Stratégie d'Implémentation

### Phase 1: Backend Core Automatique (2 semaines)

**Objectif**: Implémenter le cœur du système automatique de traitement WolfRPG.

**Tâches**:
1. **Plugins Tauri** : Ajouter `tauri-plugin-shell` et `tauri-plugin-os` dans Cargo.toml et configurer
2. **Détection OS** : Implémenter `detect_os_platform()` et logique Windows/Linux
3. **Gestion Wine** : `check_wine_installed()`, dialog de confirmation utilisateur, installation automatique
4. Créer `commands/wolfrpg_tools.rs` avec les commands de base
5. Implémenter `detect_wolfrpg_project_type` pour la classification automatique
6. Développer `validate_wolfrpg_tools` avec tests des outils externes
7. Implémenter `process_wolfrpg_project` avec logique conditionnelle UberWolf → WolfTL

**Délivrables**:
- Détection automatique du système d'exploitation (Windows/Linux)
- Gestion Wine avec confirmation utilisateur sur Linux
- Commands Rust complets pour le workflow automatique
- Gestion transparente de Wine sur Linux
- Tests unitaires et d'intégration backend
- Documentation des APIs

**Risques**: Gestion des erreurs cross-platform, sécurité de l'exécution externe.

### Phase 2: Interface Simplifiée (1 semaine)

**Objectif**: Créer l'interface minimale pour la configuration et l'intégration.

**Tâches**:
1. Créer le composable `useWolfRpgTools` pour la validation et gestion Wine
2. Créer `WineInstallationDialog.vue` pour confirmation utilisateur
3. Étendre `ProjectScanner` pour la détection automatique WolfRPG
4. Implémenter le store Pinia `wolfrpg.ts` avec gestion Wine
5. Créer le component de configuration `WolfRpgToolsConfig`
6. Intégrer la détection dans le workflow de sélection de projet

**Délivrables**:
- Interface de configuration fonctionnelle
- Dialog de confirmation Wine avec UX appropriée
- Détection automatique transparente
- Store d'état avec gestion des messages backend
- Intégration dans le scanner existant

**Risques**: Cohérence avec l'UI existante, régressions dans le workflow principal.

### Phase 3: Injection Finale (1 semaine)

**Objectif**: Implémenter l'export final automatique vers les binaires WolfRPG.

**Tâches**:
1. Développer `inject_wolfrpg_translations` avec WolfTL
2. Étendre `TranslationControls` avec bouton "Exporter vers WolfRPG"
3. Implémenter la validation de l'injection réussie
4. Gérer les erreurs et récupération automatique
5. Optimiser les performances de l'injection

**Délivrables**:
- Injection automatique fonctionnelle
- Bouton d'export intégré dans l'interface existante
- Gestion d'erreurs appropriée
- Tests d'injection

**Risques**: Compatibilité des fichiers modifiés, gestion des gros projets.

### Phase 4: Tests et Finalisation (1 semaine)

**Objectif**: Tests complets et documentation pour la production.

**Tâches**:
1. Tests cross-platform (Windows/Linux) avec projets réels
2. Tests de performance avec différents types de projets
3. Tests d'intégration end-to-end du workflow complet
4. Documentation utilisateur pour l'installation et l'usage
5. Guides de dépannage pour les erreurs communes

**Délivrables**:
- Suite de tests complète (>80% coverage)
- Documentation utilisateur et développeur
- Guides de résolution des problèmes
- Validation finale du workflow

**Risques**: Découverte de bugs tardifs, scénarios non testés.

## Dépendances et Prérequis

### Dépendances Techniques
- `tauri-plugin-shell = "2"` pour l'exécution des outils externes
- Wine ≥ 7.0 sur Linux pour exécuter les outils Windows
- UberWolf et WolfTL dans le dossier Tools/

### Dépendances Fonctionnelles
- Architecture de scanning existante (Phase 1 de LudoLingo)
- Système de traduction existant (Phase 2 de LudoLingo)
- Interface de gestion de projet existante

### Ressources Requises
- Environnements de test Windows et Linux
- Projets WolfRPG de test (chiffrés et extraits)
- Accès aux outils UberWolf et WolfTL

## Critères de Qualité

### Performance
- Détection de projet < 2 secondes
- Validation des outils < 5 secondes
- Extraction proportionnelle à la taille du projet (< 10 min pour 1GB)
- Injection < temps d'extraction

### Fiabilité
- Taux de succès > 95% pour les projets valides
- Gestion d'erreur appropriée pour tous les cas d'échec
- Récupération automatique des erreurs temporaires

### Sécurité
- Exécution limitée aux outils connus
- Validation stricte des chemins et permissions
- Pas d'exécution de code arbitraire

### Utilisabilité
- Workflow transparent pour l'utilisateur final
- Messages d'erreur clairs et actionnables
- Progression visible pour toutes les opérations longues

## Mesures de Succès

### Métriques Quantitatives
- **Temps de traitement**: Extraction/injection < 10 minutes pour projets moyens
- **Taux de succès**: > 95% des projets valides traités correctement
- **Satisfaction utilisateur**: > 4/5 dans les retours utilisateurs

### Métriques Qualitatives
- Transparence du workflow (aucune connaissance technique requise)
- Gestion d'erreur intuitive
- Performance prévisible et stable

## Plan de Rollback

En cas de problème majeur, possibilité de rollback par :
1. Désactivation du feature flag `wolfrpg_tools_enabled`
2. Suppression des nouveaux components (garder compatibilité backward)
3. Revert des modifications dans les workflows existants

## Migration des Utilisateurs Existants

### Utilisateurs avec projets extraits
- Aucun changement requis, continue à fonctionner normalement
- Nouveaux boutons disponibles mais optionnels

### Utilisateurs avec projets chiffrés
- Découverte automatique de la fonctionnalité
- Guidage pas-à-pas pour installer les outils si nécessaire
- Workflow transparent une fois configuré

## Tests d'Acceptation

### Test Scénario 1: Workflow Automatique Complet (Linux)
1. Utilisateur sélectionne un projet WolfRPG avec fichiers .wolf
2. Application détecte automatiquement le type et traite le projet en backend
3. Wine est installé automatiquement si nécessaire
4. UberWolf déchiffre puis WolfTL extrait vers dump/
5. L'utilisateur voit directement l'interface de traduction avec les textes
6. Après traduction, export final injecte automatiquement avec WolfTL

### Test Scénario 2: Projet Déjà Extrait (Windows)
1. Utilisateur sélectionne un projet avec dossier dump/ existant
2. Application détecte et passe directement au workflow de traduction
3. Aucune opération backend supplémentaire nécessaire
4. Export final fonctionne normalement avec WolfTL

### Test Scénario 3: Gestion d'Erreurs Transparente
1. Outil manquant → Erreur détectée automatiquement avec instructions claires
2. Wine manquant → Installation proposée et exécutée automatiquement
3. Traitement échoue → Message détaillé avec solutions proposées
4. Fichiers corrompus → Validation automatique et rapport d'erreurs

### Test Scénario 4: Performance et Transparence
1. Opérations longues affichent un indicateur de progression
2. Interface utilisateur reste réactive pendant les traitements backend
3. Utilisateur peut continuer à travailler normalement
4. Résultats transparents - pas de connaissance technique requise

## Documentation Finale

- **Guide utilisateur**: Installation et utilisation des outils WolfRPG
- **Guide développeur**: Architecture et extension des fonctionnalités
- **Guide dépannage**: Résolution des problèmes courants
- **API Reference**: Documentation des commands Tauri ajoutés
