# Implementation Plan: LudoLingo Game Localization Core

**Branch**: `001-game-localization` | **Date**: 2025-11-06 | **Spec**: [specs/001-game-localization/spec.md](specs/001-game-localization/spec.md)
**Input**: Feature specification from `/specs/001-game-localization/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Application desktop pour la localisation de jeux vidéo avec extraction automatique des textes traduisibles (RPG Maker MV/MZ différencié), stockage en base de données SQLite locale, traduction par lots via Ollama, et réinjection automatique des traductions. L'architecture suit la constitution Tauri + Nuxt avec séparation frontend/backend et traitement asynchrone.

**🎯 Version 1.0 Focus**: Implémentation complète pour RPG Maker MV/MZ uniquement. Support pour WolfRPG et Baki sera ajouté dans les versions futures.

## Technical Context

**Language/Version**: Rust 1.x (Tauri), TypeScript 5.x (Nuxt)
**Primary Dependencies**: Tauri 2.x, Nuxt 3.x, Nuxt UI, tauri-plugin-sql, tauri-plugin-store, ollama-rs, async-stripe
**Storage**:
- SQLite via tauri-plugin-sql : Données de traduction (texts, glossary, projects)
- Store plugin : Settings globaux (`settings.json`) + données projet (`ludolingo.json`)
**Internationalization**: Intégration native Nuxt UI avec 50+ langues supportées
**Testing**: Cargo test (backend uniquement) - TDD obligatoire
**Target Platform**: Desktop (Windows & Linux uniquement)
**Project Type**: Desktop application (Tauri + Nuxt)
**Performance Goals**: Extraction <30s pour 100MB, traduction 50 textes <5min, workflow complet <10min
**Constraints**: Architecture offline-first, données locales uniquement, transmission limitée à endpoint Ollama (localhost ou service distant comme RunPod)
**Scale/Scope**: Support 1-100 éléments simultanés, parsers spécialisés par moteur de jeu (RPG Maker MV/MZ différencié), interface utilisateur complète

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Extraction et Sécurité des Données
- [x] Architecture respecte l'extraction non-destructive des données de jeu
- [x] Données restent exclusivement locales à l'appareil
- [x] Sauvegarde automatique des fichiers originaux avant modification

### II. Architecture Tauri + Nuxt Simplifiée
- [x] Interface utilisateur en Nuxt avec Nuxt UI côté frontend
- [x] Logique métier en Rust avec plugins Tauri spécialisés côté backend
- [x] Séparation claire frontend/backend respectée

### III. Base de Données SQLite via Plugin Tauri
- [x] Persistance exclusivement locale via tauri-plugin-sql
- [x] Interface unifiée pour toutes les opérations de base de données
- [x] Pas d'autres solutions de stockage utilisées

### IV. Test-Driven Development
- [x] TDD obligatoire pour toute nouvelle fonctionnalité
- [x] Tests unitaires, d'intégration et e2e planifiés
- [x] Couverture de test >80% visée (calculée sur les lignes de code exécutables)

### V. Performance et Traitement par Lots
- [x] Support de traitement simultané de 1 à 100 éléments
- [x] Optimisation pour gros volumes de texte
- [x] Traitement asynchrone non-bloquant

## Project Structure

### Documentation (this feature)

```text
specs/001-game-localization/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Application Tauri + Nuxt (desktop)
src-tauri/
├── src/
│   ├── commands/        # Tauri commands exposés au frontend (mod.rs = exports)
│   │   ├── projects.rs  # Project validation commands (DB operations in frontend)
│   │   └── mod.rs       # Commands module exports
│   ├── migrations.rs    # Database migrations (via tauri-plugin-sql, appliquées automatiquement)
│   ├── models/          # Data structures (mod.rs = exports)
│   │   ├── project.rs   # Project model
│   │   └── mod.rs       # Models module exports
│   ├── parsers/         # Game engines parsers (mod.rs = exports)
│   │   ├── engine.rs    # Parser orchestration logic
│   │   └── mod.rs       # Parsers module exports
│   │   ├── rpg_maker/     # RPG Maker MV/MZ engine (unified with version detection)
│   │   │   ├── files/     # Game data files handlers
│   │   │   │   ├── actors.rs    # Actors.json - extract/inject noms et descriptions personnages
│   │   │   │   ├── items.rs     # Items.json - extract/inject noms et descriptions objets
│   │   │   │   ├── skills.rs    # Skills.json - extract/inject noms et descriptions compétences
│   │   │   │   ├── weapons.rs   # Weapons.json - extract/inject noms armes
│   │   │   │   ├── armors.rs    # Armors.json - extract/inject noms armures
│   │   │   │   ├── enemies.rs   # Enemies.json - extract/inject noms ennemis
│   │   │   │   ├── states.rs    # States.json - extract/inject noms états et messages
│   │   │   │   ├── system.rs    # System.json - extract/inject termes système et menus
│   │   │   │   ├── maps.rs      # MapXXX.json - extract/inject dialogues événements cartes
│   │   │   │   ├── events.rs    # CommonEvents.json - extract/inject dialogues événements communs
│   │   │   │   └── classes.rs   # Classes.json - extract/inject noms classes
│   │   │   └── engine.rs  # Main engine logic with MV/MZ differentiation (orchestrates file parsers)
│   │   ├── wolfrpg/       # WolfRPG engine (future version)
│   │   │   ├── files/     # Game data files handlers
│   │   │   └── engine.rs  # Main engine logic
│   │   ├── baki/          # Baki engine (future version)
│   │   │   ├── files/     # Game data files handlers
│   │   │   └── engine.rs  # Main engine logic
│   │   └── mod.rs         # Parsers module exports
│   ├── translation/     # Translation module (mod.rs = exports)
│   │   ├── service.rs   # Translation service logic
│   │   ├── mod.rs       # Translation module exports
│   │   └── ollama/      # Ollama integration
│   │       ├── client.rs    # Ollama HTTP client
│   │       ├── batch.rs     # Batch translation logic
│   │       ├── single.rs    # Single translation logic
│   │       ├── models.rs    # Ollama API models
│   │       └── mod.rs       # Ollama module exports
│   └── lib.rs           # Main application logic
├── Cargo.toml
└── tauri.conf.json

