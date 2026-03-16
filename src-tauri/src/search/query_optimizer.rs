// 查询优化器
// 实现查询意图识别、关键词提取和查询优化

use super::HybridSearchConfig;
use std::collections::HashSet;

/// 查询类型
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum QueryType {
    /// 关键词搜索
    Keyword,
    /// 语义搜索
    Semantic,
    /// 混合搜索
    Hybrid,
    /// 时间范围查询
    TimeRange,
    /// 文件路径查询
    FilePath,
    /// 标签查询
    Tag,
}

/// 查询意图
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryIntent {
    pub query_type: QueryType,
    pub keywords: Vec<String>,
    pub semantic_query: Option<String>,
    pub time_range: Option<TimeRange>,
    pub file_path: Option<String>,
    pub tags: Vec<String>,
    pub entity: Option<String>,
    pub action: Option<String>,
}

/// 时间范围
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimeRange {
    pub start: i64,
    pub end: i64,
    pub relative: bool,
}

/// 查询优化器
pub struct QueryOptimizer {
    // 常用关键词列表
    keywords: HashSet<String>,
    // 常用实体列表
    entities: HashSet<String>,
    // 常用标签列表
    tags: HashSet<String>,
}

impl QueryOptimizer {
    /// 创建新的查询优化器
    pub fn new() -> Self {
        Self {
            keywords: HashSet::new(),
            entities: HashSet::new(),
            tags: HashSet::new(),
        }
    }

    /// 分析查询意图
    pub fn analyze_query(&self, query: &str) -> QueryIntent {
        let mut intent = QueryIntent {
            query_type: QueryType::Keyword,
            keywords: vec![],
            semantic_query: None,
            time_range: None,
            file_path: None,
            tags: vec![],
            entity: None,
            action: None,
        };

        // 1. 识别关键词
        let keywords = self.extract_keywords(query);
        intent.keywords = keywords.clone();

        // 2. 识别时间范围
        if let Some(time_range) = self.extract_time_range(query) {
            intent.time_range = Some(time_range);
            intent.query_type = QueryType::TimeRange;
        }

        // 3. 识别文件路径
        if let Some(file_path) = self.extract_file_path(query) {
            intent.file_path = Some(file_path);
            intent.query_type = QueryType::FilePath;
        }

        // 4. 识别标签
        let tags = self.extract_tags(query);
        intent.tags = tags.clone();
        if !tags.is_empty() {
            intent.query_type = QueryType::Tag;
        }

        // 5. 识别实体
        if let Some(entity) = self.extract_entity(query) {
            intent.entity = Some(entity);
        }

        // 6. 识别动作
        if let Some(action) = self.extract_action(query) {
            intent.action = Some(action);
        }

        // 7. 如果有语义信息，转换为语义查询
        if self.has_semantic_content(query) {
            intent.semantic_query = Some(query.to_string());
            if keywords.len() > 1 {
                intent.query_type = QueryType::Hybrid;
            } else {
                intent.query_type = QueryType::Semantic;
            }
        }

        intent
    }

    /// 优化查询
    pub fn optimize_query(&self, query: &mut String) -> QueryIntent {
        let intent = self.analyze_query(query);

        // 1. 移除停用词
        if let Some(cleaned) = self.remove_stop_words(query) {
            *query = cleaned;
        }

        // 2. 标准化查询格式
        self.normalize_query(query);

        // 3. 扩展缩写
        self.expand_abbreviations(query);

        intent
    }

    /// 提取关键词
    fn extract_keywords(&self, query: &str) -> Vec<String> {
        let mut keywords = vec![];

        // 简单的关键词提取（基于空格和标点）
        let words: Vec<String> = query
            .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
            .filter(|w: &str| w.len() > 1)
            .map(|s| s.to_string())
            .collect();

        for word in words {
            // 转换为小写
            let keyword = word.to_lowercase();

            // 添加到结果
            if !keywords.contains(&keyword) {
                keywords.push(keyword);
            }
        }

        keywords
    }

    /// 提取时间范围
    fn extract_time_range(&self, query: &str) -> Option<TimeRange> {
        // TODO: 实现时间范围识别
        // 例如："今天"、"最近一周"、"2024-01-01 2024-01-31"
        None
    }

    /// 提取文件路径
    fn extract_file_path(&self, query: &str) -> Option<String> {
        // TODO: 实现文件路径识别
        // 例如："file:src/main.rs", "path:home/user/file.txt"
        None
    }

    /// 提取标签
    fn extract_tags(&self, query: &str) -> Vec<String> {
        let mut tags = vec![];

        // 识别标签（以 # 开头的词）
        let words: Vec<String> = query
            .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
            .filter(|w: &str| w.starts_with('#'))
            .map(|s| s.to_string())
            .collect();

        for tag in words {
            let tag_clean = tag.trim_start_matches('#').to_string();
            if !tags.contains(&tag_clean) {
                tags.push(tag_clean);
            }
        }

        tags
    }

    /// 提取实体
    fn extract_entity(&self, query: &str) -> Option<String> {
        // TODO: 实现实体识别（根据常见实体列表）
        None
    }

    /// 提取动作
    fn extract_action(&self, query: &str) -> Option<String> {
        // TODO: 实现动作识别
        // 例如："创建"、"删除"、"更新"、"搜索"
        None
    }

