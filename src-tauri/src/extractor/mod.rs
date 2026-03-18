// 知识提取引擎
// 使用 AI 模型处理原始数据，提取和整理知识

pub mod classification;    // 分类：项目、任务、技术栈
pub mod summarization;     // 摘要生成
pub mod insight;          // 洞察提取

use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;
use crate::agents::{SessionData, Message, Attachment};

// ============== 核心数据结构 ==============

/// 知识项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeItem {
    pub id: String,
    pub source_session_id: String,
    pub item_type: String,           // "code", "design", "insight", "task"
    pub title: String,
    pub summary: String,
    pub content: String,
    pub tags: Vec<String>,            // 技术栈标签
    pub project: Option<String>,       // 所属项目
    pub embedding: Option<Vec<f32>>,  // 向量（可选，可能由 ChromaDB 生成）
    pub timestamp: i64,
}

/// 项目节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub tasks: Vec<String>,           // 关联的任务 ID
    pub knowledge_items: Vec<String>,  // 关联的知识项 ID
    pub created_at: i64,
    pub updated_at: i64,
}

// ============== 知识提取器 ==============

/// 知识提取配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractorConfig {
    pub auto_extract: bool,
    pub extract_code: bool,
    pub extract_design: bool,
    pub extract_insights: bool,
    pub use_ai_for_summary: bool,
    pub max_content_length: usize,
}

impl Default for ExtractorConfig {
    fn default() -> Self {
        Self {
            auto_extract: true,
            extract_code: true,
            extract_design: true,
            extract_insights: true,
            use_ai_for_summary: true,
            max_content_length: 10000,
        }
    }
}

/// 提取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractResult {
    pub knowledge_items: Vec<KnowledgeItem>,
    pub project_updates: Vec<ProjectNode>,
    pub extraction_time_ms: i64,
}

/// 知识提取器
pub struct KnowledgeExtractor {
    config: ExtractorConfig,
    // 可选：AI 模型客户端
}

impl KnowledgeExtractor {
    pub fn new(config: ExtractorConfig) -> Self {
        Self {
            config,
        }
    }

    /// 从会话中提取知识
    pub async fn extract_from_session(&self, session: &SessionData) -> Result<ExtractResult, String> {
        let start_time = std::time::Instant::now();
        let mut knowledge_items = vec![];
        let mut project_updates = vec![];

        // 遍历所有消息
        for message in &session.messages {
            // 提取代码片段
            if self.config.extract_code {
                if let Some(code_items) = self.extract_code_from_message(message, &session.id).await {
                    knowledge_items.extend(code_items);
                }
            }

            // 提取设计讨论
            if self.config.extract_design {
                if let Some(design_items) = self.extract_design_from_message(message, &session.id).await {
                    knowledge_items.extend(design_items);
                }
            }

            // 提取洞察和最佳实践
            if self.config.extract_insights {
                if let Some(insight_items) = self.extract_insights_from_message(message, &session.id).await {
                    knowledge_items.extend(insight_items);
                }
            }
        }

        // 识别项目
        if let Some(project) = self.identify_project(session).await {
            project_updates.push(project);
        }

        let extraction_time_ms = start_time.elapsed().as_millis() as i64;

        Ok(ExtractResult {
            knowledge_items,
            project_updates,
            extraction_time_ms,
        })
    }

    /// 从消息中提取代码片段
    async fn extract_code_from_message(&self, message: &Message, session_id: &str) -> Option<Vec<KnowledgeItem>> {
        // 检查消息是否包含代码
        if message.attachments.is_empty() {
            return None;
        }

        let mut items = vec![];

        for attachment in &message.attachments {
            if attachment.type_field == "code" {
                // 提取代码知识
                let title = self.extract_code_title(&attachment.content, attachment.language.as_deref())
                    .unwrap_or_else(|| "Code snippet".to_string());

                let summary = if self.config.use_ai_for_summary {
                    // 使用 AI 生成摘要（待实现）
                    self.generate_summary(&attachment.content).await
                } else {
                    self.generate_simple_summary(&attachment.content)
                };

                let item = KnowledgeItem {
                    id: Uuid::new_v4().to_string(),
                    source_session_id: session_id.to_string(),
                    item_type: "code".to_string(),
                    title,
                    summary,
                    content: attachment.content.clone(),
                    tags: self.extract_tags_from_content(&attachment.content),
                    project: None,
                    embedding: None,
                    timestamp: message.timestamp,
                };

                items.push(item);
            }
        }

        if items.is_empty() {
            None
        } else {
            Some(items)
        }
    }

    /// 从消息中提取设计讨论
    async fn extract_design_from_message(&self, message: &Message, session_id: &str) -> Option<Vec<KnowledgeItem>> {
        // 检查消息内容是否包含设计相关关键词
        let design_keywords = ["设计", "架构", "design", "architecture", "方案", "架构"];
        let is_design = design_keywords.iter()
            .any(|keyword| message.content.to_lowercase().contains(keyword));

        if !is_design || message.content.len() < 50 {
            return None;
        }

        let item = KnowledgeItem {
            id: Uuid::new_v4().to_string(),
            source_session_id: session_id.to_string(),
            item_type: "design".to_string(),
            title: self.extract_title_from_text(&message.content),
            summary: self.generate_simple_summary(&message.content),
            content: message.content.clone(),
            tags: self.extract_tags_from_content(&message.content),
            project: None,
            embedding: None,
            timestamp: message.timestamp,
        };

        Some(vec![item])
    }

