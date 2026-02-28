use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeItem {
    pub id: String,
    pub source_session_id: String,
    pub item_type: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub tags: Vec<String>,
    pub project: Option<String>,
    pub embedding: Option<Vec<f32>>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub tasks: Vec<String>,
    pub knowledge_items: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

// TODO: Implement extraction logic
// pub mod classification;
// pub mod summarization;
// pub mod insight;
