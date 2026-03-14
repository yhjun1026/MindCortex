// ClaudeCode Agent Connector
// 解析 ClaudeCode 的会话数据

use super::{AgentConnector, AgentConfig, SessionData, Message, Attachment};
use std::path::{Path, PathBuf};
use chrono::Utc;

pub struct ClaudeCodeConnector {
    session_path: PathBuf,
    connected: bool,
}

impl ClaudeCodeConnector {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());
        let session_path = PathBuf::from(home)
            .join(".claude")
            .join("sessions");

        Self {
            session_path,
            connected: false,
        }
    }

    pub fn check_sessions(&self) -> bool {
        self.session_path.exists()
    }

    pub fn get_recent_sessions(&self, limit: usize) -> Result<Vec<SessionData>, String> {
        Ok(vec![])
    }
}

#[async_trait::async_trait]
impl AgentConnector for ClaudeCodeConnector {
    async fn connect(&mut self, _config: &AgentConfig) -> Result<(), String> {
        if self.check_sessions() {
            self.connected = true;
            Ok(())
        } else {
            Err("ClaudeCode sessions not found".to_string())
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
        println!("Watching ClaudeCode sessions at: {:?}", self.session_path);
        Ok(())
    }
}
