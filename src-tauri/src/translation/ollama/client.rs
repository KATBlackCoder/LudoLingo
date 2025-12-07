// Ollama HTTP client using ollama-rs crate (https://github.com/pepperoni21/ollama-rs)
// Local mode only: connects to localhost Ollama instance
// Provides access to full Ollama API: generation, chat, models, embeddings, tools
//
// Why ollama-rs instead of custom HTTP client?
// - Well-tested and maintained (936⭐, active development)
// - Complete API coverage (generation, chat, streaming, tools)
// - Proper error handling and types
// - Future-proof with Ollama API updates
// - Less code to maintain and debug

use ollama_rs::Ollama;
use crate::translation::common::functions::TranslationClient;

/// Re-export Ollama types for convenience
pub use ollama_rs::models::LocalModel as ModelInfo;

/// Ollama client configuration (local only)
#[derive(Debug, Clone)]
pub struct OllamaConfig {
    pub endpoint: String,
    pub port: u16,
}

/// Ollama HTTP client wrapper
#[derive(Clone)]
pub struct OllamaClient {
    client: Ollama,
    config: OllamaConfig,
}

impl OllamaClient {
    /// Create new Ollama client (local only)
    pub fn new(config: OllamaConfig) -> Self {
        let client = Ollama::new(config.endpoint.clone(), config.port);
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

impl TranslationClient for OllamaClient {
    fn call_api(&self, prompt: &str, model: Option<String>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, String>> + Send + '_>> {
        let prompt = prompt.to_string();
        let model = model.clone();
        let client = self.client.clone();

        Box::pin(async move {
            use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage};
            use crate::translation::ollama::get_translation_model_options;

            // Get model name or use default
            let model_name = model.unwrap_or_else(|| crate::translation::ollama::get_default_model());

            // Create chat request for translation
            let messages = vec![ChatMessage::user(prompt)];
            let request = ChatMessageRequest::new(model_name, messages)
                .options(get_translation_model_options());

            // Call Ollama API
            match client.send_chat_messages(request).await {
                Ok(response) => {
                    Ok(response.message.content)
                }
                Err(e) => Err(format!("Ollama API error: {}", e)),
            }
        })
    }

    fn list_models(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<String>, String>> + Send + '_>> {
        let client = self.client.clone();
        Box::pin(async move {
            match client.list_local_models().await {
                Ok(models) => {
                    let model_names = models.into_iter()
                        .map(|model| model.name)
                        .collect();
                    Ok(model_names)
                }
                Err(e) => Err(format!("Failed to list Ollama models: {}", e)),
            }
        })
    }

    fn test_connection(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send + '_>> {
        let client = self.client.clone();
        Box::pin(async move {
            match client.list_local_models().await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Ollama connection test failed: {}", e)),
            }
        })
    }
}

/// Check Ollama availability and get server information (local only)
/// This is the core logic for checking Ollama connection status
/// Returns a JSON value with availability status and available models
pub async fn check_ollama_status(
    host: Option<String>,
    port: Option<u16>,
) -> Result<serde_json::Value, String> {
    use tokio::time::{timeout, Duration};

    // Use provided config or defaults
    let host_str = host.unwrap_or_else(|| "localhost".to_string());
    let port_num = port.unwrap_or(11434);

    // Local mode: construct URL with hostname and port
    // Extract clean hostname (remove any existing protocol)
    let clean_host = host_str
        .replace("http://", "")
        .replace("https://", "")
        .split(':')
        .next()
        .unwrap_or("localhost")
        .to_string();

    let endpoint = format!("http://{}:{}", clean_host, port_num);

    let config = OllamaConfig {
        endpoint: endpoint.clone(),
        port: port_num,
    };
    let client = OllamaClient::new(config);

    // Test connection with timeout (3 seconds)
    match timeout(Duration::from_secs(3), client.test_connection()).await {
        Ok(Ok(_)) => {
            // Connection successful, now get models with timeout
            match timeout(Duration::from_secs(3), client.list_models()).await {
                Ok(Ok(models)) => {
                    let model_names: Vec<String> =
                        models.into_iter().map(|model| model.name).collect();

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
                })),
            }
        }
        Ok(Err(e)) => Ok(serde_json::json!({
            "available": false,
            "error": e
        })),
        Err(_) => Ok(serde_json::json!({
            "available": false,
            "error": "Connection timeout: Ollama is not responding"
        })),
    }
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost".to_string(),
            port: 11434,
        }
    }
}
