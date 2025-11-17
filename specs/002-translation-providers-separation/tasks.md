# Tasks: Séparation des Providers de Traduction

**Input**: Design documents from `/specs/002-translation-providers-separation/`
**Prerequisites**: spec.md (required), plan.md (required)

**Tests**: TDD obligatoire selon la constitution. Tests unitaires et d'intégration requis pour chaque phase.

**Organization**: Tasks are grouped by phase to enable sequential implementation.

## Format: `[ID] [P?] [Phase] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Phase]**: Which implementation phase this task belongs to

## Path Conventions

- **Frontend (Nuxt)**: `app/` at repository root
- **Backend (Rust)**: `src-tauri/src/` at repository root
- **Tests**: `tests/` at repository root

---

## Phase 1: Backend - Nettoyage Ollama (Local uniquement)

**Purpose**: Nettoyer le module Ollama pour être 100% local avec `ollama-rs`.

- [x] T001 [Phase1] Supprimer `OllamaMode::Online` de `src-tauri/src/translation/ollama/client.rs`
- [x] T002 [Phase1] Simplifier `OllamaConfig` pour local uniquement (supprimer `mode`, garder `endpoint` + `port`)
- [x] T003 [Phase1] Nettoyer `check_ollama_status` pour local uniquement dans `src-tauri/src/translation/ollama/client.rs`
- [x] T004 [Phase1] Mettre à jour les exports dans `src-tauri/src/translation/ollama/mod.rs`
- [ ] T005 [Phase1] [P] Tests unitaires pour `OllamaClient` local dans `tests/unit/translation/ollama_client_test.rs`
- [ ] T006 [Phase1] [P] Tests d'intégration pour connexion Ollama local dans `tests/integration/ollama_local_test.rs`

**Checkpoint**: ✅ Ollama fonctionne uniquement en local, code compile sans erreurs. Tests à implémenter en Phase 6.

---

## Phase 2: Backend - Création RunPod

**Purpose**: Créer le module RunPod pour gérer les connexions online avec `reqwest`.

- [x] T007 [Phase2] Ajouter `reqwest` à `src-tauri/Cargo.toml`
- [x] T008 [Phase2] Créer `src-tauri/src/translation/runpod/mod.rs` avec exports de base
- [x] T009 [Phase2] Créer `src-tauri/src/translation/runpod/client.rs` avec `RunPodClient` utilisant `reqwest`
- [x] T009a [Phase2] Implémenter construction automatique URL: `https://{pod_id}-11434.proxy.runpod.net` dans `RunPodClient::new()`
- [x] T010 [Phase2] Implémenter `list_models()` dans `RunPodClient` (GET `/api/tags`)
- [ ] T011 [Phase2] Implémenter `generate()` dans `RunPodClient` (POST `/api/generate`) - Non nécessaire, `chat()` suffit
- [x] T012 [Phase2] Implémenter `chat()` dans `RunPodClient` (POST `/api/chat`)
- [x] T013 [Phase2] Implémenter `test_connection()` dans `RunPodClient`
- [x] T014 [Phase2] Créer `src-tauri/src/translation/runpod/common.rs` réutilisant `ollama::common`
- [x] T015 [Phase2] Créer `src-tauri/src/translation/runpod/single.rs` basé sur `ollama/single.rs` adapté pour `RunPodClient`
- [x] T016 [Phase2] Créer `src-tauri/src/translation/runpod/sequential.rs` basé sur `ollama/sequential.rs` adapté pour `RunPodClient`
- [ ] T017 [Phase2] [P] Tests unitaires pour `RunPodClient` dans `tests/unit/translation/runpod_client_test.rs`
- [ ] T018 [Phase2] [P] Tests d'intégration pour connexion RunPod dans `tests/integration/runpod_test.rs`

**Checkpoint**: ✅ RunPod fonctionne en online, code compile sans erreurs. Tests à implémenter en Phase 6.

---

## Phase 3: Backend - Coordination

**Purpose**: Créer un système de routing pour utiliser le bon provider.

- [ ] T019 [Phase3] Mettre à jour `src-tauri/src/translation/service.rs` pour router selon provider - Non nécessaire, routing fait directement dans commands
- [x] T020 [Phase3] Mettre à jour `src-tauri/src/translation/mod.rs` pour exporter `runpod`
- [x] T021 [Phase3] Ajouter paramètre `provider: 'ollama' | 'runpod'` aux commands dans `src-tauri/src/commands/translation.rs`
- [x] T022 [Phase3] Créer managers globaux séparés pour Ollama et RunPod dans `src-tauri/src/commands/translation.rs`
- [x] T023 [Phase3] Router `start_sequential_translation` vers le bon manager selon provider
- [x] T024 [Phase3] Router `get_translation_suggestions` vers le bon manager selon provider
- [x] T025 [Phase3] Router `translate_single_text` vers le bon manager selon provider
- [x] T025a [Phase3] Router `get_sequential_progress`, `pause_sequential_session`, `resume_sequential_session`, `stop_sequential_session` vers le bon manager
- [x] T025b [Phase3] Ajouter commande `check_runpod_status` pour vérifier disponibilité RunPod
- [ ] T026 [Phase3] [P] Tests unitaires pour routing dans `tests/unit/translation/service_test.rs`
- [ ] T027 [Phase3] [P] Tests d'intégration pour switch provider dans `tests/integration/provider_switch_test.rs`

