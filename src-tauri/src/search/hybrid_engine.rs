// 混合检索引擎
// 实现关键词搜索和语义搜索的混合检索引擎

use super::{HybridSearchResult, ResultSource, ResultType, SearchResultMetadata, HybridSearchConfig, KeywordIndex};
use crate::vector::chromadb_client::ChromaDBClient;
use crate::vector::ollama_embeddings::OllamaEmbeddings;
use std::sync::Arc;
use serde_json::Value;

/// 混合检索引擎
pub struct HybridEngine {
    keyword_index: Arc<KeywordIndex>,
    chroma_client: Arc<ChromaDBClient>,
    embeddings: Arc<OllamaEmbeddings>,
    config: HybridSearchConfig,
}

impl HybridEngine {
    /// 创建新的混合检索引擎
    pub fn new(
        keyword_index: Arc<KeywordIndex>,
        chroma_client: Arc<ChromaDBClient>,
        embeddings: Arc<OllamaEmbeddings>,
    ) -> Self {
        Self {
            keyword_index,
            chroma_client,
            embeddings,
            config: HybridSearchConfig::default(),
        }
    }

    /// 使用自定义配置创建混合检索引擎
    pub fn with_config(
        keyword_index: Arc<KeywordIndex>,
        chroma_client: Arc<ChromaDBClient>,
        embeddings: Arc<OllamaEmbeddings>,
        config: HybridSearchConfig,
    ) -> Self {
        Self {
            keyword_index,
            chroma_client,
            embeddings,
            config,
        }
    }

    /// 更新配置
    pub fn update_config(&mut self, config: HybridSearchConfig) {
        self.config = config;
    }

    /// 执行混合搜索
    pub async fn search(&self, query: &str) -> Result<Vec<HybridSearchResult>, String> {
        let mut all_results = vec![];

        // 1. 关键词搜索
        if self.config.keyword_weight > 0.0 {
            match self.keyword_search(query).await {
                Ok(results) => all_results.extend(results),
                Err(e) => eprintln!("Keyword search failed: {}", e),
            }
        }

        // 2. 语义搜索
        if self.config.semantic_weight > 0.0 {
            match self.semantic_search(query).await {
                Ok(results) => all_results.extend(results),
                Err(e) => eprintln!("Semantic search failed: {}", e),
            }
        }

        // 3. 结果融合和排序
        let ranked_results = self.rank_results(all_results)?;

        Ok(ranked_results)
    }

    /// 关键词搜索
    async fn keyword_search(&self, query: &str) -> Result<Vec<HybridSearchResult>, String> {
        let index = self.keyword_index.clone();
        let query = query.to_string();
        let max_results = self.config.max_results;

        // 在 blocking 线程池中执行数据库操作
        let results = tokio::task::spawn_blocking(move || {
            index.search(&query, max_results)
        })
        .await
        .map_err(|e| format!("Search task failed: {}", e))??;

        Ok(results.into_iter().map(|result| HybridSearchResult {
            id: result.id,
            content: result.content,
            score: 0.9 * self.config.keyword_weight, // 默认分数
            source: ResultSource::Keyword,
            result_type: ResultType::KeywordMatch,
            metadata: SearchResultMetadata {
                source: result.file_path.clone(),
                timestamp: result.updated_at,
                session_id: None,
                message_id: None,
                file_path: Some(result.file_path.clone()),
                content_length: result.content_length,
            },
        }).collect())
    }

    /// 语义搜索
    async fn semantic_search(&self, query: &str) -> Result<Vec<HybridSearchResult>, String> {
        // TODO: 实现语义搜索
        Ok(vec![])
    }

    /// 结果排序和融合
    fn rank_results(&self, mut results: Vec<HybridSearchResult>)
        -> Result<Vec<HybridSearchResult>, String> {

        // 按分数排序
        results.sort_by(|a, b| {
            b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal)
        });

        // 去重
        let mut seen_ids = std::collections::HashSet::new();
        let mut deduplicated_results = vec![];

        for result in results {
            if seen_ids.insert(result.id.clone()) {
                deduplicated_results.push(result);
            }
        }

        // 限制结果数量
        if deduplicated_results.len() > self.config.max_results {
            deduplicated_results.truncate(self.config.max_results);
        }

        Ok(deduplicated_results)
    }

    /// 更新关键词索引
    pub async fn update_keyword_index(&self, document: &str, id: &str) -> Result<(), String> {
        let doc = super::keyword_index::IndexedDocument {
            id: id.to_string(),
            content: document.to_string(),
            file_path: "unknown".to_string(),
            content_length: document.len(),
            language: "unknown".to_string(),
            tags: vec![],
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        };

        let index = self.keyword_index.clone();

        tokio::task::spawn_blocking(move || {
            index.add_document(&doc)
        })
        .await
        .map_err(|e| format!("Update index task failed: {}", e))?
    }

    /// 更新语义索引
    pub async fn update_semantic_index(&self, document: &str, id: &str, metadata: Value)
        -> Result<(), String> {

        // TODO: 生成文档向量
        // TODO: 添加到 ChromaDB

        Ok(())
    }

    /// 批量更新索引
    pub async fn batch_update_indices(&self, documents: Vec<(String, String, Value)>)
        -> Result<(), String> {

        for (id, document, metadata) in documents {
            let _ = self.update_keyword_index(&document, &id).await;
            let _ = self.update_semantic_index(&document, &id, metadata).await;
        }

        Ok(())
    }
}

impl Default for HybridEngine {
    fn default() -> Self {
        use crate::vector::chromadb_client::ChromaDBConfig;
        use crate::vector::ollama_embeddings::OllamaConfig;

        Self::new(
            Arc::new(KeywordIndex::new(std::path::PathBuf::from("keyword_index.db")).unwrap()),
            Arc::new(ChromaDBClient::new(ChromaDBConfig::default())),
            Arc::new(OllamaEmbeddings::new(OllamaConfig::default())),
        )
    }
}
