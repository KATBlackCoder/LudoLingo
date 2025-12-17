# Feature Specification: Alignement Architecture Translation avec Pattern Parsers

**Feature Branch**: `011-translation-provider-architecture`  
**Created**: 2025-01-XX  
**Status**: Draft  
**Input**: Refactorisation du système de translation pour suivre exactement le même pattern architectural que `parsers/` avec trait commun, factory, et providers indépendants.

**🎯 Objectif**: Aligner complètement l'architecture du module `translation/` avec celle de `parsers/` pour garantir cohérence, maintenabilité et extensibilité. Chaque provider doit être aussi indépendant que `RpgMakerHandler` ou `WolfRpgHandler`.

## Contexte

### Architecture Actuelle des Parsers (Modèle à Suivre)

```
parsers/
├── handler.rs          # Trait GameEngineHandler (interface commune)
├── factory.rs          # EngineFactory (détection + création)
├── rpg_maker/
│   ├── handler.rs      # RpgMakerHandler impl GameEngineHandler
│   └── engine.rs       # RpgMakerEngine (logique métier)
└── wolfrpg/
    └── handler.rs      # WolfRpgHandler impl GameEngineHandler
```

**Principes clés des parsers** :
- ✅ Trait commun (`GameEngineHandler`) définit l'interface
- ✅ Factory (`EngineFactory`) crée les handlers appropriés
- ✅ Chaque handler est **complètement indépendant** et auto-suffisant
- ✅ Logique métier séparée dans des modules `engine.rs`
- ✅ Aucune dépendance externe, pas de singletons globaux

### Architecture Actuelle des Translations (À Refactoriser)

```
translation/
├── common/types.rs     # Types partagés
├── ollama/             # Implémentation complète mais couplée
├── runpod/             # Implémentation complète mais couplée
└── mod.rs

commands/
└── translation.rs      # Routage manuel avec managers globaux
```

**Problèmes actuels** :
- ❌ Managers globaux (`OLLAMA_SEQUENTIAL_MANAGER`, `RUNPOD_MANAGERS_CACHE`)
- ❌ Routage manuel dans chaque command avec `match provider`
- ❌ Pas de trait commun pour abstraction
- ❌ Pas de factory pour création
- ❌ Couplage fort entre commands et implémentations

## User Scenarios & Testing

### User Story 1 - Trait TranslationProvider Identique à GameEngineHandler (Priority: P1)

Le module `translation/` expose un trait `TranslationProvider` qui suit exactement le même pattern que `GameEngineHandler` dans `parsers/`.

**Why this priority**: Le trait est l'abstraction centrale qui permet la cohérence architecturale entre parsers et translations.

**Independent Test**: Peut être testé en créant des implémentations mock du trait et vérifiant que l'interface est identique en structure à `GameEngineHandler`.

**Acceptance Scenarios**:

1. **Given** le trait `TranslationProvider`, **When** on examine sa structure, **Then** elle suit le même pattern que `GameEngineHandler` (méthodes claires, documentation complète)
2. **Given** un provider implémentant `TranslationProvider`, **When** on l'utilise, **Then** il peut être utilisé via `Box<dyn TranslationProvider>` comme `Box<dyn GameEngineHandler>`
3. **Given** le trait `TranslationProvider`, **When** on l'implémente, **Then** toutes les méthodes sont requises et documentées

---

### User Story 2 - Factory Pattern Identique à EngineFactory (Priority: P1)

Le module `translation/` expose une factory `TranslationProviderFactory` qui suit exactement le même pattern que `EngineFactory` dans `parsers/`.

**Why this priority**: La factory est le point d'entrée unique pour créer les providers, garantissant la cohérence avec les parsers.

**Independent Test**: Peut être testé en comparant la structure de `TranslationProviderFactory` avec `EngineFactory` et vérifiant qu'elles suivent le même pattern.

**Acceptance Scenarios**:

