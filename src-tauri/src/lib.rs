// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod agents;
mod database;
mod extractor;
mod storage;
mod vector;
mod search;
mod graph;
mod rag;
// mod vscode;

mod models;
mod config;
mod app_state;
mod placeholders;

use database::Database;
use agents::{AgentConfig, SessionData};
use app_state::AppState;
use tauri::{State, Manager};
use std::sync::{Arc, Mutex};
use serde_json::Value;

// Phase 2, 3, 4 modules
use placeholders::ExtensionAPI;
use graph::{GraphBuilder, GraphAnalyzer, GraphData, GraphStatistics};
use rag::{QueryParser, ContextBuilder, LLMIntegration, AnswerGenerator, ParsedQuery, LLMConfig, GeneratedAnswer};
use search::QueryOptimizer;
use search::ResultRanker;

// Tauri Commands

#[tauri::command]
async fn greet(name: &str) -> Result<String, String> {
    Ok(format!("Hello, {}! You've been greeted from Tauri!", name))
}

#[tauri::command]
async fn get_app_info() -> Result<Value, String> {
    Ok(serde_json::json!({
        "name": "MindCortex",
        "version": "0.2.1",
        "description": "智能知识管理和检索系统",
    }))
}

// ========== 数据库相关 Tauri Commands ==========

#[tauri::command]
async fn init_database(state: State<'_, AppState>) -> Result<(), String> {
    let mut db_guard = state.db.lock().unwrap();

    if db_guard.is_some() {
        return Err("Database already initialized".to_string());
    }

    let db = Database::new("mindcortex.db")
        .map_err(|e| format!("Failed to create database: {}", e))?;

    *db_guard = Some(db);
    Ok(())
}

#[tauri::command]
async fn get_projects(state: State<'_, AppState>) -> Result<Vec<Value>, String> {
    let db_guard = state.db.lock().unwrap();

    let db = db_guard.as_ref()
        .ok_or("Database not initialized")?;

    let projects = db.get_all_projects()
        .map_err(|e| format!("Failed to get projects: {}", e))?;

    Ok(projects.into_iter().map(|p| serde_json::to_value(p).unwrap()).collect())
}

#[tauri::command]
async fn create_project(
    name: String,
    description: Option<String>,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let db_guard = state.db.lock().unwrap();

    let db = db_guard.as_ref()
        .ok_or("Database not initialized")?;

    let project = db.create_project(&name, description.as_deref())
        .map_err(|e| format!("Failed to create project: {}", e))?;

    Ok(serde_json::to_value(project).unwrap())
}

#[tauri::command]
async fn get_tasks(project_id: i64, state: State<'_, AppState>) -> Result<Vec<Value>, String> {
    let db_guard = state.db.lock().unwrap();

    let db = db_guard.as_ref()
        .ok_or("Database not initialized")?;

    let tasks = db.get_tasks_by_project(&project_id.to_string())
        .map_err(|e| format!("Failed to get tasks: {}", e))?;

    Ok(tasks.into_iter().map(|t| serde_json::to_value(t).unwrap()).collect())
}

// ========== Agent 连接相关 Tauri Commands ==========

#[tauri::command]
async fn add_agent_connection(
    agent_type: String,
    config: Value,
    _state: State<'_, AppState>,
) -> Result<String, String> {
    let agent_config = AgentConfig {
        agent_type,
        connection_type: config.get("connection_type")
            .and_then(|v| v.as_str())
            .unwrap_or("http")
            .to_string(),
        config,
    };

    // TODO: 保存到数据库
    println!("Agent connection added: {:?}", agent_config);
    Ok("Agent connection added".to_string())
}

#[tauri::command]
async fn sync_agent_sessions(
    agent_type: String,
    state: State<'_, AppState>,
) -> Result<Vec<SessionData>, String> {
    // TODO: 同步 agent 会话数据
    Ok(vec![])
}

// ========== 搜索相关 Tauri Commands ==========

#[tauri::command]
async fn hybrid_search(
    query: String,
    search_type: String,
    state: State<'_, AppState>,
) -> Result<Vec<Value>, String> {
    // 使用混合搜索
    // TODO: 实现实际搜索逻辑
    Ok(vec![
        serde_json::json!({
            "id": "result-1",
            "content": format!("Search results for: {}", query),
            "score": 0.9,
            "type": search_type
        })
    ])
}

#[tauri::command]
async fn vscode_get_code_context(
    file_path: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    // Phase 2 功能未实现
    Ok(serde_json::json!({"error": "VSCode integration not yet implemented (Phase 2)"}))
}

// ========== 知识图谱相关 Tauri Commands (Phase 3) ==========

#[tauri::command]
async fn graph_get_data(
    entity_id: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let graph_builder_guard = state.graph_builder.lock().unwrap();

    if let Some(graph_builder) = graph_builder_guard.as_ref() {
        let graph = graph_builder.get_graph();
        Ok(serde_json::to_value(graph).unwrap())
    } else {
        Ok(serde_json::json!({"error": "Graph builder not initialized"}))
    }
}

