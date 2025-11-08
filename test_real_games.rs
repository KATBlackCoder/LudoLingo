// Test d'intégration avec de vrais jeux RPG Maker
// Utilise les exemples MVgame et MZgame pour valider l'extraction

use std::path::Path;
use ludo_lingo::parsers::engine::{detect_engine, GameEngine};
use ludo_lingo::parsers::rpg_maker::engine::RpgMakerEngine;

#[test]
fn test_detect_mv_game() {
    let mv_game_path = Path::new("../engines/MVgame");
    let engine = detect_engine(mv_game_path).unwrap();
    assert_eq!(engine, GameEngine::RpgMakerMV);
}

#[test]
fn test_detect_mz_game() {
    let mz_game_path = Path::new("../engines/MZgame");
    let engine = detect_engine(mz_game_path).unwrap();
    assert_eq!(engine, GameEngine::RpgMakerMZ);
}

#[test]
fn test_extract_mv_game_texts() {
    let mv_game_path = Path::new("../engines/MVgame");
    let entries = RpgMakerEngine::extract_all(mv_game_path, GameEngine::RpgMakerMV).unwrap();

    // Le jeu devrait contenir des textes
    assert!(entries.len() > 0);
    println!("MV Game extracted {} text entries", entries.len());

    // Vérifier quelques exemples de textes
    let actor_entries: Vec<_> = entries.iter().filter(|e| e.id.starts_with("actors_")).collect();
    let item_entries: Vec<_> = entries.iter().filter(|e| e.id.starts_with("items_")).collect();
    let system_entries: Vec<_> = entries.iter().filter(|e| e.id.starts_with("system_")).collect();

    println!("  Actors: {} entries", actor_entries.len());
    println!("  Items: {} entries", item_entries.len());
    println!("  System: {} entries", system_entries.len());

    // Vérifier que nous avons des noms d'acteurs
    assert!(actor_entries.iter().any(|e| e.field == "name"));
}

#[test]
fn test_extract_mz_game_texts() {
    let mz_game_path = Path::new("../engines/MZgame");
    let entries = RpgMakerEngine::extract_all(mz_game_path, GameEngine::RpgMakerMZ).unwrap();

    // Le jeu devrait contenir des textes
    assert!(entries.len() > 0);
    println!("MZ Game extracted {} text entries", entries.len());

    // Vérifier quelques exemples de textes
    let actor_entries: Vec<_> = entries.iter().filter(|e| e.id.starts_with("actors_")).collect();
    let item_entries: Vec<_> = entries.iter().filter(|e| e.id.starts_with("items_")).collect();
    let map_entries: Vec<_> = entries.iter().filter(|e| e.id.starts_with("maps_")).collect();

    println!("  Actors: {} entries", actor_entries.len());
    println!("  Items: {} entries", item_entries.len());
    println!("  Maps: {} entries", map_entries.len());

    // Vérifier que nous avons des dialogues de cartes
    assert!(map_entries.iter().any(|e| e.field == "event_dialogue"));
}

#[test]
fn test_extract_specific_mv_files() {
    let mv_game_path = Path::new("../engines/MVgame");

    // Tester l'extraction d'un fichier spécifique
    let system_path = mv_game_path.join("www/data/System.json");
    assert!(system_path.exists());

    // Tester l'extraction d'un fichier d'acteurs
    let actors_path = mv_game_path.join("www/data/Actors.json");
    assert!(actors_path.exists());

    println!("MV game file structure validated");
}

#[test]
fn test_extract_specific_mz_files() {
    let mz_game_path = Path::new("../engines/MZgame");

    // Tester l'extraction d'un fichier spécifique
    let system_path = mz_game_path.join("data/System.json");
    assert!(system_path.exists());

    // Tester l'extraction d'un fichier de cartes
    let map_path = mz_game_path.join("data/Map001.json");
    assert!(map_path.exists());

    println!("MZ game file structure validated");
}
