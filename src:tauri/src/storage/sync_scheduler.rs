// Sync Scheduler
// 管理定时同步任务和手动触发同步

use super::{SyncStatus, SyncState, S3Sync, WebDAVSync, SyncConfig};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{interval, Duration};
use tokio::task::JoinHandle;

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
    s3_sync: Option<Arc<S3Sync>>,
    webdav_sync: Option<Arc<WebDAVSync>>,
    scheduler_handle: Option<JoinHandle<()>>,
    is_running: Arc<Mutex<bool>>,
}

impl SyncScheduler {
    /// 创建新的同步调度器
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            sync_status: Arc::new(Mutex::new(vec![])),
            s3_sync: None,
            webdav_sync: None,
            scheduler_handle: None,
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    /// 注册 S3 同步器
    pub fn register_s3_sync(&mut self, s3_sync: Arc<S3Sync>) {
        self.s3_sync = Some(s3_sync);
    }

    /// 注册 WebDAV 同步器
    pub fn register_webdav_sync(&mut self, webdav_sync: Arc<WebDAVSync>) {
        self.webdav_sync = Some(webdav_sync);
    }

    /// 添加定时同步任务
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

    /// 添加手动触发任务
    pub fn add_manual_task(&self, task_type: SyncTaskType) -> Result<String, String> {
        let task_id = uuid::Uuid::new_v4().to_string();
        
        let task = SyncTask {
            id: task_id.clone(),
            task_type,
            interval_minutes: None,
            last_run: None,
            next_run: None,
            enabled: true,
        };

        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(task_id.clone(), task);

        Ok(task_id)
    }

    /// 启动调度器
    pub fn start(&mut self) {
        {
            let mut is_running = self.is_running.lock().unwrap();
            if *is_running {
                return; // 已经在运行
            }
            *is_running = true;
        }

        let tasks = self.tasks.clone();
        let sync_status = self.sync_status.clone();
        let s3_sync = self.s3_sync.clone();
        let webdav_sync = self.webdav_sync.clone();
        let is_running = self.is_running.clone();

        let handle = tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(60)); // 每分钟检查一次

