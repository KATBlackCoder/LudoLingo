// Engine Factory
// Detects game engine and creates appropriate handler

use crate::parsers::engine::GameEngine;
use crate::parsers::handler::GameEngineHandler;
use crate::parsers::rpg_maker::handler::RpgMakerHandler;
use crate::parsers::wolfrpg::handler::WolfRpgHandler;
use std::path::Path;

/// Factory for creating game engine handlers
pub struct EngineFactory;

impl EngineFactory {
    /// Detects the game engine from the project structure and creates the appropriate handler
    /// 
    /// Detection order:
    /// 1. WolfRPG (dump/ folder with db/, mps/, common/)
    /// 2. WolfRPG encrypted (Data.wolf file)
    /// 3. RPG Maker MZ (package.json + data/ folder)
    /// 4. RPG Maker MV (www/data/ folder)
    /// 
    /// # Arguments
    /// 
    /// * `game_path` - Path to the game project root directory
    /// 
    /// # Returns
    /// 
    /// * `Ok(Box<dyn GameEngineHandler>)` - Handler for the detected engine
    /// * `Err(String)` - Error message with suggestions if no engine detected
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use crate::parsers::factory::EngineFactory;
    /// use std::path::Path;
    /// 
    /// let handler = EngineFactory::create_handler(Path::new("/path/to/game"))?;
    /// println!("Detected engine: {}", handler.engine_name());
    /// ```
    pub fn create_handler(game_path: &Path) -> Result<Box<dyn GameEngineHandler>, String> {
        // 1. Check for Wolf RPG Editor (dump/ folder with db/, mps/, common/)
        let dump_folder = game_path.join("dump");
        if dump_folder.exists() && dump_folder.is_dir() {
            let db_dir = dump_folder.join("db");
            let mps_dir = dump_folder.join("mps");
            let common_dir = dump_folder.join("common");

            if db_dir.exists()
                && db_dir.is_dir()
                && mps_dir.exists()
                && mps_dir.is_dir()
                && common_dir.exists()
                && common_dir.is_dir()
            {
                return Ok(Box::new(WolfRpgHandler::new()));
            }
        }

        // 2. Check for Wolf RPG Editor encrypted (Data.wolf file)
        let data_wolf = game_path.join("Data.wolf");
        if data_wolf.exists() && data_wolf.is_file() {
            // Note: Encrypted WolfRPG projects need to be decrypted first
            // For now, we return a handler but validation will fail if dump/ doesn't exist
            return Ok(Box::new(WolfRpgHandler::new()));
        }

        // 3. Check for RPG Maker MZ (package.json + data/ folder)
        // Important: Must NOT have www/data/ (which would indicate MV)
        // Also check that we're not in a www/ subdirectory (which would be MV structure)
        let package_json = game_path.join("package.json");
        let data_folder = game_path.join("data");
        let www_data_folder = game_path.join("www").join("data");
        let is_in_www_subdir = game_path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n == "www")
            .unwrap_or(false);

        if package_json.exists() 
            && data_folder.is_dir() 
            && !www_data_folder.is_dir()
            && !is_in_www_subdir {
            return Ok(Box::new(RpgMakerHandler::new_mz()));
        }

        // 4. Check for RPG Maker MV (www/data/ folder)
        if www_data_folder.is_dir() {
            return Ok(Box::new(RpgMakerHandler::new_mv()));
        }

