# Tasks Breakdown - Translation Architecture Refactoring

## Legend
- `[P]` = Parallel execution possible
- `[S]` = Sequential (depends on previous)
- `[T]` = Test task
- `[R]` = Research task

---

## Phase 1: Foundation & Analysis

### 1.1 Code Analysis [R] ✅ COMPLETED
**Objective**: Complete audit of existing duplication
**Files**: `ollama/`, `runpod/` modules
**Results**: ~670 lignes dupliquées identifiées (67% du code translation)
**Tasks**:
- [x] Compare `ollama/single.rs` vs `runpod/single.rs` line by line (281 vs 321 lines)
- [x] Compare `ollama/sequential.rs` vs `runpod/sequential.rs` line by line (524 vs 432 lines)
- [x] Document all identical structures: 9 structs/enums 100% identiques
- [x] Document all duplicated methods: 13+ méthodes 95% similaires
- [x] Identify differences (logging, naming, error messages, API calls)
- [x] Generate duplication metrics: ~670 lignes dupliquées (67% du code)

**Output**: `duplication-analysis.md` with detailed findings

### 1.2 Create Common Module Structure [S] ✅ COMPLETED
**Objective**: Set up the foundation for shared code
**Files**: `src-tauri/src/translation/common/`
**Tasks**:
- [x] Create directory `src-tauri/src/translation/common/`
- [x] Create `common/mod.rs` with basic exports
- [x] Create `common/types.rs` as empty file (to be populated)
- [x] Update `translation/mod.rs` to include common module
- [x] Verify compilation after each step

---

## Phase 2: Generic Interface

### 2.1 Create Common Functions [S]
**Objective**: Create common functions that can work with any translation client
**Files**: `src-tauri/src/translation/common/functions.rs`
**Tasks**:
- [ ] Create `functions.rs` file
- [ ] Define `translate_single_common()` function that takes a client parameter
  - `translate_single(prompt, model) -> Result<String, String>`
  - `list_available_models() -> Result<Vec<String>, String>`
  - `test_connection() -> Result<(), String>`
  - `get_provider_name() -> &'static str`
- [ ] Add proper documentation comments
- [ ] Verify trait compiles

### 2.2 Implement Common Functions [S]
**Objective**: Implement the common translation and sequential functions
**Files**: `src-tauri/src/translation/common/functions.rs`
**Tasks**:
- [ ] Implement `translate_single_common(client, request, app_handle)`
- [ ] Implement `get_progress_common(sessions, session_id)`
- [ ] Implement `pause_session_common(sessions, session_id)`
- [ ] Implement `start_session_common(sessions, client, request)`
- [ ] Add any other common functions needed
- [ ] Test the functions with mock clients

---

## Phase 3: Common Logic

### 3.1 Create Common Types [S]
**Objective**: Move all shared data structures to common module
**Files**: `src-tauri/src/translation/common/types.rs`
**Tasks**:
- [ ] Copy all identical structures from `ollama/single.rs`:
  - `SingleTranslationRequest`
  - `SingleTranslationResult`
  - `TranslationSuggestion`
- [ ] Copy all identical structures from `ollama/sequential.rs`:
  - `TranslationText`
  - `SequentialTranslationRequest`
  - `SequentialProgress`
  - `SequentialStatus`
  - `SequentialError`
  - `SuccessfulTranslation`
- [ ] Add `SequentialSession` structure for session management
- [ ] Update imports in `common/mod.rs`
- [ ] Verify compilation

### 3.3 Create Common Single Logic [S]
**Objective**: Implement generic single translation manager
**Files**: `src-tauri/src/translation/common/logic.rs` (append)
**Tasks**:
- [ ] Define `CommonSingleManager<T: TranslationApiTrait>`
- [ ] Implement `translate()` method with all common logic:
  - Request validation
  - Glossary lookup
  - Prompt building
  - Response parsing
- [ ] Keep provider-specific parts as callbacks/trait methods
- [ ] Update exports

---

## Phase 4: Module Refactoring

### 4.1 Refactor Ollama Single Implementation [S]
**Objective**: Simplify ollama/single.rs to use common functions
**Files**: `src-tauri/src/translation/ollama/single.rs`
**Tasks**:
- [ ] Replace duplicate logic with calls to `translate_single_common()`
- [ ] Keep only Ollama-specific API calls and error handling
- [ ] Reduce from ~281 lines to ~50 lines
- [ ] Verify compilation

