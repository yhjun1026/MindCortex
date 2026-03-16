// Ollama Embeddings Client
// 调用 Ollama API 生成文本嵌入向量

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ollama Embeddings 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub host: String,
    pub port: u16,
    pub model: String,
    pub timeout_seconds: u64,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 11434,
            model: "nomic-embed-text".to_string(),
            timeout_seconds: 30,
        }
    }
}

/// Ollama Embeddings 请求
#[derive(Debug, Serialize)]
struct EmbeddingsRequest {
    model: String,
    prompt: String,
}

/// Ollama Embeddings 响应
#[derive(Debug, Deserialize)]
struct EmbeddingsResponse {
    embedding: Vec<f32>,
}

/// Ollama 批量嵌入请求
#[derive(Debug, Serialize)]
struct BatchEmbeddingsRequest {
    model: String,
    input: Vec<String>,
}

/// Ollama 批量嵌入响应
#[derive(Debug, Deserialize)]
struct BatchEmbeddingsResponse {
    embeddings: Vec<Vec<f32>>,
}

/// Ollama Embeddings 客户端
pub struct OllamaEmbeddings {
    config: OllamaConfig,
    client: reqwest::Client,
    cache: HashMap<String, Vec<f32>>,
}

impl OllamaEmbeddings {
    /// 创建新的 Ollama Embeddings 客户端
    pub fn new(config: OllamaConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
            cache: HashMap::new(),
        }
    }

    /// 使用默认配置创建客户端
    pub fn default() -> Self {
        Self::new(OllamaConfig::default())
    }

    /// 构建 API URL
    fn build_url(&self, endpoint: &str) -> String {
        format!("http://{}:{}/api/{}", self.config.host, self.config.port, endpoint)
    }

    /// 检查 Ollama 服务是否可用
    pub async fn health_check(&self) -> Result<bool, String> {
        let url = self.build_url("tags");
        
        match self.client.get(&url).timeout(
            std::time::Duration::from_secs(self.config.timeout_seconds)
        ).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => Err(format!("Ollama health check failed: {}", e)),
        }
    }

    /// 生成单个文本的嵌入向量
    pub async fn embed(&mut self, text: &str) -> Result<Vec<f32>, String> {
        // 检查缓存
        if let Some(embedding) = self.cache.get(text) {
            return Ok(embedding.clone());
        }

        let url = self.build_url("embeddings");
        let request = EmbeddingsRequest {
            model: self.config.model.clone(),
            prompt: text.to_string(),
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds))
            .send()
            .await
            .map_err(|e| format!("Failed to call Ollama embeddings API: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Ollama embeddings API error: {}", error_text));
        }

        let embedding_response: EmbeddingsResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse embeddings response: {}", e))?;

        let embedding = embedding_response.embedding;
        
        // 缓存结果
        self.cache.insert(text.to_string(), embedding.clone());
        
        Ok(embedding)
    }

    /// 批量生成文本的嵌入向量
    pub async fn embed_batch(&mut self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, String> {
        if texts.is_empty() {
            return Ok(vec![]);
        }

        // 检查缓存，分离已缓存和未缓存的文本
        let mut embeddings: Vec<Vec<f32>> = vec![vec![]; texts.len()];
        let mut uncached_texts: Vec<(usize, String)> = vec![];

        for (idx, text) in texts.iter().enumerate() {
            if let Some(cached_embedding) = self.cache.get(text) {
                embeddings[idx] = cached_embedding.clone();
            } else {
                uncached_texts.push((idx, text.clone()));
            }
        }

        // 批量调用 Ollama API 处理未缓存的文本
        if !uncached_texts.is_empty() {
            let url = self.build_url("embed");
            let input_texts: Vec<String> = uncached_texts.iter()
                .map(|(_, text)| text.clone())
                .collect();

            let request = BatchEmbeddingsRequest {
                model: self.config.model.clone(),
                input: input_texts,
            };

            let response = self.client
                .post(&url)
                .json(&request)
                .timeout(std::time::Duration::from_secs(self.config.timeout_seconds * 2))
                .send()
                .await
                .map_err(|e| format!("Failed to call Ollama batch embeddings API: {}", e))?;

            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                return Err(format!("Ollama batch embeddings API error: {}", error_text));
            }

            let batch_response: BatchEmbeddingsResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse batch embeddings response: {}", e))?;

            // 将结果填充到正确位置
            for (idx, embedding) in batch_response.embeddings.into_iter().enumerate() {
                if idx < uncached_texts.len() {
                    let (original_idx, text) = &uncached_texts[idx];
                    embeddings[*original_idx] = embedding.clone();
                    
                    // 缓存结果
                    self.cache.insert(text.clone(), embedding);
                }
            }
        }

        Ok(embeddings)
    }

    /// 获取嵌入向量维度
    pub async fn get_dimensions(&mut self) -> Result<usize, String> {
        // 生成一个示例嵌入来获取维度
        let embedding = self.embed("test").await?;
        Ok(embedding.len())
    }

    /// 设置模型
    pub fn set_model(&mut self, model: String) {
        self.config.model = model;
    }

    /// 获取当前模型
    pub fn get_model(&self) -> &str {
        &self.config.model
    }

    /// 清空缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 获取缓存大小
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// 设置超时
    pub fn set_timeout(&mut self, timeout_seconds: u64) {
        self.config.timeout_seconds = timeout_seconds;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_ollama_embeddings() {
        let mut embeddings = OllamaEmbeddings::default();
        
        // 测试健康检查
        let _health = embeddings.health_check().await;
        
        // 测试单个嵌入（需要 Ollama 服务运行）
        // let embedding = embeddings.embed("Hello, world!").await;
    }
}