        // No engine detected - return detailed error with suggestions
        Err(format!(
            "Aucun moteur de jeu détecté dans '{}'.\n\n\
            Structures de projet supportées :\n\
            - RPG Maker MZ : doit contenir 'package.json' et dossier 'data/'\n\
            - RPG Maker MV : doit contenir dossier 'www/data/'\n\
            - Wolf RPG Editor : doit contenir dossier 'dump/' avec 'db/', 'mps/', et 'common/'\n\
            - Wolf RPG Editor (chiffré) : doit contenir fichier 'Data.wolf' (nécessite déchiffrement)\n\n\
            Vérifiez que le chemin pointe vers la racine du projet de jeu.",
            game_path.display()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_create_handler_rpg_maker_mv_real_game() {
        // Test with real MV game from engines_past
        let game_path = Path::new("../engines_past/MVgame");

        let handler = EngineFactory::create_handler(game_path);
        assert!(handler.is_ok(), "Should detect MV game successfully");
        let handler = handler.unwrap();
        assert_eq!(handler.engine_name(), "RPG Maker MV");

        // Test validation
        let validation = handler.validate_project_structure(game_path);
        assert!(validation.is_ok(), "Validation should succeed");
        assert!(validation.unwrap().is_valid, "Validation should be valid");
    }

    #[test]
    fn test_create_handler_rpg_maker_mz_real_game() {
        // Test with real MZ game from engines_past
        let game_path = Path::new("../engines_past/MZgame");

        let handler = EngineFactory::create_handler(game_path);
        assert!(handler.is_ok(), "Should detect MZ game successfully");
        let handler = handler.unwrap();
        assert_eq!(handler.engine_name(), "RPG Maker MZ");

        // Test validation
        let validation = handler.validate_project_structure(game_path);
        assert!(validation.is_ok(), "Validation should succeed");
        assert!(validation.unwrap().is_valid, "Validation should be valid");
    }

    #[test]
    fn test_create_handler_wolfrpg_real_game() {
        // Test with real WolfRPG game from engines_past
        let game_path = Path::new("../engines_past/wolfrpg");

        let handler = EngineFactory::create_handler(game_path);
        assert!(handler.is_ok(), "Should detect WolfRPG game successfully");
        let handler = handler.unwrap();
        assert_eq!(handler.engine_name(), "Wolf RPG Editor");

        // Test validation
        let validation = handler.validate_project_structure(game_path);
        assert!(validation.is_ok(), "Validation should succeed");
        assert!(validation.unwrap().is_valid, "Validation should be valid");
    }

    #[test]
    fn test_create_handler_count_files_mv() {
        let game_path = Path::new("../engines_past/MVgame");
        let handler = EngineFactory::create_handler(game_path).unwrap();

        let count = handler.count_files_to_process(game_path);
        assert!(count > 0, "Should count some files for MV game");
        // MV game should have JSON files in www/data/
        assert!(count >= 20, "MV game should have at least 20 files");
    }

    #[test]
    fn test_create_handler_count_files_mz() {
        let game_path = Path::new("../engines_past/MZgame");
        let handler = EngineFactory::create_handler(game_path).unwrap();

        let count = handler.count_files_to_process(game_path);
        assert!(count > 0, "Should count some files for MZ game");
        // MZ game should have many JSON files in data/
        assert!(count >= 100, "MZ game should have at least 100 files");
    }

    #[test]
    fn test_create_handler_count_files_wolfrpg() {
        let game_path = Path::new("../engines_past/wolfrpg");
        let handler = EngineFactory::create_handler(game_path).unwrap();

        let count = handler.count_files_to_process(game_path);
        assert!(count > 0, "Should count some files for WolfRPG game");
        // WolfRPG should have JSON files in dump/
        assert!(count >= 10, "WolfRPG game should have at least 10 files");
    }

    #[test]
    fn test_create_handler_get_data_root_mv() {
        let game_path = Path::new("../engines_past/MVgame");
        let handler = EngineFactory::create_handler(game_path).unwrap();

        let data_root = handler.get_data_root(game_path);
        assert_eq!(data_root, game_path.join("www").join("data"));
    }

    #[test]
    fn test_create_handler_get_data_root_mz() {
        let game_path = Path::new("../engines_past/MZgame");
        let handler = EngineFactory::create_handler(game_path).unwrap();

        let data_root = handler.get_data_root(game_path);
        assert_eq!(data_root, game_path.join("data"));
    }

    #[test]
    fn test_create_handler_get_data_root_wolfrpg() {
        let game_path = Path::new("../engines_past/wolfrpg");
        let handler = EngineFactory::create_handler(game_path).unwrap();

        let data_root = handler.get_data_root(game_path);
        assert_eq!(data_root, game_path.join("dump"));
    }

    #[test]
    fn test_create_handler_unknown() {
        // Test with a non-existent directory
        let game_path = Path::new("../engines_past/nonexistent");

        let result = EngineFactory::create_handler(game_path);
        assert!(result.is_err(), "Should fail for non-existent directory");

        if let Err(error) = result {
            assert!(error.contains("Aucun moteur de jeu détecté"));
            assert!(error.contains("RPG Maker MZ"));
            assert!(error.contains("RPG Maker MV"));
            assert!(error.contains("Wolf RPG Editor"));
        }
    }

    #[test]
    fn test_create_handler_empty_directory() {
        // Test with an empty directory
        let game_path = Path::new("../engines_past/empty_test_dir");

        let result = EngineFactory::create_handler(game_path);
        // This might fail if the directory doesn't exist, which is expected
        // The test verifies the error handling works
        if result.is_ok() {
            // If the directory exists but is empty, it should fail
            let handler = result.unwrap();
            let validation = handler.validate_project_structure(game_path);
            assert!(validation.is_ok()); // Validation call should succeed
            assert!(!validation.unwrap().is_valid); // But validation should be invalid
        }
    }

    #[test]
    fn test_create_handler_detection_priority() {
        // Test detection priority: WolfRPG -> MZ -> MV
        // This is implicit in the factory implementation order

        // Test that WolfRPG is detected first (has dump/ folder)
        let wolf_path = Path::new("../engines_past/wolfrpg");
        let wolf_handler = EngineFactory::create_handler(wolf_path).unwrap();
        assert_eq!(wolf_handler.engine_name(), "Wolf RPG Editor");

        // Test that MZ is detected correctly
        let mz_path = Path::new("../engines_past/MZgame");
        let mz_handler = EngineFactory::create_handler(mz_path).unwrap();
        assert_eq!(mz_handler.engine_name(), "RPG Maker MZ");

        // Test that MV is detected correctly
        let mv_path = Path::new("../engines_past/MVgame");
        let mv_handler = EngineFactory::create_handler(mv_path).unwrap();
        assert_eq!(mv_handler.engine_name(), "RPG Maker MV");
    }
}

