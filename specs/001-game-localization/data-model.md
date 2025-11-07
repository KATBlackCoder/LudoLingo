# Data Model: LudoLingo Game Localization Core

**Date**: 2025-11-06
**Spec**: [specs/001-game-localization/spec.md](specs/001-game-localization/spec.md)

**Architecture**: SQLite accessible directement depuis le frontend via tauri-plugin-sql (pas de couche backend intermédiaire)

## Database Schema (SQLite)

### Tables Overview

```sql
-- Projects table: organizes translation work
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    source_language TEXT NOT NULL DEFAULT 'ja', -- Japanese by default for JRPGs
    target_language TEXT NOT NULL DEFAULT 'fr', -- French default
    game_engine TEXT, -- 'rpgmaker', 'wolfrpg', 'baki', etc.
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Game files table: tracks scanned files
CREATE TABLE game_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    file_format TEXT NOT NULL, -- 'json', 'rxdata', 'binary', etc.
    file_size INTEGER, -- in bytes
    checksum TEXT, -- SHA-256 for change detection
    last_modified DATETIME,
    scan_status TEXT DEFAULT 'pending', -- 'pending', 'scanning', 'completed', 'error'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    UNIQUE(project_id, file_path)
);

-- Translation entries table: extracted texts with translations
CREATE TABLE translation_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    game_file_id INTEGER,
    source_text TEXT NOT NULL,
    translated_text TEXT,
    context TEXT, -- surrounding text or location info
    text_type TEXT DEFAULT 'dialogue', -- 'dialogue', 'system', 'item', 'skill', etc.
    status TEXT DEFAULT 'extracted', -- 'extracted', 'translated', 'reviewed', 'finalized'
    translation_source TEXT, -- 'manual', 'ollama', 'glossary'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (game_file_id) REFERENCES game_files(id) ON DELETE SET NULL
);

-- Glossary table: standardized terms and translations
CREATE TABLE glossary_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_term TEXT NOT NULL,
    translated_term TEXT NOT NULL,
    context TEXT, -- usage context or examples
    category TEXT DEFAULT 'general', -- 'character', 'item', 'location', 'system', etc.
    frequency INTEGER DEFAULT 0, -- usage count across projects
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(source_term, translated_term)
);

-- Translation batches table: tracks batch processing
CREATE TABLE translation_batches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    batch_name TEXT,
    status TEXT DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'error'
    total_entries INTEGER DEFAULT 0,
    processed_entries INTEGER DEFAULT 0,
    error_count INTEGER DEFAULT 0,
    started_at DATETIME,
    completed_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Batch entries junction table
CREATE TABLE batch_entries (
    batch_id INTEGER NOT NULL,
    translation_entry_id INTEGER NOT NULL,
    status TEXT DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'error'
    error_message TEXT,
    processed_at DATETIME,
    PRIMARY KEY (batch_id, translation_entry_id),
    FOREIGN KEY (batch_id) REFERENCES translation_batches(id) ON DELETE CASCADE,
    FOREIGN KEY (translation_entry_id) REFERENCES translation_entries(id) ON DELETE CASCADE
);
```

## Indexes for Performance

```sql
-- Projects indexes
CREATE INDEX idx_projects_name ON projects(name);
CREATE INDEX idx_projects_updated ON projects(updated_at);

-- Game files indexes
CREATE INDEX idx_game_files_project ON game_files(project_id);
CREATE INDEX idx_game_files_status ON game_files(scan_status);

-- Translation entries indexes
CREATE INDEX idx_translations_project ON translation_entries(project_id);
CREATE INDEX idx_translations_status ON translation_entries(status);
CREATE INDEX idx_translations_type ON translation_entries(text_type);
CREATE INDEX idx_translations_updated ON translation_entries(updated_at);
CREATE INDEX idx_translations_source_hash ON translation_entries(source_text); -- for deduplication queries

-- Glossary indexes
CREATE INDEX idx_glossary_term ON glossary_entries(source_term);
CREATE INDEX idx_glossary_category ON glossary_entries(category);
CREATE INDEX idx_glossary_frequency ON glossary_entries(frequency DESC);

-- Batch indexes
CREATE INDEX idx_batches_project ON translation_batches(project_id);
CREATE INDEX idx_batches_status ON translation_batches(status);
CREATE INDEX idx_batch_entries_status ON batch_entries(status);
```

## Data Validation Rules

### Projects
- `name`: Required, unique, 1-100 characters
- `source_language`: ISO 639-1 code (ja, en, fr, etc.)
- `target_language`: ISO 639-1 code
- `game_engine`: Optional, validated against known engines

### Game Files
- `file_path`: Must exist and be readable
- `file_format`: Must match known formats for the game engine
- `checksum`: SHA-256 hash for integrity checking

### Translation Entries
- `source_text`: Required, non-empty, max 10,000 characters
- `translated_text`: Optional, same max length as source
- `context`: Optional context info, max 1,000 characters
- `status`: Must be one of: extracted, translated, reviewed, finalized
- `translation_source`: Must be one of: manual, ollama, glossary

### Glossary Entries
- `source_term`: Required, unique per translation, 1-200 characters
- `translated_term`: Required, 1-200 characters
- `category`: Optional, defaults to 'general'

## Entity Relationships

```
Project (1) ──── (N) GameFile
    │
    ├─── (N) TranslationEntry
    │       └── belongs to (1) GameFile
    │
    └─── (N) TranslationBatch
            └── (N) BatchEntry ─── (1) TranslationEntry

GlossaryEntry (independent, shared across projects)
```

## State Transitions

### TranslationEntry Status Flow
```
extracted → translated → reviewed → finalized
    ↑           ↑           ↑
    └───────────┴───────────┘ (can go back for corrections)
```

### TranslationBatch Status Flow
```
pending → processing → completed
               ↓
             error (terminal state)
```

### GameFile Scan Status Flow
```
pending → scanning → completed
               ↓
             error (can retry)
```

## Data Integrity Constraints

- **Referential Integrity**: All foreign keys with CASCADE/SET NULL as appropriate
- **Unique Constraints**: Project names, file paths per project, glossary term pairs
- **Check Constraints**: Status values limited to valid enums
- **Triggers**: Auto-update timestamps, increment glossary frequency on usage

## Migration Strategy

**Version 1.0** (initial schema)
- Basic tables with all relationships
- Indexes for common queries
- Triggers for data consistency

**Future versions** will add:
- Audit logging table
- Translation history for versioning
- User preferences table
- Performance metrics table

## Backup and Recovery

- **Automatic Backups**: Before any file injection operation
- **Database Dumps**: Scheduled exports to JSON/CSV
- **Recovery Points**: After major operations (batch translation, project completion)

This data model supports the complete localization workflow while maintaining data integrity and performance for large game projects.
