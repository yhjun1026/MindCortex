// Agent 连接器抽象和实现
// 支持不同类型的 AI coding agents (Claude Code, Cursor, OpenCode 等)

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use super::task::AgentTask;

/// 连接器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectorType {
    ClaudeCode,
    Cursor,
    OpenCode,
    OpenClaw,
    Custom,
}

impl ConnectorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectorType::ClaudeCode => "claude-code",
            ConnectorType::Cursor => "cursor",
            ConnectorType::OpenCode => "opencode",
            ConnectorType::OpenClaw => "openclaw",
            ConnectorType::Custom => "custom",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "claude-code" | "claudecode" => Some(ConnectorType::ClaudeCode),
            "cursor" => Some(ConnectorType::Cursor),
            "opencode" | "open-code" => Some(ConnectorType::OpenCode),
            "openclaw" | "open-claw" => Some(ConnectorType::OpenClaw),
            "custom" => Some(ConnectorType::Custom),
            _ => None,
        }
    }
}

impl std::fmt::Display for ConnectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 连接器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorConfig {
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub custom_headers: Option<std::collections::HashMap<String, String>>,
    pub auth_config: Option<AuthConfig>,
}

impl Default for ConnectorConfig {
    fn default() -> Self {
        Self {
            api_endpoint: None,
            api_key: None,
            timeout_seconds: 30,
            max_retries: 3,
            custom_headers: None,
            auth_config: None,
        }
    }
}

/// 认证配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub auth_type: String, // "api_key", "oauth", "token"
    pub credentials: serde_json::Value,
}

/// Agent 连接器 trait
#[async_trait]
pub trait AgentConnector: Send + Sync {
    /// 连接到 Agent
    async fn connect(&self) -> Result<(), String>;
    
    /// 断开连接
    async fn disconnect(&self) -> Result<(), String>;
    
    /// 检查连接状态
    async fn ping(&self) -> Result<bool, String>;
    
    /// 执行任务
    async fn execute_task(&self, task: &AgentTask) -> Result<serde_json::Value, String>;
    
    /// 获取会话列表
    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String>;
    
    /// 获取连接器类型
    fn get_type(&self) -> ConnectorType;
    
    /// 获取连接器名称
    fn get_name(&self) -> &str;
}

/// 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,               // "user", "assistant", "system"
    pub content: String,
    pub timestamp: i64,
    pub attachments: Vec<Attachment>,
}

/// 附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub r#type: String,             // "code", "image", "file"
    pub content: String,
    pub language: Option<String>,   // for code
}

/// 会话数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub session_id: String,
    pub id: String,  // Alias for session_id, used by extractor
    pub title: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub message_count: i32,
    pub status: String,
    pub messages: Vec<Message>,  // Messages in this session
    pub timestamp: i64,  // Alias for created_at, used by extractor
    pub metadata: Option<serde_json::Value>,
}

/// 创建连接器工厂函数
pub fn create_connector(
    connector_type: &ConnectorType,
    config: &ConnectorConfig,
) -> Result<Arc<dyn AgentConnector + Send + Sync>, String> {
    match connector_type {
        ConnectorType::ClaudeCode => {
            Ok(Arc::new(ClaudeCodeConnector::new(config.clone())))
        }
        ConnectorType::Cursor => {
            Ok(Arc::new(CursorConnector::new(config.clone())))
        }
        ConnectorType::OpenCode => {
            Ok(Arc::new(OpenCodeConnector::new(config.clone())))
        }
        ConnectorType::OpenClaw => {
            Ok(Arc::new(OpenClawConnector::new(config.clone())))
        }
        ConnectorType::Custom => {
            Err("Custom connectors require additional implementation".to_string())
        }
    }
}

// Claude Code 连接器实现
pub struct ClaudeCodeConnector {
    config: ConnectorConfig,
    connected: Arc<tokio::sync::RwLock<bool>>,
}

impl ClaudeCodeConnector {
    pub fn new(config: ConnectorConfig) -> Self {
        Self {
            config,
            connected: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }
}

#[async_trait]
impl AgentConnector for ClaudeCodeConnector {
    async fn connect(&self) -> Result<(), String> {
        // 实际实现：连接到 Claude Code API
        // 这里是示例实现
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        *self.connected.write().await = true;
        Ok(())
    }