app/                     # Nuxt frontend
├── components/          # Vue components organized by feature
│   ├── common/          # Shared/reusable components
│   │   └── LanguageSwitcher.vue # Language selection component (uses Nuxt UI locales)
│   ├── settings/        # Settings-related components
│   │   ├── SettingsForm.vue    # Main settings form
│   │   ├── OllamaConfig.vue    # Ollama configuration panel
│   │   └── ThemeSelector.vue   # Theme selection component
│   ├── projects/        # Project management components
│   │   ├── ProjectList.vue     # Projects list view
│   │   ├── ProjectCard.vue     # Individual project card
│   │   ├── ProjectForm.vue     # Create/edit project form
│   │   └── GameScanner.vue     # Game scanning interface
│   ├── translation/     # Translation workflow components
│   │   ├── TranslationView.vue # Main translation interface
│   │   ├── TextEditor.vue      # Text editing component
│   │   ├── BatchProgress.vue   # Batch translation progress
│   │   └── GlossaryPanel.vue   # Glossary management
│   └── ui/              # Base UI components (if needed)
│       └── layouts/     # Layout components
├── composables/         # Vue composables
│   ├── db/             # Database operations ← tauri-plugin-sql usage
│   │   ├── translation/   # Translation CRUD operations
│   │   │   ├── create.ts
│   │   │   ├── read.ts
│   │   │   ├── update.ts
│   │   │   └── delete.ts
│   │   └── glossary/      # Glossary CRUD operations
│   │       ├── create.ts
│   │       ├── read.ts
│   │       ├── update.ts
│   │       └── delete.ts
│   └── useDatabase.ts   # Database utilities
├── pages/              # Application pages/routes
│   ├── index.vue         # Home page with overview
│   ├── projects.vue      # Projects list and management page
│   ├── project.vue       # Individual project page ([id] parameter)
│   ├── translation.vue   # Translation interface for current project
│   └── settings.vue      # Application settings page
├── stores/             # Pinia stores
│   ├── projects.ts         # Project management state
│   ├── translations.ts     # Translation entries state ← uses db/translation/*
│   ├── glossary.ts         # Glossary terms state ← uses db/glossary/*
│   ├── batch.ts            # Batch operations state
│   └── settings.ts         # Application settings state (includes language management)
├── server/             # Nuxt server API (if needed)
├── public/             # Static assets
├── app.vue             # Root component with UApp locale configuration
└── nuxt.config.ts