### 4.2 Refactor Ollama Sequential Implementation [S]
**Objective**: Simplify ollama/sequential.rs to use common functions
**Files**: `src-tauri/src/translation/ollama/sequential.rs`
**Tasks**:
- [ ] Replace duplicate logic with calls to common functions:
  - `get_progress_common()` instead of duplicated progress logic
  - `pause_session_common()` instead of duplicated pause logic
  - etc.
- [ ] Keep only Ollama-specific session management structure
- [ ] Reduce from ~524 lines to ~200 lines
- [ ] Verify compilation

### 4.3 Refactor Runpod Implementations [P] (parallel with 4.1-4.2)
**Objective**: Simplify runpod modules to use common functions
**Files**: `src-tauri/src/translation/runpod/single.rs`, `runpod/sequential.rs`
**Tasks**:
- [ ] Same refactoring process as Ollama modules
- [ ] Adapt to RunPod-specific API calls and error handling
- [ ] Reduce single.rs from ~321 lines to ~50 lines
- [ ] Reduce sequential.rs from ~432 lines to ~200 lines
- [ ] Verify compilation

### 4.6 Update Translation Commands [S]
**Objective**: Update all imports in the commands module
**Files**: `src-tauri/src/commands/translation.rs`
**Tasks**:
- [ ] Update all imports to use new module structure
- [ ] Verify that all Tauri commands still work
- [ ] Test compilation and basic functionality

---

## Phase 5: Testing & Validation

### 5.1 Unit Tests for Common Functions [T]
**Objective**: Test the common functions independently
**Files**: `src-tauri/src/translation/common/` (new test files)
**Tasks**:
- [ ] Create unit tests for `translate_single_common()`
- [ ] Create unit tests for `get_progress_common()`
- [ ] Create unit tests for `pause_session_common()`
- [ ] Create unit tests for `start_session_common()`
- [ ] Mock the client interfaces for testing

### 5.2 Integration Tests [T]
**Objective**: Test end-to-end functionality
**Files**: Existing test files or new integration tests
**Tasks**:
- [ ] Test Ollama translation flow (single + sequential)
- [ ] Test RunPod translation flow (single + sequential)
- [ ] Verify Tauri API compatibility (back-end only, front-end unchanged)
- [ ] Performance regression testing

### 5.3 Manual Testing [T]
**Objective**: Validate user-facing functionality
**Tasks**:
- [ ] Build the application (`pnpm tauri build`)
- [ ] Test translation features in the UI
- [ ] Verify error messages and user feedback
- [ ] Test both Ollama and RunPod providers

---

## Phase 6: Cleanup & Documentation

### 6.1 Remove Dead Code [S]
**Objective**: Eliminate all remaining duplication
**Files**: All modified files
**Tasks**:
- [ ] Remove commented-out duplicated code
- [ ] Remove unused imports
- [ ] Run `cargo check` and fix all warnings
- [ ] Final compilation verification

### 6.2 Update Documentation [S]
**Objective**: Document the new architecture
**Files**: Code comments and README files
**Tasks**:
- [ ] Add comprehensive documentation to `TranslationApiTrait`
- [ ] Document how to add new translation providers
- [ ] Update module-level documentation
- [ ] Create examples in doc comments

### 6.3 Final Validation [T]
**Objective**: Ensure everything works perfectly
**Tasks**:
- [ ] Full application build and test
- [ ] Code review of all changes
- [ ] Update this task list with completion status
- [ ] Generate final metrics (lines saved, duplication eliminated)

---

## Task Dependencies & Parallelization

```
1.1 → 1.2 → 2.1 → 2.2 + 2.3 → 3.1 → 3.2 → 3.3
                     ↓
4.1 + 4.2 → 4.3 + 4.4 + 4.5 → 4.6 → 5.1 + 5.2 + 5.3 → 6.1 → 6.2 → 6.3
```

**Parallel opportunities**:
- 2.2 & 2.3 (implement traits for both clients)
- 4.1 & 4.2 (update both module exports)
- 4.3-4.5 (refactor all implementation files)
- 5.1-5.3 (all testing phases)

## Completion Checklist

- [ ] All tasks completed
- [ ] No compilation errors
- [ ] All tests pass
- [ ] No performance regression
- [ ] Code review approved
- [ ] Documentation updated
- [ ] Backward compatibility maintained
