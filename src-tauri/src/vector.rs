use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct VectorConfig {
    pub db_path: String,
    pub collection_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub content: String,
    pub metadata: serde_json::Value,
}

// TODO: Implement ChromaDB integration
// pub mod chromadb;
