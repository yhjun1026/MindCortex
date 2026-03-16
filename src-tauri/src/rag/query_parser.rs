/**
 * 查询解析器
 * 解析自然语言查询，识别意图和关键信息
 */

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QueryIntent {
    Search,
    Explain,
    Compare,
    List,
    Summarize,
    Analyze,
    Generate,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedQuery {
    pub original: String,
    pub intent: QueryIntent,
    pub entities: Vec<String>,
    pub keywords: Vec<String>,
    pub filters: QueryFilters,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilters {
    pub language: Option<String>,
    pub file_type: Option<String>,
    pub date_range: Option<(String, String)>,
    pub tags: Vec<String>,
    pub max_results: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryContext {
    pub previous_queries: Vec<ParsedQuery>,
    pub current_file: Option<String>,
    pub current_language: Option<String>,
    pub user_preferences: QueryPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPreferences {
    pub preferred_language: Option<String>,
    pub max_results: usize,
    pub include_code_examples: bool,
    pub include_explanations: bool,
}

pub struct QueryParser {
    context: QueryContext,
}

impl QueryParser {
    pub fn new() -> Self {
        QueryParser {
            context: QueryContext {
                previous_queries: Vec::new(),
                current_file: None,
                current_language: None,
                user_preferences: QueryPreferences {
                    preferred_language: None,
                    max_results: 10,
                    include_code_examples: true,
                    include_explanations: true,
                },
            },
        }
    }

    pub fn parse(&mut self, query: &str) -> ParsedQuery {
        let intent = self.detect_intent(query);
        let entities = self.extract_entities(query);
        let keywords = self.extract_keywords(query);
        let filters = self.extract_filters(query);
        let confidence = self.calculate_conf(query, &intent);

        let parsed = ParsedQuery {
            original: query.to_string(),
            intent,
            entities,
            keywords,
            filters,
            confidence,
        };

        // 添加到历史
        self.context.previous_queries.push(parsed.clone());

        parsed
    }

    fn detect_intent(&self, query: &str) -> QueryIntent {
        let query_lower = query.to_lowercase();

        if query_lower.contains("search") || query_lower.contains("find") || query_lower.contains("查找") {
            QueryIntent::Search
        } else if query_lower.contains("explain") || query_lower.contains("what is") || query_lower.contains("解释") {
            QueryIntent::Explain
        } else if query_lower.contains("compare") || query_lower.contains("difference") || query_lower.contains("比较") {
            QueryIntent::Compare
        } else if query_lower.contains("list") || query_lower.contains("show all") || query_lower.contains("列出") {
            QueryIntent::List
        } else if query_lower.contains("summarize") || query_lower.contains("summary") || query_lower.contains("总结") {
            QueryIntent::Summarize
        } else if query_lower.contains("analyze") || query_lower.contains("analysis") || query_lower.contains("分析") {
            QueryIntent::Analyze
        } else if query_lower.contains("generate") || query_lower.contains("create") || query_lower.contains("生成") {
            QueryIntent::Generate
        } else {
            QueryIntent::Other(query_lower)
        }
    }

    fn extract_entities(&self, query: &str) -> Vec<String> {
        let mut entities = Vec::new();

        // 简化的实体提取（实际应用中应使用 NER）
        let words: Vec<&str> = query.split_whitespace().collect();

        for word in words {
            // 假设大写开头的词为实体
            if word.len() > 1 && word.chars().next().map_or(false, |c| c.is_uppercase()) {
                entities.push(word.to_string());
            }
        }

        entities
    }

    fn extract_keywords(&self, query: &str) -> Vec<String> {
        let mut keywords = Vec::new();

        // 移除停用词和标点
        let stop_words: Vec<String> = vec![
            "the", "a", "an", "is", "are", "was", "were", "be", "been",
            "have", "has", "had", "do", "does", "did", "will", "would",
            "could", "should", "may", "might", "must", "can", "to", "from",
            "in", "on", "at", "by", "for", "with", "about", "into",
            "的", "是", "在", "有", "和", "与", "对", "关于", "从", "到"
        ].into_iter().map(|s| s.to_string()).collect();

        let words: Vec<String> = query
            .split_whitespace()
            .map(|word| {
                word.chars()
                    .filter(|c: &char| c.is_alphanumeric() || *c == '_')
                    .collect::<String>()
            })
            .filter(|word| !stop_words.contains(&word.to_lowercase()))
            .filter(|word| word.len() > 2)
            .collect();

        keywords = words;
        keywords
    }

    fn extract_filters(&self, query: &str) -> QueryFilters {
        let mut filters = QueryFilters {
            language: None,
            file_type: None,
            date_range: None,
            tags: Vec::new(),
            max_results: None,
        };

        let query_lower = query.to_lowercase();

        // 提取语言过滤器
        for lang in &["rust", "python", "javascript", "typescript", "java", "go", "cpp"] {
            if query_lower.contains(lang) {
                filters.language = Some(lang.to_string());
                break;
            }
        }

        // 提取文件类型过滤器
        if query_lower.contains(".rs") {
            filters.file_type = Some("rust".to_string());
        } else if query_lower.contains(".py") {
            filters.file_type = Some("python".to_string());
        } else if query_lower.contains(".js") || query_lower.contains(".ts") {
            filters.file_type = Some("javascript".to_string());
        }

        filters
    }

    fn calculate_conf(&self, query: &str, intent: &QueryIntent) -> f64 {
        let mut confidence: f32 = 0.5; // 基础置信度

        // 基于查询长度
        let word_count = query.split_whitespace().count();
        if word_count >= 3 && word_count <= 10 {
            confidence += 0.2;
        } else if word_count > 10 {
            confidence += 0.1;
        }

        // 基于意图类型
        match intent {
            QueryIntent::Search | QueryIntent::Explain => confidence += 0.2,
            QueryIntent::Other(_) => confidence -= 0.1,
            _ => {}
        }

        // 确保在 0-1 范围内
        confidence.max(0.0).min(1.0) as f64
    }

    pub fn update_context(&mut self, file: Option<String>, language: Option<String>) {
        self.context.current_file = file;
        self.context.current_language = language;
    }

    pub fn get_context(&self) -> &QueryContext {
        &self.context
    }

    pub fn set_preferences(&mut self, preferences: QueryPreferences) {
        self.context.user_preferences = preferences;
    }

    pub fn get_history(&self) -> &[ParsedQuery] {
        &self.context.previous_queries
    }

    pub fn clear_history(&mut self) {
        self.context.previous_queries.clear();
    }
}

impl Default for QueryParser {
    fn default() -> Self {
        Self::new()
    }
}