            loop {
                ticker.tick().await;

                // 检查是否仍在运行
                {
                    let running = is_running.lock().unwrap();
                    if !*running {
                        break;
                    }
                }

                // 获取当前时间
                let now = chrono::Utc::now().timestamp();

                // 检查并执行到期的任务
                let task_ids: Vec<String> = {
                    let tasks = tasks.lock().unwrap();
                    tasks.iter()
                        .filter(|(_, task)| {
                            task.enabled && 
                            task.next_run.is_some() && 
                            task.next_run.unwrap() <= now
                        })
                        .map(|(id, _)| id.clone())
                        .collect()
                };

                for task_id in task_ids {
                    Self::execute_task(
                        &task_id,
                        &tasks,
                        &sync_status,
                        &s3_sync,
                        &webdav_sync
                    ).await;
                }
            }
        });

        self.scheduler_handle = Some(handle);
    }

    /// 停止调度器
    pub fn stop(&mut self) {
        {
            let mut is_running = self.is_running.lock().unwrap();
            *is_running = false;
        }

        if let Some(handle) = self.scheduler_handle.take() {
            handle.abort();
        }
    }

    /// 手动触发任务
    pub async fn trigger_task(&self, task_id: &str) -> Result<(), String> {
        Self::execute_task(
            task_id,
            &self.tasks,
            &self.sync_status,
            &self.s3_sync,
            &self.webdav_sync
        ).await;

        Ok(())
    }

    /// 执行单个任务
    async fn execute_task(
        task_id: &str,
        tasks: &Arc<Mutex<HashMap<String, SyncTask>>>,
        sync_status: &Arc<Mutex<Vec<SyncStatus>>>,
        s3_sync: &Option<Arc<S3Sync>>,
        webdav_sync: &Option<Arc<WebDAVSync>>,
    ) {
        // 获取任务信息
        let task_info = {
            let tasks = tasks.lock().unwrap();
            tasks.get(task_id).cloned()
        };

        if let Some(task) = task_info {
            match task.task_type {
                SyncTaskType::S3 => {
                    if let Some(s3) = s3_sync {
                        Self::run_s3_sync(s3, sync_status).await;
                    }
                }
                SyncTaskType::WebDAV => {
                    if let Some(webdav) = webdav_sync {
                        Self::run_webdav_sync(webdav, sync_status).await;
                    }
                }
            }

            // 更新任务状态
            let now = chrono::Utc::now().timestamp();
            let mut tasks = tasks.lock().unwrap();
            if let Some(t) = tasks.get_mut(task_id) {
                t.last_run = Some(now);
                if let Some(interval) = t.interval_minutes {
                    t.next_run = Some(now + (interval * 60) as i64);
                }
            }
        }
    }

    /// 运行 S3 同步
    async fn run_s3_sync(s3_sync: &Arc<S3Sync>, sync_status: &Arc<Mutex<Vec<SyncStatus>>>) {
        // TODO: 实际的 S3 同步逻辑
        // 扫描本地目录并上传到 S3
        println!("Running S3 sync...");
        
        let sync_id = uuid::Uuid::new_v4().to_string();
        let status = SyncStatus {
            sync_id: sync_id.clone(),
            sync_type: "s3".to_string(),
            status: SyncState::Running,
            total_files: 0,
            uploaded_files: 0,
            failed_files: 0,
            started_at: chrono::Utc::now().timestamp(),
            completed_at: None,
            error_message: None,
        };

        let mut status_list = sync_status.lock().unwrap();
        status_list.push(status);
        drop(status_list);

        // 模拟同步完成
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        let mut status_list = sync_status.lock().unwrap();
        if let Some(s) = status_list.iter_mut().find(|s| s.sync_id == sync_id) {
            s.status = SyncState::Completed;
            s.completed_at = Some(chrono::Utc::now().timestamp());
        }
    }

    /// 运行 WebDAV 同步
    async fn run_webdav_sync(webdav_sync: &Arc<WebDAVSync>, 
                             sync_status: &Arc<Mutex<Vec<SyncStatus>>>) {
        // TODO: 实际的 WebDAV 同步逻辑
        println!("Running WebDAV sync...");
        
        let sync_id = uuid::Uuid::new_v4().to_string();
        let status = SyncStatus {
            sync_id: sync_id.clone(),
            sync_type: "webdav".to_string(),
            status: SyncState::Running,
            total_files: 0,
            uploaded_files: 0,
            failed_files: 0,
            started_at: chrono::Utc::now().timestamp(),
            completed_at: None,
            error_message: None,
        };

        let mut status_list = sync_status.lock().unwrap();
        status_list.push(status);
        drop(status_list);

        // 模拟同步完成
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        let mut status_list = sync_status.lock().unwrap();
        if let Some(s) = status_list.iter_mut().find(|s| s.sync_id == sync_id) {
            s.status = SyncState::Completed;
            s.completed_at = Some(chrono::Utc::now().timestamp());
        }
    }

    /// 获取所有任务
    pub fn get_tasks(&self) -> HashMap<String, SyncTask> {
        self.tasks.lock().unwrap().clone()
    }

    /// 获取任务状态
    pub fn get_task(&self, task_id: &str) -> Option<SyncTask> {
        self.tasks.lock().unwrap().get(task_id).cloned()
    }

    /// 启用/禁用任务
    pub fn set_task_enabled(&self, task_id: &str, enabled: bool) -> Result<(), String> {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(task) = tasks.get_mut(task_id) {
            task.enabled = enabled;
            Ok(())
        } else {
            Err(format!("Task not found: {}", task_id))
        }
    }

    /// 删除任务
    pub fn remove_task(&self, task_id: &str) -> Result<(), String> {
        let mut tasks = self.tasks.lock().unwrap();
        if tasks.remove(task_id).is_some() {
            Ok(())
        } else {
            Err(format!("Task not found: {}", task_id))
        }
    }

    /// 获取同步历史
    pub fn get_sync_history.sync_status() -> Vec<SyncStatus> {
        self.sync_status.lock().unwrap().clone()
    }

    /// 清空同步历史
    pub fn clear_sync_history(&self) {
        self.sync_status.lock().unwrap().clear();
    }

    /// 是否正在运行
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
        
        // 添加定时任务
        let task_id = scheduler.add_scheduled_task(SyncTaskType::S3, 60).unwrap();
        
        // 获取任务
        let task = scheduler.get_task(&task_id);
        assert!(task.is_some());
        assert_eq!(task.unwrap().task_type, SyncTaskType::S3);
        
        // 启用/禁用任务
        assert!(scheduler.set_task_enabled(&task_id, false).is_ok());
        
        // 删除任务
        assert!(scheduler.remove_task(&task_id).is_ok());
    }
}
