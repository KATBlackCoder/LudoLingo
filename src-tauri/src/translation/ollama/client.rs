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

/// Extract port number from a URL string
/// Returns None if no port is found in the URL
fn extract_port_from_url(url: &str) -> Option<u16> {
    // Try to extract port from URL patterns like:
    // - http://host:port
    // - https://host:port
    // - https://host:port/path
    
    if let Some(after_protocol) = url.split("://").nth(1) {
        // Remove path if present
        let host_port = after_protocol.split('/').next().unwrap_or(after_protocol);
        
        // Check if there's a colon (indicating a port)
        if let Some(port_str) = host_port.split(':').nth(1) {
            // Remove any remaining path or query params
            let port_clean = port_str.split('/').next().unwrap_or(port_str)
                .split('?').next().unwrap_or(port_str);
            
            if let Ok(port) = port_clean.parse::<u16>() {
                return Some(port);
            }
        }
    }
    
    None
}

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
                // For online mode, extract port from URL if present, otherwise use default
                // ollama-rs requires a port, but for full URLs like https://host:port/path,
                // we need to extract the port from the URL
                let port = if let Some(port_from_url) = extract_port_from_url(&config.endpoint) {
                    port_from_url
                } else {
                    // Default port for HTTPS (443) or HTTP (80)
                    if config.endpoint.starts_with("https://") {
                        443
                    } else {
                        80
                    }
                };
                Ollama::new(config.endpoint.clone(), port)
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

/// Check Ollama availability and get server information
/// This is the core logic for checking Ollama connection status
/// Returns a JSON value with availability status and available models
/// 
/// Automatically detects mode based on endpoint format:
/// - If endpoint starts with http:// or https:// → Online mode (full URL, no port needed)
/// - Otherwise → Local mode (hostname + port)
pub async fn check_ollama_status(
    host: Option<String>,
    port: Option<u16>
) -> Result<serde_json::Value, String> {
    use tokio::time::{timeout, Duration};
    
    // Use provided config or defaults
    let host_str = host.unwrap_or_else(|| "localhost".to_string());
    let port_num = port.unwrap_or(11434);
    
    // Detect if endpoint is a full URL (starts with http:// or https://)
    let is_full_url = host_str.starts_with("http://") || host_str.starts_with("https://");
    
    let (endpoint, mode, port_config) = if is_full_url {
        // Online mode: use the full URL as-is, no port needed
        // Example: https://{POD_ID}-11434.proxy.runpod.net
        (host_str, OllamaMode::Online, None)
    } else {
        // Local mode: construct URL with hostname and port
        // Extract clean hostname (remove any existing protocol)
        let clean_host = host_str
            .replace("http://", "")
            .replace("https://", "")
            .split(':')
            .next()
            .unwrap_or("localhost")
            .to_string();
        
        let endpoint_url = if clean_host == "localhost" || clean_host == "127.0.0.1" {
            format!("http://{}:{}", clean_host, port_num)
        } else {
            format!("http://{}:{}", clean_host, port_num)
        };
        
        (endpoint_url, OllamaMode::Local, Some(port_num))
    };

    let config = OllamaConfig {
        endpoint: endpoint.clone(),
        port: port_config,
        mode,
    };
    let client = OllamaClient::new(config);

    // Test connection with timeout (3 seconds)
    match timeout(Duration::from_secs(3), client.test_connection()).await {
        Ok(Ok(_)) => {
            // Connection successful, now get models with timeout
            match timeout(Duration::from_secs(3), client.list_models()).await {
                Ok(Ok(models)) => {
                    let model_names: Vec<String> = models.into_iter()
                        .map(|model| model.name)
                        .collect();

                    Ok(serde_json::json!({
                        "available": true,
                        "models_available": model_names
                    }))
                }
                Ok(Err(e)) => Ok(serde_json::json!({
                    "available": false,
                    "error": format!("Failed to list models: {}", e)
                })),
                Err(_) => Ok(serde_json::json!({
                    "available": false,
                    "error": "Connection timeout: Ollama took too long to respond"
                }))
            }
        }
        Ok(Err(e)) => Ok(serde_json::json!({
            "available": false,
            "error": e
        })),
        Err(_) => Ok(serde_json::json!({
            "available": false,
            "error": "Connection timeout: Ollama is not responding"
        }))
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
