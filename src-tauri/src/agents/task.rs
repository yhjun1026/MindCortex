// Agent 任务管理
// 定义任务的生命周期和状态

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Running => "running",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
        }
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 任务优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low = 0,
    Medium = 1,
    High = 2,
    Urgent = 3,
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskPriority::Low => "low",
            TaskPriority::Medium => "medium",
            TaskPriority::High => "high",
            TaskPriority::Urgent => "urgent",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "low" => Some(TaskPriority::Low),
            "medium" => Some(TaskPriority::Medium),
            "high" => Some(TaskPriority::High),
            "urgent" => Some(TaskPriority::Urgent),
            _ => None,
        }
    }
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Agent 任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    pub id: String,
    pub agent_id: String,
    pub task_type: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub payload: serde_json::Value,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub duration_ms: Option<i64>,
    pub attempts: u32,
    pub max_attempts: u32,
    pub metadata: HashMap<String, String>,
}

impl AgentTask {
    pub fn is_completed(&self) -> bool {
        self.status == TaskStatus::Completed
    }

    pub fn is_failed(&self) -> bool {
        self.status == TaskStatus::Failed
    }

    pub fn is_running(&self) -> bool {
        self.status == TaskStatus::Running
    }

    pub fn can_retry(&self) -> bool {
        self.status == TaskStatus::Failed && self.attempts < self.max_attempts
    }

    pub fn get_duration(&self) -> Option<i64> {
        self.duration_ms
    }

    pub fn get_progress(&self) -> f32 {
        match self.status {
            TaskStatus::Pending => 0.0,
            TaskStatus::Running => {
                if let (Some(started), Some(completed)) = (self.started_at, self.completed_at) {
                    if self.duration_ms.is_some() {
                        1.0
                    } else {
                        let now = Utc::now().timestamp();
                        let elapsed = now - started;
                        // 假设任务预计运行 30 秒（可根据实际调整）
                        let estimated = 30.0;
                        let progress = (elapsed as f32) / estimated;
                        progress.min(0.95).max(0.05)
                    }
                } else {
                    0.1
                }
            }
            TaskStatus::Completed => 1.0,
            TaskStatus::Failed | TaskStatus::Cancelled => 0.0,
        }
    }
}

/// 任务配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    pub task_type: String,
    pub payload: serde_json::Value,
    pub priority: TaskPriority,
    pub timeout_seconds: Option<u64>,
    pub metadata: Option<HashMap<String, String>>,
}

impl TaskConfig {
    pub fn new(task_type: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            task_type: task_type.into(),
            payload,
            priority: TaskPriority::Medium,
            timeout_seconds: None,
            metadata: None,
        }
    }

    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// 任务队列
pub struct TaskQueue {
    queue: Vec<AgentTask>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    pub fn push(&mut self, task: AgentTask) {
        self.queue.push(task);
        self.sort_by_priority();
    }

    pub fn pop(&mut self) -> Option<AgentTask> {
        self.queue.pop()
    }

    pub fn peek(&self) -> Option<&AgentTask> {
        self.queue.last()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn get_by_id(&self, id: &str) -> Option<&AgentTask> {
        self.queue.iter().find(|t| t.id == id)
    }

    pub fn update_task(&mut self, task: AgentTask) {
        if let Some(index) = self.queue.iter().position(|t| t.id == task.id) {
            self.queue[index] = task;
            self.sort_by_priority();
        }
    }

    pub fn remove_by_id(&mut self, id: &str) -> Option<AgentTask> {
        if let Some(index) = self.queue.iter().position(|t| t.id == id) {
            Some(self.queue.remove(index))
        } else {
            None
        }
    }

    pub fn get_pending_tasks(&self) -> Vec<&AgentTask> {
        self.queue.iter()
            .filter(|t| t.status == TaskStatus::Pending)
            .collect()
    }

    pub fn get_running_tasks(&self) -> Vec<&AgentTask> {
        self.queue.iter()
            .filter(|t| t.status == TaskStatus::Running)
            .collect()
    }

    pub fn get_all_tasks(&self) -> Vec<&AgentTask> {
        self.queue.iter().collect()
    }

    fn sort_by_priority(&mut self) {
        self.queue.sort_by(|a, b| {
            // 优先级高的在前
            if b.priority.cmp(&a.priority) != std::cmp::Ordering::Equal {
                return b.priority.cmp(&a.priority);
            }
            // 同优先级下，创建时间早的在前
            a.created_at.cmp(&b.created_at)
        });
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}
