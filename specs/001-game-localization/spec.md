# Feature Specification: LudoLingo Game Localization Core

**Feature Branch**: `001-game-localization`
**Created**: 2025-11-06
**Status**: Draft
**Input**: User description: "Créer une application desktop LudoLingo pour la localisation de jeux vidéo avec une base de données structurée. L'application doit permettre aux utilisateurs de : 1. Scanner automatiquement des dossiers pour détecter des jeux compatibles (RPG Maker, WolfRPG, Baki, etc.) 2. Extraire automatiquement les textes traduisibles des fichiers de jeu SANS LES MODIFIER et LES STOCKER IMMÉDIATEMENT dans une base de données SQLite locale avec deux tables principales : - **translation** : stocke les textes extraits avec leur contexte, langue source, et traductions - **glossary** : contient les termes techniques et noms propres avec leurs traductions standardisées 3. Gérer les textes extraits dans la base de données avec organisation par projets de traduction 4. Traduire les textes par lots (1-100 éléments simultanément) via Ollama avec mise à jour automatique des tables translation et glossary 5. Réinjecter automatiquement les traductions depuis la base de données vers les fichiers originaux 6. FOURNIR DES FONCTIONNALITÉS D'EXTRACTION/INJECTION DIRECTE depuis l'interface : - Extraire des termes depuis translation vers glossary pour standardisation - Injecter des traductions depuis glossary vers translation pour cohérence - Éditer manuellement les entrées glossary et translation 7. Fournir une interface utilisateur intuitive pour gérer les projets, suivre la progression des traductions, et administrer le glossary Fonctionnalités principales : - Base de données SQLite avec tables translation et glossary - Extraction sécurisée avec stockage immédiat en base de données locale - Gestion complète des projets de traduction avec liaison translation ↔ glossary - Interface de traduction par lots avec suivi du progrès - Support multi-formats avec parsers spécialisés par moteur de jeu - Système de sauvegarde automatique et restauration depuis la base de données - Outils d'administration glossary pour maintenir la cohérence des traductions L'application doit être entièrement offline-first, avec TOUTES les données (textes extraits, traductions, métadonnées, glossary) stockées localement dans SQLite."

**🎯 Version 1.0 Scope**: Cette spécification se concentre sur le support RPG Maker MV/MZ. Le support pour WolfRPG et Baki sera ajouté dans les versions futures.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Extraction Automatique des Textes (Priority: P1)

Localisateur professionnel scanne un dossier contenant des jeux RPG Maker (MV/MZ compatibles) et extrait automatiquement tous les textes traduisibles sans modifier les fichiers originaux.

**Why this priority**: C'est la fonctionnalité de base qui permet d'importer du contenu dans l'application - sans extraction, rien d'autre ne peut fonctionner.

**Independent Test**: Peut être testé indépendamment en vérifiant qu'un dossier de jeu scanné produit des entrées dans la base de données translation avec les textes extraits intacts.

**Acceptance Scenarios**:

1. **Given** un dossier contenant un jeu compatible, **When** l'utilisateur lance le scan, **Then** tous les textes traduisibles sont extraits et stockés dans la table translation
2. **Given** un fichier de jeu avec textes corrompus, **When** le scan est lancé, **Then** le système signale l'erreur sans planter et continue avec les autres fichiers
3. **Given** un jeu déjà scanné, **When** l'utilisateur relance le scan, **Then** les nouveaux textes sont ajoutés sans dupliquer les existants

---

### User Story 2 - Gestion Base de Données et Projets (Priority: P1)

Localisateur organise ses projets de traduction en créant des projets dans l'application et gère les textes extraits dans les tables translation et glossary.

**Why this priority**: La gestion des données est essentielle pour organiser le travail de localisation - sans organisation, les traductions deviennent ingérables.

**Independent Test**: Peut être testé en créant un projet, ajoutant des entrées translation, créant des entrées glossary, et vérifiant que les données sont correctement organisées et liées.

**Acceptance Scenarios**:

1. **Given** des textes extraits, **When** l'utilisateur crée un nouveau projet, **Then** les textes sont associés au projet et organisés par catégories
2. **Given** une entrée translation avec un terme technique, **When** l'utilisateur l'extrait vers le glossary, **Then** le terme apparaît dans le glossary avec sa traduction
3. **Given** une entrée glossary standardisée, **When** l'utilisateur l'injecte dans une translation, **Then** la traduction est automatiquement appliquée

---

### User Story 3 - Traduction par Lots via Ollama (Priority: P1)

Localisateur sélectionne un lot de textes (1-100) et lance une traduction automatique via Ollama qui met à jour automatiquement les tables translation et glossary.

**Why this priority**: La traduction automatique est le cœur de la productivité - elle permet de traiter rapidement de gros volumes de texte.

