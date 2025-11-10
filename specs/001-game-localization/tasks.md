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

**Stratégie**: Backend d'abord (validation), puis frontend (CRUD), enfin UI et intégration

**Checkpoint**: Phase 4 réactivée - US1 + US2 (projets uniquement) avant US3

---

## Phase 5: User Story 3 - Traduction par Lots via Ollama (Priority: P1)

**Goal**: Implémenter la traduction automatique par lots avec mise à jour des données

**Independent Test**: Peut être testé en lançant une traduction par lots et vérifiant les résultats

### Tests for User Story 3 (OBLIGATOIRE - TDD selon constitution) ⚠️

- [ ] T037 [P] [US3] Unit tests for Ollama client in tests/unit/ollama-client.test.ts
- [ ] T038 [P] [US3] Unit tests for batch processing in tests/unit/batch-processing.test.ts
- [ ] T039 [P] [US3] Integration tests for translation workflow in tests/integration/translation-workflow.test.ts

### Implementation for User Story 3

- [ ] T040 [US3] Implement Ollama client in src-tauri/src/translation/ollama/client.rs
- [ ] T041 [US3] Create batch translation logic in src-tauri/src/translation/ollama/batch.rs
- [ ] T042 [US3] Add single translation functionality in src-tauri/src/translation/ollama/single.rs
- [ ] T043 [US3] Implement translation commands in src-tauri/src/commands/translation.rs
- [ ] T044 [US3] Create translation composables in app/composables/db/translation/
- [ ] T045 [US3] Add batch translation UI in app/components/BatchTranslation.vue
- [ ] T046 [US3] Implement translation progress tracking in app/stores/batch.ts
- [ ] T047 [US3] Add glossary integration in translation logic
- [ ] T048 [US3] Create translation cancellation and resume functionality

**Checkpoint**: All P1 user stories should now be independently functional

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

```bash
# Launch all tests for User Story 1 together:
Task: "Unit tests for file scanning in tests/unit/scanning.test.ts"
Task: "Unit tests for text extraction in tests/unit/extraction.test.ts"
Task: "Integration tests for scan workflow in tests/integration/scan-workflow.test.ts"

# Launch all models for User Story 1 together:
Task: "Implement RPG Maker MV/MZ engine with version differentiation in src-tauri/src/parsers/rpg_maker/engine.rs"
Task: "Create scanning commands in src-tauri/src/commands/scanning.rs"
Task: "Add scanning composables in app/composables/db/scanning/"
```

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
