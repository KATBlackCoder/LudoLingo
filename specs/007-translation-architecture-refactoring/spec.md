# Translation Architecture Refactoring - Spec 007

## Executive Summary

Refactoriser l'architecture de traduction pour éliminer la duplication massive entre les modules `ollama` et `runpod`, en créant une base commune tout en maintenant la séparation claire des implémentations client.

## Current State Analysis

### Duplication Identified

#### Data Structures (100% duplicated)
**9 structures/enums identiques dans ollama/ et runpod/ :**
```rust
pub struct SingleTranslationRequest { /* 8 champs identiques */ }
pub struct SingleTranslationResult { /* 4 champs identiques */ }
pub struct TranslationSuggestion { /* 3 champs, source diffère */ }
pub struct TranslationText { /* 4 champs identiques */ }
pub struct SequentialTranslationRequest { /* 6 champs identiques */ }
pub struct SequentialProgress { /* 8 champs identiques */ }
pub enum SequentialStatus { Idle, Running, Paused, Completed, Error }
pub struct SequentialError { /* 2 champs identiques */ }
pub struct SuccessfulTranslation { /* 4 champs identiques */ }
```

#### Methods (95%+ duplicated)
**13 méthodes publiques/privées similaires :**
```rust
// Publiques (6 méthodes)
pub async fn translate()           // ~80 lignes, 95% identiques
pub async fn get_suggestions()     // Logique similaire
pub async fn get_progress()        // 100% identique
pub async fn pause_session()       // 100% identique
pub async fn resume_session()      // 100% identique
pub async fn stop_session()        // 100% identique
pub async fn get_active_sessions() // 100% identique
pub async fn start_session()       // 90% identique

// Privées (7 méthodes)
fn process_session()               // Logique de boucle identique
fn generate_session_id()           // Même logique, préfixe différent
fn process_next_entry()            // 85% identique
fn validate_request()              // 100% identique
fn get_translation_settings()      // 100% identique
```

#### Business Logic (90%+ duplicated)
- **Validation** : `validate_translation_request()`, `validate_request()`
- **Glossary** : `lookup_glossary_terms()`, mapping `text_type→category`
- **Prompt building** : `build_translation_prompt()` (déjà factorisé ✅)
- **Response parsing** : `parse_translation_response()` (déjà factorisé ✅)
- **Session management** : Processus séquentiel identique
- **Error handling** : Gestion d'erreurs similaire

### Impact Metrics
- **Lines of duplicated code**: ~670+ lines (67% of total translation code)
- **Files affected**: 4 core files (single.rs, sequential.rs per provider)
- **Structures duplicated**: 9 structs/enums 100% identiques
- **Methods duplicated**: 13+ méthodes publiques/privées ~95% similaires
- **Maintenance burden**: Changes require 2x modifications
- **Bug risk**: Inconsistencies between implementations

## Proposed Architecture

### New Module Structure

```
src-tauri/src/translation/
├── common/
│   ├── mod.rs           # Exports communs
│   ├── types.rs         # 9 structures partagées (SingleTranslationRequest, etc.)
│   └── functions.rs     # Fonctions communes free-standing
├── ollama/              # Implémentation Ollama
│   ├── mod.rs           # Exports spécifiques Ollama
│   ├── client.rs        # Client Ollama avec méthodes spécifiques
│   ├── single.rs        # Wrapper léger (~50 lignes vs 281 actuelles)
│   └── sequential.rs    # Implémentation Sequential Ollama
├── runpod/              # Implémentation RunPod
│   ├── mod.rs           # Exports spécifiques RunPod
│   ├── client.rs        # Client RunPod avec méthodes spécifiques
│   ├── single.rs        # Wrapper léger (~50 lignes vs 321 actuelles)
│   └── sequential.rs    # Implémentation Sequential RunPod
└── mod.rs               # Exports principaux
```

### Fonctions Communes

Au lieu d'un trait générique, utiliser des **fonctions free-standing** qui prennent les clients spécifiques en paramètre :

```rust
// common/functions.rs - Fonctions communes
pub async fn translate_single_common(
    client: &impl TranslationClient,  // OllamaClient ou RunpodClient
    request: SingleTranslationRequest,
    app_handle: &AppHandle,
) -> Result<SingleTranslationResult, String> {
    // Logique commune de traduction
    validate_translation_request(&request.source_text)?;
    // ... logique commune ...
    client.call_api(prompt, model).await  // Appel spécifique au client
}

// Même approche pour les fonctions séquentielles
pub async fn start_session_common(
    sessions: &mut HashMap<String, SequentialSession>,
    client: &impl TranslationClient,
    request: SequentialTranslationRequest,
) -> Result<String, String> {
    // Logique commune de session
}
```

### Common Types Module

