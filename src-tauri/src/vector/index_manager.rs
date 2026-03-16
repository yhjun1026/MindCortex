// Index Manager
// 管理知识索引，支持自动索引和增量更新

use super::VectorDocument;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{interval, Duration};

/// 索引状态
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexStatus {
    pub total_documents: usize,
    pub indexed_documents: usize,
    pub pending_documents: usize,
    pub failed_documents: usize,
    pub last_index_time: Option<i64>,
    pub is_indexing: bool,
}

/// 索引配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexConfig {
    pub auto_index_enabled: bool,
    pub auto_index_interval_seconds: u64,
    pub batch_size: usize,
    pub max_retry_attempts: u32,
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            auto_index_enabled: true,
            auto_index_interval_seconds: 3600,
            batch_size: 100,
            max_retry_attempts: 3,
        }
    }
}

/// 索引任务
struct IndexTask {
    id: String,
    document: VectorDocument,
    retry_count: u32,
}

/// 索引管理器
pub struct IndexManager {
    config: IndexConfig,
    status: Arc<Mutex<IndexStatus>>,
    pending_tasks: Arc<Mutex<Vec<IndexTask>>>,
    failed_tasks: Arc<Mutex<Vec<IndexTask>>>,
    auto_index_handle: Option<tokio::task::JoinHandle<()>>,
}

impl IndexManager {
    pub fn new(config: IndexConfig) -> Self {
        Self {
            config,
            status: Arc::new(Mutex::new(IndexStatus {
                total_documents: 0,
                indexed_documents: 0,
                pending_documents: 0,
                failed_documents: 0,
                last_index_time: None,
                is_indexing: false,
            })),
            pending_tasks: Arc::new(Mutex::new(vec![])),
            failed_tasks: Arc::new(Mutex::new(vec![])),
            auto_index_handle: None,
        }
    }

    pub fn default() -> Self {
        Self::new(IndexConfig::default())
    }

    pub fn add_document(&self, document: VectorDocument) {
        let task = IndexTask {
            id: document.id.clone(),
            document,
            retry_count: 0,
        };

        let mut pending = self.pending_tasks.lock().unwrap();
        pending.push(task);

        let mut status = self.status.lock().unwrap();
        status.total_documents += 1;
        status.pending_documents += 1;
    }

    pub fn get_status(&self) -> IndexStatus {
        self.status.lock().unwrap().clone()
    }
}

impl Default for IndexManager {
    fn default() -> Self {
        Self::new(IndexConfig::default())
    }
}
