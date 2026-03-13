use serde::{Deserialize, Serialize};
use crate::agents::openclaw_real::OpenClawConnector as RealOpenClawConnector;
use crate::extractor::{ExtractedTask, CodeSnippet};

pub mod openclaw_real;
pub mod opencode;

pub use openclaw_real::OpenClawConnector as RealOpenClawConnector;
pub use extractor::KnowledgeItem;

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

/// 会话回调类型
pub type SessionCallback = Box<dyn Fn(SessionData) + Send + Sync>;

/// 创建 Agent 连接器
pub fn create_agent_connector(agent_type: &str) -> Box<dyn AgentConnector> {
    match agent_type {
        "openclaw" => Box::new(OpenClawAdapter::new()),
        "opencode" => Box::new(OpenCodeAdapter::new()),
        _ => Box::new(GenericAdapter::new()),
    }
}

/// OpenClaw 适配器
pub struct OpenClawAdapter {
    connector: RealOpenClawConnector,
}

impl OpenClawAdapter {
    pub fn new() -> Self {
        OpenClawAdapter {
            connector: RealOpenClawConnector::new(),
        }
    }
}

/// OpenCode 适配器
pub struct OpenCodeAdapter {
    // TODO: 实现 OpenCode 连接器
}

impl OpenCodeAdapter {
    pub fn new() -> Self {
        OpenCodeAdapter {}
    }
}

/// Generic 适配器（占位符）
pub struct GenericAdapter;

impl GenericAdapter {
    pub fn new() -> Self {
        GenericAdapter
    }
}

/// Agent 连接器 trait
#[async_trait::async_trait]
pub trait AgentConnector: Send + Sync {
    async fn connect(&mut self, config: &AgentConfig) -> Result<(), String>;
    fn disconnect(&mut self) -> Result<(), String>;
    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String>;
    fn watch_session(&self, callback: SessionCallback) -> Result<(), String>;
}

#[async_trait::async_trait]
impl AgentConnector for OpenClawAdapter {
    async fn connect(&mut self, _config: &AgentConfig) -> Result<(), String> {
        if !self.connector.check_workspace() {
            return Err("OpenClaw workspace not found".to_string());
        }
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String> {
        self.connector.fetch_sessions().await
    }

    fn watch_session(&self, _callback: SessionCallback) -> Result<(), String> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl AgentConnector for OpenCodeAdapter {
    async fn connect(&mut self, _config: &AgentConfig) -> Result<(), String> {
        // TODO: 实现 OpenCode 连接逻辑
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String> {
        // TODO: 实现 OpenCode 会话获取
        Ok(vec![])
    }

    fn watch_session(&self, _callback: SessionCallback) -> Result<(), String> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl AgentConnector for GenericAdapter {
    async fn connect(&mut self, _config: &AgentConfig) -> Result<(), String> {
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String> {
        Ok(vec![])
    }

    fn watch_session(&self, _callback: SessionCallback) -> Result<(), String> {
        Ok(())
    }
}