1. **Given** `TranslationProviderFactory`, **When** on examine sa structure, **Then** elle suit le même pattern que `EngineFactory` (méthode statique `create_provider()`)
2. **Given** un provider type "ollama", **When** on appelle `create_provider()`, **Then** elle retourne un `OllamaProvider` configuré
3. **Given** un provider type "runpod" avec pod_id, **When** on appelle `create_provider()`, **Then** elle retourne un `RunPodProvider` configuré
4. **Given** un provider inconnu, **When** on appelle `create_provider()`, **Then** elle retourne une erreur claire comme `EngineFactory`

---

### User Story 3 - Providers Indépendants comme RpgMakerHandler (Priority: P1)

Chaque provider (`OllamaProvider`, `RunPodProvider`) est aussi indépendant et auto-suffisant que `RpgMakerHandler` ou `WolfRpgHandler`.

**Why this priority**: L'indépendance complète garantit la maintenabilité et permet l'ajout facile de nouveaux providers sans modifier le code existant.

**Independent Test**: Chaque provider peut être créé et testé indépendamment sans contexte externe, exactement comme `RpgMakerHandler::new()`.

**Acceptance Scenarios**:

1. **Given** `OllamaProvider`, **When** on le crée avec `new()`, **Then** il peut être créé sans contexte externe (pas de managers globaux)
2. **Given** `RunPodProvider`, **When** on le crée avec `new()`, **Then** il peut être créé avec juste `pod_id` et `config` (pas de cache global)
3. **Given** plusieurs instances de `OllamaProvider`, **When** on les crée, **Then** elles peuvent coexister indépendamment
4. **Given** plusieurs instances de `RunPodProvider` avec le même pod_id, **When** on les crée, **Then** chaque instance gère son propre cache interne
5. **Given** un nouveau provider (ex: OpenAI), **When** on l'ajoute, **Then** il peut être ajouté sans modifier les commands existantes

---

### User Story 4 - Commands Simplifiées comme dans Parsers (Priority: P1)

Le fichier `commands/translation.rs` utilise la factory pour obtenir le provider et délègue toute la logique, exactement comme les commands de parsing utilisent `EngineFactory`.

**Why this priority**: La simplification des commands garantit la cohérence et facilite la maintenance.

**Independent Test**: Les commands peuvent être comparées avec les commands de parsing et doivent suivre le même pattern de délégation.

**Acceptance Scenarios**:

1. **Given** `start_sequential_translation()`, **When** on examine son code, **Then** il utilise `TranslationProviderFactory::create_provider()` comme les commands de parsing utilisent `EngineFactory::create_handler()`
2. **Given** toutes les commands de translation, **When** on les examine, **Then** elles ne contiennent plus de `match provider` explicite ni de managers globaux
3. **Given** les APIs publiques des commands, **When** on les compare avec l'ancien code, **Then** elles sont identiques (backward compatibility)
4. **Given** un nouveau provider ajouté, **When** on utilise les commands existantes, **Then** elles fonctionnent sans modification

---

## Architecture Technique

### Structure Cible (Alignée avec Parsers)

```
translation/
├── provider.rs         # Trait TranslationProvider (comme handler.rs)
├── factory.rs          # TranslationProviderFactory (comme factory.rs)
├── common/
│   └── types.rs        # Types partagés (inchangé)
├── ollama/
│   ├── provider.rs     # OllamaProvider impl TranslationProvider (comme handler.rs)
│   ├── engine.rs       # OllamaEngine (logique métier, comme engine.rs)
│   ├── client.rs       # Existant (inchangé)
│   ├── single.rs       # Existant (inchangé)
│   └── sequential.rs   # Existant (inchangé)
└── runpod/
    ├── provider.rs     # RunPodProvider impl TranslationProvider (comme handler.rs)
    ├── engine.rs       # RunPodEngine (logique métier, comme engine.rs)
    ├── client.rs       # Existant (inchangé)
    ├── single.rs       # Existant (inchangé)
    └── sequential.rs   # Existant (inchangé)
```

### Comparaison Architecturale

