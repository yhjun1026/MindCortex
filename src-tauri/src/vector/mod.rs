pub mod chromadb_client;
pub mod ollama_embeddings;
pub mod search_manager;
pub mod index_manager;

pub use chromadb_client::ChromaDBClient;
pub use ollama_embeddings::OllamaEmbeddings;

use serde::{Deserialize, Serialize};

/// 向量搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub content: String,
    pub score: f64,
    pub metadata: SearchResultMetadata,
}

/// 搜索结果元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultMetadata {
    pub source: String,
    pub timestamp: i64,
    pub session_id: Option<String>,
    pub message_id: Option<String>,
    pub file_path: Option<String>,
}

/// 向量文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDocument {
    pub id: String,
    pub text: String,
    pub metadata: serde_json::Value,
}
