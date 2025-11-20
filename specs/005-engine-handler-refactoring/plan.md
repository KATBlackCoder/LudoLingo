# Plan d'Implémentation : Refactorisation Architecture Handler Moteurs

## Vue d'Ensemble

Ce plan décrit l'implémentation de la refactorisation pour créer un système factory avec des handlers indépendants pour chaque moteur de jeu. L'objectif est d'éliminer la duplication de code et de découpler les commands Tauri des implémentations spécifiques des moteurs.

## Architecture Cible

### Structure des Fichiers

```
src-tauri/src/parsers/
├── mod.rs                    # Exports mis à jour
├── engine.rs                 # Types communs uniquement (TextEntry, TranslationEntry, etc.)
├── handler.rs                # NOUVEAU - Trait GameEngineHandler
├── factory.rs                 # NOUVEAU - EngineFactory pour détection et création
├── rpg_maker/
│   ├── mod.rs
│   ├── engine.rs             # Gardé pour compatibilité (déprécié progressivement)
│   ├── handler.rs            # NOUVEAU - RpgMakerHandler impl GameEngineHandler
│   └── files/
│       └── handler.rs        # Fonctions existantes (extract_all_texts, inject_all_texts)
└── wolfrpg/
    ├── mod.rs
    ├── engine.rs             # Gardé pour compatibilité (déprécié progressivement)
    ├── handler.rs            # NOUVEAU - WolfRpgHandler impl GameEngineHandler
    └── files/
        └── handler.rs        # Fonctions existantes (extract_all_texts, inject_all_texts)
```

### Trait GameEngineHandler

```rust
use std::path::{Path, PathBuf};
use crate::parsers::engine::{TextEntry, TranslationEntry};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub detected_engine: Option<String>,
}

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
use std::path::Path;
use crate::parsers::handler::GameEngineHandler;
use crate::parsers::rpg_maker::handler::RpgMakerHandler;
use crate::parsers::wolfrpg::handler::WolfRpgHandler;

pub struct EngineFactory;

impl EngineFactory {
    /// Détecte le moteur et crée le handler approprié
    pub fn create_handler(game_path: &Path) -> Result<Box<dyn GameEngineHandler>, String> {
        // 1. Vérifier WolfRPG (dump/ avec db/, mps/, common/)
        let dump_folder = game_path.join("dump");
        if dump_folder.exists() && dump_folder.is_dir() {
            let db_dir = dump_folder.join("db");
            let mps_dir = dump_folder.join("mps");
            let common_dir = dump_folder.join("common");

            if db_dir.exists() && db_dir.is_dir()
                && mps_dir.exists() && mps_dir.is_dir()
                && common_dir.exists() && common_dir.is_dir()
            {
                return Ok(Box::new(WolfRpgHandler::new()));
            }
        }

        // 2. Vérifier WolfRPG chiffré (Data.wolf)
        let data_wolf = game_path.join("Data.wolf");
        if data_wolf.exists() && data_wolf.is_file() {
            return Ok(Box::new(WolfRpgHandler::new()));
        }

        // 3. Vérifier RPG Maker MZ (package.json + data/)
        let package_json = game_path.join("package.json");
        let data_folder = game_path.join("data");
        if package_json.exists() && data_folder.is_dir() {
            return Ok(Box::new(RpgMakerHandler::new_mz()));
        }

        // 4. Vérifier RPG Maker MV (www/data/)
        let www_data_folder = game_path.join("www").join("data");
        if www_data_folder.is_dir() {
            return Ok(Box::new(RpgMakerHandler::new_mv()));
        }

        Err(format!(
            "Structure de projet non reconnue dans '{}'. \
            Pour RPG Maker MZ : doit contenir 'package.json' et dossier 'data/'. \
            Pour RPG Maker MV : doit contenir dossier 'www/data/'. \
            Pour Wolf RPG Editor : doit contenir dossier 'dump/' avec 'db/', 'mps/', et 'common/', ou fichier 'Data.wolf'.",
            game_path.display()
        ))
    }
}
```

## Phases d'Implémentation

### Phase 1 : Création du Trait et Factory (1-2 jours)

**Objectif** : Créer les abstractions de base (trait et factory) sans modifier le code existant.