# Tests unifiés
tests/
├── unit/               # Tests unitaires (frontend + backend)
├── integration/        # Tests d'intégration
└── e2e/               # Tests end-to-end avec Playwright
```

**Structure Decision**: Architecture Tauri classique avec séparation claire des responsabilités et organisation modulaire :

#### **Organisation Frontend** :
- **Components** : Organisés par domaine fonctionnel (`settings/`, `projects/`, `translation/`)
  - `common/` : Composants partagés (LanguageSwitcher)
  - `settings/` : Gestion des paramètres utilisateur
  - `projects/` : Gestion des projets de localisation
  - `translation/` : Interface de traduction et glossaire
- **Pages** : Pages directes à la racine avec paramètres dynamiques
  - `projects.vue` : Liste des projets
  - `project.vue` : Projet individuel (avec paramètre id)
  - `translation.vue` : Interface de traduction
- **Composables** : Logique métier pure + accès DB
- **Stores (Pinia)** : État global réactif

#### **Organisation Backend** :
- **Commands** : Validation et logique métier côté Rust
- **Parsers** : Moteurs de jeu spécialisés (RPG Maker MV/MZ différencié)
- **Translation** : Intégration Ollama pour traduction par lots
- Tests unifiés dans un dossier commun pour simplifier la CI/CD

#### **Avantages** :
- **Maintenance** : Composants regroupés par fonctionnalité
- **Réutilisabilité** : Composants communs partagés
- **Évolutivité** : Ajout de nouvelles features simplifié
- **Navigation** : Structure intuitive pour les développeurs

## Organisation des Modules Rust

**Convention standard respectée** : `mod.rs` contient uniquement les exports et déclarations de modules.

### **Structure par module :**
```rust
// src/commands/mod.rs - Exports uniquement
pub mod projects;  // Déclare le module projects.rs

// Exports publics
pub use projects::*;

// src/commands/projects.rs - Validation et logique métier uniquement
// Les opérations DB sont faites côté frontend via tauri-plugin-sql
#[tauri::command]
pub fn validate_project_name(name: &str) -> Result<(), String> {
    // Validation côté Rust pour sécurité
    if name.trim().is_empty() {
        return Err("Project name cannot be empty".to_string());
    }
    if name.len() > 255 {
        return Err("Project name too long".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn validate_game_path(path: &str) -> Result<(), String> {
    // Validation du chemin côté Rust
    // (Logique de validation sécurisée)
    Ok(())
}
```

### **Gestion des migrations (via tauri-plugin-sql) :**
```rust
// src/migrations.rs - Migrations définies pour le plugin
use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration<'static>> {
    vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE projects (id INTEGER PRIMARY KEY, name TEXT NOT NULL);",
            kind: MigrationKind::Up,
        }
    ]
}

// Dans lib.rs - Intégration avec le plugin
.plugin(
    tauri_plugin_sql::Builder::default()
        .add_migrations("sqlite:ludolingo.db", migrations::get_migrations())
        .build(),
)
```

### **Accès base de données (frontend via plugin) :**
```typescript
// Dans composables/db/projects.ts
import Database from '@tauri-apps/plugin-sql';

const db = await Database.load('sqlite:ludolingo.db');

export async function createProject(name: string, gamePath: string) {
  const result = await db.execute(
    'INSERT INTO projects (name, game_path) VALUES (?, ?)',
    [name, gamePath]
  );
  return result.lastInsertId;
}