#[tauri::command]
async fn graph_build_from_code(
    code: String,
    language: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let mut graph_builder_guard = state.graph_builder.lock().unwrap();

    if graph_builder_guard.is_none() {
        *graph_builder_guard = Some(graph::GraphBuilder::new());
    }

    if let Some(graph_builder) = graph_builder_guard.as_mut() {
        graph_builder.analyze_code(&code, &language);

        let graph = graph_builder.get_graph();
        Ok(serde_json::to_value(graph).unwrap())
    } else {
        Ok(serde_json::json!({"error": "Failed to initialize graph builder"}))
    }
}

#[tauri::command]
async fn graph_build_from_document(
    content: String,
    title: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let mut graph_builder_guard = state.graph_builder.lock().unwrap();

    if graph_builder_guard.is_none() {
        *graph_builder_guard = Some(graph::GraphBuilder::new());
    }

    if let Some(graph_builder) = graph_builder_guard.as_mut() {
        graph_builder.analyze_document(&content, &title);

        let graph = graph_builder.get_graph();
        Ok(serde_json::to_value(graph).unwrap())
    } else {
        Ok(serde_json::json!({"error": "Failed to initialize graph builder"}))
    }
}

#[tauri::command]
async fn graph_analyze_connections(
    entity_id: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let graph_builder_guard = state.graph_builder.lock().unwrap();

    if let Some(graph_builder) = graph_builder_guard.as_ref() {
        let graph_data = graph_builder.get_graph();
        let analyzer = graph::GraphAnalyzer::new(graph_data.clone());

        let related_nodes = analyzer.find_related_nodes(&entity_id, 10);
        let key_nodes = analyzer.find_key_nodes(10);
        let statistics = analyzer.get_statistics();

        Ok(serde_json::json!({
            "entity_id": entity_id,
            "related_nodes": related_nodes,
            "key_nodes": key_nodes,
            "statistics": statistics
        }))
    } else {
        Ok(serde_json::json!({"error": "Graph builder not initialized"}))
    }
}

#[tauri::command]
async fn graph_find_path(
    start: String,
    end: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let graph_builder_guard = state.graph_builder.lock().unwrap();

    if let Some(graph_builder) = graph_builder_guard.as_ref() {
        let graph_data = graph_builder.get_graph();
        let analyzer = graph::GraphAnalyzer::new(graph_data.clone());

        let path = graph_data.find_shortest_path(&start, &end);

        Ok(serde_json::json!({
            "start": start,
            "end": end,
            "path": path,
            "found": path.is_some()
        }))
    } else {
        Ok(serde_json::json!({"error": "Graph builder not initialized"}))
    }
}

#[tauri::command]
async fn graph_get_statistics(
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let graph_builder_guard = state.graph_builder.lock().unwrap();

    if let Some(graph_builder) = graph_builder_guard.as_ref() {
        let graph_data = graph_builder.get_graph();
        let analyzer = graph::GraphAnalyzer::new(graph_data.clone());

        let statistics = analyzer.get_statistics();

        Ok(serde_json::to_value(statistics).unwrap())
    } else {
        Ok(serde_json::json!({"error": "Graph builder not initialized"}))
    }
}

// ========== RAG 相关 Tauri Commands (Phase 4) ==========

#[tauri::command]
async fn rag_query(
    query: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Phase 4 功能未实现
    Ok("RAG query not yet implemented (Phase 4)".to_string())
}

#[tauri::command]
async fn rag_chat(
    query: String,
    conversation_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    // Phase 4 功能未实现
    Ok(serde_json::json!({"answer": "RAG chat not yet implemented (Phase 4)"}))
}

// ========== 数据库和设置 Tauri Commands ==========

#[tauri::command]
async fn get_settings(state: State<'_, AppState>) -> Result<Value, String> {
    Ok(serde_json::json!({
        "theme": "dark",
        "language": "zh-CN",
        "auto_save": true,
        "notification_enabled": true,
    }))
}

#[tauri::command]
async fn update_settings(
    settings: Value,
    state: State<'_, AppState>,
) -> Result<(), String> {
    Ok(())
}

// ========== 初始化和命令系统 ==========

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AppState {
                db: Arc::new(Mutex::new(None)),
                config_path: "config.json".to_string(),
                hybrid_search: Arc::new(Mutex::new(None)),
                query_optimizer: Arc::new(Mutex::new(QueryOptimizer::new())),
                result_ranker: Arc::new(Mutex::new(ResultRanker::new())),
                keyword_index: Arc::new(Mutex::new(None)),
                chroma_client: Arc::new(Mutex::new(None)),
                embeddings: Arc::new(Mutex::new(None)),
                vscode_api: Arc::new(Mutex::new(None)),
                graph_builder: Arc::new(Mutex::new(None)),
                graph_analyzer: Arc::new(Mutex::new(None)),
                query_parser: Arc::new(Mutex::new(QueryParser::new())),
                context_builder: Arc::new(Mutex::new(ContextBuilder::new())),
                llm_integration: Arc::new(Mutex::new(LLMIntegration::new())),
                answer_generator: Arc::new(Mutex::new(AnswerGenerator::new())),
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

            hybrid_search,
            vscode_get_code_context,

            graph_get_data,
            graph_build_from_code,
            graph_build_from_document,
            graph_analyze_connections,
            graph_find_path,
            graph_get_statistics,

            rag_query,
            rag_chat,

            get_settings,
            update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
