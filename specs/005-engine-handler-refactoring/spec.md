# Feature Specification: Refactorisation Architecture Handler Moteurs de Jeu

**Feature Branch**: `005-engine-handler-refactoring`  
**Created**: 2025-01-XX  
**Status**: Draft  
**Input**: Refactorisation de la logique de détection et d'utilisation des moteurs de jeu pour éliminer la duplication et améliorer la maintenabilité

**🎯 Objectif**: Refactoriser l'architecture actuelle pour créer un système factory avec des handlers indépendants pour chaque moteur de jeu, éliminant ainsi la logique spécifique aux moteurs dans `scanning.rs`, `injection.rs` et `projects.rs`.

## Contexte

Actuellement, la détection et l'utilisation des moteurs de jeu sont dispersées dans plusieurs fichiers :

1. **`scanning.rs`** : Contient des match explicites sur `GameEngine` enum et appelle directement `RpgMakerEngine::extract_all()` ou `WolfRpgEngine::extract_all()`
2. **`injection.rs`** : Contient des match explicites sur `GameEngine` enum et appelle directement les fonctions d'injection spécifiques
3. **`projects.rs`** : Contient une fonction `detect_game_engine()` qui duplique la logique de `parsers/engine.rs::detect_engine()`

Cette architecture crée plusieurs problèmes :
- **Duplication** : La détection de moteur est répétée dans 3 fichiers différents
- **Couplage** : Les commands Tauri sont directement couplés aux implémentations spécifiques
- **Maintenance** : Ajouter un nouveau moteur nécessite de modifier plusieurs fichiers
- **Violation DRY** : La logique de validation est dupliquée

## User Scenarios & Testing

### User Story 1 - Factory Pattern pour Détection Moteurs (Priority: P1)

L'application utilise un système factory centralisé pour détecter et créer les handlers appropriés pour chaque moteur de jeu.

**Why this priority**: Fondation de toute la refactorisation. Sans factory, on ne peut pas découpler les commands des implémentations.

**Independent Test**: Peut être testé en créant des structures de projet différentes et vérifiant que la factory retourne le bon handler.

**Acceptance Scenarios**:

1. **Given** un projet RPG Maker MZ (package.json + data/), **When** la factory détecte le projet, **Then** elle retourne un `RpgMakerHandler` configuré pour MZ
2. **Given** un projet RPG Maker MV (www/data/), **When** la factory détecte le projet, **Then** elle retourne un `RpgMakerHandler` configuré pour MV
3. **Given** un projet WolfRPG (dump/ avec db/, mps/, common/), **When** la factory détecte le projet, **Then** elle retourne un `WolfRpgHandler`
4. **Given** un projet avec fichiers chiffrés WolfRPG (Data.wolf), **When** la factory détecte le projet, **Then** elle retourne un `WolfRpgHandler` avec indication de chiffrement
5. **Given** un projet non reconnu, **When** la factory tente la détection, **Then** elle retourne une erreur claire avec suggestions

---

### User Story 2 - Trait GameEngineHandler pour Abstraction (Priority: P1)

Tous les moteurs de jeu implémentent un trait commun `GameEngineHandler` qui expose une interface uniforme pour extraction, injection et validation.

**Why this priority**: Le trait est l'abstraction centrale qui permet aux commands d'utiliser n'importe quel moteur sans connaître l'implémentation.

**Independent Test**: Peut être testé en créant des implémentations mock du trait et vérifiant que les commands fonctionnent avec elles.

**Acceptance Scenarios**:

1. **Given** un handler implémentant `GameEngineHandler`, **When** on appelle `validate_project_structure()`, **Then** il retourne un résultat de validation avec erreurs/warnings détaillés
2. **Given** un handler implémentant `GameEngineHandler`, **When** on appelle `extract_all_texts()`, **Then** il retourne une liste de `TextEntry` extraits
3. **Given** un handler implémentant `GameEngineHandler`, **When** on appelle `inject_all_texts()`, **Then** il injecte les traductions dans les fichiers appropriés
4. **Given** un handler implémentant `GameEngineHandler`, **When** on appelle `get_engine_name()`, **Then** il retourne le nom lisible du moteur (ex: "RPG Maker MZ")
5. **Given** un handler implémentant `GameEngineHandler`, **When** on appelle `count_files_to_process()`, **Then** il retourne le nombre de fichiers qui seront traités

---

### User Story 3 - Refactorisation scanning.rs (Priority: P1)

Le fichier `scanning.rs` utilise la factory pour obtenir le handler approprié et délègue toute la logique spécifique au moteur au handler.

