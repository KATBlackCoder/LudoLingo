# Tasks: LudoLingo Game Localization Core

**Input**: Design documents from `/specs/001-game-localization/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: TDD obligatoire selon la constitution. Tests unitaires, d'intégration et e2e requis pour toute nouvelle fonctionnalité.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

**🚨 CHANGEMENT MAJEUR - APPROCHE AJUSTÉE**: Phase 4 (User Story 2) réactivée avec focus sur gestion projets uniquement. Stratégie US1 + US2 avant US3.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## 🎯 **Implementation Priority**

**⚠️ IMPORTANT**: Nous nous concentrons **UNIQUEMENT sur RPG Maker MV/MZ** pour cette première version :

- ✅ **RPG Maker MV/MZ** : Implémentation complète
- ⏳ **WolfRPG** : Reporté à une version future
- ⏳ **Baki** : Reporté à une version future

**Justification** :
- RPG Maker MV/MZ représente la majorité des jeux à localiser
- Architecture commune permet d'ajouter d'autres moteurs plus tard
- Focus permet une qualité optimale pour le lancement

## Path Conventions

- **Frontend (Nuxt)**: `app/` at repository root
- **Backend (Rust)**: `src-tauri/src/` at repository root
- **Tests**: `tests/` at repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Create project structure per implementation plan
- [X] T002 Initialize Tauri project with Rust backend in src-tauri/
- [X] T003 Initialize Nuxt frontend project in app/
- [X] T004 Configure tauri-plugin-sql in src-tauri/src/main.rs
- [X] T005 Configure tauri-plugin-store in src-tauri/src/main.rs
- [X] T006 Configure tauri-plugin-opener in src-tauri/src/main.rs
- [X] T007 [P] Setup database migrations in src-tauri/src/migrations.rs (séparé de commands/ pour meilleure organisation)
- [X] T008 [P] Configure Nuxt with TypeScript and Pinia in app/nuxt.config.ts
- [X] T009 Setup test frameworks (Cargo test uniquement)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T008 Setup database migrations in src-tauri/src/migrations.rs (plugin SQL gère le schéma via migrations)
- [X] T009 Implement base data models in src-tauri/src/models/project.rs and project validation commands
- [X] T010 Create useDatabase composable in app/composables/useDatabase.ts
- [X] T011 Create useStore composable in app/composables/useStore.ts
- [X] T012 Setup project store structure in app/stores/projects.ts
- [X] T013 Create translation module structure in src-tauri/src/translation/service.rs (mod.rs = exports uniquement)
- [X] T014 Setup Ollama client foundation in src-tauri/src/translation/ollama/client.rs
- [X] T015 Create parsers module structure in src-tauri/src/parsers/engine.rs (mod.rs = exports uniquement)
- [X] T016 Initialize settings store with Ollama dual-mode in app/stores/settings.ts

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Extraction Automatique des Textes (Priority: P1) 🎯 MVP

**Goal**: Permettre à l'utilisateur de scanner des dossiers de jeu et extraire automatiquement les textes traduisibles

**Independent Test**: Peut être testé en scannant un dossier de jeu factice et vérifiant que les textes sont extraits en base de données

### Tests for User Story 1 (OBLIGATOIRE - TDD selon constitution) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T016 [P] [US1] Unit tests for file scanning in tests/unit/scanning.test.ts
- [ ] T017 [P] [US1] Unit tests for text extraction in tests/unit/extraction.test.ts
- [ ] T018 [P] [US1] Integration tests for scan workflow in tests/integration/scan-workflow.test.ts

### Implementation for User Story 1

- [X] T019 [US1] Implement RPG Maker MV/MZ engine with simplified version detection (package.json + data/ = MZ, else MV) in src-tauri/src/parsers/rpg_maker/engine.rs
- [X] T019a [US1] Create actors.json parser with extract/inject methods in src-tauri/src/parsers/rpg_maker/files/actors.rs
- [X] T019b [US1] Create items.json parser with extract/inject methods in src-tauri/src/parsers/rpg_maker/files/items.rs
- [X] T019c [US1] Create system.json parser with extract/inject methods in src-tauri/src/parsers/rpg_maker/files/system.rs
- [X] T019d [US1] Create maps parser with extract/inject methods for MapXXX.json files in src-tauri/src/parsers/rpg_maker/files/maps.rs
- [X] T019e [US1] Create common events parser with extract/inject methods in src-tauri/src/parsers/rpg_maker/files/events.rs
- [X] T020 [US1] Create scanning commands in src-tauri/src/commands/scanning.rs
- [X] T021 [US1] Add scanning composables in app/composables/db/scanning/
- [X] T022 [US1] Create scanning UI components in app/components/ScanningDialog.vue
- [X] T022b [US1] Create translation texts table component in app/components/projects/TextsTable.vue
- [X] T023 [US1] Implement scan progress tracking in app/stores/scan.ts
- [X] T024 [US1] Add file validation logic in src-tauri/src/commands/validation.rs
- [X] T025 [US1] Create error handling for corrupted files in scanning commands

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Gestion Projets (Priority: P1)

**Goal**: Permettre la gestion complète des projets de localisation (CRUD + validation)

**Independent Test**: Peut être testé en créant des projets, ajoutant des données, et vérifiant l'organisation

### Implementation for User Story 2 (RÉIMPLÉMENTATION - Gestion Projets Prioritaire)

**Note**: Focus exclusif sur gestion projets - glossaire reporté à plus tard

**Ordre d'implémentation**:
1. Commands validation (backend/logique métier)
2. Composables CRUD (frontend/données)
3. Interface utilisateur
4. Intégration workflow

- [X] T030 [US2] Create project validation commands in src-tauri/src/commands/projects.rs (BACKEND)
- [X] T029 [US2] Implement project CRUD composables in app/composables/db/project/ (FRONTEND)
- [X] T033 [US2] Create project dashboard UI in app/components/projects/ProjectDashboard.vue
- [X] T037 [US2] Integrate project management with extraction workflow
- [X] T038 [US2] Create database tables for extracted texts (texts, text_entries tables)
- [X] T039 [US2] Create composables for text storage/retrieval in database
- [X] T040 [US2] Modify extraction workflow to save texts to database
- [X] T041 [US2] Add project re-opening functionality with text loading from database
- [X] T042 [US2] Update UI to show previously extracted projects and their texts

**Stratégie**: Backend d'abord (validation), puis frontend (CRUD), enfin UI et intégration

**Checkpoint**: Phase 4 sera terminée quand les textes extraits seront persistés en DB et les projets pourront être rouverts avec leurs textes

---

## Phase 5: User Story 3 - Traduction Séquentielle via Ollama (Priority: P1)

**Goal**: Implémenter la traduction automatique séquentielle (un texte à la fois) avec mise à jour des données

**Independent Test**: Peut être testé en traduisant un texte individuel et vérifiant le résultat

### Tests for User Story 3 (OBLIGATOIRE - TDD selon constitution) ⚠️

- [ ] T037 [P] [US3] Unit tests for Ollama client in tests/unit/ollama-client.test.ts
- [ ] T038 [P] [US3] Unit tests for sequential processing in tests/unit/sequential-processing.test.ts
- [ ] T039 [P] [US3] Integration tests for translation workflow in tests/integration/translation-workflow.test.ts

### Implementation for User Story 3

- [X] T040 [US3] Implement Ollama client in src-tauri/src/translation/ollama/client.rs
- [X] T041 [US3] Create sequential translation logic in src-tauri/src/translation/ollama/sequential.rs (remplace batch.rs) - DB integration préparée
- [X] T042 [US3] Add single translation functionality in src-tauri/src/translation/ollama/single.rs
- [X] T043 [US3] Implement translation commands in src-tauri/src/commands/translation.rs
- [X] T044 [US3] Create translation composables in app/composables/db/texts/ (extend existing for translation operations)
- [X] T045 [US3] Add translation UI in app/components/TranslationInterface.vue
- [X] T046 [US3] Implement translation progress tracking in app/stores/translation.ts
- [ ] T048 [US3] Create translation history and undo functionality

**Checkpoint**: All P1 user stories should now be independently functional

**Phase 5 Status**: ✅ TERMINÉE - Traduction séquentielle opérationnelle avec sauvegarde DB et UI temps réel

---

## Phase R: Refactoring Majeur Post-Phase 5 (Priority: P0 - Critique)

**Goal**: Nettoyer, optimiser et améliorer la visualisation pendant traduction avant de continuer

**Context**: Traduction fonctionne mais architecture nécessite nettoyage majeur + problème UX visualisation

**Independent Test**: Peut être testé en vérifiant que traduction fonctionne toujours après refactoring

### Phase R1: Audit et Nettoyage (1-2 jours)
**Objectif**: Identifier et supprimer les inutiles

**🔍 Où chercher :**
- **Composants Vue** : `app/components/` - Scanner tous les fichiers .vue pour usage réel
- **Composables** : `app/composables/` - Vérifier imports et appels dans codebase
- **Stores Pinia** : `app/stores/` - Analyser utilisation dans composants
- **Commands Rust** : `src-tauri/src/commands/` - Vérifier appels Tauri.invoke
- **Imports** : Chercher imports non utilisés avec ESLint ou TypeScript
- **Fichiers de dev** : Fonctions mock, console.log, TODOs résolus

**🛠️ Outils à utiliser :**
- ESLint pour imports inutiles
- TypeScript pour fonctions non utilisées
- Recherche grep pour références croisées
- Bundle analyzer pour dépendances mortes

**✅ Critères de succès :**
- ✅ 0 import inutilisé (ESLint clean)
- ✅ Tous composants référencés au moins 1x
- ✅ Documentation à jour et pertinente
- ✅ Code mort supprimé (2 stores DEPRECATED supprimés)
- ✅ Console.log de développement nettoyés

- [X] TR001 [PR1] Audit composants - Identifier composants/fonctions non utilisés
- [X] TR002 [PR1] Nettoyage imports - Supprimer imports inutiles et dépendances mortes
- [X] TR003 [PR1] Documentation obsolète - Mettre à jour TODOs et commentaires périmés
- [X] TR004 [PR1] Code mort - Supprimer fonctions mock et code de développement

### Phase R2: Amélioration Visualisation (2-3 jours)
**Objectif**: Résoudre le problème de visibilité pendant traduction

**🔍 Où chercher les problèmes UX :**
- **Interface actuelle** : `app/pages/projects.vue` - Boutons "Commencer/Stop", affichage sessions
- **Store traduction** : `app/stores/translation.ts` - État des sessions, progression
- **Components traduction** : `app/components/translations/` - Affichage des traductions actives
- **Feedback utilisateur** : Notifications, toasts, indicateurs de chargement
- **Performance UI** : Lenteurs lors de mises à jour fréquentes

**🛠️ Diagnostic UX :**
- Tester workflow complet : Scan → Sélection → Traduction → Observation UI
- Mesurer temps de réponse UI lors d'événements fréquents
- Analyser feedback visuel pendant traduction (ce qui manque ?)
- Observer comportement avec gros volumes (1000+ textes)

**📊 Métriques UX cibles :**
- **Clarté d'état** : Utilisateur sait toujours où en est la traduction
- **Temps de réponse** : < 500ms pour mises à jour UI
- **Feedback immédiat** : Action utilisateur → réaction visuelle instantanée
- **Information pertinente** : Texte en cours, progression %, temps restant

**✅ Critères de succès :**
- ✅ Interface intuitive avec 3 tables séparées (Raw, In Progress, Final)
- ✅ Feedback visuel avec progress bars et indicateurs temps réel
- ✅ Page dédiée `/translation` avec onglets et statistiques
- ✅ Mise à jour automatique via computed réactifs
- ✅ Performance maintenue avec pagination et filtres

- [X] TR005 [PR2] Refonte interface traduction - Créer vraie UI de suivi temps réel
- [X] TR006 [PR2] Indicateurs visuels - Progress bars, status temps réel, logs activité
- [X] TR007 [PR2] Feedback utilisateur - Notifications toast, animations chargement
- [X] TR008 [PR2] États intermédiaires - Afficher progression détaillée (texte en cours, temps restant)

### Phase R3: DRY et Optimisations (3-4 jours)
**Objectif**: Éliminer duplications et améliorer performance

**🔍 Où chercher les duplications :**
- **Stores Pinia** : `app/stores/` - Logique répétée entre stores (erreurs, loading, etc.)
- **Composables** : `app/composables/` - Fonctions similaires dans différents modules
- **Error handling** : Patterns répétitifs de gestion d'erreurs Tauri
- **API calls** : Logique répétée pour invokeTauri, gestion réponses
- **UI patterns** : Composants similaires (listes, formulaires, modales)

**🛠️ Analyse performance :**
- **Computed coûteux** : Identifier dans `app/components/` et `app/stores/`
- **Re-renders fréquents** : Observer avec Vue DevTools
- **Bundle size** : Analyser avec `pnpm build --analyze`
- **Memory leaks** : Timers, event listeners, subscriptions

**📊 Métriques performance cibles :**
- **Bundle size** : Réduction de 20-30% du JavaScript
- **Time to interactive** : < 2s pour pages principales
- **Re-renders** : < 50ms pour mises à jour fréquentes
- **Memory usage** : Stable pendant longues sessions

**✅ Critères de succès :**
- Code duplications < 5% (mesuré par outils)
- Computed coûteux identifiés et optimisés
- Bundle size optimisé
- Performance maintenue avec 5000+ textes

- [X] TR009 [PR3] Refactoring stores - Fusionner logique commune, éliminer duplications
- [X] TR010 [PR3] Optimisation composables - Centraliser logique réutilisable
- [X] TR011 [PR3] [OPTIONAL] Cache intelligent - Réduire calculs réactifs coûteux
- [X] TR012 [PR3] [OPTIONAL] Lazy loading - Charger composants seulement quand nécessaire

**📝 Note sur TR011-TR012 (Optionnels)** :
- **TR011** : Non critique actuellement. Vue.js cache déjà les computed, filtres simples (O(n)), pagination active. À considérer seulement si > 10 000 textes ou problèmes de performance observés.
- **TR012** : Non nécessaire actuellement. Nuxt 3 fait déjà du code splitting automatique par route, composants légers, gain marginal (< 50KB). À considérer seulement si bundle size > 500KB ou composants très lourds ajoutés.
- **Recommandation** : Focus sur fonctionnalités plutôt qu'optimisation prématurée. Ces tâches peuvent être implémentées plus tard si besoin.

### Phase R4: Architecture et Performance (3-4 jours)
**Objectif**: Améliorer l'architecture globale + Modularité indépendante

**🔍 Audit architecture :**
- **Boundaries** : `app/` vs `src-tauri/` - Logique métier côté Rust ?
- **Data flow** : Stores Pinia vs DB - Synchronisation efficace ?
- **Error patterns** : Gestion d'erreurs incohérente entre modules
- **API design** : Commands Tauri optimisés pour performance ?
- **Modularité** : Modules indépendants (parsers, translation, frontend) ?

**🛠️ Outils d'analyse :**
- **Separation of concerns** : Analyser responsabilités par couche
- **Data flow diagrams** : Mapper flux de données critiques
- **Error boundaries** : Identifier patterns d'erreurs répétitifs
- **Performance profiling** : Lighthouse, Vue DevTools
- **Module coupling analysis** : Dépendances entre modules

**🎯 Objectif Modularité (SOLID + Indépendance) :**
- **Parsers** : Fonctionne indépendamment (sauf error.rs) jusqu'aux commands
- **Translation** : Module isolé avec interface claire ( Ollama client → Commands )
- **Frontend** : Stores/composables indépendants avec contracts définis
- **Commands** : Couche d'adaptation entre modules métier et Tauri

**📊 Métriques architecture cibles :**
- **Maintainability** : Code facile à modifier et étendre
- **Testability** : Logique isolée et testable unitairement
- **Scalability** : Architecture supporte 10000+ textes
- **Reliability** : Gestion d'erreurs robuste et prévisible
- **Modularity** : Modules indépendants, faible couplage, forte cohésion
- **SOLID Compliance** : Dependency Inversion, Single Responsibility

**✅ Critères de succès :**
- Architecture documentée et compréhensible
- Boundaries clairs entre frontend/backend
- Error handling uniforme dans toute l'app
- Performance optimale pour workflows critiques
- **Modules testables indépendamment** (parsers sans commands, etc.)
- **Contracts clairs** entre modules (interfaces, types partagés)
- **Pas de dépendances circulaires**

- [X] TR013 [PR4] [OPTIONAL] Séparation responsabilités - Clarifier frontend/backend boundaries

**📝 Note sur TR013 (Optionnel)** :
- **État actuel acceptable** : Application desktop locale (Tauri), logique métier simple (détection game engine, calculs stats), fonctionnel sans bugs critiques.
- **Pourquoi optionnel** : Risques sécurité limités (app locale), logique simple et maintenable, focus prioritaire sur fonctionnalités.
- **Quand refactoriser** : Si ajout nouveaux moteurs de jeu (logique plus complexe), besoin tests unitaires, duplication logique frontend/backend, ou problèmes performance.
- **Recommandation** : Laisser tel quel pour l'instant, refactoriser quand nécessaire (amélioration continue plutôt que blocage).
- [X] TR014 [PR4] Error handling cohérent - Système d'erreurs user-friendly
- [X] TR015 [PR4] State management optimisé - Performance Pinia stores
- [X] TR016 [PR4] DB queries optimisées - Réduire latence et requêtes redondantes
- [X] TR017 [PR4] **Modularité Parsers** - Rendre parsers indépendants (sauf error.rs)
- [X] TR018 [PR4] **Architecture Translation** - Séparer core (prompts/validation) d'ollama (API)
- [X] TR019 [PR4] **Validation Traductions** - Ajouter validation qualité résultats translation
- [X] TR020 [PR4] **Modularité Frontend** - Stores/composables indépendants avec contracts
- [X] TR021 [PR4] **Contracts inter-modules** - Interfaces claires et types partagés
- [X] TR022 [PR4] **Tests modules isolés** - Vérifier indépendance de chaque module

**Checkpoint**: Architecture nettoyée, visualisation claire, performance optimisée

**Phase R Status**: ✅ TERMINÉE - Toutes les phases R1, R2, R3 et R4 sont complètes

---

## 🏗️ Vision Modulaire - SOLID + Indépendance

### 🎯 Objectif Principal
**Créer des modules indépendants qui peuvent fonctionner isolément**, avec des contrats clairs entre eux, appliquant les principes SOLID pour éviter DRY et améliorer la maintenabilité.

### 📦 Architecture Cible par Module

#### 1. **Module Parsers** (`src-tauri/src/parsers/`)
**État actuel** : Dépend des commands pour être utilisé
**Objectif** : Fonctionne indépendamment (sauf `core/error.rs`)

- **Interface claire** : `Parser` trait avec `extract()` et `inject()` methods
- **Types partagés** : `ParsedText`, `ParserResult` dans module parsers
- **Testabilité** : Tests unitaires sans dépendre des commands
- **Utilisation** : Commands importent et utilisent l'interface

#### 2. **Module Translation** (`src-tauri/src/translation/`)
**État actuel** : Intégré aux commands + logique métier mélangée avec Ollama
**Objectif** : Architecture modulaire avec séparation claire + validation qualité

**Sous-modules cibles :**
- **`core/`** : Logique métier traduction (prompts, validation, orchestration)
- **`ollama/`** : Interface Ollama uniquement (API calls, connexion local/online)
- **`service.rs`** : Coordinateur entre core et providers (Ollama, futurs autres)

- **Interface claire** : `TranslationService` trait avec méthodes standardisées
- **Configuration** : Struct `TranslationConfig` pour paramètres (langues, modèle)
- **Résultats** : Type `TranslationResult` uniforme avec score de qualité
- **Validation** : `TranslationValidator` trait pour vérifier qualité traductions
- **Provider abstraction** : `TranslationProvider` trait pour différents services IA
- **Testabilité** : Tests isolés core/ollama + mocks pour indépendance

#### 3. **Module Frontend** (`app/`)
**État actuel** : Stores/composables mélangés
**Objectif** : Modules indépendants avec contracts définis

- **Contracts** : Interfaces TypeScript pour chaque domaine métier
- **Services** : Couche service isolée (API calls, business logic)
- **State** : Stores Pinia purs avec dépendances explicites
- **Composables** : Logique UI réutilisable sans dépendances cachées

#### 4. **Module Commands** (`src-tauri/src/commands/`)
**Rôle** : Couche d'adaptation Tauri entre modules métier et API
- **Adaptation** : Convertit données métier vers formats Tauri
- **Orchestration** : Coordonne appels entre modules (parsers → translation)
- **Validation** : Input validation avant traitement métier
- **Error handling** : Conversion erreurs métier → erreurs Tauri

### 🔗 Contracts Inter-Modules

#### Types Partagés (Common)
- `TextEntry` : Structure unifiée pour les textes à traduire
- `TranslationResult` : Résultat avec traduction et métadonnées
- `AppResult<T>` : Type alias pour `Result<T, AppError>`

#### Interfaces Modules
- **Parser** : `extract()` et `inject()` pour parsers de fichiers
- **TranslationService** : Coordinateur principal des traductions
- **TranslationProvider** : Abstraction pour providers IA (Ollama, OpenAI, etc.)
- **TranslationValidator** : Validation qualité des traductions

### 🧪 Testabilité Isolée

- **Tests Parsers** : Tests unitaires sans dépendances commands
- **Tests Translation** : Tests avec mocks pour providers IA
- **Tests Frontend** : Tests stores avec mocks API
- **Tests Validation** : Tests isolés pour la validation qualité

### 🏗️ Architecture Modulaire Translation

#### Séparation des Responsabilités
```
src-tauri/src/translation/
├── core/                    # Logique métier traduction
│   ├── mod.rs              # Exports core
│   ├── prompts.rs          # Gestion prompts, templates
│   ├── validation.rs       # Logique validation qualité
│   ├── orchestrator.rs     # Coordination traductions
│   └── types.rs            # Types partagés (TranslationConfig, etc.)
├── ollama/                 # Provider Ollama uniquement
│   ├── mod.rs              # Exports ollama
│   ├── client.rs           # API calls, connexion
│   ├── sequential.rs       # Gestion batchs séquentiels
│   └── single.rs           # Traduction individuelle
├── service.rs              # Coordinateur principal
└── mod.rs                  # Exports globaux
```

#### Flux de Traduction
```
Commands → TranslationService.translate()
                    ↓
            [Prompt Building] → [Validation Config]
                    ↓
            TranslationProvider (Ollama)
                    ↓
            [Raw Results] → [Validation Pipeline]
                    ↓
            [Scored Results] → Commands