    async fn disconnect(&self) -> Result<(), String> {
        *self.connected.write().await = false;
        Ok(())
    }

    async fn ping(&self) -> Result<bool, String> {
        let connected = *self.connected.read().await;
        if connected {
            // 实际实现：发送 ping 请求
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn execute_task(&self, task: &AgentTask) -> Result<serde_json::Value, String> {
        // 实际实现：执行 Claude Code 任务
        Ok(serde_json::json!({
            "task_id": task.id,
            "status": "completed",
            "result": "Task executed successfully"
        }))
    }

    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String> {
        // 实际实现：获取 Claude Code 会话列表
        Ok(vec![])
    }

    fn get_type(&self) -> ConnectorType {
        ConnectorType::ClaudeCode
    }

    fn get_name(&self) -> &str {
        "Claude Code"
    }
}

// Cursor 连接器实现
pub struct CursorConnector {
    config: ConnectorConfig,
    connected: Arc<tokio::sync::RwLock<bool>>,
}

impl CursorConnector {
    pub fn new(config: ConnectorConfig) -> Self {
        Self {
            config,
            connected: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }
}

#[async_trait]
impl AgentConnector for CursorConnector {
    async fn connect(&self) -> Result<(), String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        *self.connected.write().await = true;
        Ok(())
    }

    async fn disconnect(&self) -> Result<(), String> {
        *self.connected.write().await = false;
        Ok(())
    }

    async fn ping(&self) -> Result<bool, String> {
        let connected = *self.connected.read().await;
        Ok(connected)
    }

    async fn execute_task(&self, task: &AgentTask) -> Result<serde_json::Value, String> {
        Ok(serde_json::json!({
            "task_id": task.id,
            "status": "completed",
            "result": "Task executed successfully"
        }))
    }

    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String> {
        Ok(vec![])
    }

    fn get_type(&self) -> ConnectorType {
        ConnectorType::Cursor
    }

    fn get_name(&self) -> &str {
        "Cursor"
    }
}

// OpenCode 连接器实现
pub struct OpenCodeConnector {
    config: ConnectorConfig,
    connected: Arc<tokio::sync::RwLock<bool>>,
}

impl OpenCodeConnector {
    pub fn new(config: ConnectorConfig) -> Self {
        Self {
            config,
            connected: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }
}

#[async_trait]
impl AgentConnector for OpenCodeConnector {
    async fn connect(&self) -> Result<(), String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        *self.connected.write().await = true;
        Ok(())
    }

    async fn disconnect(&self) -> Result<(), String> {
        *self.connected.write().await = false;
        Ok(())
    }

    async fn ping(&self) -> Result<bool, String> {
        let connected = *self.connected.read().await;
        Ok(connected)
    }

    async fn execute_task(&self, task: &AgentTask) -> Result<serde_json::Value, String> {
        Ok(serde_json::json!({
            "task_id": task.id,
            "status": "completed",
            "result": "Task executed successfully"
        }))
    }

    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String> {
        Ok(vec![])
    }

    fn get_type(&self) -> ConnectorType {
        ConnectorType::OpenCode
    }

    fn get_name(&self) -> &str {
        "OpenCode"
    }
}

// OpenClaw 连接器实现
pub struct OpenClawConnector {
    config: ConnectorConfig,
    connected: Arc<tokio::sync::RwLock<bool>>,
}

impl OpenClawConnector {
    pub fn new(config: ConnectorConfig) -> Self {
        Self {
            config,
            connected: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }
}

#[async_trait]
impl AgentConnector for OpenClawConnector {
    async fn connect(&self) -> Result<(), String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        *self.connected.write().await = true;
        Ok(())
    }

    async fn disconnect(&self) -> Result<(), String> {
        *self.connected.write().await = false;
        Ok(())
    }

    async fn ping(&self) -> Result<bool, String> {
        let connected = *self.connected.read().await;
        Ok(connected)
    }

    async fn execute_task(&self, task: &AgentTask) -> Result<serde_json::Value, String> {
        Ok(serde_json::json!({
            "task_id": task.id,
            "status": "completed",
            "result": "Task executed successfully"
        }))
    }

    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String> {
        Ok(vec![])
    }

    fn get_type(&self) -> ConnectorType {
        ConnectorType::OpenClaw
    }

    fn get_name(&self) -> &str {
        "OpenClaw"
    }
}