**Independent Test**: Peut être testé en sélectionnant des textes non traduits, lançant la traduction par lots, et vérifiant que les traductions apparaissent dans la base de données.

**Acceptance Scenarios**:

1. **Given** 50 textes non traduits, **When** l'utilisateur lance la traduction par lots, **Then** tous les textes sont traduits et stockés dans la table translation
2. **Given** un texte avec des termes du glossary, **When** la traduction est lancée, **Then** les termes du glossary sont utilisés de manière cohérente
3. **Given** une traduction en cours, **When** l'utilisateur annule l'opération, **Then** les traductions déjà complétées sont sauvegardées et les autres restent non traduites

---

### User Story 4 - Réinjection des Traductions (Priority: P2)

Localisateur ayant terminé ses traductions peut automatiquement réinjecter les textes traduits dans les fichiers de jeu originaux.

**Why this priority**: La réinjection permet de finaliser le processus de localisation - c'est l'étape qui rend le jeu jouable dans la nouvelle langue.

**Independent Test**: Peut être testé en prenant des fichiers originaux, appliquant des traductions depuis la base de données, et vérifiant que les fichiers modifiés contiennent les bonnes traductions.

**Acceptance Scenarios**:

1. **Given** des fichiers originaux et leurs traductions en base, **When** l'utilisateur lance la réinjection, **Then** les fichiers sont modifiés avec les traductions correctes
2. **Given** une traduction partielle (certains textes non traduits), **When** la réinjection est lancée, **Then** seuls les textes traduits sont modifiés, les autres restent inchangés
3. **Given** des fichiers déjà modifiés, **When** l'utilisateur relance la réinjection, **Then** les modifications sont mises à jour sans corruption

---

### User Story 5 - Administration Glossary (Priority: P2)

Localisateur gère manuellement le glossary pour maintenir la cohérence des traductions et peut extraire/injecter des termes entre translation et glossary.

**Why this priority**: Un glossary bien maintenu est crucial pour la qualité des traductions - il assure la cohérence terminologique dans tous les projets.

**Independent Test**: Peut être testé en ajoutant des termes au glossary, les éditant, et vérifiant qu'ils sont correctement utilisés lors des traductions automatiques.

**Acceptance Scenarios**:

1. **Given** une entrée translation avec un terme récurrent, **When** l'utilisateur l'extrait vers le glossary, **Then** le terme est ajouté avec son contexte et traduction
2. **Given** un terme dans le glossary, **When** l'utilisateur l'injecte dans des translations similaires, **Then** toutes les occurrences sont mises à jour automatiquement
3. **Given** une traduction dans le glossary, **When** l'utilisateur l'édite manuellement, **Then** les changements sont sauvegardés et peuvent être propagés aux translations liées

---

### User Story 6 - Interface Utilisateur Complète (Priority: P3)

Localisateur utilise une interface intuitive pour gérer ses projets, suivre la progression des traductions, et administrer le glossary.

**Why this priority**: Une bonne interface utilisateur rend l'outil accessible et efficace - c'est ce qui fait la différence entre un outil professionnel et amateur.

**Independent Test**: Peut être testé en naviguant dans l'interface pour créer un projet, voir la progression, gérer le glossary, et vérifier que toutes les fonctionnalités sont accessibles.

**Acceptance Scenarios**:

1. **Given** l'application ouverte, **When** l'utilisateur navigue vers la section projets, **Then** il voit tous ses projets avec leur statut de progression
2. **Given** un projet en cours, **When** l'utilisateur consulte la progression, **Then** il voit le pourcentage de textes traduits et le statut par catégories
3. **Given** la section glossary, **When** l'utilisateur recherche un terme, **Then** il trouve rapidement les entrées avec leurs traductions et contexte

---

### User Story 7 - Système de Donations avec Stripe (Priority: P3)

Utilisateur peut faire des donations ponctuelles via Payment Links Stripe pour supporter le développement de l'outil.

**Why this priority**: Les donations permettent de soutenir le développement continu tout en gardant l'outil gratuit et accessible à tous.

**Independent Test**: Peut être testé en initiant une donation, étant redirigé vers Stripe, et vérifiant que le don est enregistré avec un message de remerciement.

**Acceptance Scenarios**:

1. **Given** un utilisateur souhaite faire une donation, **When** il sélectionne un montant, **Then** il est redirigé vers une page de paiement Stripe hébergée
2. **Given** une donation réussie sur Stripe, **When** l'utilisateur revient à l'application, **Then** il voit un message de remerciement sans fonctionnalité premium débloquée
3. **Given** une donation échouée, **When** l'utilisateur revient à l'application, **Then** il voit un message d'erreur informatif

### Edge Cases

**Scénarios critiques pour développement solo :**