**Why this priority**: `scanning.rs` est un fichier critique utilisé pour l'extraction initiale des textes.

**Independent Test**: Peut être testé en vérifiant que les extractions fonctionnent identiquement avant et après refactorisation.

**Acceptance Scenarios**:

1. **Given** un projet RPG Maker MZ valide, **When** `extract_texts_from_folder()` est appelé, **Then** il utilise la factory pour obtenir le handler et extrait les textes correctement
2. **Given** un projet WolfRPG valide, **When** `extract_texts_from_folder()` est appelé, **Then** il utilise la factory pour obtenir le handler et extrait les textes correctement
3. **Given** un projet invalide, **When** `extract_texts_from_folder()` est appelé, **Then** il retourne une erreur avec message détaillé du handler
4. **Given** un projet non reconnu, **When** `extract_texts_from_folder()` est appelé, **Then** il retourne une erreur avec suggestions de structure attendue
5. **Given** aucun changement dans les APIs publiques, **When** les tests existants s'exécutent, **Then** ils passent sans modification

---

### User Story 4 - Refactorisation injection.rs (Priority: P1)

Le fichier `injection.rs` utilise la factory pour obtenir le handler approprié et délègue toute la logique spécifique au moteur au handler.

**Why this priority**: `injection.rs` est un fichier critique utilisé pour la réinjection des traductions.

**Independent Test**: Peut être testé en vérifiant que les injections fonctionnent identiquement avant et après refactorisation.

**Acceptance Scenarios**:

1. **Given** un projet RPG Maker MZ avec traductions, **When** `start_injection()` est appelé, **Then** il utilise la factory pour obtenir le handler et injecte les traductions correctement
2. **Given** un projet WolfRPG avec traductions, **When** `start_injection()` est appelé, **Then** il utilise la factory pour obtenir le handler et injecte les traductions correctement
3. **Given** un projet invalide, **When** `validate_injection()` est appelé, **Then** il utilise le handler pour valider et retourne des issues détaillées
4. **Given** aucun changement dans les APIs publiques, **When** les tests existants s'exécutent, **Then** ils passent sans modification
5. **Given** une injection partielle, **When** elle échoue, **Then** les erreurs sont détaillées par le handler

---

### User Story 5 - Refactorisation projects.rs (Priority: P1)

Le fichier `projects.rs` utilise la factory pour obtenir le handler approprié et délègue toute la logique de validation au handler.

**Why this priority**: `projects.rs` est utilisé pour la validation des chemins de projet lors de la création de projets.

**Independent Test**: Peut être testé en vérifiant que les validations fonctionnent identiquement avant et après refactorisation.

**Acceptance Scenarios**:

1. **Given** un chemin RPG Maker MZ valide, **When** `validate_game_path()` est appelé, **Then** il utilise la factory pour obtenir le handler et valide correctement
2. **Given** un chemin WolfRPG valide, **When** `validate_game_path()` est appelé, **Then** il utilise la factory pour obtenir le handler et valide correctement
3. **Given** un chemin invalide, **When** `validate_game_path()` est appelé, **Then** il retourne des erreurs détaillées du handler
4. **Given** un chemin non reconnu, **When** `validate_game_path()` est appelé, **Then** il retourne des warnings avec suggestions
5. **Given** aucun changement dans les APIs publiques, **When** les tests existants s'exécutent, **Then** ils passent sans modification

---

### User Story 6 - Handlers Indépendants par Moteur (Priority: P1)

Chaque moteur de jeu a son propre handler qui encapsule toute la logique spécifique (validation, extraction, injection, comptage fichiers).

**Why this priority**: Les handlers indépendants permettent une maintenance et une extension faciles.

**Independent Test**: Chaque handler peut être testé indépendamment avec des structures de projet mock.

**Acceptance Scenarios**:

1. **Given** `RpgMakerHandler`, **When** il est créé pour MZ, **Then** il connaît automatiquement la structure MZ (data/, package.json)
2. **Given** `RpgMakerHandler`, **When** il est créé pour MV, **Then** il connaît automatiquement la structure MV (www/data/)
3. **Given** `WolfRpgHandler`, **When** il est créé, **Then** il connaît automatiquement la structure WolfRPG (dump/db/, dump/mps/, dump/common/)
4. **Given** un nouveau moteur (ex: Baki), **When** on crée un nouveau handler, **Then** il peut être ajouté sans modifier les commands existants
5. **Given** chaque handler, **When** on appelle ses méthodes, **Then** elles retournent des résultats cohérents avec la structure du moteur

---

## Architecture Technique