export async function getAllProjects() {
  const result = await db.select('SELECT * FROM projects');
  return result;
}
```

### **Séparation Validation/DB :**
- **Rust (Backend)** : Validation sécurisée, logique métier sensible
- **TypeScript (Frontend)** : Opérations de base de données via tauri-plugin-sql
- **Avantages** : Sécurité côté backend, simplicité côté frontend

### **Avantages de cette approche :**
- **Convention standard** : Respecte les pratiques Rust établies
- **Plugin intégré** : Migrations gérées automatiquement par tauri-plugin-sql
- **Sécurité** : Validation côté Rust, données sensibles protégées
- **Simplicité** : DB operations simplifiées côté frontend
- **Séparation claire** : Exports vs logique métier
- **Évolutivité** : Facilite l'ajout de nouveaux modules
- **Maintenance** : Plus facile à naviguer dans de gros projets

## Tauri Command Pattern Reference

**Note**: Reference for future implementation - Basic pattern for calling Rust from frontend (from https://tauri.app/develop/calling-rust/)

**Rust side** (`src-tauri/src/lib.rs`):
```rust
#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JavaScript!");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

**Frontend side** (JavaScript/TypeScript):
```javascript
import { invoke } from '@tauri-apps/api/core';

invoke('my_custom_command');
```

**Key Points**:
- Commands must be unique across the application
- Commands defined in `lib.rs` cannot be `pub` (limitation)
- For multiple commands: `tauri::generate_handler![cmd1, cmd2, cmd3]`
- Commands can accept arguments and return values
- Error handling available

## Parsing Architecture

**Modular File Parsing**: Chaque type de fichier de données RPG Maker a son propre parser avec méthodes `extract()` et `inject()` dédiées.

**Interface commune par fichier**:
```rust
pub trait FileParser {
    fn extract(&self, file_path: &Path, version: RpgMakerVersion) -> Result<Vec<TextEntry>>;
    fn inject(&self, file_path: &Path, translations: &[TranslationEntry], version: RpgMakerVersion) -> Result<()>;
}
```

**Avantages**:
- **Maintenabilité** : Logique d'extraction/injection isolée par type de fichier
- **Performance** : Parsing parallèle possible des fichiers indépendants
- **Cohérence** : Interface commune pour tous les parsers
- **Testabilité** : Tests unitaires par parser individuel

**Flux de parsing**:
1. `engine.rs` détecte la version MV/MZ et scanne les fichiers
2. Pour chaque fichier → appelle `parser.extract()` approprié
3. Collecte tous les textes avec métadonnées (fichier source, champ, etc.)
4. Pour l'injection → appelle `parser.inject()` avec les traductions filtrées

**Exemple concret par fichier**:

**Actors.json** (`actors.rs`):
- `extract()` : Parse chaque acteur → champs `name`, `nickname`, `profile` → crée `TextEntry`
- `inject()` : Reçoit traductions filtrées → met à jour JSON avec textes traduits

**Items.json** (`items.rs`):
- `extract()` : Parse chaque objet → champs `name`, `description` → crée `TextEntry`
- `inject()` : Reçoit traductions → met à jour `name` et `description` des objets

**Maps.json** (`maps.rs`):
- `extract()` : Parse événements de carte → dialogues dans commands → crée `TextEntry` par ligne
- `inject()` : Reçoit traductions → remplace textes dans les événements de carte

**System.json** (`system.rs`):
- `extract()` : Parse termes système → menus, messages d'erreur, etc.
- `inject()` : Met à jour tous les termes d'interface utilisateur

**Exemple d'implémentation** (`actors.rs`):
```rust
pub struct ActorsParser;

impl FileParser for ActorsParser {
    fn extract(&self, file_path: &Path, version: RpgMakerVersion) -> Result<Vec<TextEntry>> {
        let content: Vec<Actor> = serde_json::from_reader(File::open(file_path)?)?;
        let mut entries = Vec::new();

        for (index, actor) in content.iter().enumerate() {
            if !actor.name.is_empty() {
                entries.push(TextEntry {
                    id: format!("actors_{}_name", index),
                    source_file: "Actors.json".to_string(),
                    field: "name".to_string(),
                    original_text: actor.name.clone(),
                    context: format!("Actor {}", index),
                    ..Default::default()
                });
            }
            // Même chose pour nickname, profile, etc.
        }
        Ok(entries)
    }

    fn inject(&self, file_path: &Path, translations: &[TranslationEntry], version: RpgMakerVersion) -> Result<()> {
        let mut content: Vec<Actor> = serde_json::from_reader(File::open(file_path)?)?;

        for translation in translations {
            if let Some((_, index_str, field)) = parse_translation_id(&translation.id) {
                let index: usize = index_str.parse()?;
                if index < content.len() {
                    match field.as_str() {
                        "name" => content[index].name = translation.translated_text.clone(),
                        "nickname" => content[index].nickname = translation.translated_text.clone(),
                        // ...
                    }
                }
            }
        }

        serde_json::to_writer_pretty(File::create(file_path)?, &content)?;
        Ok(())
    }
}
```

