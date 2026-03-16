pub mod s3_sync;
pub mod webdav_sync;
pub mod sync_scheduler;
pub mod conflict_resolver;

pub use s3_sync::S3Sync;
pub use webdav_sync::WebDAVSync;
pub use sync_scheduler::SyncScheduler;
pub use conflict_resolver::ConflictResolver;

use serde::{Deserialize, Serialize};

/// 同步状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub sync_id: String,
    pub sync_type: String,
    pub status: SyncState,
    pub total_files: usize,
    pub uploaded_files: usize,
    pub failed_files: usize,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub error_message: Option<String>,
}

/// 同步状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncState {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// 同步配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub sync_type: String,
    pub auto_sync_enabled: bool,
    pub sync_interval_minutes: u64,
    pub conflict_resolution: ConflictResolutionStrategy,
}

/// 冲突解决策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    /// 保留本地版本
    KeepLocal,
    /// 保留远程版本
    KeepRemote,
    /// 保留较新的版本
    KeepNewer,
    /// 手动解决
    Manual,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub modified_at: i64,
    pub checksum: Option<String>,
}
