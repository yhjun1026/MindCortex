use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub agent_type: String,
    pub connection_type: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub id: String,
    pub agent_type: String,
    pub timestamp: i64,
    pub messages: Vec<Message>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: i64,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub r#type: String,
    pub content: String,
    pub language: Option<String>,
}

// TODO: Implement specific agent connectors
// pub mod opencode;
// pub mod claudecode;
// pub mod openclaw;
// pub mod cursor;
// pub mod generic;
