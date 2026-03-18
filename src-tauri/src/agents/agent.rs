// Agent 核心实现
// 定义 Agent 的核心状态和行为

use super::connector::AgentConnector;
use super::task::{AgentTask, TaskConfig, TaskStatus, TaskPriority};
use super::connector::ConnectorConfig;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::{Arc, RwLock};

/// Agent 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    Inactive,
    Starting,
    Active,
    Paused.
    Error,
    ShuttingDown,
}

impl AgentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AgentStatus::Inactive => "inactive",
            AgentStatus::Starting => "starting",
            AgentStatus::Active => "active",
            AgentStatus::Paused => "paused",
            AgentStatus::Error => "error",
            AgentStatus::ShuttingDown => "shutting_down",
        }
    }
}

impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Agent 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub max_concurrent_tasks: Option<u32>,
    pub task_timeout_seconds: Option<u64>,
    pub retry_on_failure: bool,
    pub max_retries: u32,
    pub auto_heartbeat: bool,
    pub heartbeat_interval_seconds: u64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: Some(3),
            task_timeout_seconds: Some(300),
            retry_on_failure: true,
            max_retries: 3,
            auto_heartbeat: true,
            heartbeat_interval_seconds: 60,
        }
    }
}

/// Agent - 核心类
pub struct Agent {
    pub id: String,
    pub name: String,
    pub agent_type: String,
    pub connector: Arc<dyn AgentConnector + Send + Sync>,
    
    pub status: AgentStatus,
    pub config: AgentConfig,
    
    pub collection_enabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub last_activity: Option<i64>,
    
    // 内部状态
    tasks: Arc<RwLock<Vec<String>>>, // 任务 ID 列表
    error_message: Option<String>,
}

impl Agent {
    pub fn new(
        id: String,
        name: String,
        agent_type: String,
        connector: Arc<dyn AgentConnector + Send + Sync>,
    ) -> Self {
        let now = Utc::now().timestamp();
        
        Agent {
            id,
            name,
            agent_type,
            connector,
            status: AgentStatus::Inactive,
            config: AgentConfig::default(),
            collection_enabled: false,
            created_at: now,
            updated_at: now,
            last_activity: None,
            tasks: Arc::new(RwLock::new(Vec::new())),
            error_message: None,
        }
    }

    /// 应用配置
    pub fn apply_config(&mut self, config: AgentConfig) -> Result<(), String> {
        self.config = config;
        self.updated_at = Utc::now().timestamp();
        Ok(())
    }

    /// 激活 Agent
    pub async fn activate(&mut self) -> Result<(), String> {
        if self.status == AgentStatus::Active {
            return Ok(());
        }

        self.status = AgentStatus::Starting;
        
        // 测试连接
        match self.connector.connect().await {
            Ok(_) => {
                self.status = AgentStatus::Active;
                self.updated_at = Utc::now().timestamp();
                self.last_activity = Some(Utc::now().timestamp());
                Ok(())
            }
            Err(e) => {
                self.status = AgentStatus::Error;
                self.error_message = Some(format!("Failed to connect: {}", e));
                Err(format!("Failed to activate agent: {}", e))
            }
        }
    }

    /// 暂停 Agent
    pub async fn pause(&mut self) -> Result<(), String> {
        if self.status != AgentStatus::Active {
            return Err("Agent is not active".to_string());
        }

        self.status = AgentStatus::Paused;
        self.updated_at = Utc::now().timestamp();
        Ok(())
    }

    /// 恢复 Agent
    pub async fn resume(&mut self) -> Result<(), String> {
        if self.status != AgentStatus::Paused {
            return Err("Agent is not paused".to_string());
        }

        self.status = AgentStatus::Active;
        self.updated_at = Utc::now().timestamp();
        self.last_activity = Some(Utc::now().timestamp());
        Ok(())
    }

    /// 关闭 Agent
    pub async fn shutdown(&mut self) -> Result<(), String> {
        self.status = AgentStatus::ShuttingDown;
        
        // 断开连接
        self.connector.disconnect().await?;
        
        self.status = AgentStatus::Inactive;
        self.updated_at = Utc::now().timestamp();
        Ok(())
    }

    /// 创建任务
    pub async fn create_task(&self, config: TaskConfig) -> Result<AgentTask, String> {
        if self.status != AgentStatus::Active {
            return Err("Agent is not active".to_string());
        }

        let task_id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp();

        let task = AgentTask {
            id: task_id,
            agent_id: self.id.clone(),
            task_type: config.task_type.clone(),
            status: TaskStatus::Pending,
            priority: config.priority,
            payload: config.payload,
            result: None,
            error: None,
            created_at: now,
            started_at: None,
            completed_at: None,
            duration_ms: None,
            attempts: 0,
            max_attempts: self.config.max_retries,
            metadata: config.metadata.unwrap_or_default(),
        };

        // 添加到任务列表
        {
            let mut tasks = self.tasks.write().unwrap();
            tasks.push(task_id.clone());
        }

        Ok(task)
    }

    /// 执行任务
    pub async fn execute_task(&self, task: &mut AgentTask) -> Result<serde_json::Value, String> {
        if self.status != AgentStatus::Active {
            return Err("Agent is not active".to_string());
        }

        task.status = TaskStatus::Running;
        task.started_at = Some(Utc::now().timestamp());

        // 通过连接器执行任务
        let result = self.connector.execute_task(&task).await;

        task.completed_at = Some(Utc::now().timestamp());
        
        if let Some(started_at) = task.started_at {
            task.duration_ms = Some(task.completed_at.unwrap() - started_at);
        }

        match result {
            Ok(value) => {
                task.status = TaskStatus::Completed;
                task.result = Some(value.clone());
                self.last_activity = Some(Utc::now().timestamp());
                Ok(value)
            }
            Err(e) => {
                task.status = TaskStatus::Failed;
                task.error = Some(e.clone());
                
                // 重试逻辑
                if self.config.retry_on_failure && task.attempts < task.max_attempts {
                    task.attempts += 1;
                    task.status = TaskStatus::Pending;
                    return self.execute_task(task).await;
                }
                
                Err(e)
            }
        }
    }

    /// 获取任务列表
    pub fn get_task_ids(&self) -> Vec<String> {
        self.tasks.read().unwrap().clone()
    }

    /// 启用知识收集
    pub fn enable_collection(&mut self) {
        self.collection_enabled = true;
    }

    /// 禁用知识收集
    pub fn disable_collection(&mut self) {
        self.collection_enabled = false;
    }

    /// 更新活动时间
    pub fn update_activity(&mut self) {
        self.last_activity = Some(Utc::now().timestamp());
    }

    /// 获取错误信息
    pub fn get_error(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    /// 健康检查
    pub async fn health_check(&self) -> Result<bool, String> {
        if self.status != AgentStatus::Active {
            return Ok(false);
        }

        self.connector.ping().await
    }
}