**Responsabilités communes**:
- Chaque parser gère sa propre validation JSON
- Respecte les différences MV/MZ (champs additionnels en MZ)
- Génère des IDs uniques pour chaque texte (fichier + index + champ)
- Préserve la structure JSON originale lors de l'injection

## MV/MZ Version Detection Strategy

**Règle principale simplifiée**:
- **MZ** : `package.json` + dossier `data/` présents dans le root du jeu
- **MV** : Sinon (dossier `www/data/` pour les fichiers)

**Structure des fichiers**:
- **MV** : Fichiers dans `www/data/Actors.json`, `www/data/Items.json`, etc.
- **MZ** : Fichiers dans `data/Actors.json`, `data/Items.json`, etc.

**Implémentation de la détection**:
```rust
pub enum RpgMakerVersion {
    MV,
    MZ,
}

impl RpgMakerEngine {
    pub fn detect_version(game_path: &Path) -> Result<RpgMakerVersion> {
        // Règle simple : package.json + data/ = MZ
        let package_exists = game_path.join("package.json").exists();
        let data_folder_exists = game_path.join("data").is_dir();

        if package_exists && data_folder_exists {
            Ok(RpgMakerVersion::MZ)
        } else {
            Ok(RpgMakerVersion::MV)  // www/data/ par défaut
        }
    }

    pub fn get_data_root(&self, game_path: &Path, version: RpgMakerVersion) -> PathBuf {
        match version {
            RpgMakerVersion::MZ => game_path.join("data"),
            RpgMakerVersion::MV => game_path.join("www/data"),
        }
    }
}
```

## Data Persistence Strategy

