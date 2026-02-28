use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::{Result, Context};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
    pub models: ModelConfig,
    pub vector_db: VectorDbConfig,
    pub storage: StorageConfig,
    pub agents: Vec<super::agents::AgentConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub embedding: ProviderConfig,
    pub rerank: ProviderConfig,
    pub reasoning: ProviderConfig,
    pub image: ProviderConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: String,
    pub model: String,
    pub api_key: String,
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDbConfig {
    #[serde(rename = "type")]
    pub db_type: String,
    pub path: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub base_path: String,
    pub auto_sync: bool,
    pub sync_interval: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            version: "1.0.0".to_string(),
            models: ModelConfig {
                embedding: ProviderConfig {
                    provider: "ollama".to_string(),
                    model: "nomic-embed-text".to_string(),
                    api_key: String::new(),
                    base_url: "http://localhost:11434".to_string(),
                },
                rerank: ProviderConfig {
                    provider: "ollama".to_string(),
                    model: "bge-reranker-v2".to_string(),
                    api_key: String::new(),
                    base_url: "http://localhost:11434".to_string(),
                },
                reasoning: ProviderConfig {
                    provider: "ollama".to_string(),
                    model: "llama3.2".to_string(),
                    api_key: String::new(),
                    base_url: "http://localhost:114114".to_string(),
                },
                image: ProviderConfig {
                    provider: "ollama".to_string(),
                    model: "llava".to_string(),
                    api_key: String::new(),
                    base_url: "http://localhost:11434".to_string(),
                },
            },
            vector_db: VectorDbConfig {
                db_type: "chromadb".to_string(),
                path: "./data/chromadb".to_string(),
                collection_name: "cortexmind".to_string(),
            },
            storage: StorageConfig {
                base_path: "./knowledge".to_string(),
                auto_sync: true,
                sync_interval: 300,
            },
            agents: vec![],
        }
    }
}

pub fn load_config(path: &str) -> Result<AppConfig> {
    if let Ok(content) = fs::read_to_string(path) {
        serde_json::from_str(&content).context("Failed to parse config file")
    } else {
        // Return default config if file doesn't exist
        Ok(AppConfig::default())
    }
}

pub fn save_config(path: &str, config: &AppConfig) -> Result<()> {
    let content = serde_json::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}
