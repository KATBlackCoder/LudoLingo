<!--
Sync Impact Report - speckit.constitution 2025-11-06
Version change: none → 1.0.0 (new constitution)
Added sections: Core Principles (5 principles), Stack Technique Détaillée, Contraintes et Standards, Workflow de Développement
Amendment 2025-11-10: Changed from "batch simultaneous" to "sequential" translation approach for realistic Ollama constraints
Templates requiring updates: ✅ .specify/templates/plan-template.md (constitution check updated), ✅ .specify/templates/tasks-template.md (TDD adapted for solo development)
Follow-up TODOs: None - all templates synchronized
-->
# Constitution de LudoLingo

## Core Principles

### I. Extraction et Sécurité des Données

Toute extraction de données de jeu doit être non-destructive et sécurisée. Les fichiers originaux ne sont jamais modifiés sans sauvegarde préalable. Toutes les données restent locales à l'appareil.

### II. Architecture Tauri + Nuxt Simplifiée

Séparation claire : interface utilisateur en Nuxt avec Nuxt UI côté frontend, logique métier en Rust avec plugins Tauri spécialisés côté backend.

### III. Base de Données SQLite via Plugin Tauri

Persistance exclusivement locale avec le plugin tauri-plugin-sql. Interface unifiée pour toutes les opérations de base de données.

### IV. Test-Driven Development

Tests backend obligatoires pour les fonctionnalités critiques (développement solo). Tests unitaires et d'intégration pour logique métier complexe uniquement.

### V. Performance et Traitement Séquentiel

Traitement séquentiel des traductions (un texte à la fois) pour garantir la stabilité et respecter les contraintes matérielles d'Ollama. Support de sessions de traduction longues avec contrôle pause/reprise.

## Stack Technique Détaillée

### Frontend Nuxt/Vue (Windows & Linux uniquement)

**UI et Composants:**

- `@nuxt/ui` : bibliothèque UI complète et moderne (déjà présent)

- `pinia` : gestion d'état (stores setup uniquement)

**Intégration Tauri:**

- `@tauri-apps/api` : API Tauri pour communiquer avec le backend Rust

**Internationalization:**

- **Nuxt UI Native i18n** : Support multilingue intégré (50+ langues)
- **Gestion automatique** : Locales, drapeaux, sélecteur de langue
- **Messages personnalisés** : Système `useMessages` pour textes d'application

### Backend Rust/Tauri (Windows & Linux uniquement)

**Plugins Tauri Officiels:**

- `tauri-plugin-dialog` : dialogues de fichiers et alertes

- `tauri-plugin-filesystem` : accès au système de fichiers

- `tauri-plugin-notification` : notifications système

- `tauri-plugin-opener` : ouverture des liens de paiement Stripe dans le navigateur

- `tauri-plugin-os-info` : informations sur le système d'exploitation

- `tauri-plugin-sql` : base de données SQLite intégrée

- `tauri-plugin-store` : stockage persistant clé-valeur

- `tauri-plugin-updater` : mises à jour automatiques

- `tauri-plugin-window-state` : gestion de l'état des fenêtres

**IA et Traitement:**

- `ollama-rs` : client Rust pour Ollama (traductions IA locales)

- `lazy-regex` : expressions régulières optimisées pour parsing

**Paiements Stripe:**

- `async-stripe` : client Rust officiel pour l'API Stripe (création de Payment Links)

**Utilitaires de Base (présents):**

- `serde` + `serde_json` : sérialisation

- `log` + `tauri-plugin-log` : logging

- `tauri` : framework principal

## Contraintes et Standards

### Plateformes

- **Support exclusif** : Windows et Linux uniquement

- **Pas de macOS** : réduction de la complexité de développement

### Performance de Traduction

- **Sessions séquentielles** : traitement un texte à la fois pour stabilité optimale

- **Contrôle utilisateur** : pause/reprise/arrêt des sessions de traduction

- **Traitement asynchrone** : interface non-bloquante avec progression temps réel

### Intégration Stripe

- **Payment Links** : redirection vers des liens de paiement hébergés par Stripe

- **Workflow** : Création du Payment Link via `async-stripe` → Ouverture via `tauri-plugin-opener`

- **Sécurité** : Clés API stockées de manière sécurisée côté backend uniquement

### Sécurité

- Aucune donnée ne quitte l'appareil sans consentement explicite

- Validation stricte de toutes les entrées utilisateur

- Sanitisation des chemins de fichiers

- Isolation des opérations de fichiers

### Qualité de Code

- TypeScript strict mode obligatoire

- ESLint + Prettier configurés

- Tests backend couvrant fonctionnalités critiques (développement solo)

- Documentation automatique avec TSDoc

## Workflow de Développement

1. Issues GitHub avec spécifications détaillées

2. Revue de code systématique

3. Tests automatiques sur CI/CD (Windows + Linux)

4. Releases automatiques avec changelog

5. Déploiement via GitHub Actions

## Governance

Cette constitution définit la stack technique obligatoire pour LudoLingo. Tout changement nécessite une justification technique documentée et validation collective.

**Version**: 1.0.0 | **Ratified**: 2025-11-06 | **Last Amended**: 2025-11-10
