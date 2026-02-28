// Application state management
// This module will contain state shared across Tauri commands

use std::sync::{Arc, Mutex};
use crate::database::Database;

pub struct AppState {
    pub db: Arc<Mutex<Option<Database>>>,
    pub config_path: String,
}

impl AppState {
    pub fn new(config_path: String) -> Self {
        AppState {
            db: Arc::new(Mutex::new(None)),
            config_path,
        }
    }
}
