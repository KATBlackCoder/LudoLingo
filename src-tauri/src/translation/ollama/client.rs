// Ollama HTTP client using ollama-rs crate (https://github.com/pepperoni21/ollama-rs)
// Supports dual-mode: local (localhost:port) and online (custom endpoint)
// Provides access to full Ollama API: generation, chat, models, embeddings, tools
//
// Why ollama-rs instead of custom HTTP client?
// - Well-tested and maintained (936⭐, active development)
// - Complete API coverage (generation, chat, streaming, tools)
// - Proper error handling and types
// - Future-proof with Ollama API updates
// - Less code to maintain and debug

use ollama_rs::Ollama;

/// Re-export Ollama types for convenience
pub use ollama_rs::models::LocalModel as ModelInfo;

/// Ollama connection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OllamaMode {
    Local,
    Online,
}

/// Ollama client configuration
#[derive(Debug, Clone)]
pub struct OllamaConfig {
    pub mode: OllamaMode,
    pub endpoint: String,
    pub port: Option<u16>, // Only used in Local mode
}

/// Ollama HTTP client wrapper
#[derive(Clone)]
pub struct OllamaClient {
    client: Ollama,
    config: OllamaConfig,
}

impl OllamaClient {
    /// Create new Ollama client
    pub fn new(config: OllamaConfig) -> Self {
        let client = match config.mode {
            OllamaMode::Local => {
                let port = config.port.unwrap_or(11434);
                Ollama::new(config.endpoint.clone(), port)
            }
            OllamaMode::Online => {
                // For online mode, we use a dummy port since ollama-rs expects a port
                // The actual endpoint handling will be done by the underlying HTTP client
                Ollama::new(config.endpoint.clone(), 80)
            }
        };

        Self { client, config }
    }

    /// Get the underlying Ollama client
    pub fn inner(&self) -> &Ollama {
        &self.client
    }

    /// Get the underlying Ollama client mutably
    pub fn inner_mut(&mut self) -> &mut Ollama {
        &mut self.client
    }

    /// Get configuration
    pub fn config(&self) -> &OllamaConfig {
        &self.config
    }

    /// Test connection to Ollama server
    pub async fn test_connection(&self) -> Result<(), String> {
        // Try to list models as a connection test
        match self.client.list_local_models().await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Connection test failed: {}", e)),
        }
    }

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, String> {
        match self.client.list_local_models().await {
            Ok(models) => Ok(models),
            Err(e) => Err(format!("Failed to list models: {}", e)),
        }
    }
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            mode: OllamaMode::Local,
            endpoint: "http://localhost".to_string(),
            port: Some(11434),
        }
    }
}
