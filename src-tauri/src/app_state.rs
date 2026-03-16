// Application state management
// This module will contain state shared across Tauri commands

use std::sync::{Arc, Mutex};
use crate::database::Database;
use crate::search::hybrid_engine::HybridEngine;
use crate::search::query_optimizer::QueryOptimizer;
use crate::search::keyword_index::KeywordIndex;
use crate::search::result_ranker::ResultRanker;
use crate::vector::chromadb_client::ChromaDBClient;
use crate::vector::ollama_embeddings::OllamaEmbeddings;
use crate::graph::{GraphBuilder, GraphAnalyzer};
use crate::rag::{QueryParser, ContextBuilder, LLMIntegration, AnswerGenerator};
use crate::placeholders::ExtensionAPI;

pub struct AppState {
    pub db: Arc<Mutex<Option<Database>>>,
    pub config_path: String,

    // Search components (Phase 1)
    pub hybrid_search: Arc<Mutex<Option<HybridEngine>>>,
    pub query_optimizer: Arc<Mutex<QueryOptimizer>>,
    pub result_ranker: Arc<Mutex<ResultRanker>>,
    pub keyword_index: Arc<Mutex<Option<KeywordIndex>>>,
    pub chroma_client: Arc<Mutex<Option<ChromaDBClient>>>,
    pub embeddings: Arc<Mutex<Option<OllamaEmbeddings>>>,

    // VSCode integration (Phase 2 - not yet implemented)
    pub vscode_api: Arc<Mutex<Option<ExtensionAPI>>>,

    // Graph visualization (RAG components (Phase 3 & 4)
    pub graph_builder: Arc<Mutex<Option<GraphBuilder>>>,
    pub graph_analyzer: Arc<Mutex<Option<GraphAnalyzer>>>,

    // RAG components (Phase 4)
    pub query_parser: Arc<Mutex<QueryParser>>,
    pub context_builder: Arc<Mutex<ContextBuilder>>,
    pub llm_integration: Arc<Mutex<LLMIntegration>>,
    pub answer_generator: Arc<Mutex<AnswerGenerator>>,
}

impl AppState {
    pub fn new(config_path: String) -> Self {
        AppState {
            db: Arc::new(Mutex::new(None)),
            config_path,
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
        }
    }
}
