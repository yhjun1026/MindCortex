// Vector Search Manager
// 管理文本到向量再到搜索的完整流程

use super::{VectorDocument, SearchResult, SearchResultMetadata};
use crate::vector::ChromaDBClient;
use crate::vector::OllamaEmbeddings;
use std::collections::HashSet;

/// 搜索管理器
pub struct SearchManager {
    chroma_client: ChromaDBClient,
    embeddings: OllamaEmbeddings,
}

impl SearchManager {
    /// 创建新的搜索管理器
    pub fn new(chroma_client: ChromaDBClient, embeddings: OllamaEmbeddings) -> Self {
        Self {
            chroma_client,
            embeddings,
        }
    }

    /// 初始化搜索管理器
    pub async fn initialize(&mut self) -> Result<(), String> {
        // 初始化 ChromaDB collection
        self.chroma_client.get_or_create_collection().await?;
        
        // 检查 Ollama 可用性
        let health = self.embeddings.health_check().await?;
        if !health {
            return Err("Ollama service is not available".to_string());
        }
        
        Ok(())
    }

    /// 搜索相关内容
    pub async fn search(&mut self, query: &str, top_k: usize) -> Result<Vec<SearchResult>, String> {
        // 生成查询向量
        let _embedding = self.embeddings.embed(query).await?;
        
        // 执行向量搜索
        let result = self.chroma_client
            .query(vec![query.to_string()], top_k)
            .await?;
        
        // 解析搜索结果
        let search_results = self.parse_search_results(result)?;
        
        Ok(search_results)
    }

    /// 批量搜索
    pub async fn search_batch(&mut self, queries: Vec<String>, top_k: usize) 
        -> Result<Vec<Vec<SearchResult>>, String> {
        
        let mut all_results = vec![];
        
        for query in &queries {
            let results = self.search(query, top_k).await?;
            all_results.push(results);
        }
        
        Ok(all_results)
    }

    /// 添加文档到索引
    pub async fn add_documents(&self, documents: Vec<VectorDocument>) -> Result<(), String> {
        self.chroma_client.add_documents(documents).await
    }

    /// 删除文档
    pub async fn delete_documents(&self, ids: Vec<String>) -> Result<(), String> {
        self.chroma_client.delete_documents(ids).await
    }

    /// 获取索引大小
    pub async fn get_index_size(&self) -> Result<usize, String> {
        self.chroma_client.count().await
    }

    /// 解析搜索结果
    fn parse_search_results(&self, query_result: super::chromadb_client::QueryResult) 
        -> Result<Vec<SearchResult>, String> {
        
        let mut results = vec![];
        
        let ids = query_result.ids.unwrap_or_default();
        let documents = query_result.documents.unwrap_or_default();
        let metadatas = query_result.metadatas.unwrap_or_default();
        let distances = query_result.distances.unwrap_or_default();
        
        for (i, id_list) in ids.iter().enumerate() {
            for (j, id) in id_list.iter().enumerate() {
                let content = documents.get(i)
                    .and_then(|doc_list| doc_list.get(j))
                    .cloned()
                    .unwrap_or_else(|| "".to_string());
                
                let metadata_value = metadatas.get(i)
                    .and_then(|meta_list| meta_list.get(j))
                    .cloned()
                    .unwrap_or_else(|| serde_json::json!({}));
                
                let distance = distances.get(i)
                    .and_then(|dist_list| dist_list.get(j))
                    .cloned()
                    .unwrap_or(1.0);
                
                // 转换距离为相似度分数 (1 - distance)
                let score = (1.0 - distance).max(0.0);
                
                // 解析元数据
                let metadata = SearchResultMetadata {
                    source: metadata_value["source"]
                        .as_str()
                        .unwrap_or("unknown")
                        .to_string(),
                    timestamp: metadata_value["timestamp"]
                        .as_i64()
                        .unwrap_or(chrono::Utc::now().timestamp()),
                    session_id: metadata_value["session_id"]
                        .as_str()
                        .map(|s| s.to_string()),
                    message_id: metadata_value["message_id"]
                        .as_str()
                        .map(|s| s.to_string()),
                    file_path: metadata_value["file_path"]
                        .as_str()
                        .map(|s| s.to_string()),
                };
                
                results.push(SearchResult {
                    id: id.clone(),
                    content,
                    score,
                    metadata,
                });
            }
        }
        
        Ok(results)
    }

    /// 去重搜索结果
    pub fn deduplicate_results(&self, results: Vec<SearchResult>) -> Vec<SearchResult> {
        let mut seen_ids = HashSet::new();
        let mut deduplicated = vec![];
        
        for result in results {
            if seen_ids.insert(result.id.clone()) {
                deduplicated.push(result);
            }
        }
        
        deduplicated
    }

    /// 过滤结果（基于元数据）
    pub fn filter_results(&self, results: Vec<SearchResult>, source_filter: Option<&str>) 
        -> Vec<SearchResult> {
        
        if let Some(source) = source_filter {
            results.into_iter()
                .filter(|r| r.metadata.source == source)
                .collect()
        } else {
            results
        }
    }

    /// 分页结果
    pub fn paginate_results(&self, results: Vec<SearchResult>, page: usize, per_page: usize) 
        -> Vec<SearchResult> {
        
        let start = page * per_page;
        let end = start + per_page;
        
        if start >= results.len() {
            return vec![];
        }
        
        results[start..end.min(results.len())].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_search_manager() {
        // 测试需要实际的 ChromaDB 和 Ollama 服务
        // 在集成测试中运行
    }
}
