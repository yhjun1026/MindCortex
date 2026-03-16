pub mod hybrid_engine;
pub mod keyword_index;
pub mod query_optimizer;
pub mod result_ranker;

pub use hybrid_engine::HybridEngine;
pub use keyword_index::KeywordIndex;
pub use query_optimizer::QueryOptimizer;
pub use result_ranker::ResultRanker;

use serde::{Deserialize, Serialize};

/// 混合搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSearchResult {
    pub id: String,
    pub content: String,
    pub score: f64,
    pub source: ResultSource,
    pub result_type: ResultType,
    pub metadata: SearchResultMetadata,
}

/// 结果来源
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ResultSource {
    Keyword,
    Semantic,
    Both,
}

/// 结果类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResultType {
    /// 关键词匹配
    KeywordMatch,
    /// 语义匹配
    SemanticMatch,
    /// 混合匹配
    Hybrid,
}

/// 搜索结果元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultMetadata {
    pub source: String,
    pub timestamp: i64,
    pub session_id: Option<String>,
    pub message_id: Option<String>,
    pub file_path: Option<String>,
    pub content_length: usize,
}

/// 混合搜索配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSearchConfig {
    pub keyword_weight: f64,
    pub semantic_weight: f64,
    pub max_results: usize,
    pub enable_fuzzy_search: bool,
    pub enable_time_filter: bool,
    pub fuzzy_threshold: f64,
}

impl Default for HybridSearchConfig {
    fn default() -> Self {
        Self {
            keyword_weight: 0.4,
            semantic_weight: 0.6,
            max_results: 10,
            enable_fuzzy_search: true,
            enable_time_filter: true,
            fuzzy_threshold: 0.7,
        }
    }
}
