use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub base_path: String,
    pub projects_path: String,
    pub agents_path: String,
    pub timeline_path: String,
}

// TODO: Implement file storage logic
// pub mod file_manager;
// pub mod project_tree;
