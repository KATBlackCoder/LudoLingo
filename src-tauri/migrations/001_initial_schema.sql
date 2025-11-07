-- Initial database schema for LudoLingo
-- Based on data-model.md specification

-- Projects table: organizes translation work
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    source_language TEXT NOT NULL DEFAULT 'ja',
    target_language TEXT NOT NULL DEFAULT 'fr',
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
CREATE TABLE IF NOT EXISTS translation_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    game_file_id INTEGER,
    source_text TEXT NOT NULL,
    translated_text TEXT,
    context TEXT,
    text_type TEXT DEFAULT 'dialogue',
    status TEXT DEFAULT 'extracted',
    translation_source TEXT,
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
    context TEXT,
    category TEXT DEFAULT 'general',
    frequency INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(source_term, translated_term)
);

-- Translation batches table: tracks batch processing
CREATE TABLE IF NOT EXISTS translation_batches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    batch_name TEXT,
    status TEXT DEFAULT 'pending',
    total_entries INTEGER DEFAULT 0,
    processed_entries INTEGER DEFAULT 0,
    error_count INTEGER DEFAULT 0,
    started_at DATETIME,
    completed_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Batch entries junction table
CREATE TABLE IF NOT EXISTS batch_entries (
    batch_id INTEGER NOT NULL,
    translation_entry_id INTEGER NOT NULL,
    status TEXT DEFAULT 'pending',
    error_message TEXT,
    processed_at DATETIME,
    PRIMARY KEY (batch_id, translation_entry_id),
    FOREIGN KEY (batch_id) REFERENCES translation_batches(id) ON DELETE CASCADE,
    FOREIGN KEY (translation_entry_id) REFERENCES translation_entries(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_projects_name ON projects(name);
CREATE INDEX IF NOT EXISTS idx_projects_updated ON projects(updated_at);

CREATE INDEX IF NOT EXISTS idx_game_files_project ON game_files(project_id);
CREATE INDEX IF NOT EXISTS idx_game_files_status ON game_files(scan_status);

CREATE INDEX IF NOT EXISTS idx_translations_project ON translation_entries(project_id);
CREATE INDEX IF NOT EXISTS idx_translations_status ON translation_entries(status);
CREATE INDEX IF NOT EXISTS idx_translations_type ON translation_entries(text_type);
CREATE INDEX IF NOT EXISTS idx_translations_updated ON translation_entries(updated_at);

CREATE INDEX IF NOT EXISTS idx_glossary_term ON glossary_entries(source_term);
CREATE INDEX IF NOT EXISTS idx_glossary_category ON glossary_entries(category);
CREATE INDEX IF NOT EXISTS idx_glossary_frequency ON glossary_entries(frequency DESC);

CREATE INDEX IF NOT EXISTS idx_batches_project ON translation_batches(project_id);
CREATE INDEX IF NOT EXISTS idx_batches_status ON translation_batches(status);
CREATE INDEX IF NOT EXISTS idx_batch_entries_status ON batch_entries(status);