**Tâches** :
1. Créer `parsers/handler.rs` avec le trait `GameEngineHandler`
2. Créer `parsers/factory.rs` avec `EngineFactory`
3. Créer `parsers/rpg_maker/handler.rs` avec `RpgMakerHandler` (implémentation basique)
4. Créer `parsers/wolfrpg/handler.rs` avec `WolfRpgHandler` (implémentation basique)
5. Mettre à jour `parsers/mod.rs` pour exporter les nouveaux modules
6. Tests unitaires pour la factory (détection correcte)

**Délivrables** :
- Trait `GameEngineHandler` défini
- Factory `EngineFactory` fonctionnelle
- Handlers basiques créés (peuvent être incomplets à ce stade)
- Tests de détection passent

### Phase 2 : Implémentation RpgMakerHandler (1 jour)

**Objectif** : Implémenter complètement `RpgMakerHandler` en encapsulant la logique existante de `RpgMakerEngine`.

**Tâches** :
1. Implémenter `engine_name()` pour MZ et MV
2. Implémenter `validate_project_structure()` en utilisant `RpgMakerEngine::validate_project_structure()`
3. Implémenter `extract_all_texts()` en utilisant `RpgMakerEngine::extract_all()`
4. Implémenter `inject_all_texts()` en utilisant `RpgMakerEngine::inject_all()`
5. Implémenter `count_files_to_process()` avec logique RPG Maker
6. Implémenter `get_data_root()` en utilisant `RpgMakerEngine::get_data_root()`
7. Tests unitaires complets pour `RpgMakerHandler`

**Délivrables** :
- `RpgMakerHandler` complètement implémenté
- Tous les tests passent
- Compatibilité avec code existant vérifiée

### Phase 3 : Implémentation WolfRpgHandler (1 jour)

**Objectif** : Implémenter complètement `WolfRpgHandler` en encapsulant la logique existante de `WolfRpgEngine`.

**Tâches** :
1. Implémenter `engine_name()` pour WolfRPG
2. Implémenter `validate_project_structure()` en utilisant `WolfRpgEngine::validate_project_structure()`
3. Implémenter `extract_all_texts()` en utilisant `WolfRpgEngine::extract_all()`
4. Implémenter `inject_all_texts()` en utilisant `WolfRpgEngine::inject_all()`
5. Implémenter `count_files_to_process()` avec logique WolfRPG
6. Implémenter `get_data_root()` en utilisant `WolfRpgEngine::get_data_root()`
7. Tests unitaires complets pour `WolfRpgHandler`

**Délivrables** :
- `WolfRpgHandler` complètement implémenté
- Tous les tests passent
- Compatibilité avec code existant vérifiée

### Phase 4 : Refactorisation scanning.rs (1 jour)

**Objectif** : Refactoriser `scanning.rs` pour utiliser la factory au lieu de la détection manuelle.

**Tâches** :
1. Remplacer `detect_engine()` par `EngineFactory::create_handler()`
2. Remplacer les match explicites sur `GameEngine` par des appels au handler
3. Utiliser `handler.extract_all_texts()` au lieu de `RpgMakerEngine::extract_all()` ou `WolfRpgEngine::extract_all()`
4. Mettre à jour les messages d'erreur pour utiliser `handler.engine_name()`
5. Tests de régression pour vérifier que tout fonctionne identiquement
6. Supprimer les imports inutiles (`RpgMakerEngine`, `WolfRpgEngine`)

**Délivrables** :
- `scanning.rs` refactorisé et simplifié
- Tous les tests existants passent
- Aucune régression fonctionnelle

### Phase 5 : Refactorisation injection.rs (1 jour)

**Objectif** : Refactoriser `injection.rs` pour utiliser la factory au lieu des match explicites.

**Tâches** :
1. Remplacer `detect_engine()` par `EngineFactory::create_handler()`
2. Remplacer les match explicites sur `GameEngine` par des appels au handler
3. Utiliser `handler.inject_all_texts()` au lieu des fonctions spécifiques
4. Utiliser `handler.count_files_to_process()` au lieu de `count_files_to_process()`
5. Utiliser `handler.validate_project_structure()` dans `validate_injection()`
6. Mettre à jour les messages d'erreur pour utiliser `handler.engine_name()`
7. Tests de régression pour vérifier que tout fonctionne identiquement
8. Supprimer les imports inutiles

**Délivrables** :
- `injection.rs` refactorisé et simplifié
- Tous les tests existants passent
- Aucune régression fonctionnelle

