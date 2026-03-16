// Agent Session Sync Manager
// 管理 Agent 会话数据的同步和调度

use crate::agents::{AgentConnector, SessionData};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::interval;

/// Agent 连接器包装，支持 Send + Sync
struct SyncableConnector {
    connector: Arc<dyn AgentConnector + Send + Sync>,
}

impl SyncableConnector {
    fn new(connector: Arc<dyn AgentConnector + Send + Sync>) -> Self {
        Self { connector }
    }
}

pub struct AgentSessionSync {
    connectors: Arc<Mutex<HashMap<String, Arc<dyn AgentConnector + Send + Sync>>>>,
    last_sync: Arc<Mutex<HashMap<String, i64>>>,
}

impl AgentSessionSync {
    pub fn new() -> Self {
        Self {
            connectors: Arc::new(Mutex::new(HashMap::new())),
            last_sync: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 注册 Agent 连接器
    pub async fn register_agent(&self, agent_type: &str, connector: Arc<dyn AgentConnector + Send + Sync>) {
        let mut connectors = self.connectors.lock().unwrap();
        connectors.insert(agent_type.to_string(), connector);
    }

    /// 同步所有 Agent 的会话
    pub async fn sync_all_agents(&self) -> Result<Vec<SessionData>, String> {
        let mut all_sessions = vec![];
        let connectors = self.connectors.lock().unwrap();
        
        // 收集所有 connector 的 Arc 引用
        let connector_refs: Vec<(String, Arc<dyn AgentConnector + Send + Sync>)> = 
            connectors.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        
        drop(connectors); // 释放锁

        for (agent_type, connector) in connector_refs {
            match connector.fetch_sessions().await {
                Ok(sessions) => {
                    println!("Synced {} sessions from {}", sessions.len(), agent_type);
                    
                    let mut last_sync = self.last_sync.lock().unwrap();
                    last_sync.insert(agent_type.clone(), chrono::Utc::now().timestamp());
                    
                    all_sessions.extend(sessions);
                }
                Err(e) => {
                    eprintln!("Failed to sync {}: {}", agent_type, e);
                }
            }
        }

        Ok(all_sessions)
    }

    /// 同步指定 Agent
    pub async fn sync_agent(&self, agent_type: &str) -> Result<Vec<SessionData>, String> {
        let connectors = self.connectors.lock().unwrap();
        
        if let Some(connector) = connectors.get(agent_type) {
            let connector_arc = connector.clone();
            drop(connectors);
            
            let sessions = connector_arc.fetch_sessions().await?;
            
            let mut last_sync = self.last_sync.lock().unwrap();
            last_sync.insert(agent_type.to_string(), chrono::Utc::now().timestamp());
            
            Ok(sessions)
        } else {
            Err(format!("Agent {} not registered", agent_type))
        }
    }

    /// 启动自动同步任务
    pub fn start_auto_sync(&self, interval_seconds: u64) {
        let connectors = self.connectors.clone();
        let last_sync = self.last_sync.clone();
        
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(interval_seconds));
            
            loop {
                ticker.tick().await;
                println!("Auto-sync tick triggered");
                
                // 获取所有 connector 的 Arc 引用
                let connector_refs: Vec<(String, Arc<dyn AgentConnector + Send + Sync>)> = {
                    let connectors = connectors.lock().unwrap();
                    connectors.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
                };
                
                for (agent_type, connector) in connector_refs {
                    match connector.fetch_sessions().await {
                        Ok(sessions) => {
                            println!("Auto-synced {} sessions from {}", sessions.len(), agent_type);
                            
                            let mut last_sync = last_sync.lock().unwrap();
                            last_sync.insert(agent_type.clone(), chrono::Utc::now().timestamp());
                        }
                        Err(e) => {
                            eprintln!("Auto-sync failed for {}: {}", agent_type, e);
                        }
                    }
                }
            }
        });
    }

    /// 获取同步状态
    pub fn get_sync_status(&self) -> HashMap<String, i64> {
        self.last_sync.lock().unwrap().clone()
    }

    /// 检查是否需要同步
    pub fn needs_sync(&self, agent_type: &str, interval_seconds: i64) -> bool {
        let last_sync = self.last_sync.lock().unwrap();
        
        if let Some(last) = last_sync.get(agent_type) {
            let now = chrono::Utc::now().timestamp();
            let elapsed = now - last;
            elapsed > interval_seconds
        } else {
            true // 从未同步过
        }
    }
}

impl Default for AgentSessionSync {
    fn default() -> Self {
        Self::new()
    }
}
