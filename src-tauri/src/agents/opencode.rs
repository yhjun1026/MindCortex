// OpenCode Agent Connector
// 解析 OpenCode 的日志文件和会话数据

use super::{AgentConnector, AgentConfig, SessionData, Message};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc, TimeZone};
use serde_json::Value;

/// OpenCode 日志格式
#[derive(Debug, Clone)]
pub struct OpenCodeLog {
    pub timestamp: i64,
    pub role: String,
    pub content: String,
    pub metadata: OpenCodeMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeMetadata {
    pub session_id: Option<String>,
    pub model: Option<String>,
    pub tokens: Option<i64>,
    pub file_path: Option<String>,
}

pub struct OpenCodeConnector {
    log_path: PathBuf,
    connected: bool,
}

impl OpenCodeConnector {
    pub fn new() -> Self {
        // 默认 OpenCode 日志路径
        let home = std::env::var("HOME").unwrap_or_else(|| ".".to_string());
        let log_path = PathBuf::from(home)
            .join(".opencode")
            .join("logs")
            .join("sessions.log");

        Self {
            log_path,
            connected: false,
        }
    }

    pub fn with_log_path(mut self, log_path: &str) -> Self {
        self.log_path = PathBuf::from(log_path);
        self
    }

    /// 检查 OpenCode 日志是否存在
    pub fn check_logs(&self) -> bool {
        self.log_path.exists()
    }

    /// 解析 OpenCode 日志文件
    pub fn parse_logs(&self) -> Result<Vec<SessionData>, String> {
        if !self.log_path.exists() {
            return Ok(vec![]);
        }

        let content = std::fs::read_to_string(&self.log_path)
            .map_err(|e| format!("Failed to read OpenCode logs: {}", e))?;

        let sessions = self.parse_log_content(&content)?;
        Ok(sessions)
    }

    /// 解析日志内容
    fn parse_log_content(&self, content: &str) -> Result<Vec<SessionData>, String> {
        let mut sessions: Vec<SessionData> = vec![];
        let mut current_messages: Vec<Message> = vec![];
        let mut current_session_id: Option<String> = None;
        let mut session_start: i64 = Utc::now().timestamp();

        // OpenCode 日志格式解析
        for line in content.lines() {
            let trimmed = line.trim();

            // 会话开始标记
            if trimmed.starts_with("=== Session:") || trimmed.contains("New conversation") {
                // 保存上一个会话
                if !current_messages.is_empty() {
                    if let Some(sid) = current_session_id {
                        sessions.push(self.create_session_data(
                            &sid,
                            current_messages.clone(),
                            session_start,
                        ));
                    }
                }

                // 开始新会话
                current_session_id = Some(self.extract_session_id(line));
                current_messages.clear();
                session_start = Utc::now().timestamp();
            }
            // 消息行
            else if trimmed.starts_with("[User]:") || trimmed.starts_with("[Assistant]:") {
                if let Some(msg) = self.parse_message(line) {
                    current_messages.push(msg);
                }
            }
        }

        // 保存最后一个会话
        if !current_messages.is_empty() {
            if let Some(sid) = current_session_id {
                sessions.push(self.create_session_data(
                    &sid,
                    current_messages,
                    session_start,
                ));
            }
        }

        Ok(sessions)
    }

    /// 提取会话 ID
    fn extract_session_id(&self, line: &str) -> String {
        // 尝试从日志行提取 ID
        if let Some(pos) = line.find("Session:") {
)            let id_part = line[pos + "Session:".len()..].trim();
            if !id_part.is_empty() {
                return id_part.to_string();
            }
        }
        uuid::Uuid::new_v4().to_string()
    }

    /// 解析消息行
    fn parse_message(&self, line: &str) -> Option<Message> {
        let trimmed = line.trim();

        if trimmed.starts_with("[User]:") {
            Some(Message {
                role: "user".to_string(),
                content: trimmed["[User]:".len()..].trim().to_string(),
                timestamp: Utc::now().timestamp(),
                attachments: vec![],
            })
        } else if trimmed.starts_with("[Assistant]:") {
            Some(Message {
                role: "assistant".to_string(),
                content: trimmed["[Assistant]:".len()..].trim().to_string(),
                timestamp: Utc::now().timestamp(),
                attachments: vec![],
            })
        } else {
            None
        }
    }

    /// 创建会话数据
    fn create_session_data(&self, id: &str, messages: Vec<Message>, timestamp: i64) -> SessionData {
        SessionData {
            id: id.to_string(),
            agent_type: "opencode".to_string(),
            timestamp,
            messages,
            metadata: serde_json::json!({
                "source": "logfile",
                "message_count": messages.len()
            }),
        }
    }

    /// 获取最近的会话
    pub fn get_recent_sessions(&self, limit: usize) -> Result<Vec<SessionData>, String> {
        let mut sessions = self.parse_logs()?;
        sessions.reverse(); // 最新的在前
        sessions.truncate(limit);
        Ok(sessions)
    }

    /// 监听日志文件变化
    pub fn watch_logs(&self) -> Result<(), String> {
        // TODO: 实现文件监听器
        // 使用 notify crate 监听日志文件变化
        println!("Watching OpenCode logs at: {:?}", self.log_path);
        Ok(())
    }
}

#[async_trait::async_trait]
impl AgentConnector for OpenCodeConnector {
    async fn connect(&mut self, _config: &AgentConfig) -> Result<(), String> {
        if self.check_logs() {
            self.connected = true;
            Ok(())
        } else {
            Err("OpenCode logs not found".to_string())
        }
    }

    fn disconnect(&mut self) -> Result<(), String> {
        self.connected = false;
        Ok(())
    }

    async fn fetch_sessions(&self) -> Result<Vec<SessionData>, String> {
        self.get_recent_sessions(100) // 获取最近 100 个会话
    }

    fn watch_session(&self, _callback: crate::agents::SessionCallback) -> Result<(), String> {
        self.watch_logs()
    }
}