**Checkpoint**: ✅ Le système route correctement vers le bon provider, code compile sans erreurs. Tests à implémenter en Phase 6.

---

## Phase 4: Frontend - Settings

**Purpose**: Mettre à jour les settings pour gérer les deux providers.

- [x] T028 [Phase4] Mettre à jour `AppSettings` dans `app/composables/useTauriSetting.ts` avec nouvelle structure
- [x] T029 [Phase4] Migration automatique supprimée - ancien format sera supprimé directement
- [x] T030 [Phase4] Créer `app/components/settings/RunPodConfig.vue` pour configuration RunPod (champ POD_ID uniquement)
- [x] T031 [Phase4] Nettoyer `app/components/settings/OllamaConfig.vue` pour local uniquement
- [x] T032 [Phase4] Ajouter sélecteur de provider dans `app/pages/settings.vue`
- [x] T033 [Phase4] Afficher `OllamaConfig` ou `RunPodConfig` selon provider sélectionné dans `app/pages/settings.vue`
- [ ] T034 [Phase4] [P] Tests unitaires pour migration settings dans `tests/unit/frontend/settings_migration_test.ts`
- [ ] T035 [Phase4] [P] Tests e2e pour configuration providers dans `tests/e2e/settings_providers_test.ts`

**Checkpoint**: ✅ Settings gèrent les deux providers, interface complète fonctionnelle. Tests à implémenter en Phase 6.

---

## Phase 5: Frontend - Stores et Composants

**Purpose**: Mettre à jour les stores et composants pour utiliser le bon provider.

- [x] T036 [Phase5] Adapter `app/stores/ollama.ts` pour local uniquement
- [x] T037 [Phase5] Créer `app/stores/runpod.ts` pour RunPod avec gestion complète du statut et des modèles
- [x] T038 [Phase5] Mettre à jour `app/composables/db/texts/translation.ts` pour passer provider et pod_id aux commands
- [x] T039 [Phase5] Créer `app/composables/translation/useRunpodCheck.ts` pour vérification connexion RunPod
- [x] T040 [Phase5] Adapter `app/composables/translation/useOllamaCheck.ts` pour être 100% Ollama (suppression logique RunPod)
- [x] T041 [Phase5] Mettre à jour `app/components/translations/TranslationControls.vue` pour utiliser le bon provider et modèle
- [x] T042 [Phase5] Mettre à jour `app/components/translations/EditTranslationModal.vue` pour utiliser le bon provider et modèle
- [x] T043 [Phase5] Ajouter sélection de modèle dans `app/components/settings/RunPodConfig.vue`
- [x] T044 [Phase5] Créer `app/components/settings/RunPodStatusBadge.vue` pour affichage statut RunPod
- [x] T045 [Phase5] Mettre à jour `app/components/settings/OllamaStatusBadge.vue` pour afficher uniquement une icône (pas de bouton)
- [x] T046 [Phase5] Mettre à jour `app/components/Header.vue` pour afficher le bon badge selon le provider sélectionné
- [ ] T047 [Phase5] [P] Tests unitaires pour stores dans `tests/unit/frontend/stores_test.ts`
- [ ] T048 [Phase5] [P] Tests e2e pour traduction avec providers dans `tests/e2e/translation_providers_test.ts`

**Checkpoint**: ✅ Frontend utilise le bon provider selon configuration, stores et composants mis à jour. Badges de statut affichés conditionnellement dans le header. Tests à implémenter en Phase 6.

---

## Phase 6: Tests et Validation

**Purpose**: Valider que tout fonctionne correctement.

- [ ] T049 [Phase6] Exécuter tous les tests backend et vérifier passage à 100%
- [ ] T050 [Phase6] Exécuter tous les tests frontend et vérifier passage à 100%
- [ ] T051 [Phase6] Test manuel: Configuration Ollama local et traduction
- [ ] T052 [Phase6] Test manuel: Configuration RunPod online et traduction
- [ ] T053 [Phase6] Test manuel: Switch entre providers
- [ ] T054 [Phase6] Test manuel: Validation modèle RunPod invalide (fallback automatique vers premier modèle disponible)
- [ ] T055 [Phase6] Vérifier backward compatibility (pas de régression)
- [ ] T056 [Phase6] Mettre à jour documentation dans `README.md`
- [ ] T057 [Phase6] Mettre à jour `CHANGELOG.md` avec changements (déjà fait)

**Checkpoint**: Tous les tests passent, pas de régression, documentation complète.

---

## Summary

**Total Tasks**: 57 (4 nouvelles tâches Phase 5, 1 nouvelle tâche Phase 6)
**Phases**: 6
**Estimated Time**: 
- Phase 1: 2-3 heures ✅ TERMINÉE
- Phase 2: 4-6 heures ✅ TERMINÉE
- Phase 3: 2-3 heures ✅ TERMINÉE
- Phase 4: 2-3 heures ✅ TERMINÉE
- Phase 5: 3-4 heures ✅ TERMINÉE
- Phase 6: 2-3 heures ⏳ EN ATTENTE
**Total**: ~15-22 heures
**Progression**: 5/6 phases terminées (83%)

## Notes

- Toutes les phases doivent être complétées séquentiellement
- Les tests marqués [P] peuvent être exécutés en parallèle
- Backward compatibility est critique - migration automatique requise
- Documentation doit être mise à jour à chaque phase

