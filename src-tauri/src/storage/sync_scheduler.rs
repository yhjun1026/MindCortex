// Sync Scheduler
// 管理定时同步任务和手动触发同步

use super::{SyncStatus, SyncState};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{interval, Duration};

/// 同步任务类型
#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SyncTaskType {
    S3,
    WebDAV,
}

/// 同步任务
#[derive(Debug, Clone)]
struct SyncTask {
    id: String,
    task_type: SyncTaskType,
    interval_minutes: Option<u64>,
    last_run: Option<i64>,
    next_run: Option<i64>,
    enabled: bool,
}

/// 同步调度器
pub struct SyncScheduler {
    tasks: Arc<Mutex<HashMap<String, SyncTask>>>,
    sync_status: Arc<Mutex<Vec<SyncStatus>>>,
    scheduler_handle: Option<tokio::task::JoinHandle<()>>,
    is_running: Arc<Mutex<bool>>,
}

impl SyncScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            sync_status: Arc::new(Mutex::new(vec![])),
            scheduler_handle: None,
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn add_scheduled_task(&self, task_type: SyncTaskType, interval_minutes: u64) 
        -> Result<String, String> {
        
        let task_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        
        let task = SyncTask {
            id: task_id.clone(),
            task_type,
            interval_minutes: Some(interval_minutes),
            last_run: None,
            next_run: Some(now + (interval_minutes * 60) as i64),
            enabled: true,
        };

        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(task_id.clone(), task);

        Ok(task_id)
    }

    pub fn get_tasks(&self) -> HashMap<String, SyncTask> {
        self.tasks.lock().unwrap().clone()
    }

    pub fn get_task(&self, task_id: &str) -> Option<SyncTask> {
        self.tasks.lock().unwrap().get(task_id).cloned()
    }

    pub fn get_sync_history(&self) -> Vec<SyncStatus> {
        self.sync_status.lock().unwrap().clone()
    }

    pub fn clear_sync_history(&self) {
        self.sync_status.lock().unwrap().clear();
    }

    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }
}

impl Default for SyncScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_sync_scheduler() {
        let scheduler = SyncScheduler::new();
        
        let task_id = scheduler.add_scheduled_task(SyncTaskType::S3, 60).unwrap();
        
        let task = scheduler.get_task(&task_id);
        assert!(task.is_some());
        assert_eq!(task.unwrap().task_type, SyncTaskType::S3);
    }
}