    /// 从消息中提取洞察和最佳实践
    async fn extract_insights_from_message(&self, message: &Message, session_id: &str) -> Option<Vec<KnowledgeItem>> {
        // 检查是否包含洞察相关关键词
        let insight_keywords = ["经验", "教训", "最佳实践", "best practice", "lesson", "learned"];
        let is_insight = insight_keywords.iter()
            .any(|keyword| message.content.to_lowercase().contains(keyword));

        if !is_insight || message.content.len() < 50 {
            return None;
        }

        let item = KnowledgeItem {
            id: Uuid::new_v4().to_string(),
            source_session_id: session_id.to_string(),
            item_type: "insight".to_string(),
            title: self.extract_title_from_text(&message.content),
            summary: self.generate_simple_summary(&message.content),
            content: message.content.clone(),
            tags: self.extract_tags_from_content(&message.content),
            project: None,
            embedding: None,
            timestamp: message.timestamp,
        };

        Some(vec![item])
    }

    /// 识别项目
    async fn identify_project(&self, session: &SessionData) -> Option<ProjectNode> {
        // 简单实现：从会话内容中提取项目名称
        // 实际实现可以使用 AI 或更复杂的模式匹配

        // 收集所有消息内容
        let all_content = session.messages.iter()
            .map(|m| m.content.clone())
            .collect::<Vec<_>>()
            .join(" ");

        // 查找项目关键词（"项目"、"project" 等）
        let project_pattern = regex::Regex::new(r"(?:项目|project|Project|PROJECT)\s*[:：]\s*([^，。\n]+)")
            .ok()?;

        if let Some(captures) = project_pattern.captures(&all_content) {
            if let Some(project_name) = captures.get(1) {
                let project = ProjectNode {
                    id: Uuid::new_v4().to_string(),
                    name: project_name.as_str().trim().to_string(),
                    description: format!("From session {}", session.id),
                    technologies: vec![],
                    tasks: vec![],
                    knowledge_items: vec![],
                    created_at: session.timestamp,
                    updated_at: Utc::now().timestamp(),
                };
                return Some(project);
            }
        }

        None
    }

    /// 提取代码标题
    fn extract_code_title(&self, content: &str, language: Option<&str>) -> Option<String> {
        // 查找函数/类名
        let func_pattern = regex::Regex::new(r"(?:def\s+(\w+)|(?:class|struct|interface)\s+(\w+))")
            .ok()?;

        if let Some(captures) = func_pattern.captures(content) {
            let name = captures.get(1).or_else(|| captures.get(2))?;
            return Some(format!("{}: {}", language.unwrap_or("code"), name.as_str()));
        }

        // 提取第一行注释
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") || trimmed.starts_with("#") {
                let comment = trimmed.trim_start_matches("//").trim_start_matches("#").trim();
                if !comment.is_empty() {
                    return Some(comment.to_string());
                }
            }
        }

        Some("Code snippet".to_string())
    }

    /// 提取文本标题
    fn extract_title_from_text(&self, text: &str) -> String {
        // 提取第一行
        let first_line = text.lines().next()
            .unwrap_or("Untitled")
            .trim();

        // 如果太长，截断
        if first_line.len() > 100 {
            format!("{}...", &first_line[..97])
        } else {
            first_line.to_string()
        }
    }

    /// 生成简单摘要（不使用 AI）
    fn generate_simple_summary(&self, content: &str) -> String {
        // 取前 200 个字符作为摘要
        let content = content.trim();
        
        if content.len() <= 200 {
            content.to_string()
        } else {
            format!("{}...", &content[..197])
        }
    }

    /// 使用 AI 生成摘要（待实现）
    async fn generate_summary(&self, content: &str) -> String {
        // TODO: 集成 AI 模型生成摘要
        // 暂时使用简单摘要
        self.generate_simple_summary(content)
    }

    /// 从内容中提取标签
    fn extract_tags_from_content(&self, content: &str) -> Vec<String> {
        let mut tags = vec![];

        // 常见技术栈关键词
        let tech_keywords = [
            "python", "javascript", "typescript", "rust", "go", "java",
            "react", "vue", "angular", "svelte",
            "node", "express", "django", "flask", "spring",
            "postgresql", "mysql", "mongodb", "redis",
            "docker", "kubernetes", "aws", "gcp",
            "git", "linux", "macos", "windows",
        ];

        let content_lower = content.to_lowercase();

        for keyword in &tech_keywords {
            if content_lower.contains(keyword) {
                tags.push(keyword.to_string());
            }
        }

        tags
    }

    /// 批量提取
    pub async fn extract_from_sessions(&self, sessions: &[SessionData]) -> Result<ExtractResult, String> {
        let start_time = std::time::Instant::now();
        let mut all_knowledge_items = vec![];
        let mut all_project_updates = vec![];

        for session in sessions {
            match self.extract_from_session(session).await {
                Ok(result) => {
                    all_knowledge_items.extend(result.knowledge_items);
                    all_project_updates.extend(result.project_updates);
                }
                Err(e) => {
                    eprintln!("Failed to extract from session {}: {}", session.id, e);
                }
            }
        }

        let extraction_time_ms = start_time.elapsed().as_millis() as i64;

        Ok(ExtractResult {
            knowledge_items: all_knowledge_items,
            project_updates: all_project_updates,
            extraction_time_ms,
        })
    }
}

impl Default for KnowledgeExtractor {
    fn default() -> Self {
        Self::new(ExtractorConfig::default())
    }
}
