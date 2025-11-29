# Feature Specification: Factory Pattern pour Providers de Traduction

**Feature Branch**: `009-translation-factory`  
**Created**: 2025-01-XX  
**Status**: Draft  
**Input**: Refactorisation de l'architecture de traduction pour utiliser un pattern factory similaire à celui utilisé dans `parsers`, permettant de découpler `commands/translation.rs` des implémentations spécifiques des providers (Ollama, RunPod).

**🎯 Objectif**: Créer un système factory avec un trait `TranslationProvider` pour abstraire les providers de traduction, éliminant ainsi la logique spécifique aux providers dans `commands/translation.rs` et permettant une extension facile vers de nouveaux providers.

## Contexte

Actuellement, la gestion des providers de traduction est dispersée dans `commands/translation.rs` :

1. **`commands/translation.rs`** : Contient des match explicites sur `provider: String` ("ollama" | "runpod") et appelle directement les managers spécifiques (`OLLAMA_SEQUENTIAL_MANAGER`, `RUNPOD_MANAGERS_CACHE`, etc.)
2. **Logique dupliquée** : Chaque command répète la même logique de routing selon le provider
3. **Couplage fort** : Les commands Tauri sont directement couplés aux implémentations spécifiques
4. **Maintenance difficile** : Ajouter un nouveau provider nécessite de modifier toutes les commands

Cette architecture crée plusieurs problèmes :
- **Duplication** : La logique de routing est répétée dans chaque command
- **Couplage** : Les commands sont directement couplés aux implémentations spécifiques
- **Maintenance** : Ajouter un nouveau provider nécessite de modifier plusieurs fonctions
- **Violation DRY** : La logique de conversion et de routing est dupliquée

## User Scenarios & Testing

### User Story 1 - Trait TranslationProvider pour Abstraction (Priority: P1)

Tous les providers de traduction implémentent un trait commun `TranslationProvider` qui expose une interface uniforme pour les opérations de traduction.

**Why this priority**: Le trait est l'abstraction centrale qui permet aux commands d'utiliser n'importe quel provider sans connaître l'implémentation.

**Independent Test**: Peut être testé en créant des implémentations mock du trait et vérifiant que les commands fonctionnent avec elles.

**Acceptance Scenarios**:

1. **Given** un provider implémentant `TranslationProvider`, **When** on appelle `check_status()`, **Then** il retourne le statut du provider avec informations détaillées
2. **Given** un provider implémentant `TranslationProvider`, **When** on appelle `start_sequential_translation()`, **Then** il démarre une session de traduction séquentielle
3. **Given** un provider implémentant `TranslationProvider`, **When** on appelle `get_sequential_progress()`, **Then** il retourne le progrès de la session
4. **Given** un provider implémentant `TranslationProvider`, **When** on appelle `translate_single_text()`, **Then** il traduit un texte individuel
5. **Given** un provider implémentant `TranslationProvider`, **When** on appelle `get_translation_suggestions()`, **Then** il retourne des suggestions de traduction

---

### User Story 2 - Factory Pattern pour Création Providers (Priority: P1)

L'application utilise un système factory centralisé pour créer les providers appropriés selon le type demandé.

**Why this priority**: Fondation de toute la refactorisation. Sans factory, on ne peut pas découpler les commands des implémentations.

**Independent Test**: Peut être testé en demandant différents types de providers et vérifiant que la factory retourne le bon provider.

**Acceptance Scenarios**:

1. **Given** un provider "ollama", **When** la factory crée le provider, **Then** elle retourne un `OllamaProvider` configuré
2. **Given** un provider "runpod" avec pod_id, **When** la factory crée le provider, **Then** elle retourne un `RunPodProvider` configuré avec le pod_id
3. **Given** un provider inconnu, **When** la factory tente la création, **Then** elle retourne une erreur claire
4. **Given** un provider "runpod" sans pod_id, **When** la factory tente la création, **Then** elle retourne une erreur indiquant que pod_id est requis
5. **Given** un provider créé, **When** on l'utilise pour traduire, **Then** il fonctionne correctement avec sa configuration

---

### User Story 3 - Refactorisation translation.rs (Priority: P1)

Le fichier `commands/translation.rs` utilise la factory pour obtenir le provider approprié et délègue toute la logique spécifique au provider.

**Why this priority**: `translation.rs` est le fichier critique utilisé pour toutes les opérations de traduction.

**Independent Test**: Peut être testé en vérifiant que les traductions fonctionnent identiquement avant et après refactorisation.

**Acceptance Scenarios**:

1. **Given** un provider "ollama", **When** `start_sequential_translation()` est appelé, **Then** il utilise la factory pour obtenir le provider et démarre la traduction correctement
2. **Given** un provider "runpod" avec pod_id, **When** `start_sequential_translation()` est appelé, **Then** il utilise la factory pour obtenir le provider et démarre la traduction correctement
3. **Given** un provider invalide, **When** une command est appelée, **Then** elle retourne une erreur avec message détaillé
4. **Given** aucun changement dans les APIs publiques, **When** les tests existants s'exécutent, **Then** ils passent sans modification
5. **Given** une traduction en cours, **When** on récupère le progrès, **Then** il utilise le provider approprié pour obtenir le progrès

---

### User Story 4 - Providers Indépendants (Priority: P1)

Chaque provider de traduction a sa propre implémentation qui encapsule toute la logique spécifique (clients, managers, configuration).

**Why this priority**: Les providers indépendants permettent une maintenance et une extension faciles.

**Independent Test**: Chaque provider peut être testé indépendamment avec des configurations mock.

**Acceptance Scenarios**:

1. **Given** `OllamaProvider`, **When** il est créé, **Then** il encapsule `OllamaClient`, `OllamaSingleManager`, `OllamaSequentialManager`
2. **Given** `RunPodProvider`, **When** il est créé avec pod_id, **Then** il encapsule `RunPodClient`, `RunPodSingleManager`, `RunPodSequentialManager` avec le pod_id
3. **Given** un nouveau provider (ex: OpenAI), **When** on crée un nouveau provider, **Then** il peut être ajouté sans modifier les commands existantes
4. **Given** chaque provider, **When** on appelle ses méthodes, **Then** elles utilisent la bonne implémentation (Ollama local, RunPod HTTP, etc.)
5. **Given** un provider RunPod, **When** on crée plusieurs instances avec le même pod_id, **Then** elles partagent la même session/cache

---

## Architecture Technique

### Trait TranslationProvider

```rust
pub trait TranslationProvider: Send + Sync {
    /// Nom du provider (ex: "ollama", "runpod")
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
    ) -> Result<SequentialProgress, String>;
    
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
pub struct TranslationFactory;

impl TranslationFactory {
    /// Crée le provider approprié selon le type et la configuration
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
            _ => Err(format!("Unknown provider: {}", provider_type))
        }
    }
}
```

### Structure des Providers

```
translation/
├── provider.rs (trait TranslationProvider)
├── factory.rs (TranslationFactory)
├── ollama/
│   └── provider.rs (OllamaProvider impl TranslationProvider)
└── runpod/
    └── provider.rs (RunPodProvider impl TranslationProvider)
```

### Indépendance des Providers

Chaque provider doit être **complètement indépendant** et auto-suffisant, similaire à `RpgMakerHandler` et `WolfRpgHandler` :

#### Principe d'Indépendance

- ✅ **Encapsulation complète** : Chaque provider encapsule toute sa logique interne (clients, managers, cache)
- ✅ **Pas de dépendances externes** : Les providers ne dépendent pas de `commands/translation.rs` ou de managers globaux
- ✅ **Auto-suffisance** : Chaque provider peut être créé et utilisé indépendamment sans contexte externe
- ✅ **Gestion interne** : Toute la création de clients, managers et cache est gérée dans le provider lui-même

#### Structure OllamaProvider

```rust
pub struct OllamaProvider {
    // Managers encapsulés dans le provider
    sequential_manager: Arc<OllamaSequentialManager>,
    single_manager: Arc<OllamaSingleManager>,
    config: OllamaConfig,
}

impl OllamaProvider {
    pub fn new(config: ProviderConfig) -> Result<Self, String> {
        // Création interne des clients et managers
        // Aucune dépendance externe
        let ollama_config = OllamaConfig {
            host: config.host.unwrap_or_else(|| "localhost".to_string()),
            port: config.port.unwrap_or(11434),
        };
        let client = Arc::new(OllamaClient::new(ollama_config));
        let single_manager = Arc::new(OllamaSingleManager::new(Arc::clone(&client)));
        let sequential_manager = Arc::new(OllamaSequentialManager::new(Arc::clone(&single_manager)));
        
        Ok(Self {
            sequential_manager,
            single_manager,
            config: ollama_config,
        })
    }
}
```

#### Structure RunPodProvider

```rust
pub struct RunPodProvider {
    // Cache interne géré par le provider
    managers_cache: Arc<Mutex<HashMap<String, RunPodManagers>>>,
    pod_id: String,
    config: RunPodConfig,
}

impl RunPodProvider {
    pub fn new(config: ProviderConfig, pod_id: String) -> Result<Self, String> {
        // Création interne du cache et des managers
        // Aucune dépendance externe
        let runpod_config = RunPodConfig {
            pod_id: pod_id.clone(),
        };
        
        Ok(Self {
            managers_cache: Arc::new(Mutex::new(HashMap::new())),
            pod_id,
            config: runpod_config,
        })
    }
    
    // Méthode interne pour obtenir les managers (gestion cache interne)
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
```

#### Comparaison avec Parsers