    /// 检查查询是否具有语义内容
    fn has_semantic_content(&self, query: &str) -> bool {
        // 长查询通常具有语义
        // 包含多个非技术词的查询可能是语义查询
        let words: Vec<String> = query
            .split(|c: char| c.is_whitespace())
            .filter(|w: &str| w.len() > 2)
            .map(|s| s.to_string())
            .collect();

        words.len() >= 2
    }

    /// 移除停用词
    fn remove_stop_words(&self, query: &str) -> Option<String> {
        let stop_words = [
            "the", "a", "an", "is", "are", "was", "were", "be", "been",
            "to", "of", "in", "on", "at", "by", "for", "with", "from",
            "和", "的", "是", "在", "从", "使用", "通过", "关于",
            "如何", "什么", "为什么", "怎么", "哪个", "哪些",
            "这个", "这些", "那个", "那些", "所有", "一些",
            "this", "these", "that", "those", "all", "some",
            "please", "help", "show", "find", "get", "list",
            "的", "一个", "几个", "很多", "更多", "最少",
            "的", "的", "的", "的", "的",
        ];

        let mut result = query.to_string();
        let mut removed = false;

        for stop_word in &stop_words {
            let pattern = format!(" {} ", stop_word);
            result = result.replace(&pattern, " ");
            
            if result.contains(&format!("{} ", stop_word)) {
                removed = true;
            }
        }

        if removed {
            // 清理多余空格
            while result.contains("  ") {
                result = result.replace("  ", " ");
            }
        }

        if result.trim().is_empty() {
            None
        } else {
            Some(result.trim().to_string())
        }
    }

    /// 标准化查询格式
    fn normalize_query(&self, query: &mut String) {
        // 移除多余空格
        while query.contains("  ") {
            *query = query.replace("  ", " ");
        }
        
        // 移除首尾空格
        *query = query.trim().to_string();

        // 转换为小写（用于搜索）
        // 但保留原始格式用于某些分析
    }

    /// 扩展缩写
    fn expand_abbreviations(&self, query: &mut String) {
        // TODO: 实现缩写扩展
        // 例如："k8s" -> "kubernetes", "db" -> "database"
        let abbreviations = vec![
            ("k8s", "kubernetes"),
            ("k8s", "k8s"),
            ("db", "database"),
            ("fe", "frontend"),
            ("be", "backend"),
            ("api", "api"),
            ("sdk", "software development kit"),
            ("cli", "command line interface"),
            ("gui", "graphical user interface"),
            ("ui", "user interface"),
            ("ux", "user experience"),
            ("ai", "artificial intelligence"),
            ("ml", "machine learning"),
            ("dl", "deep learning"),
            ("nlp", "natural language processing"),
            ("llm", "large language model"),
            ("rpa", "retrieval-augmented-generation"),
            ("agi", "artificial general intelligence"),
        ];

        for (abbrev, full) in abbreviations {
            *query = query.replace(abbrev, full);
        }
    }

    /// 添加关键词到常用列表
    pub fn add_keyword(&mut self, keyword: String) {
        self.keywords.insert(keyword.to_lowercase());
    }

    /// 添加实体到常用列表
    pub fn add_entity(&mut self, entity: String) {
        self.entities.insert(entity.to_lowercase());
    }

    /// 添加标签到常用列表
    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag.to_lowercase());
    }

    /// 获取搜索建议
    pub fn get_search_suggestions(&self, partial_query: &str, limit: usize) 
        -> Vec<String> {
        let mut suggestions = vec![];

        // 基于关键词提供建议
        let partial_lower = partial_query.to_lowercase();

        for keyword in self.keywords.iter() {
            if keyword.starts_with(&partial_lower) && !suggestions.contains(keyword) {
                suggestions.push(keyword.clone());
                if suggestions.len() >= limit {
                    break;
                }
            }
        }

        suggestions.sort();
        suggestions
    }

    /// 从历史中学习
    pub fn learn_from_search(&mut self, query: &str, results_count: usize) {
        // 提取关键词
        let keywords = self.extract_keywords(query);
        for keyword in keywords {
            self.add_keyword(keyword);
        }

        // TODO: 提取其他模式（文件路径、标签等）
    }
}

impl Default for QueryOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keyword_extraction() {
        let optimizer = QueryOptimizer::new();
        let query = "如何使用 k8s 创建 database服务";
        let keywords = optimizer.extract_keywords(query);
        
        assert!(keywords.contains(&"如何".to_string()));
        assert!(keywords.contains(&"k8s".to_string()));
        assert!(keywords.contains(&"创建".to_string()));
        assert!(keywords.contains(&"数据库".to_string()));
        assert!(keywords.contains(&"服务".to_string()));
    }
    
    #[test]
    fn test_tag_extraction() {
        let optimizer = QueryOptimizer::new();
        let query = "搜索 #frontend #api 相关的问题";
        let tags = optimizer.extract_tags(query);
        
        assert!(tags.contains("frontend".to_string()));
        assert!(tags.contains("api".to_string()));
    }
    
    #[test]
    fn test_query_intent() {
        let optimizer = QueryOptimizer::new();
        let intent = optimizer.analyze_query("搜索最近一周 #frontend 的代码");
        
        assert_eq!(intent.tags, vec!["frontend".to_string()]);
        assert_eq!(intent.keywords.len(), 3);
        assert!(intent.query_type, QueryType::Tag);
    }
}