| Aspect | Parsers | Translation (Cible) |
|--------|---------|-------------------|
| **Trait commun** | `GameEngineHandler` | `TranslationProvider` |
| **Factory** | `EngineFactory` | `TranslationProviderFactory` |
| **Handler/Provider** | `RpgMakerHandler` | `OllamaProvider` |
| **Engine** | `RpgMakerEngine` | `OllamaEngine` |
| **Indépendance** | ✅ Complète | ✅ Complète |
| **Managers globaux** | ❌ Aucun | ❌ Aucun |
| **Création** | `EngineFactory::create_handler()` | `TranslationProviderFactory::create_provider()` |
| **Utilisation** | `Box<dyn GameEngineHandler>` | `Box<dyn TranslationProvider>` |

### Trait TranslationProvider

```rust
/// Trait commun pour tous les providers de traduction
/// Suit le même pattern que GameEngineHandler dans parsers/
pub trait TranslationProvider: Send + Sync {
    /// Retourne le nom du provider (ex: "ollama", "runpod")
    fn provider_name(&self) -> &str;
    
    /// Vérifie le statut du provider et retourne des informations détaillées
    fn check_status(&self, config: ProviderConfig) -> Result<serde_json::Value, String>;
    
    /// Démarre une session de traduction séquentielle
    fn start_sequential_translation(
        &self,
        app: AppHandle,
        request: SequentialTranslationRequest,
    ) -> Result<String, String>; // Returns session_id
    
    /// Récupère le progrès d'une session de traduction séquentielle
    fn get_sequential_progress(
        &self,
        session_id: &str,
    ) -> Result<Option<SequentialProgress>, String>;
    
    /// Met en pause une session de traduction séquentielle
    fn pause_sequential_session(&self, session_id: &str) -> Result<(), String>;
    
    /// Reprend une session de traduction séquentielle
    fn resume_sequential_session(&self, session_id: &str) -> Result<(), String>;
    
    /// Arrête une session de traduction séquentielle
    fn stop_sequential_session(&self, session_id: &str) -> Result<(), String>;
    
    /// Traduit un texte individuel
    fn translate_single_text(
        &self,
        app: AppHandle,
        request: SingleTranslationRequest,
    ) -> Result<SingleTranslationResult, String>;
    
    /// Récupère des suggestions de traduction
    fn get_translation_suggestions(
        &self,
        app: AppHandle,
        source_text: &str,
        context: Option<&str>,
        count: usize,
    ) -> Result<Vec<TranslationSuggestion>, String>;
}
```

### Factory Pattern

```rust
/// Factory pour créer les providers de traduction appropriés
/// Suit le même pattern que EngineFactory dans parsers/
pub struct TranslationProviderFactory;

impl TranslationProviderFactory {
    /// Crée le provider approprié selon le type et la configuration
    /// 
    /// # Arguments
    /// 
    /// * `provider_type` - Type de provider ("ollama" ou "runpod")
    /// * `config` - Configuration du provider
    /// 
    /// # Returns
    /// 
    /// * `Ok(Box<dyn TranslationProvider>)` - Provider configuré
    /// * `Err(String)` - Erreur avec message détaillé
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// let provider = TranslationProviderFactory::create_provider(
    ///     "ollama",
    ///     ProviderConfig::default()
    /// )?;
    /// ```
    pub fn create_provider(
        provider_type: &str,
        config: ProviderConfig,
    ) -> Result<Box<dyn TranslationProvider>, String> {
        match provider_type {
            "ollama" => Ok(Box::new(OllamaProvider::new(config)?)),
            "runpod" => {
                let pod_id = config.pod_id.ok_or_else(|| {
                    "pod_id is required for RunPod provider".to_string()
                })?;
                Ok(Box::new(RunPodProvider::new(config, pod_id)?))
            }
            _ => Err(format!(
                "Unknown provider: {}. Supported providers: ollama, runpod",
                provider_type
            ))
        }
    }
}
```

### Structure OllamaProvider (Indépendant comme RpgMakerHandler)

```rust
/// Provider Ollama pour traduction locale
/// Suit le même pattern d'indépendance que RpgMakerHandler
pub struct OllamaProvider {
    // Managers encapsulés dans le provider (pas de singletons globaux)
    sequential_manager: Arc<OllamaSequentialManager>,
    single_manager: Arc<OllamaSingleManager>,
    client: Arc<OllamaClient>,
    config: OllamaConfig,
}

