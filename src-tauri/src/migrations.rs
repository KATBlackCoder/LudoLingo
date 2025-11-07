// Database migrations for SQLite
// Managed by tauri-plugin-sql

use tauri_plugin_sql::{Migration, MigrationKind};

/// Get all database migrations
pub fn get_migrations() -> Vec<Migration> {
    vec![
        // Migration 1: Initial schema
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: include_str!("../migrations/001_initial_schema.sql"),
            kind: MigrationKind::Up,
        },
    ]
}