1. **Fichiers corrompus** : JSON malformé, encodage incorrect → Gestion d'erreur gracieuse sans crash
2. **Données vides/massives** : Arrays vides, textes de 10k+ caractères, 1000+ entrées → Performance maintenue
3. **Ollama indisponible** : Timeout, connexion perdue, modèle non disponible → Messages d'erreur clairs
4. **Espace disque insuffisant** : Pendant sauvegarde/réinjection → Avertissement préventif
5. **Opérations simultanées** : Scan + traduction en parallèle → Pas de corruption de données
6. **Formats inconnus** : Fichiers non-RPG Maker → Message informatif, pas de crash

**Critères d'échec simples pour tests :**
- Application crash = ÉCHEC
- Données perdues = ÉCHEC
- Message d'erreur incompréhensible = ÉCHEC
- Performance >5 secondes pour opérations normales = ÉCHEC
- Interface bloquée >10 secondes = ÉCHEC

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST scan folders automatically to detect compatible games (RPG Maker MV/MZ, WolfRPG, Baki, etc.)
- **FR-002**: System MUST extract translatable texts from game files WITHOUT modifying them and store them immediately in local SQLite database
- **FR-003**: System MUST maintain two main tables: translation (texts with context, source language, translations) and glossary (technical terms and proper names with standardized translations)
- **FR-004**: System MUST organize extracted texts by translation projects in the database
- **FR-005**: System MUST translate texts in batches (1-100 items simultaneously) via Ollama with automatic updates to translation and glossary tables (Ollama is a mandatory prerequisite installed by user; IA translates all extracted texts including duplicates in single pass)
- **FR-006**: System MUST automatically reinject translations from database back to original game files
- **FR-007**: System MUST provide direct extraction/injection capabilities between translation and glossary tables through interface
- **FR-008**: System MUST allow manual editing of glossary and translation entries
- **FR-009**: System MUST provide intuitive user interface for project management, translation progress tracking, and glossary administration
- **FR-010**: System MUST be fully offline-first with all data stored locally in SQLite (no external data transmission, no security measures needed for sensitive data)
- **FR-011**: System MUST support multiple game formats with specialized parsers per game engine (RPG Maker MV/MZ differentiated - WolfRPG and Baki support planned for future versions)
- **FR-012**: System MUST provide automatic backup system and restoration from database
- **FR-013**: System MUST provide structured logging for operations tracking and progress monitoring
- **FR-014**: System MUST support Stripe Payment Links for user donations
- **FR-015**: System MUST integrate donation flow with external Stripe-hosted checkout pages

### Key Entities *(include if feature involves data)*

- **Project**: Represents a translation project containing multiple game files and their associated translations
- **GameFile**: Represents a scanned game file with its metadata (path, format, size, last modified)
- **TranslationEntry**: Text extracted from game files with source text, context, target translation, and status
- **GlossaryEntry**: Standardized term or proper name with source term, translation, and usage context
- **TranslationBatch**: Group of translation entries processed together via Ollama

## Clarifications

### Session 2025-11-06

- Q: Quelle stratégie d'unicité appliquer aux entrées de base de données ? → A: Tous les textes extraits sont stockés en base même s'ils se répètent (pas de déduplication)
- Q: Quelle stratégie appliquer quand Ollama n'est pas disponible pendant une traduction par lots ? → A: Prérequis obligatoire - Ollama doit être installé et configuré par l'utilisateur (pas de fallback pour le moment)
- Q: Quelle stratégie appliquer pour résoudre les conflits quand le même terme a plusieurs traductions différentes ? → A: Pas de conflits - l'IA traduit tous les textes extraits (même les doublons) en une seule passe
- Q: Quelles mesures de sécurité appliquer pour protéger les données sensibles dans les fichiers de jeu ? → A: Aucune sécurité nécessaire - tout est local (base de données + fournisseur IA) selon principe offline-first
- Q: Quelle stratégie d'observabilité appliquer pour le logging et monitoring de l'application ? → A: Structured logging with progress tracking - logging structuré pour les opérations importantes avec suivi de progression en temps réel

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can scan a game folder and see extracted texts in under 30 seconds for games up to 100MB
- **SC-002**: System successfully extracts 95% of translatable texts from supported game formats without data loss
- **SC-003**: Users can translate batches of 50 texts in under 5 minutes with 80% translation quality acceptance
- **SC-004**: Users can complete full localization workflow (scan → translate → reinject) for a 50MB game in under 10 minutes
- **SC-005**: System maintains 100% data integrity during extraction, translation, and reinjection processes
- **SC-006**: Users can manage glossary of 1000+ terms with sub-second search and update response times
- **SC-007**: 90% of users successfully complete their first game localization project within 15 minutes of interface familiarization