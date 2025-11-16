// Project data model
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub source_language: String,
    pub target_language: String,
    pub game_engine: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            id: None,
            name: String::new(),
            description: None,
            source_language: "ja".to_string(), // Japanese default for JRPGs
            target_language: "fr".to_string(), // French default
            game_engine: None,
            created_at: None,
            updated_at: None,
        }
    }
}