impl OllamaProvider {
    /// Crée un nouveau provider Ollama
    /// 
    /// # Arguments
    /// 
    /// * `config` - Configuration du provider
    /// 
    /// # Returns
    /// 
    /// * `Ok(Self)` - Provider configuré
    /// * `Err(String)` - Erreur de configuration
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// let provider = OllamaProvider::new(ProviderConfig::default())?;
    /// ```
    pub fn new(config: ProviderConfig) -> Result<Self, String> {
        // Création interne de tous les composants (pas de dépendance externe)
        let ollama_config = OllamaConfig {
            host: config.host.unwrap_or_else(|| "localhost".to_string()),
            port: config.port.unwrap_or(11434),
        };
        
        let client = Arc::new(OllamaClient::new(ollama_config.clone()));
        let single_manager = Arc::new(OllamaSingleManager::new(Arc::clone(&client)));
        let sequential_manager = Arc::new(OllamaSequentialManager::new(Arc::clone(&single_manager)));
        
        Ok(Self {
            sequential_manager,
            single_manager,
            client,
            config: ollama_config,
        })
    }
}

impl TranslationProvider for OllamaProvider {
    fn provider_name(&self) -> &str {
        "ollama"
    }
    
    // ... implémentation de toutes les méthodes du trait
}
```

### Structure RunPodProvider (Indépendant comme WolfRpgHandler)

```rust
/// Provider RunPod pour traduction en ligne
/// Suit le même pattern d'indépendance que WolfRpgHandler
pub struct RunPodProvider {
    // Cache interne géré par le provider (pas de cache global)
    managers_cache: Arc<Mutex<HashMap<String, RunPodManagers>>>,
    pod_id: String,
    config: RunPodConfig,
}

impl RunPodProvider {
    /// Crée un nouveau provider RunPod
    /// 
    /// # Arguments
    /// 
    /// * `config` - Configuration du provider
    /// * `pod_id` - ID du pod RunPod (requis)
    /// 
    /// # Returns
    /// 
    /// * `Ok(Self)` - Provider configuré
    /// * `Err(String)` - Erreur de configuration
    pub fn new(config: ProviderConfig, pod_id: String) -> Result<Self, String> {
        let runpod_config = RunPodConfig {
            pod_id: pod_id.clone(),
        };
        
        Ok(Self {
            managers_cache: Arc::new(Mutex::new(HashMap::new())),
            pod_id,
            config: runpod_config,
        })
    }
    
    /// Méthode interne pour obtenir les managers (gestion cache interne)
    async fn get_managers(&self) -> (Arc<RunPodSequentialManager>, Arc<RunPodSingleManager>) {
        let mut cache = self.managers_cache.lock().await;
        
        if let Some(existing) = cache.get(&self.pod_id) {
            return (Arc::clone(&existing.0), Arc::clone(&existing.1));
        }
        
        let client = Arc::new(RunPodClient::new(self.config.clone()));
        let single_manager = Arc::new(RunPodSingleManager::new(Arc::clone(&client)));
        let sequential_manager = Arc::new(RunPodSequentialManager::new(Arc::clone(&single_manager)));
        
        let managers_tuple = (Arc::clone(&sequential_manager), Arc::clone(&single_manager));
        cache.insert(self.pod_id.clone(), managers_tuple.clone());
        
        (sequential_manager, single_manager)
    }
}

impl TranslationProvider for RunPodProvider {
    fn provider_name(&self) -> &str {
        "runpod"
    }
    