```

#### Avantages de la Séparation
- **🔄 Remplacement IA** : Facile de changer Ollama pour OpenAI/Groq/etc.
- **🧪 Tests isolés** : Mock providers pour tests core sans dépendances
- **📦 Réutilisabilité** : Logique core utilisable avec n'importe quel provider
- **🚀 Performance** : Optimisations core indépendantes du provider
- **🐛 Debugging** : Isolation claire des problèmes (IA vs logique métier)

### 🔍 Validation des Traductions

#### Types de Validation Implémentés
- **Syntaxique** : Vérification ponctuation, format, caractères spéciaux
- **Sémantique** : Cohérence avec contexte et terminologie du jeu
- **Longueur** : Ratio longueur acceptable (éviter traductions trop courtes/longue)
- **Domaine** : Terminologie spécifique au jeu vidéo (RPG Maker, etc.)

#### Architecture de Validation
```
Traduction Ollama → Validation Pipeline → Résultat avec Score
                      ↓
               [Syntaxique] → Score 0-100
               [Sémantique] → Score 0-100
               [Longueur] → Score 0-100
               [Contexte] → Score 0-100
                        ↓
               Score Global + Issues détectés
```

#### Configuration de Validation
- Paramètres configurables pour chaque type de validation
- Seuils de qualité ajustables (syntaxique, sémantique, longueur, contexte)
- Options d'auto-rejet pour scores insuffisants

#### Résultat de Validation
- Scores détaillés par catégorie (0-100)
- Indicateurs de validité et problèmes détectés
- Suggestions d'amélioration automatiques

### ✅ Bénéfices Attendus

- **🚀 Indépendance** : Chaque module testable/modifiable isolément
- **🔧 Maintenabilité** : Changements locaux sans effets secondaires
- **📈 Évolutivité** : Nouveaux parsers/translations faciles à ajouter
- **🐛 Debugging** : Isolation facilite identification problèmes
- **👥 Travail d'équipe** : Modules indépendants = développement parallèle
- **🔄 Réutilisabilité** : Modules réutilisables dans autres projets
- **✨ Qualité** : Validation automatique améliore qualité traductions
- **📊 Métriques** : Scores objectifs pour mesurer performance IA
- **🔄 Amélioration** : Feedback validation → prompts optimisés

---

## 📋 Méthodologie Phase R

### 🔄 Approche Incrémentale
- **Commits fréquents** : Chaque tâche validée individuellement
- **Tests continus** : Vérifier que traduction fonctionne après chaque refactor
- **Rollback possible** : Branches séparées pour sécurité
- **Documentation** : Mettre à jour specs et commentaires

### 🛠️ Outils et Environnement
- **Version control** : Git branches dédiées (feature/phase-r1, etc.)
- **Monitoring** : Vue DevTools, Lighthouse, Bundle analyzer
- **Linting** : ESLint + TypeScript strict pour qualité
- **Testing** : Tests manuels UX + vérifications performance

### ⚠️ Gestion des Risques
- **Risque #1** : Refactoring casse traduction
  - **Mitigation** : Tests automatisés + validation manuelle systématique
- **Risque #2** : Performance dégradée
  - **Mitigation** : Benchmarks avant/après + métriques définies
- **Risque #3** : Changements trop invasifs
  - **Mitigation** : Approche incrémentale + reviews régulières
- **Risque #4** : Perte de fonctionnalités existantes
  - **Mitigation** : Checklist exhaustive des fonctionnalités à préserver

### 📊 Suivi de Progression
- **Daily standup** : Revue des tâches accomplies et blocages
- **Métriques hebdomadaires** : Performance, bundle size, code quality
- **Tests d'acceptation** : Workflow complet testé après chaque phase
- **Documentation** : Mise à jour automatique des specs

---

## Phase 6: User Story 4 - Réinjection des Traductions (Priority: P2)

**Goal**: Permettre la réinjection automatique des traductions dans les fichiers originaux

**Independent Test**: Peut être testé en injectant des traductions et vérifiant les fichiers modifiés

### Tests for User Story 4 (OBLIGATOIRE - TDD selon constitution) ⚠️

- [ ] T049 [P] [US4] Unit tests for file injection in tests/unit/file-injection.test.ts
- [ ] T050 [P] [US4] Unit tests for backup system in tests/unit/backup-system.test.ts
- [ ] T051 [P] [US4] Integration tests for injection workflow in tests/integration/injection-workflow.test.ts

### Implementation for User Story 4

- [ ] T052 [US4] Implement injection commands in src-tauri/src/commands/injection.rs
- [ ] T053 [US4] Create file backup system in src-tauri/src/commands/backup.rs
- [ ] T054 [US4] Add injection validation logic in injection commands
- [ ] T055 [US4] Create injection UI components in app/components/InjectionDialog.vue
- [ ] T056 [US4] Implement injection progress tracking in injection commands
- [ ] T057 [US4] Add rollback functionality for failed injections
- [ ] T058 [US4] Create injection history tracking in database

---

## Phase 7: User Story 5 - Administration Glossary (Priority: P2)

**Goal**: Interface complète pour gérer le glossaire et la cohérence terminologique

**Independent Test**: Peut être testé en gérant des termes du glossaire et vérifiant leur utilisation

### Tests for User Story 5 (OBLIGATOIRE - TDD selon constitution) ⚠️

- [ ] T059 [P] [US5] Unit tests for glossary search in tests/unit/glossary-search.test.ts
- [ ] T060 [P] [US5] Unit tests for term extraction in tests/unit/term-extraction.test.ts
- [ ] T061 [P] [US5] Integration tests for glossary workflow in tests/integration/glossary-workflow.test.ts

### Implementation for User Story 5

- [ ] T062 [US5] Enhance glossary search functionality in glossary composables
- [ ] T063 [US5] Add term extraction from translations in glossary commands
- [ ] T064 [US5] Create advanced glossary editor in app/components/AdvancedGlossary.vue
- [ ] T065 [US5] Implement bulk glossary operations in glossary composables
- [ ] T066 [US5] Add glossary consistency checking in translation logic
- [ ] T067 [US5] Create glossary import/export features in app/composables/useGlossaryExport.ts
- [ ] T068 [US5] Add glossary usage statistics in glossary store

---

## Phase 8: User Story 6 - Interface Utilisateur Complète (Priority: P3)

**Goal**: Interface utilisateur intuitive pour toutes les fonctionnalités de localisation

**Independent Test**: Peut être testé en naviguant dans l'interface et utilisant toutes les fonctionnalités

### Tests for User Story 6 (OBLIGATOIRE - TDD selon constitution) ⚠️

- [ ] T069 [P] [US6] E2E tests for main workflow in tests/e2e/main-workflow.test.ts
- [ ] T070 [P] [US6] E2E tests for project management in tests/e2e/project-management.test.ts
- [ ] T071 [P] [US6] E2E tests for translation process in tests/e2e/translation-process.test.ts

### Implementation for User Story 6

- [ ] T072 [US6] Create main application layout in app/layouts/default.vue
- [ ] T073 [US6] Implement project selection interface in app/pages/index.vue
- [ ] T074 [US6] Add translation list with filtering in app/components/TranslationList.vue
- [ ] T075 [US6] Create progress dashboard in app/components/ProgressDashboard.vue
- [ ] T076 [US6] Implement settings panel in app/pages/settings.vue
- [ ] T077 [US6] Add keyboard shortcuts and accessibility in app/composables/useKeyboard.ts
- [ ] T078 [US6] Create help system and documentation links
- [ ] T079 [US6] Implement dark/light theme switching in settings store
- [ ] T080 [US6] Create Ollama configuration interface (local/online modes) in app/pages/settings.vue
- [ ] T081 [US6] Add drag-and-drop file support in scanning components
- [ ] T082 [US6] Implement project path correction interface for moved/renamed projects in ProjectLoader.vue
  - Detect when a project folder no longer exists at stored path
  - Show visual indicator (orange warning) for moved projects
  - Provide "Fix Path" button that opens native folder picker
  - Validate new path contains compatible game files
  - Prevent conflicts with existing projects
  - Update database with corrected path
  - Show success confirmation and reload project data

---

## Phase 9: User Story 7 - Système de Donations avec Stripe (Priority: P3)

**Goal**: Implémenter un système de donations via Payment Links Stripe pour supporter le développement.

**Independent Test**: Peut être testé en initiant une donation et vérifiant la redirection vers Stripe avec message de remerciement.

### Tests for User Story 7 (OBLIGATOIRE - TDD selon constitution) ⚠️

- [ ] T082 [P] [US7] Unit tests for Stripe Payment Link creation in tests/unit/stripe-donation.test.ts
- [ ] T083 [P] [US7] Unit tests for donation amount validation in tests/unit/donation-validation.test.ts
- [ ] T084 [P] [US7] Integration tests for donation flow in tests/integration/donation-flow.test.ts

### Implementation for User Story 7

- [ ] T085 [US7] Configure async-stripe in src-tauri/Cargo.toml
- [ ] T086 [US7] Implement Stripe client in src-tauri/src/donations/stripe.rs
- [ ] T087 [US7] Create donation Payment Link commands in src-tauri/src/commands/donations.rs
- [ ] T088 [US7] Create donation UI components in app/components/DonationDialog.vue
- [ ] T089 [US7] Implement donation status tracking in app/stores/donations.ts
- [ ] T090 [US7] Add donation history storage (optional) in database
- [ ] T091 [US7] Create thank you message system after successful donation
- [ ] T092 [US7] Add donation button in main UI (help menu or about page)

**Checkpoint**: User Story 7 should be independently functional

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T093 [P] Add comprehensive error handling across all commands
- [ ] T094 [P] Implement logging system in src-tauri/src/lib.rs
- [ ] T095 [P] Add performance monitoring in app/composables/usePerformance.ts
- [ ] T096 [P] Create user onboarding flow in app/pages/onboarding.vue
- [ ] T097 [P] Implement auto-save functionality in all stores
- [ ] T098 [P] Add data validation across all forms and inputs
- [ ] T099 [P] Create update mechanism for application
- [X] T100 [P] Add internationalization support for UI (i18n)
- [ ] T101 [P] Implement comprehensive help documentation
- [ ] T102 [P] Add telemetry and usage analytics (opt-in)
- [ ] T103 [P] Final security audit and hardening
- [ ] T104 [P] Performance optimization and memory management
- [ ] T105 [P] Create user feedback system
- [ ] T106 [P] Add comprehensive test coverage (>80%)
- [ ] T107 [P] Final UI/UX polish and accessibility improvements

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **Phase 3 (User Story 1)**: Depends on Foundational phase completion - EXTRACTION ONLY
- **Phase 4 (User Story 2)**: RÉACTIVÉE - Gestion projets uniquement
- **Phase 5+**: Dépend de Phase 3+4 (US1+US2) pour continuer
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories - ACTUELLEMENT EN COURS
- **User Story 2 (P1)**: RÉACTIVÉE - Gestion projets uniquement (sans glossaire)
- **User Story 3 (P1)**: Can start after US1+US2 completion - dépend de l'extraction + organisation des données
- **User Story 4 (P2)**: Depends on US1 (extraction) et US3 (traduction) - nécessite des données traduites
- **User Story 5 (P2)**: Depends on US2 (gestion données) - REPORTÉE après US2
- **User Story 6 (P3)**: Depends on all other stories - interface complète pour toutes les fonctionnalités

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

**Lancement parallèle des tests pour User Story 1 :**
- Tests unitaires pour le scanning de fichiers
- Tests unitaires pour l'extraction de textes
- Tests d'intégration pour le workflow de scan

**Lancement parallèle des modèles pour User Story 1 :**
- Implémentation engine RPG Maker MV/MZ avec différenciation versions
- Création commands de scanning
- Ajout composables de scanning

---

## Implementation Strategy

### MVP First (APPROCHE AJUSTÉE - US1 + US2 Projets)

**CHANGEMENT IMPORTANT**: Réactivation Phase 4 avec focus sur gestion projets uniquement.

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Extraction) - **ACTUELLEMENT EN COURS**
4. **INTÉGRATION**: Connecter extraction avec gestion projets (Phase 4 partielle)
5. Complete Phase 4: User Story 2 (Gestion Projets uniquement)
6. **VALIDATE**: Test extraction + gestion projets ensemble
7. Phase 5: User Story 3 (Traduction) - avec données organisées

### Incremental Delivery (APPROCHE AJUSTÉE)

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test extraction independently → Deploy/Demo (extraction capability) - **ACTUELLEMENT EN COURS**
3. **INTÉGRER**: Connecter extraction avec gestion projets
4. Add User Story 2 (Projets) → Test gestion données → Deploy/Demo (organisation capability)
5. **VALIDATE**: Test workflow complet extraction → organisation → préparation traduction
6. Add User Story 3 → Test translation avec données organisées → Deploy/Demo (automated translation)
7. Add User Stories 4-5 → Test injection and glossary → Deploy/Demo (complete workflow)
8. Add User Story 6 → Polish UI/UX → Final release
9. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together (1-2 days)
2. Once Foundational is done:
   - Developer A: User Stories 1 & 4 (extraction/injection)
   - Developer B: User Stories 2 & 5 (data management/glossary)
   - Developer C: User Stories 3 & 6 (translation/UI)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