| Aspect | Parsers (RpgMakerHandler) | Translation (OllamaProvider) |
|--------|---------------------------|------------------------------|
| **Encapsulation** | ✅ Toute la logique dans le handler | ✅ Toute la logique dans le provider |
| **Création** | Via `EngineFactory::create_handler()` | Via `TranslationFactory::create_provider()` |
| **Dépendances** | ❌ Aucune dépendance externe | ❌ Aucune dépendance externe |
| **Managers globaux** | ❌ Pas de managers globaux | ❌ Pas de managers globaux |
| **Utilisation** | `Box<dyn GameEngineHandler>` | `Box<dyn TranslationProvider>` |
| **Indépendance** | ✅ Complètement indépendant | ✅ Complètement indépendant |

### Types Communs

```rust
/// Configuration pour un provider
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub pod_id: Option<String>, // Requis pour RunPod
    pub host: Option<String>,   // Pour Ollama
    pub port: Option<u16>,      // Pour Ollama
    pub model: Option<String>,
    pub source_language: Option<String>,
    pub target_language: Option<String>,
}

/// Requête de traduction séquentielle (format commun)
#[derive(Debug, Clone)]
pub struct SequentialTranslationRequest {
    pub project_id: i64,
    pub texts: Vec<TranslationText>,
    pub start_from: Option<i32>,
    pub source_language: Option<String>,
    pub target_language: Option<String>,
    pub model: Option<String>,
}

/// Requête de traduction individuelle (format commun)
#[derive(Debug, Clone)]
pub struct SingleTranslationRequest {
    pub source_text: String,
    pub source_language: Option<String>,
    pub target_language: Option<String>,
    pub context: Option<String>,
    pub model: Option<String>,
    pub project_id: Option<i64>,
    pub text_type: Option<String>,
}
```

## Contraintes et Exigences

### Backward Compatibility

- ✅ Les APIs publiques des commands Tauri restent **inchangées**
- ✅ Les types de données (`TranslationText`, `SequentialProgress`, etc.) restent **identiques**
- ✅ Aucun changement requis côté **frontend**
- ✅ Les tests existants doivent **continuer à passer**

### Performance

- La factory ne doit pas introduire de overhead significatif
- La création de provider doit rester < 100ms
- Les traductions doivent avoir les mêmes performances qu'avant

### Maintenabilité

- Chaque provider doit être testable indépendamment
- Ajouter un nouveau provider ne doit nécessiter que :
  1. Créer un nouveau provider implémentant le trait
  2. Ajouter la création dans la factory
  3. Aucune modification des commands existantes

### Gestion des Sessions

- Les sessions RunPod doivent être partagées par pod_id (cache interne au provider)
- Les sessions Ollama sont gérées par chaque instance de provider
- Le cache RunPod est géré **interne au RunPodProvider**, pas globalement
- Chaque instance de provider gère ses propres ressources de manière indépendante

## Tests d'Acceptation

### Test 1 : Factory Création Correcte
1. Créer des providers pour chaque type ("ollama", "runpod")
2. Appeler `TranslationFactory::create_provider()` pour chaque type
3. Vérifier que le bon provider est retourné
4. Vérifier que les providers ont les bonnes configurations

### Test 2 : Traduction Identique
1. Traduire des textes avec l'ancien code (baseline)
2. Traduire des textes avec le nouveau code (refactorisé)
3. Comparer les résultats : doivent être identiques

### Test 3 : Gestion Sessions Identique
1. Démarrer des sessions avec l'ancien code (baseline)
2. Démarrer des sessions avec le nouveau code (refactorisé)
3. Comparer les comportements : doivent être identiques

### Test 4 : Extensibilité
1. Créer un provider mock pour un nouveau provider
2. Ajouter la création dans la factory
3. Vérifier que les commands existantes fonctionnent avec le nouveau provider
4. Aucune modification des commands requise

## Risques et Mitigation

### Risque 1 : Régression Fonctionnelle
**Mitigation** : Tests de régression complets avant/après, validation que tous les tests existants passent

### Risque 2 : Performance Dégradée
**Mitigation** : Benchmarks avant/après, optimisation de la factory si nécessaire

### Risque 3 : Complexité Accrue
**Mitigation** : Documentation complète, code review, architecture claire et modulaire

### Risque 4 : Gestion Cache RunPod
**Mitigation** : Préserver le comportement existant du cache RunPod, tests spécifiques pour le cache

## Dépendances

- Architecture translation existante (`translation/ollama/`, `translation/runpod/`)
- Commands Tauri existantes (`commands/translation.rs`)
- Types communs (`TranslationText`, `SequentialProgress`, etc.)

## Documentation

- **README.md** : Vue d'ensemble de la refactorisation
- **plan.md** : Plan d'implémentation détaillé avec architecture technique
- **tasks.md** : Breakdown des tâches par phase

