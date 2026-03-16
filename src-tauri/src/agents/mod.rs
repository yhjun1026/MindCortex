use serde::{Deserialize, Serialize};

pub mod opencode;
pub mod claudecode;
pub mod cursor;
pub mod agent_sync;

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
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: String,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedTask {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub language: String,
    pub code: String,
}

/// 会话回调类型
pub type SessionCallback = Box<dyn Fn(SessionData) + Send + Sync>;

/// 创建 Agent 连接器
pub fn create_agent_connector(agent_type: &str) -> Box<dyn AgentConnector> {
    match agent_type {
        "opencode" => Box::new(OpenCodeAdapter::new()),
        "claudecode" => Box::new(ClaudeCodeAdapter::new()),
        "cursor" => Box::new(CursorAdapter::new()),
        _ => Box::new(GenericAdapter::new()),
    }
}

/// OpenCode 适配器
pub struct OpenCodeAdapter {
    connector: opencode::OpenCodeConnector,
}

impl OpenCodeAdapter {
    pub fn new() -> Self {
        OpenCodeAdapter {
            connector: opencode::OpenCodeConnector::new(),
        }
    }
}

/// ClaudeCode 适配器
pub struct ClaudeCodeAdapter {
    connector: claudecode::ClaudeCodeConnector,
}

impl ClaudeCodeAdapter {
    pub fn new() -> Self {
        ClaudeCodeAdapter {
            connector: claudecode::ClaudeCodeConnector::new(),
        }
    }
}

/// Cursor 适配器
pub struct CursorAdapter {
    connector: cursor::CursorConnector,
}

impl CursorAdapter {
    pub fn new() -> Self {
        CursorAdapter {
            connector: cursor::CursorConnector::new(),
        }
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
    fn watch_session(&self, callback: Option<SessionCallback>) -> Result<(), String>;
}

#[async_trait::async_trait]
impl AgentConnector for OpenCodeAdapter {
    async fn connect(&mut self, _config: &AgentConfig) -> Result<(), String> {
        if !self.connector.check_logs() {
            return Err("OpenCode logs not found".to_string());
        }
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String> {
        self.connector.get_recent_sessions(100)
    }

    fn watch_session(&self, _callback: Option<SessionCallback>) -> Result<(), String> {
        self.connector.watch_logs()
    }
}

#[async_trait::async_trait]
impl AgentConnector for ClaudeCodeAdapter {
    async fn connect(&mut self, _config: &AgentConfig) -> Result<(), String> {
        if !self.connector.check_sessions() {
            return Err("ClaudeCode sessions not found".to_string());
        }
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String> {
        self.connector.get_recent_sessions(100)
    }

    fn watch_session(&self, _callback: Option<SessionCallback>) -> Result<(), String> {
        self.connector.watch_session(_callback)
    }
}

#[async_trait::async_trait]
impl AgentConnector for CursorAdapter {
    async fn connect(&mut self, _config: &AgentConfig) -> Result<(), String> {
        if !self.connector.check_logs() {
            return Err("Cursor logs not found".to_string());
        }
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String> {
        self.connector.get_recent_sessions(100)
    }

    fn watch_session(&self, _callback: Option<SessionCallback>) -> Result<(), String> {
        // CursorConnector 的 watch_session 实现在 async_trait 中
        // 这里返回成功，实际监听逻辑在 connector 内部
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

    fn watch_session(&self, _callback: Option<SessionCallback>) -> Result<(), String> {
        Ok(())
    }
}
