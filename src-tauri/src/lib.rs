// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod agents;
mod database;
mod extractor;
mod storage;
mod vector;
mod models;
mod config;
mod app_state;

use agents::{AgentConfig, SessionData};
use database::{Database, Project, Task};
use database::KnowledgeItem as DbKnowledgeItem;
use extractor::KnowledgeItem;
use app_state::AppState;
use tauri::{State, Manager};
use std::sync::{Arc, Mutex};

// Tauri Commands

#[tauri::command]
async fn greet(name: &str) -> Result<String, String> {
    Ok(format!("Hello, {}! Welcome to CortexMind!", name))
}

#[tauri::command]
async fn get_app_info() -> serde_json::Value {
    serde_json::json!({
        "name": "CortexMind",
        "version": env!("CARGO_PKG_VERSION"),
        "description": env!("CARGO_PKG_DESCRIPTION")
    })
}

#[tauri::command]
async fn init_database(state: State<'_, AppState>) -> Result<(), String> {
    let mut db_guard = state.db.lock().unwrap();

    let db = Database::new("cortexmind.db")
        .map_err(|e| format!("Failed to initialize database: {}", e))?;

    db.init_tables()
        .map_err(|e| format!("Failed to create tables: {}", e))?;

    *db_guard = Some(db);
    Ok(())
}

#[tauri::command]
async fn get_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().ok_or("Database not initialized")?;

    db.get_all_projects()
        .map_err(|e| format!("Failed to get projects: {}", e))
}

#[tauri::command]
async fn create_project(
    name: String,
    description: Option<String>,
    state: State<'_, AppState>,
) -> Result<Project, String> {
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().ok_or("Database not initialized")?;

    db.create_project(&name, description.as_deref())
        .map_err(|e| format!("Failed to create project: {}", e))
}

#[tauri::command]
async fn get_tasks(
    project_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<Task>, String> {
    let db_guard = state.db.lock().unwrap();
    let db = db_guard.as_ref().ok_or("Database not initialized")?;

    if let Some(pid) = project_id {
        db.get_tasks_by_project(&pid)
            .map_err(|e| format!("Failed to get tasks: {}", e))
    } else {
        db.get_all_tasks()
            .map_err(|e| format!("Failed to get tasks: {}", e))
    }
}

#[tauri::command]
async fn add_agent_connection(
    config: AgentConfig,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // TODO: Implement agent connection logic
    Ok(format!("Agent {} connected successfully", config.agent_type))
}

#[tauri::command]
async fn sync_agent_sessions(
    agent_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<SessionData>, String> {
    // TODO: Implement session sync logic
    Ok(vec![])
}

#[tauri::command]
async fn search_knowledge(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, String> {
    // TODO: Implement vector search
    Ok(vec![])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AppState {
                db: Arc::new(Mutex::new(None)),
                config_path: "config.json".to_string(),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_app_info,
            init_database,
            get_projects,
            create_project,
            get_tasks,
            add_agent_connection,
            sync_agent_sessions,
            search_knowledge
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