    // ... implémentation de toutes les méthodes du trait
}
```

### Refactorisation commands/translation.rs

**Avant (Routage manuel)** :
```rust
match provider_enum {
    TranslationProvider::Ollama => {
        OLLAMA_SEQUENTIAL_MANAGER.start_session(app, request).await
    }
    TranslationProvider::RunPod => {
        let (manager, _) = get_runpod_managers(pod_id).await;
        manager.start_session(app, request).await
    }
}
```

**Après (Via factory, comme parsers)** :
```rust
let provider = TranslationProviderFactory::create_provider(&provider, config)?;
provider.start_sequential_translation(app, request).await
```

## Contraintes et Exigences

### Alignement Architectural

- ✅ **Structure identique** : `translation/` suit exactement la même structure que `parsers/`
- ✅ **Trait commun** : `TranslationProvider` suit le même pattern que `GameEngineHandler`
- ✅ **Factory identique** : `TranslationProviderFactory` suit le même pattern que `EngineFactory`
- ✅ **Indépendance complète** : Chaque provider est aussi indépendant que `RpgMakerHandler` ou `WolfRpgHandler`
- ✅ **Pas de singletons** : Aucun manager global, tout est encapsulé dans les providers

### Backward Compatibility

- ✅ Les APIs publiques des commands Tauri restent **inchangées**
- ✅ Les types de données restent **identiques**
- ✅ Aucun changement requis côté **frontend**
- ✅ Les tests existants doivent **continuer à passer**

### Principes d'Indépendance (Identiques aux Parsers)

Chaque provider doit être **complètement indépendant** :

- ✅ **Encapsulation complète** : Toute la logique interne est dans le provider
- ✅ **Pas de dépendances externes** : Les providers ne dépendent pas de `commands/translation.rs` ou de managers globaux
- ✅ **Auto-suffisance** : Chaque provider peut être créé et utilisé indépendamment sans contexte externe
- ✅ **Gestion interne** : Toute la création de clients, managers et cache est gérée dans le provider lui-même

## Tests d'Acceptation

### Test 1 : Structure Identique aux Parsers
1. Comparer la structure de `translation/provider.rs` avec `parsers/handler.rs`
2. Comparer la structure de `translation/factory.rs` avec `parsers/factory.rs`
3. Vérifier que les patterns sont identiques

### Test 2 : Indépendance Complète
1. Créer `OllamaProvider` sans contexte externe
2. Créer `RunPodProvider` sans contexte externe
3. Vérifier qu'aucun singleton global n'est utilisé
4. Vérifier que plusieurs instances peuvent coexister

### Test 3 : Factory Identique
1. Comparer `TranslationProviderFactory::create_provider()` avec `EngineFactory::create_handler()`
2. Vérifier que les patterns sont identiques
3. Vérifier que les erreurs sont gérées de la même manière

### Test 4 : Commands Simplifiées
1. Comparer `commands/translation.rs` avec les commands de parsing
2. Vérifier que le pattern de délégation est identique
3. Vérifier qu'aucun routage manuel n'existe

## Risques et Mitigation

### Risque 1 : Non-alignement Architectural
**Mitigation** : Comparaison systématique avec `parsers/` à chaque étape, validation que la structure est identique

### Risque 2 : Régression Fonctionnelle
**Mitigation** : Tests de régression complets avant/après, validation que tous les tests existants passent

### Risque 3 : Perte d'Indépendance
**Mitigation** : Validation que chaque provider peut être créé sans contexte externe, tests d'indépendance

## Dépendances

- Architecture parsers existante (`parsers/handler.rs`, `parsers/factory.rs`)
- Architecture translation existante (`translation/ollama/`, `translation/runpod/`)
- Commands Tauri existantes (`commands/translation.rs`)

## Documentation

- **README.md** : Vue d'ensemble de l'alignement architectural
- **plan.md** : Plan d'implémentation détaillé avec comparaison avec parsers
- **tasks.md** : Breakdown des tâches par phase avec validation d'alignement

