// Test d'intégration pour valider l'extraction sur de vrais jeux RPG Maker

use crate::parsers::engine::{detect_engine, GameEngine};
use crate::parsers::rpg_maker::engine::RpgMakerEngine;
use std::env;
use std::path::Path;

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn get_engines_path() -> std::path::PathBuf {
        // Depuis src-tauri, remonter au répertoire racine du projet
        let current_dir = env::current_dir().unwrap();
        current_dir.parent().unwrap().join("engines")
    }

    #[test]
    fn test_detect_mv_game_engine() {
        let engines_path = get_engines_path();
        let mv_game_path = engines_path.join("MVgame");
        if mv_game_path.exists() {
            let engine = detect_engine(&mv_game_path).unwrap();
            assert_eq!(engine, GameEngine::RpgMakerMV);
            println!("✅ MV game engine detected correctly");
        } else {
            println!("⚠️  MV game not found at {:?}, skipping test", mv_game_path);
        }
    }

    #[test]
    fn test_detect_mz_game_engine() {
        let engines_path = get_engines_path();
        let mz_game_path = engines_path.join("MZgame");
        if mz_game_path.exists() {
            let engine = detect_engine(&mz_game_path).unwrap();
            assert_eq!(engine, GameEngine::RpgMakerMZ);
            println!("✅ MZ game engine detected correctly");
        } else {
            println!("⚠️  MZ game not found at {:?}, skipping test", mz_game_path);
        }
    }

    #[test]
    fn test_extract_mv_game_texts() {
        let engines_path = get_engines_path();
        let mv_game_path = engines_path.join("MVgame");
        if mv_game_path.exists() {
            let entries =
                RpgMakerEngine::extract_all(&mv_game_path, GameEngine::RpgMakerMV).unwrap();

            println!("📊 MV Game extraction results:");
            println!("   Total entries: {}", entries.len());

            let actor_entries: Vec<_> = entries
                .iter()
                .filter(|e| e.entry_type.starts_with("actor_"))
                .collect();
            let item_entries: Vec<_> = entries
                .iter()
                .filter(|e| e.entry_type.starts_with("item_"))
                .collect();
            let system_entries: Vec<_> = entries
                .iter()
                .filter(|e| e.entry_type.starts_with("system_"))
                .collect();
            let event_entries: Vec<_> = entries
                .iter()
                .filter(|e| e.entry_type.starts_with("event_"))
                .collect();

            println!("   Actors: {} entries", actor_entries.len());
            println!("   Items: {} entries", item_entries.len());
            println!("   System: {} entries", system_entries.len());
            println!("   Events: {} entries", event_entries.len());

            // Afficher quelques exemples
            if let Some(first_actor) = actor_entries.first() {
                println!(
                    "   Sample actor: '{}' ({})",
                    first_actor.source_text, first_actor.context
                );
            }
            if let Some(first_item) = item_entries.first() {
                println!(
                    "   Sample item: '{}' ({})",
                    first_item.source_text, first_item.context
                );
            }

            assert!(entries.len() > 0, "Should extract some texts from MV game");
            println!("✅ MV game text extraction successful");
        } else {
            println!("⚠️  MV game not found, skipping test");
        }
    }

    #[test]
    fn test_extract_mz_game_texts() {
        let engines_path = get_engines_path();
        let mz_game_path = engines_path.join("MZgame");
        if mz_game_path.exists() {
            let entries =
                RpgMakerEngine::extract_all(&mz_game_path, GameEngine::RpgMakerMZ).unwrap();

            println!("📊 MZ Game extraction results:");
            println!("   Total entries: {}", entries.len());

            let actor_entries: Vec<_> = entries
                .iter()
                .filter(|e| e.entry_type.starts_with("actor_"))
                .collect();
            let item_entries: Vec<_> = entries
                .iter()
                .filter(|e| e.entry_type.starts_with("item_"))
                .collect();
            let map_entries: Vec<_> = entries
                .iter()
                .filter(|e| e.entry_type.starts_with("map_"))
                .collect();
            let event_entries: Vec<_> = entries
                .iter()
                .filter(|e| e.entry_type.starts_with("event_"))
                .collect();

            println!("   Actors: {} entries", actor_entries.len());
            println!("   Items: {} entries", item_entries.len());
            println!("   Maps: {} entries", map_entries.len());
            println!("   Events: {} entries", event_entries.len());

            // Afficher quelques exemples
            if let Some(first_actor) = actor_entries.first() {
                println!(
                    "   Sample actor: '{}' ({})",
                    first_actor.source_text, first_actor.context
                );
            }
            if let Some(first_map) = map_entries.first() {
                println!(
                    "   Sample map dialogue: '{}' ({})",
                    first_map.source_text, first_map.context
                );
            }

            assert!(entries.len() > 0, "Should extract some texts from MZ game");
            println!("✅ MZ game text extraction successful");
        } else {
            println!("⚠️  MZ game not found, skipping test");
        }
    }

    #[test]
    fn test_compare_mv_vs_mz_extraction() {
        let engines_path = get_engines_path();
        let mv_game_path = engines_path.join("MVgame");
        let mz_game_path = engines_path.join("MZgame");

        if mv_game_path.exists() && mz_game_path.exists() {
            let mv_entries =
                RpgMakerEngine::extract_all(&mv_game_path, GameEngine::RpgMakerMV).unwrap();
            let mz_entries =
                RpgMakerEngine::extract_all(&mz_game_path, GameEngine::RpgMakerMZ).unwrap();

            println!("📊 Comparison MV vs MZ:");
            println!("   MV entries: {}", mv_entries.len());
            println!("   MZ entries: {}", mz_entries.len());

            // Comparer par type
            let mv_actors: Vec<_> = mv_entries
                .iter()
                .filter(|e| e.entry_type.starts_with("actor_"))
                .collect();
            let mz_actors: Vec<_> = mz_entries
                .iter()
                .filter(|e| e.entry_type.starts_with("actor_"))
                .collect();

            println!("   MV actors: {}", mv_actors.len());
            println!("   MZ actors: {}", mz_actors.len());

            assert!(
                mv_entries.len() > 0 && mz_entries.len() > 0,
                "Both games should have extractable texts"
            );
            println!("✅ MV vs MZ comparison successful");
        } else {
            println!("⚠️  Both games not found, skipping comparison test");
        }
    }
}
