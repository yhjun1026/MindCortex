// ChromaDB Client
// 实现 ChromaDB HTTP 客户端，用于向量存储和检索

use super::VectorDocument;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ChromaDB 客户端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromaDBConfig {
    pub host: String,
    pub port: u16,
    pub collection_name: String,
}

impl Default for ChromaDBConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8000,
            collection_name: "mindcortex".to_string(),
        }
    }
}

/// ChromaDB 客户端
pub struct ChromaDBClient {
    config: ChromaDBConfig,
    client: reqwest::Client,
    collection_id: Option<String>,
}

impl ChromaDBClient {
    /// 创建新的 ChromaDB 客户端
    pub fn new(config: ChromaDBConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
            collection_id: None,
        }
    }

    /// 使用默认配置创建客户端
    pub fn default() -> Self {
        Self::new(ChromaDBConfig::default())
    }

    /// 构建 API URL
    fn build_url(&self, path: &str) -> String {
        format!("http://{}:{}{}", self.config.host, self.config.port, path)
    }

    /// 检查 ChromaDB 健康状态
    pub async fn health_check(&self) -> Result<bool, String> {
        let url = self.build_url("/api/v1/heartbeat");
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => Err(format!("Health check failed: {}", e)),
        }
    }

    /// 创建或获取 Collection
    pub async fn get_or_create_collection(&mut self) -> Result<String, String> {
        // 先尝试获取已存在的 collection
        let collections = self.list_collections().await?;
        
        for coll in &collections {
            if coll.name == self.config.collection_name {
                self.collection_id = Some(coll.id.clone());
                return Ok(coll.id.clone());
            }
        }
        
        // 不存在，创建新的
        self.create_collection().await
    }

    /// 列出所有 collections
    async fn list_collections(&self) -> Result<Vec<CollectionInfo>, String> {
        let url = self.build_url("/api/v1/collections");
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to list collections: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("List collections failed with status: {}", response.status()));
        }
        
        let result: CollectionsResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse collections response: {}", e))?;
        
        Ok(result.collections)
    }

    /// 创建新的 collection
    async fn create_collection(&mut self) -> Result<String, String> {
        let url = self.build_url("/api/v1/collections");
        
        let payload = serde_json::json!({
            "name": self.config.collection_name,
            "metadata": {
                "description": "MindCortex knowledge base",
                "created_at": chrono::Utc::now().to_rfc3339()
            }
        });
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Failed to create collection: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Create collection failed: {}", error_text));
        }
        
        let result: CollectionInfo = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse collection response: {}", e))?;
        
        self.collection_id = Some(result.id.clone());
        Ok(result.id)
    }

    /// 添加文档到 collection
    pub async fn add_documents(&self, documents: Vec<VectorDocument>) -> Result<(), String> {
        if self.collection_id.is_none() {
            return Err("Collection not initialized. Call get_or_create_collection first.".to_string());
        }
        
        if documents.is_empty() {
            return Ok(());
        }
        
        let collection_id = self.collection_id.as_ref().unwrap();
        let url = format!("{}/api/v1/collections/{}/add", 
                         self.build_url(""), collection_id);
        
        let ids: Vec<String> = documents.iter().map(|d| d.id.clone()).collect();
        let texts: Vec<String> = documents.iter().map(|d| d.text.clone()).collect();
        let metadatas: Vec<serde_json::Value> = documents.iter().map(|d| d.metadata.clone()).collect();
        
        let payload = serde_json::json!({
            "ids": ids,
            "documents": texts,
            "metadatas": metadatas
        });
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Failed to add documents: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Add documents failed: {}", error_text));
        }
        
        Ok(())
    }

    /// 删除文档
    pub async fn delete_documents(&self, ids: Vec<String>) -> Result<(), String> {
        if self.collection_id.is_none() {
            return Err("Collection not initialized.".to_string());
        }
        
        let collection_id = self.collection_id.as_ref().unwrap();
        let url = format!("{}/api/v1/collections/{}/delete",
                         self.build_url(""), collection_id);
        
        let payload = serde_json::json!({
            "ids": ids
        });
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Failed to delete documents: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Delete documents failed: {}", error_text));
        }
        
        Ok(())
    }

    /// 查询 collection
    pub async fn query(&self, query_texts: Vec<String>, n_results: usize) -> Result<QueryResult, String> {
        if self.collection_id.is_none() {
            return Err("Collection not initialized.".to_string());
        }
        
        let collection_id = self.collection_id.as_ref().unwrap();
        let url = format!("{}/api/v1/collections/{}/get",
                         self.build_url(""), collection_id);
        
        let payload = serde_json::json!({
            "query_texts": query_texts,
            "n_results": n_results
        });
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Failed to query: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Query failed: {}", error_text));
        }
        
        let result: QueryResult = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse query response: {}", e))?;
        
        Ok(result)
    }

    /// 获取 collection 统计信息
    pub async fn count(&self) -> Result<usize, String> {
        if self.collection_id.is_none() {
            return Err("Collection not initialized.".to_string());
        }
        
        let collection_id = self.collection_id.as_ref().unwrap();
        let url = format!("{}/api/v1/collections/{}/count",
                         self.build_url(""), collection_id);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to get count: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("Get count failed with status: {}", response.status()));
        }
        
        let result: CountResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse count response: {}", e))?;
        
        Ok(result.count)
    }

    /// 清空 collection
    pub async fn clear_collection(&self) -> Result<(), String> {
        // 获取所有文档 ID
        let collection_id = self.collection_id.as_ref().ok_or("Collection not initialized")?;
        let url = format!("{}/api/v1/collections/{}/get",
                         self.build_url(""), collection_id);
        
        let payload = serde_json::json!({
            "limit": 10000,
            "include": ["documents", "metadatas"]
        });
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Failed to get documents: {}", e))?;
        
        if response.status().is_success() {
            let result: QueryResult = response
                .json()
                .await
                .unwrap_or_else(|_| QueryResult::default());
            
            // 展平嵌套的 ID 结构
            if let Some(nested_ids) = result.ids {
                let flat_ids: Vec<String> = nested_ids.into_iter().flatten().collect();
                if !flat_ids.is_empty() {
                    self.delete_documents(flat_ids).await?;
                }
            }
        }
        
        Ok(())
    }
}

/// Collection 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub id: String,
    pub name: String,
    pub metadata: Option<serde_json::Value>,
}

/// Collections 响应
#[derive(Debug, Serialize, Deserialize)]
struct CollectionsResponse {
    pub collections: Vec<CollectionInfo>,
}

/// 查询结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryResult {
    pub ids: Option<Vec<Vec<String>>>,
    pub embeddings: Option<Vec<Vec<Vec<f32>>>>,
    pub documents: Option<Vec<Vec<String>>>,
    pub metadatas: Option<Vec<Vec<serde_json::Value>>>,
    pub distances: Option<Vec<Vec<f64>>>,
}

/// Count 响应
#[derive(Debug, Serialize, Deserialize)]
struct CountResponse {
    pub count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_chromadb_client_init() {
        let mut client = ChromaDBClient::default();
        
        // 注意：这些测试需要 ChromaDB 服务运行
        // 在实际使用中跳过或集成到 CI 中
        
        // 创建或获取 collection
        let _collection_id = client.get_or_create_collection().await;
    }
}
