# Spécification 004: Intégration des Outils WolfRPG

## Vue d'Ensemble

Cette spécification décrit l'intégration **automatique et transparente** des outils externes **UberWolf** et **WolfTL** dans LudoLingo. Le focus est sur un backend robuste qui gère tout automatiquement tandis que l'utilisateur voit seulement l'interface de traduction familière.

## Problème

Actuellement, LudoLingo ne peut travailler qu'avec des projets WolfRPG déjà déchiffrés (contenant un dossier `dump/` avec des fichiers JSON). Cependant, la plupart des projets WolfRPG distribués contiennent des données chiffrées dans des formats propriétaires (.wolf, .data, .pak, etc.) qui nécessitent des outils spécialisés pour être extraites.

## Solution

Un **workflow entièrement automatique** qui cache la complexité technique derrière une interface simple :

1. **Sélection du projet** → LudoLingo détecte automatiquement le type WolfRPG
2. **Traitement automatique** → Backend gère UberWolf + WolfTL de manière transparente
3. **Traduction normale** → Interface existante avec les textes extraits
4. **Export final** → Injection automatique des traductions dans les binaires

**L'utilisateur ne voit que l'interface de traduction familière - tout le reste est automatique !**

## Workflow Technique Automatique

### Backend (Transparent pour l'utilisateur)
```
Sélection Projet → Détection Type → UberWolf (si chiffré) → WolfTL (toujours) → JSON Prêts
```

### Interface Utilisateur (Comme d'habitude)
```
JSON Chargés → Traduction Normale → Export → Injection Automatique → Jeu Final
```

**Le workflow technique est entièrement géré par le backend - l'utilisateur ne voit que la traduction !**

## Structure de la Spécification

```
specs/004-wolfrpg-tools-integration/
├── spec.md              # Spécification complète avec user stories
├── plan.md              # Plan d'implémentation détaillé
├── tasks.md             # Décomposition des tâches
├── contracts/           # Définition des APIs
│   ├── wolfrpg-tools-commands.ts    # Commands Tauri
│   ├── wolfrpg-frontend-types.ts    # Types frontend
│   └── external-tools-api.ts        # APIs outils externes
└── README.md           # Ce fichier
```

## User Stories Clés

### 1. Workflow Automatique Transparent
- **Objectif**: L'utilisateur sélectionne n'importe quel projet WolfRPG et voit directement l'interface de traduction
- **Critères**: Aucune connaissance technique requise, processus entièrement automatique

### 2. Gestion Automatique des Outils
- **Objectif**: Backend gère UberWolf et WolfTL automatiquement selon les besoins
- **Critères**: Détection automatique des outils requis, installation Wine transparente

### 3. Compatibilité Cross-Platform Transparente
- **Objectif**: Même expérience utilisateur sur Windows et Linux
- **Critères**: Wine installé/configuré automatiquement, exécution native sur Windows

### 4. Injection Finale Automatique
- **Objectif**: Export final injecte automatiquement les traductions dans les binaires
- **Critères**: Bouton d'export simple, injection sécurisée et validée

## Technologies Utilisées

### Backend (Rust/Tauri) - Cœur du Système
- `tauri-plugin-shell`: Exécution sécurisée des outils externes
- Commands automatiques pour le workflow complet
- Gestion Wine transparente sur Linux

### Frontend (Vue.js/Nuxt) - Interface Simplifiée
- Extension minimale des components existants
- Store Pinia léger pour la configuration
- Interface utilisateur inchangée pour la traduction

### Outils Externes - Gestion Automatique
- **UberWolf**: Déchiffrement conditionnel des données chiffrées
- **WolfTL**: Extraction systématique et injection des traductions
- **Wine**: Installation/configuration automatique sur Linux

## Critères de Succès

- ✅ **Détection**: 100% de précision sur les projets valides
- ✅ **Fiabilité**: > 95% de succès pour les opérations valides
- ✅ **Performance**: < 10 minutes pour projets moyens
- ✅ **UX**: Workflow transparent sans connaissance technique
- ✅ **Sécurité**: Exécution sandboxée, validation stricte

## Dépendances

### Fonctionnelles
- Plugin shell Tauri configuré
- Outils UberWolf et WolfTL disponibles
- Wine installé sur Linux (installation automatique proposée)

### Techniques
- Architecture scanning existante
- Système de traduction LudoLingo
- Interface de gestion de projet

## Estimation

- **Total**: ~59 heures de développement (vs 67h initial)
- **Backend**: ~35 heures (inchangé - cœur du système)
- **Frontend**: ~12 heures (réduit - interface simplifiée)
- **Tests**: ~12 heures (inchangé)

## Phases d'Implémentation Simplifiées

1. **Phase 1** (2 semaines): Backend automatique complet
2. **Phase 2** (1 semaine): Interface simplifiée
3. **Phase 3** (1 semaine): Injection finale
4. **Phase 4** (1 semaine): Tests et documentation

**Réduction de 8 heures grâce à l'élimination des dialogs complexes et stores lourds !**

## Risques et Mitigations

### Risques Identifiés
- **Outils externes non fiables**: Validation stricte et tests complets
- **Performance Wine**: Optimisations et alternatives si nécessaire
- **Complexité d'intégration**: Développement incrémental
- **Sécurité**: Sandboxing Tauri et restrictions strictes

### Stratégie de Rollback
- Feature flag `wolfrpg_tools_enabled` pour désactivation
- Commandes isolées pour suppression facile
- Compatibilité backward maintenue

## Documentation Associée

- [Spécification complète](./spec.md)
- [Plan d'implémentation](./plan.md)
- [Décomposition des tâches](./tasks.md)
- [APIs et contrats](./contracts/)

## Prochaines Étapes

1. Revue et validation de la spécification
2. Approbation du plan d'implémentation
3. Attribution des tâches et début du développement
4. Tests réguliers et validation itérative
