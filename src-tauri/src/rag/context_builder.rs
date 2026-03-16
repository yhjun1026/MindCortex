/**
 * Context Builder
 * 构建查询的上下文，用于 RAG 系统的检索
 */

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryContext {
    pub relevant_documents: Vec<DocumentContext>,
    pub code_examples: Vec<CodeExample>,
    pub entities: Vec<EntityContext>,
    pub metadata: ContextMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentContext {
    pub id: String,
    pub title: String,
    pub content: String,
    pub relevance_score: f64,
    pub file_path: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub id: String,
    pub description: String,
    pub code: String,
    pub language: String,
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityContext {
    pub name: String,
    pub entity_type: String,
    pub description: Option<String>,
    pub relations: Vec<EntityRelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRelation {
    pub target: String,
    pub relation_type: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMetadata {
    pub max_context_length: usize,
    pub include_code: bool,
    pub include_explanations: bool,
    pub current_timestamp: String,
}

pub struct ContextBuilder {
    max_context_length: usize,
    include_code: bool,
    include_explanations: bool,
}

impl ContextBuilder {
    pub fn new() -> Self {
        ContextBuilder {
            max_context_length: 4000,
            include_code: true,
            include_explanations: true,
        }
    }

    pub fn build_context(
        &self,
        query: &str,
        search_results: Vec<serde_json::Value>,
    ) -> QueryContext {
        let relevant_documents = self.extract_documents(search_results.clone());
        let code_examples = self.extract_code_examples(search_results);
        let entities = self.extract_entities(query);

        QueryContext {
            relevant_documents,
            code_examples,
            entities,
            metadata: ContextMetadata {
                max_context_length: self.max_context_length,
                include_code: self.include_code,
                include_explanations: self.include_explanations,
                current_timestamp: chrono::Utc::now().to_rfc3339(),
            },
        }
    }

    fn extract_documents(&self, results: Vec<serde_json::Value>) -> Vec<DocumentContext> {
        results
            .iter()
            .filter_map(|result| {
                let title = result.get("title").and_then(|v| v.as_str()).unwrap_or("Untitled");
                let content = result.get("content").and_then(|v| v.as_str()).unwrap_or("");
                let score = result.get("score").and_then(|v| v.as_f64()).unwrap_or(0.5);

                Some(DocumentContext {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: title.to_string(),
                    content: content.to_string(),
                    relevance_score: score,
                    file_path: result.get("filePath").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    language: result.get("language").and_then(|v| v.as_str()).map(|s| s.to_string()),
                })
            })
            .collect()
    }

    fn extract_code_examples(&self, results: Vec<serde_json::Value>) -> Vec<CodeExample> {
        results
            .iter()
            .filter_map(|result| {
                let content = result.get("content").and_then(|v| v.as_str()).unwrap_or("");
                let language = result.get("language").and_then(|v| v.as_str()).unwrap_or("unknown");

                // 只处理代码类型的结果
                if result.get("type").and_then(|v| v.as_str()) != Some("code") {
                    return None;
                }

                Some(CodeExample {
                    id: uuid::Uuid::new_v4().to_string(),
                    description: "Example from search results".to_string(),
                    code: content.to_string(),
                    language: language.to_string(),
                    usage: "Found in codebase".to_string(),
                })
            })
            .collect()
    }

    fn extract_entities(&self, query: &str) -> Vec<EntityContext> {
        let mut entities = Vec::new();

        // 简化的实体提取
        let words: Vec<&str> = query.split_whitespace().collect();

        for word in words {
            if word.len() > 3 && word.chars().next().map_or(false, |c| c.is_uppercase()) {
                entities.push(EntityContext {
                    name: word.to_string(),
                    entity_type: "unknown".to_string(),
                    description: None,
                    relations: Vec::new(),
                });
            }
        }

        entities
    }

    pub fn set_max_context_length(&mut self, length: usize) {
        self.max_context_length = length;
    }

    pub fn set_include_code(&mut self, include: bool) {
        self.include_code = include;
    }

    pub fn set_include_explanations(&mut self, include: bool) {
        self.include_explanations = include;
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}