### Phase 6 : Refactorisation projects.rs (1 jour)

**Objectif** : Refactoriser `projects.rs` pour utiliser la factory au lieu de la détection manuelle.

**Tâches** :
1. Supprimer la fonction `detect_game_engine()` (dupliquée)
2. Remplacer par `EngineFactory::create_handler()`
3. Utiliser `handler.validate_project_structure()` dans `validate_game_path()`
4. Utiliser `handler.engine_name()` pour `detected_engine`
5. Mettre à jour les messages d'erreur pour utiliser le handler
6. Tests de régression pour vérifier que tout fonctionne identiquement
7. Supprimer les imports inutiles

**Délivrables** :
- `projects.rs` refactorisé et simplifié
- Tous les tests existants passent
- Aucune régression fonctionnelle
- Plus de duplication de code de détection

### Phase 7 : Tests et Validation (1 jour)

**Objectif** : Valider que toute la refactorisation fonctionne correctement et qu'il n'y a pas de régression.

**Tâches** :
1. Exécuter tous les tests unitaires existants
2. Exécuter tous les tests d'intégration existants
3. Tests manuels avec projets réels (RPG Maker MV, MZ, WolfRPG)
4. Vérifier les performances (benchmarks avant/après)
5. Vérifier que les APIs publiques sont inchangées
6. Documentation des changements internes

**Délivrables** :
- Tous les tests passent
- Aucune régression détectée
- Performances identiques ou meilleures
- Documentation à jour

### Phase 8 : Nettoyage et Documentation (0.5 jour)

**Objectif** : Nettoyer le code et documenter l'architecture finale.

**Tâches** :
1. Supprimer le code mort (si `RpgMakerEngine` et `WolfRpgEngine` ne sont plus utilisés directement)
2. Ajouter des commentaires de documentation sur le trait et la factory
3. Mettre à jour les commentaires dans les commands refactorisées
4. Vérifier la cohérence des noms et de l'organisation
5. Mettre à jour `progress.md` avec la refactorisation complétée

**Délivrables** :
- Code propre et documenté
- Architecture claire et maintenable
- Documentation à jour

## Détails Techniques

### Gestion des Erreurs

Les handlers doivent retourner des erreurs détaillées avec :
- Messages en français pour l'utilisateur
- Contexte sur la structure attendue
- Suggestions de correction si applicable

### Validation

La méthode `validate_project_structure()` doit retourner un `ValidationResult` avec :
- `is_valid` : booléen indiquant si le projet est valide
- `errors` : liste des erreurs bloquantes
- `warnings` : liste des avertissements non-bloquants
- `detected_engine` : nom du moteur détecté

### Compatibilité

- Les anciennes structures (`RpgMakerEngine`, `WolfRpgEngine`) peuvent être gardées temporairement pour compatibilité
- Les handlers peuvent utiliser ces structures en interne
- Migration progressive vers les handlers uniquement

## Risques et Mitigation

### Risque 1 : Régression Fonctionnelle
**Mitigation** :
- Tests de régression complets avant chaque phase
- Validation que tous les tests existants passent
- Tests manuels avec projets réels

### Risque 2 : Performance Dégradée
**Mitigation** :
- Benchmarks avant/après chaque phase
- Optimisation de la factory si nécessaire
- Pas d'overhead significatif attendu (détection centralisée)

### Risque 3 : Complexité Accrue
**Mitigation** :
- Architecture claire et modulaire
- Documentation complète
- Code review à chaque phase

## Critères de Succès

- ✅ Aucune duplication de code de détection
- ✅ Les commands ne connaissent plus les implémentations spécifiques
- ✅ Ajouter un nouveau moteur nécessite uniquement de créer un handler
- ✅ Tous les tests existants passent
- ✅ Aucune régression fonctionnelle
- ✅ Performances identiques ou meilleures
- ✅ Code plus maintenable et extensible

## Estimation Totale

**Durée estimée** : 6-7 jours de développement
- Phase 1 : 1-2 jours
- Phase 2 : 1 jour
- Phase 3 : 1 jour
- Phase 4 : 1 jour
- Phase 5 : 1 jour
- Phase 6 : 1 jour
- Phase 7 : 1 jour
- Phase 8 : 0.5 jour

**Complexité** : Moyenne à élevée (refactorisation majeure nécessitant une attention aux détails)