**Store Plugin Usage** ([tauri.app/plugin/store/](https://tauri.app/plugin/store/)) :

- **`settings.json`** : Configuration globale (langue, mode Ollama local/distant, endpoints, preferences utilisateur)
- **`{project-name}.json`** : Métadonnées projet (dernier scan, statistiques, préférences spécifiques)

**SQLite Usage** : Données de traduction massives (texts, glossary, historique) - optimisé pour les requêtes complexes et volumétrie importante.

## Internationalization Strategy

**Nuxt UI Native Integration** ([ui.nuxt.com/docs/getting-started/integrations/i18n/nuxt](https://ui.nuxt.com/docs/getting-started/integrations/i18n/nuxt)) :

- **50+ langues supportées** : Toutes les locales intégrées de Nuxt UI
- **9 langues sélectionnées** : Français (défaut), Anglais, Espagnol, Allemand, Italien, Portugais, Japonais, Coréen, Chinois
- **RTL/LTR support** : Direction automatique selon la langue
- **SEO-friendly** : Attributs `lang` et `dir` sur `<html>`

**Architecture frontend** :
```typescript
// app.vue - Configuration globale
<template>
  <UApp :locale="currentLocale">
    <NuxtPage />
  </UApp>
</template>

<script setup>
// Locale réactive basée sur les paramètres utilisateur
const settingsStore = useSettingsStore()
const currentLocale = computed(() => {
  const userLocale = settingsStore.settings.ui.language
  return locales[userLocale] || locales.fr
})

// Synchronisation HTML
useHead({
  htmlAttrs: {
    lang: currentLocale.value.code,
    dir: currentLocale.value.dir
  }
})
</script>

// Store settings avec type sécurisé
export type SupportedLocale = 'fr' | 'en' | 'es' | 'de' | 'it' | 'pt' | 'ja' | 'ko' | 'zh'

interface Settings {
  ui: {
    language: SupportedLocale  // Type sécurisé
  }
}

// LanguageSwitcher avec locales Nuxt UI
<script setup>
import * as locales from '@nuxt/ui/locale'
import { supportedLanguages, getLocaleFlag } from '~/i18n/locales'

const localeOptions = computed(() =>
  Object.entries(locales)
    .filter(([code]) => supportedLanguages.includes(code))
    .map(([code, locale]) => ({
      label: `${getLocaleFlag(code)} ${locale.name}`, // Fonction centralisée
      value: code
    }))
)
</script>
```

**Messages personnalisés LudoLingo - Auto-découverte** :
```typescript
// app/i18n/locales/index.ts - Auto-découverte des langues
import fr from './fr'
import en from './en'

export const availableLocales = { fr, en } as const
export type SupportedLanguage = keyof typeof availableLocales

// app/i18n/messages.ts - Utilisation automatique
import { availableLocales, type AvailableLocales } from './locales'
export const messages = availableLocales
export type Messages = AvailableLocales

// Ajout d'une nouvelle langue :
// 1. Créer app/i18n/locales/es.ts
// 2. Ajouter import es from './es' dans index.ts
// 3. Ajouter es dans availableLocales
// → TypeScript détecte automatiquement la nouvelle langue !

// Utilisation dans les composants
const { tm } = useMessages()
{{ tm('projects', 'title') }}  // → "Projets de Localisation" ou "Localization Projects"
```

**Ajout automatique de nouvelles langues** :
```typescript
// Étapes pour ajouter l'espagnol :

// 1. Créer le fichier des messages
// app/i18n/locales/es.ts
export default {
  projects: { title: 'Proyectos de Localización' },
  settings: { title: 'Configuración' },
  // ... tous les textes en espagnol
} as const

// 2. Mettre à jour l'index
// app/i18n/locales/index.ts
import fr from './fr'
import en from './en'
import es from './es'  // ← Nouveau

export const availableLocales = {
  fr,
  en,
  es  // ← Nouveau
} as const

// → TypeScript détecte automatiquement 'es' comme langue supportée !
// → LanguageSwitcher affiche automatiquement l'espagnol
// → useMessages() fonctionne immédiatement avec l'espagnol
```

**Avantages de l'approche Nuxt UI** :
- **Zéro configuration** : Pas de fichiers de traduction à maintenir pour les composants UI
- **Locales professionnelles** : Traductions natives des composants UI via Nuxt UI
- **Messages personnalisés** : Système séparé pour les textes spécifiques à LudoLingo
- **Auto-découverte** : Nouvelles langues détectées automatiquement
- **Type safety** : Types générés automatiquement depuis les fichiers
- **Performance** : Lazy-loading automatique des locales Nuxt UI
- **Maintenance** : Mises à jour automatiques avec Nuxt UI

## Donations Strategy

**Stripe Payment Links Integration** ([docs.stripe.com/payment-links](https://docs.stripe.com/payment-links)) :

- **Payment Links** : URLs hébergées par Stripe pour les donations
- **Client Rust** : `async-stripe` pour créer les liens de donation
- **Workflow** : Création du Payment Link côté backend → Redirection via `tauri-plugin-opener`
- **Sécurité** : Clés API stockées côté backend uniquement

**Donation Amounts** :
- Montants prédéfinis (5€, 10€, 20€, 50€)
- Montant personnalisé
- Pas d'obligations ni abonnements

**Architecture** :
- Payment Links créés à la demande pour donations
- Message de remerciement après donation réussie
- Historique des donations stocké localement (optionnel)
- Application reste entièrement gratuite

## Ollama Configuration Strategy

**Dual-mode Ollama support** :

- **Mode Local** : `http://localhost` + port configurable (défaut 11434)
- **Mode Online** : URL complète sans port (ex: RunPod, services cloud)

**Frontend inputs uniquement** (pas de configuration backend) :
- Sélecteur mode (Local/Online)
- Input endpoint (URL ou hostname)
- Input port (uniquement en mode Local)
- Bouton test connexion
- Sélection modèle disponible

**Configuration automatique** :
- Mode détecté automatiquement depuis l'URL saisie
- Validation temps réel des inputs
- Test de connexion avant sauvegarde
- Fallback local si distant indisponible

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
