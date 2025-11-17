// RunPod HTTP client using reqwest
// Connects to RunPod Ollama instance via HTTP
// URL format: https://{pod_id}-11434.proxy.runpod.net

use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Model information (matching Ollama API response)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub modified_at: Option<String>,
    pub size: Option<u64>,
}

/// RunPod client configuration
#[derive(Debug, Clone)]
pub struct RunPodConfig {
    pub pod_id: String,
}

/// RunPod HTTP client wrapper
#[derive(Clone)]
pub struct RunPodClient {
    client: Client,
    config: RunPodConfig,
    base_url: String,
}

impl RunPodClient {
    /// Create new RunPod client
    /// Automatically constructs URL: https://{pod_id}-11434.proxy.runpod.net
    pub fn new(config: RunPodConfig) -> Self {
        let base_url = format!("https://{}-11434.proxy.runpod.net", config.pod_id);
        let client = Client::new();
        
        Self {
            client,
            config,
            base_url,
        }
    }

    /// Get configuration
    pub fn config(&self) -> &RunPodConfig {
        &self.config
    }

    /// Get base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Test connection to RunPod Ollama server
    pub async fn test_connection(&self) -> Result<(), String> {
        // Try to list models as a connection test
        match self.list_models().await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Connection test failed: {}", e)),
        }
    }

    /// List available models
    /// GET /api/tags
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, String> {
        let url = format!("{}/api/tags", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        #[derive(Deserialize)]
        struct TagsResponse {
            models: Vec<ModelInfo>,
        }

        let tags_response: TagsResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(tags_response.models)
    }

    /// Chat with Ollama model
    /// POST /api/chat
    pub async fn chat(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
        options: Option<ChatOptions>,
    ) -> Result<ChatResponse, String> {
        let url = format!("{}/api/chat", self.base_url);
        
        println!("🌐 [Rust] RunPod chat request to: {}", url);
        println!("🌐 [Rust] Model: {}, Messages count: {}", model, messages.len());

        #[derive(Serialize)]
        struct ChatRequest {
            model: String,
            messages: Vec<ChatMessage>,
            #[serde(skip_serializing_if = "Option::is_none")]
            options: Option<ChatOptions>,
            stream: bool,
        }

        let request = ChatRequest {
            model: model.to_string(),
            messages,
            options,
            stream: false,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                println!("❌ [Rust] Failed to send RunPod chat request: {}", e);
                format!("Failed to send request: {}", e)
            })?;

        let status = response.status();
        println!("📡 [Rust] RunPod response status: {}", status);

        if !status.is_success() {
            // Try to read error body for more details
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error body".to_string());
            println!("❌ [Rust] RunPod HTTP error {}: {}", status, error_body);
            return Err(format!("HTTP error {}: {}", status, error_body));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| {
                println!("❌ [Rust] Failed to parse RunPod chat response: {}", e);
                format!("Failed to parse response: {}", e)
            })?;

        println!("✅ [Rust] RunPod chat response received successfully");
        Ok(chat_response)
    }
}

/// Chat message (matching Ollama API format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String, // "user", "assistant", "system"
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: String) -> Self {
        Self {
            role: "user".to_string(),
            content,
        }
    }

    pub fn assistant(content: String) -> Self {
        Self {
            role: "assistant".to_string(),
            content,
        }
    }

    pub fn system(content: String) -> Self {
        Self {
            role: "system".to_string(),
            content,
        }
    }
}

/// Chat options (matching Ollama API format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_last_n: Option<i32>,
}

/// Chat response (matching Ollama API format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
}

impl Default for RunPodConfig {
    fn default() -> Self {
        Self {
            pod_id: String::new(),
        }
    }
}

/// Check RunPod availability and get server information
/// Returns a JSON value with availability status and available models
pub async fn check_runpod_status(
    pod_id: Option<String>
) -> Result<serde_json::Value, String> {
    use tokio::time::{timeout, Duration};
    
    let pod_id_str = pod_id.ok_or_else(|| {
        println!("❌ [Rust] POD_ID is None");
        "POD_ID is required".to_string()
    })?;
    
    println!("🔍 [Rust] check_runpod_status: pod_id_str = {:?}", pod_id_str);
    
    if pod_id_str.is_empty() {
        println!("❌ [Rust] POD_ID is empty");
        return Ok(serde_json::json!({
            "available": false,
            "error": "POD_ID cannot be empty"
        }));
    }

    let config = RunPodConfig {
        pod_id: pod_id_str.clone(),
    };
    let client = RunPodClient::new(config);
    
    println!("🌐 [Rust] Testing connection to RunPod: https://{}-11434.proxy.runpod.net", pod_id_str);

    // Test connection with timeout (5 seconds for RunPod)
    match timeout(Duration::from_secs(5), client.test_connection()).await {
        Ok(Ok(_)) => {
            println!("✅ [Rust] RunPod connection test successful");
            // Connection successful, now get models with timeout
            match timeout(Duration::from_secs(5), client.list_models()).await {
                Ok(Ok(models)) => {
                    let model_names: Vec<String> = models.into_iter()
                        .map(|model| model.name)
                        .collect();
                    
                    println!("✅ [Rust] Found {} models: {:?}", model_names.len(), model_names);

                    Ok(serde_json::json!({
                        "available": true,
                        "models_available": model_names
                    }))
                }
                Ok(Err(e)) => {
                    println!("❌ [Rust] Failed to list models: {}", e);
                    Ok(serde_json::json!({
                        "available": false,
                        "error": format!("Failed to list models: {}", e)
                    }))
                },
                Err(_) => {
                    println!("❌ [Rust] Timeout while listing models");
                    Ok(serde_json::json!({
                        "available": false,
                        "error": "Connection timeout: RunPod took too long to respond"
                    }))
                }
            }
        }
        Ok(Err(e)) => {
            println!("❌ [Rust] Connection test failed: {}", e);
            Ok(serde_json::json!({
                "available": false,
                "error": e
            }))
        },
        Err(_) => {
            println!("❌ [Rust] Connection timeout");
            Ok(serde_json::json!({
                "available": false,
                "error": "Connection timeout: RunPod is not responding"
            }))
        }
    }
}