```rust
// types.rs - Structures partagées
pub struct SingleTranslationRequest { ... }
pub struct SingleTranslationResult { ... }
pub struct SequentialTranslationRequest { ... }
pub struct SequentialProgress { ... }
pub struct SequentialStatus { ... }
pub struct TranslationText { ... }
```

### Fonctions Séquentielles Communes

```rust
// common/functions.rs - Fonctions séquentielles communes
pub struct SequentialSession {
    session_id: String,
    texts: Vec<TranslationText>,
    current_index: usize,
    processed_entries: HashMap<i32, SuccessfulTranslation>,
    errors: Vec<SequentialError>,
    status: SequentialStatus,
    // ...
}

pub async fn get_progress_common(
    sessions: &Mutex<HashMap<String, SequentialSession>>,
    session_id: &str
) -> Option<SequentialProgress> {
    // Logique commune de calcul de progression
}

pub async fn pause_session_common(
    sessions: &mut HashMap<String, SequentialSession>,
    session_id: &str
) -> Result<(), String> {
    // Logique commune de pause
}

// ... autres fonctions communes
```

### Managers Spécifiques par Provider

Chaque provider garde son propre manager mais utilise les fonctions communes :

```rust
// ollama/sequential.rs
pub struct OllamaSequentialManager {
    client: Arc<OllamaClient>,
    active_sessions: Arc<Mutex<HashMap<String, SequentialSession>>>,
    session_counter: Arc<Mutex<i32>>,
}

impl OllamaSequentialManager {
    pub async fn get_progress(&self, session_id: &str) -> Option<SequentialProgress> {
        get_progress_common(&self.active_sessions, session_id).await
    }

    pub async fn pause_session(&self, session_id: &str) -> Result<(), String> {
        pause_session_common(&mut *self.active_sessions.lock().await, session_id).await
    }
    // ... autres méthodes qui délèguent aux fonctions communes
}
```

## Implementation Constraints

### Zero External Dependencies
Cette refactorisation est **strictement architecturale** et n'ajoute **aucune dépendance externe** :
- ❌ **Pas de `async-trait`** ou autre crate externe
- ✅ **Futures Rust standard** uniquement (`std::future::Future`, `std::pin::Pin`)
- ✅ **Dépendances existantes** préservées
- ✅ **Taille binaire** inchangée

### Rationale
- Refactorisation pure : éliminer la duplication sans ajouter de complexité
- Maintenance réduite : moins de dépendances = moins de breaking changes externes
- Performance : éviter l'overhead des crates externes pour un refactoring interne

## Implementation Phases

### Phase 1: Common Foundation
1. Créer `translation/common/types.rs` avec les 9 structures partagées
2. Créer `translation/common/functions.rs` avec les fonctions communes free-standing

### Phase 2: Fonctions Communes
1. Implémenter `translate_single_common()` et autres fonctions de base
2. Implémenter `get_progress_common()`, `pause_session_common()` et fonctions séquentielles
3. Tester les fonctions communes indépendamment

### Phase 3: Module Refactoring
1. Refactoriser `ollama/single.rs` : wrapper qui appelle `translate_single_common()`
2. Refactoriser `ollama/sequential.rs` : manager qui délègue aux fonctions communes
3. Faire de même pour `runpod/` modules
4. Mettre à jour tous les exports dans `mod.rs`

### Phase 4: Testing & Validation
1. Tests unitaires pour `CommonSequentialManager<T>`
2. Tests d'intégration pour chaque provider
3. Validation API publique inchangée

## Backward Compatibility

L'API publique doit rester **strictement identique** :
- Même noms de fonctions
- Même signatures de types
- Même comportement observé
- Changements internes uniquement

## Success Criteria

- ✅ **~670 lignes supprimées** (67% de code dupliqué éliminé)
- ✅ **9 structures factorisées** dans module commun
- ✅ **13 méthodes communes** centralisées
- ✅ **API publique inchangée** (backward compatible)
- ✅ **Front-end non affecté** (pas de changements requis)
- ✅ **Tests passent** pour les deux providers
- ✅ **Performance maintenue** (pas de regression >2%)
- ✅ **Maintenance simplifiée** (changement dans un seul endroit)

## Risk Assessment

### High Risk
- **Breaking changes** si l'API publique est modifiée
- **Performance impact** si la généricité ajoute de l'overhead

### Medium Risk
- **Complexité accrue** du code générique
- **Debugging plus difficile** avec les traits

### Low Risk
- **Tests existants** devraient couvrir la majorité des cas

## Dependencies

- **002-translation-providers-separation** - Architecture de base requise
- Nécessite les deux implémentations (Ollama + RunPod) pour validation

## Future Considerations

Cette refactorisation prépare le terrain pour :
- **009-translation-factory** - Pattern Factory pour ajouter de nouveaux providers
- Support facile de nouveaux providers (OpenAI, Anthropic, etc.)
- Configuration centralisée des providers
- Load balancing et failover automatique
