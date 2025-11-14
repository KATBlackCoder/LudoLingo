-- Initial database schema for LudoLingo
-- Based on data-model.md specification
-- Migration 001: Base schema with projects, game_path, and structured location identifier
-- Note: Translations are processed one-by-one (no batch processing)

-- Projects table: organizes translation work
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    source_language TEXT NOT NULL DEFAULT 'ja',
    target_language TEXT NOT NULL DEFAULT 'fr',
    game_path TEXT NOT NULL,
    game_engine TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Game files table: tracks scanned files
CREATE TABLE IF NOT EXISTS game_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    file_format TEXT NOT NULL,
    file_size INTEGER,
    checksum TEXT,
    last_modified DATETIME,
    scan_status TEXT DEFAULT 'pending',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    UNIQUE(project_id, file_path)
);

-- Translation entries table: extracted texts with translations
-- location format: "object_type:object_id:field" (e.g., "actor:1:name", "map:9:event:1:message:12")
-- This structured format allows reconstructing parser_id for injection:
-- parser_id = location.replace(':', '_') → "actor_1_name" or "map_9_event_1_message_12"
-- 
-- Examples:
--   - Actors: "actor:1:name" → parser_id: "actor_1_name"
--   - Items: "item:5:description" → parser_id: "item_5_description"
--   - Map events: "map:9:event:1:message:12" → parser_id: "map_9_event_1_message_12"
--   - System: "system:game_title" → parser_id: "system_game_title"
-- 
-- For UI display, location can be formatted: "actor:1:name" → "Actor 1: name"
CREATE TABLE IF NOT EXISTS translation_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    game_file_id INTEGER,
    source_text TEXT NOT NULL,
    translated_text TEXT,
    location TEXT NOT NULL,  -- Structured identifier: "object_type:object_id:field" for parser_id reconstruction
                             -- Examples: "actor:1:name", "item:5:description", "map:9:event:1:message:12"
    text_type TEXT DEFAULT 'dialogue',  -- 'dialogue', 'system', 'item', 'skill', 'other'
    status TEXT DEFAULT 'extracted',  -- 'extracted', 'translated', 'reviewed'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (game_file_id) REFERENCES game_files(id) ON DELETE SET NULL
);

-- Glossary table: standardized terms and translations
CREATE TABLE IF NOT EXISTS glossary_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_term TEXT NOT NULL,
    translated_term TEXT NOT NULL,
    context TEXT,  -- Usage context or examples (different from translation_entries.location)
    category TEXT DEFAULT 'general',  -- 'character', 'item', 'location', 'system', etc.
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(source_term, translated_term)
);

-- Indexes for performance

-- Projects indexes
CREATE INDEX IF NOT EXISTS idx_projects_name ON projects(name);
CREATE INDEX IF NOT EXISTS idx_projects_updated ON projects(updated_at);

-- Game files indexes
CREATE INDEX IF NOT EXISTS idx_game_files_project ON game_files(project_id);
CREATE INDEX IF NOT EXISTS idx_game_files_status ON game_files(scan_status);
CREATE INDEX IF NOT EXISTS idx_game_files_path ON game_files(file_path);

-- Translation entries indexes
CREATE INDEX IF NOT EXISTS idx_translations_project ON translation_entries(project_id);
CREATE INDEX IF NOT EXISTS idx_translations_status ON translation_entries(status);
CREATE INDEX IF NOT EXISTS idx_translations_type ON translation_entries(text_type);
CREATE INDEX IF NOT EXISTS idx_translations_updated ON translation_entries(updated_at);
CREATE INDEX IF NOT EXISTS idx_translations_location ON translation_entries(location);  -- For parser_id reconstruction and fast lookup during injection
CREATE INDEX IF NOT EXISTS idx_translations_file ON translation_entries(game_file_id);
CREATE INDEX IF NOT EXISTS idx_translations_source_text ON translation_entries(source_text);  -- For deduplication

-- Glossary indexes
CREATE INDEX IF NOT EXISTS idx_glossary_term ON glossary_entries(source_term);
CREATE INDEX IF NOT EXISTS idx_glossary_category ON glossary_entries(category);
