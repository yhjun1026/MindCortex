// OpenCode Agent Connector
// 解析 OpenCode 的日志文件和会话数据

use super::{AgentConnector, AgentConfig, SessionData, Message, Attachment};
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
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
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

    pub fn check_logs(&self) -> bool {
        self.log_path.exists()
    }

    pub fn parse_logs(&self) -> Result<Vec<SessionData>, String> {
        if !self.log_path.exists() {
            return Ok(vec![]);
        }

        let content = std::fs::read_to_string(&self.log_path)
            .map_err(|e| format!("Failed to read OpenCode logs: {}", e))?;

        let sessions = self.parse_log_content(&content)?;
        Ok(sessions)
    }

    fn parse_log_content(&self, content: &str) -> Result<Vec<SessionData>, String> {
        let mut sessions: Vec<SessionData> = vec![];
        let mut current_messages: Vec<Message> = vec![];
        let mut current_session_id: Option<String> = None;
        let mut session_start: i64 = Utc::now().timestamp();

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("=== Session:") || trimmed.contains("New conversation") {
                if !current_messages.is_empty() {
                    if let Some(sid) = &current_session_id {
                        sessions.push(self.create_session_data(
                            sid,
                            &current_messages,
                            session_start,
                        ));
                    }
                }

                current_session_id = Some(self.extract_session_id_id(line));
                current_messages.clear();
                session_start = Utc::now().timestamp();
            }
            else if trimmed.starts_with("[User]:") || trimmed.starts_with("[Assistant]:") {
                if let Some(msg) = self.parse_message(line) {
                    current_messages.push(msg);
                }
            }
        }

        if !current_messages.is_empty() {
            if let Some(sid) = &current_session_id {
                sessions.push(self.create_session_data(
                    sid,
                    &current_messages,
                    session_start,
                ));
            }
        }

        Ok(sessions)
    }

    fn extract_session_id_id(&self, line: &str) -> String {
        if let Some(pos) = line.find("Session:") {
            let id_part = line[pos + "Session:".len()..].trim();
            if !id_part.is_empty() {
                return id_part.to_string();
            }
        }
        uuid::Uuid::new_v4().to_string()
    }

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

    fn create_session_data(&self, id: &str, messages: &Vec<Message>, timestamp: i64) -> SessionData {
        SessionData {
            id: id.to_string(),
            agent_type: "opencode".to_string(),
            timestamp,
            messages: messages.clone(),
            metadata: serde_json::json!({
                "source": "logfile",
                "message_count": messages.len()
            }),
        }
    }

    pub fn get_recent_sessions(&self, limit: usize) -> Result<Vec<SessionData>, String> {
        let mut sessions = self.parse_logs()?;
        sessions.reverse();
        sessions.truncate(limit);
        Ok(sessions)
    }

    pub fn watch_logs(&self) -> Result<(), String> {
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
        self.get_recent_sessions(100)
    }

    fn watch_session(&self, _callback: Option<super::SessionCallback>) -> Result<(), String> {
        self.watch_logs()
    }
}