### Trait GameEngineHandler

```rust
pub trait GameEngineHandler: Send + Sync {
    /// Nom lisible du moteur (ex: "RPG Maker MZ", "Wolf RPG Editor")
    fn engine_name(&self) -> &str;
    
    /// Valide la structure du projet et retourne des erreurs/warnings détaillés
    fn validate_project_structure(&self, game_path: &Path) -> Result<ValidationResult, String>;
    
    /// Extrait tous les textes traduisibles du projet
    fn extract_all_texts(&self, game_path: &Path) -> Result<Vec<TextEntry>, String>;
    
    /// Injecte les traductions dans les fichiers du projet
    fn inject_all_texts(&self, game_path: &Path, translations: &[TranslationEntry]) -> Result<(), String>;
    
    /// Compte le nombre de fichiers qui seront traités
    fn count_files_to_process(&self, game_path: &Path) -> usize;
    
    /// Retourne le chemin racine des données (ex: "data/", "www/data/", "dump/")
    fn get_data_root(&self, game_path: &Path) -> PathBuf;
}
```

### Factory Pattern

```rust
pub struct EngineFactory;

impl EngineFactory {
    /// Détecte le moteur et crée le handler approprié
    pub fn create_handler(game_path: &Path) -> Result<Box<dyn GameEngineHandler>, String> {
        // Logique de détection centralisée
        // Retourne le handler approprié
    }
}
```

### Structure des Handlers

```
parsers/
├── handler.rs (trait GameEngineHandler)
├── factory.rs (EngineFactory)
├── rpg_maker/
│   └── handler.rs (RpgMakerHandler impl GameEngineHandler)
└── wolfrpg/
    └── handler.rs (WolfRpgHandler impl GameEngineHandler)
```

## Contraintes et Exigences

### Backward Compatibility

- ✅ Les APIs publiques des commands Tauri restent **inchangées**
- ✅ Les types de données (`TextEntry`, `TranslationEntry`) restent **identiques**
- ✅ Aucun changement requis côté **frontend**
- ✅ Les tests existants doivent **continuer à passer**

### Performance

- La factory ne doit pas introduire de overhead significatif
- La détection doit rester < 2 secondes
- L'extraction/injection doivent avoir les mêmes performances qu'avant

### Maintenabilité

- Chaque handler doit être testable indépendamment
- Ajouter un nouveau moteur ne doit nécessiter que :
  1. Créer un nouveau handler implémentant le trait
  2. Ajouter la détection dans la factory
  3. Aucune modification des commands existantes

## Tests d'Acceptation

### Test 1 : Factory Détection Correcte
1. Créer des structures de projet pour chaque moteur
2. Appeler `EngineFactory::create_handler()` pour chaque structure
3. Vérifier que le bon handler est retourné
4. Vérifier que les handlers ont les bonnes configurations

### Test 2 : Extraction Identique
1. Extraire des textes avec l'ancien code (baseline)
2. Extraire des textes avec le nouveau code (refactorisé)
3. Comparer les résultats : doivent être identiques

### Test 3 : Injection Identique
1. Injecter des traductions avec l'ancien code (baseline)
2. Injecter des traductions avec le nouveau code (refactorisé)
3. Comparer les fichiers modifiés : doivent être identiques

### Test 4 : Validation Identique
1. Valider des projets avec l'ancien code (baseline)
2. Valider des projets avec le nouveau code (refactorisé)
3. Comparer les résultats : doivent être identiques

### Test 5 : Extensibilité
1. Créer un handler mock pour un nouveau moteur
2. Ajouter la détection dans la factory
3. Vérifier que les commands existantes fonctionnent avec le nouveau handler
4. Aucune modification des commands requise

## Risques et Mitigation

### Risque 1 : Régression Fonctionnelle
**Mitigation** : Tests de régression complets avant/après, validation que tous les tests existants passent

### Risque 2 : Performance Dégradée
**Mitigation** : Benchmarks avant/après, optimisation de la factory si nécessaire

### Risque 3 : Complexité Accrue
**Mitigation** : Documentation complète, code review, architecture claire et modulaire

## Dépendances

- Architecture parsers existante (`parsers/rpg_maker/`, `parsers/wolfrpg/`)
- Commands Tauri existantes (`scanning.rs`, `injection.rs`, `projects.rs`)
- Types communs (`TextEntry`, `TranslationEntry`, `GameEngine`)

## Documentation

- **README.md** : Vue d'ensemble de la refactorisation
- **plan.md** : Plan d'implémentation détaillé avec architecture technique
- **tasks.md** : Breakdown des tâches par phase

