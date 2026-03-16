// Agent Health Check Module
// 监控和检查 Agent 连接器的健康状态

use crate::agents::{AgentConnector, AgentConfig, create_agent_connector};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHealthStatus {
    pub agent_type: String,
    pub is_connected: bool,
    pub last_check: i64,
    pub error_message: Option<String>,
    pub logs_accessible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub agent_type: String,
    pub is_healthy: bool,
    pub details: AgentHealthStatus,
}

pub struct AgentHealthChecker {
    agents: Arc<Mutex<HashMap<String, Box<dyn AgentConnector>>>>,
    health_status: Arc<Mutex<HashMap<String, AgentHealthStatus>>>,
}

impl AgentHealthChecker {
    pub fn new() -> Self {
        Self {
            agents: Arc::new(Mutex::new(HashMap::new())),
            health_status: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 注册 Agent
    pub async fn register_agent(&self, agent_type: &str) -> Result<(), String> {
        let mut agents = self.agents.lock().unwrap();
        let connector = create_agent_connector(agent_type);
        agents.insert(agent_type.to_string(), connector);
        Ok(())
    }

    /// 检查所有 Agent 健康状态
    pub async fn check_all_agents(&self) -> Result<Vec<HealthCheckResult>, String> {
        let agents = self.agents.lock().unwrap();
        let mut results = vec![];

        for (agent_type, connector) in agents.iter() {
            let result = self.check_agent_health(agent_type, connector).await;
            results.push(result);
        }

        Ok(results)
    }

    /// 检查单个 Agent 健康状态
    async fn check_agent_health(&self, agent_type: &str, connector: &Box<dyn AgentConnector>) -> HealthCheckResult {
        let mut health_status = AgentHealthStatus {
            agent_type: agent_type.to_string(),
            is_connected: false,
            last_check: chrono::Utc::now().timestamp(),
            error_message: None,
            logs_accessible: false,
        };

        let mut connector_copy = unsafe {
            std::ptr::read(connector as *const _ as usize, std::mem::size_of_val(*connector))
        };

        match connector_copy.connect(&AgentConfig {
            agent_type: agent_type.to_string(),
            connection_type: "auto".to_string(),
            config: serde_json::json!({}),
        }).await {
            Ok(_) => {
                health_status.is_connected = true;
                health_status.logs_accessible = true;
            }
            Err(e) => {
                health_status.error_message = Some(e);
            }
        }

        HealthCheckResult {
            agent_type: agent_type.to_string(),
            is_healthy: health_status.is_connected && health_status.logs_accessible,
            details: health_status,
        }
    }

    /// 获取所有 Agent 的健康状态
    pub fn get_all_health_status(&self) -> HashMap<String, AgentHealthStatus> {
        self.health_status.lock().unwrap().clone()
    }

    /// 获取指定 Agent 的健康状态
    pub fn get_health_status(&self, agent_type: &str) -> Option<AgentHealthStatus> {
        self.health_status.lock().unwrap().get(agent_type).cloned()
    }
}

impl Default for AgentHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}
