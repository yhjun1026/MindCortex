// 知识收集器
// 从 Agent 会话和任务中提取有价值的信息

use super::task::AgentTask;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 收集配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    pub enabled: bool,
    pub auto_collect: bool,
    pub collect_from_tasks: bool,
    pub collect_from_sessions: bool,
    pub max_items_per_run: Option<u32>,
    pub collection_interval_minutes: u64,
    pub filters: CollectionFilters,
}

impl Default for CollectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_collect: true,
            collect_from_tasks: true,
            collect_from_sessions: true,
            max_items_per_run: Some(100),
            collection_interval_minutes: 60,
            filters: CollectionFilters::default(),
        }
    }
}

/// 收集过滤器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionFilters {
    pub min_task_duration_ms: Option<i64>,
    pub exclude_task_types: Vec<String>,
    pub only_successful_tasks: bool,
    pub min_message_length: Option<usize>,
    pub exclude_patterns: Vec<String>,
}

impl Default for CollectionFilters {
    fn default() -> Self {
        Self {
            min_task_duration_ms: Some(1000), // 至少 1 秒
            exclude_task_types: vec![],
            only_successful_tasks: true,
            min_message_length: Some(10),
            exclude_patterns: vec![],
        }
    }
}

/// 知识项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeItem {
    pub id: String,
    pub source: String, // "task" | "session" | "code_analysis"
    pub source_id: String,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub tags: Vec<String>,
    pub category: String, // "solution" | "error" | "pattern" | "documentation"
    pub confidence: f32,
    pub collected_at: i64,
    pub metadata: HashMap<String, String>,
}

/// 收集结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionResult {
    pub items_collected: usize,
    pub items_failed: usize,
    pub duration_ms: i64,
    pub sources_processed: usize,
    pub message: String,
}

/// 知识收集器
pub struct KnowledgeCollector {
    agent_id: String,
    config: CollectionConfig,
    total_collected: usize,
    last_collection_time: Option<i64>,
}

impl KnowledgeCollector {
    pub fn new(agent_id: String, config: CollectionConfig) -> Self {
        Self {
            agent_id,
            config,
            total_collected: 0,
            last_collection_time: None,
        }
    }

    /// 收集知识
    pub async fn collect(&mut self, tasks: &[AgentTask]) -> Result<CollectionResult, String> {
        if !self.config.enabled {
            return Ok(CollectionResult {
                items_collected: 0,
                items_failed: 0,
                duration_ms: 0,
                sources_processed: 0,
                message: "Collection is disabled".to_string(),
            });
        }

        let start_time = std::time::Instant::now();
        let mut items_collected = 0;
        let mut items_failed = 0;
        let mut sources_processed = 0;

        if self.config.collect_from_tasks {
            match self.collect_from_tasks(tasks).await {
                Ok(count) => {
                    items_collected += count;
                    sources_processed += tasks.len();
                }
                Err(e) => {
                    eprintln!("Failed to collect from tasks: {}", e);
                    items_failed += 1;
                }
            }
        }

        let duration_ms = start_time.elapsed().as_millis() as i64;
        self.total_collected += items_collected;
        self.last_collection_time = Some(Utc::now().timestamp());

        Ok(CollectionResult {
            items_collected,
            items_failed,
            duration_ms,
            sources_processed,
            message: format!(
                "Collected {} items from {} sources in {}ms",
                items_collected,
                sources_processed,
                duration_ms
            ),
        })
    }

    /// 从任务收集知识
    async fn collect_from_tasks(&self, tasks: &[AgentTask]) -> Result<usize, String> {
        let mut collected = 0;
        let max_items = self.config.max_items_per_run.unwrap_or(100) as usize;

        for task in tasks.iter() {
            if collected >= max_items {
                break;
            }

            // 过滤任务
            if !self.should_collect_task(task) {
                continue;
            }

            // 提取知识
            if let Some(item) = self.extract_knowledge_from_task(task) {
                // 保存知识项到存储
                // TODO: 实际实现应该保存到数据库或向量存储
                println!("Knowledge item extracted: {}", item.title);
                collected += 1;
            }
        }

        Ok(collected)
    }

    /// 判断是否应该收集该任务
    fn should_collect_task(&self, task: &AgentTask) -> bool {
        // 检查是否只收集成功的任务
        if self.config.filters.only_successful_tasks && !task.is_completed() {
            return false;
        }

        // 检查最小任务持续时间
        if let Some(min_duration) = self.config.filters.min_task_duration_ms {
            if let Some(duration) = task.duration_ms {
                if duration < min_duration {
                    return false;
                }
            } else {
                return false;
            }
        }

        // 检查排除的任务类型
        if self.config.filters.exclude_task_types.contains(&task.task_type) {
            return false;
        }

        true
    }

    /// 从任务提取知识
    fn extract_knowledge_from_task(&self, task: &AgentTask) -> Option<KnowledgeItem> {
        // 分析任务内容
        let payload = &task.payload;
        
        // 尝试提取标题
        let title = payload.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or(&task.task_type)
            .to_string();

        // 尝试提取内容
        let content = payload.get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // 检查最小内容长度
        if let Some(min_length) = self.config.filters.min_message_length {
            if content.len() < min_length {
                return None;
            }
        }

        // 提取标签
        let mut tags = vec![];
        if let Some(task_tags) = payload.get("tags") {
            if let Some(tag_array) = task_tags.as_array() {
                for tag in tag_array {
                    if let Some(tag_str) = tag.as_str() {
                        tags.push(tag_str.to_string());
                    }
                }
            }
        }

        // 自动分类
        let category = self.classify_content(&content);

        // 提取摘要
        let summary = self.extract_summary(&content);

        Some(KnowledgeItem {
            id: uuid::Uuid::new_v4().to_string(),
            source: "task".to_string(),
            source_id: task.id.clone(),
            title,
            content,
            summary,
            tags,
            category,
            confidence: 0.8, // 默认置信度
            collected_at: Utc::now().timestamp(),
            metadata: HashMap::new(),
        })
    }

    /// 分类内容
    fn classify_content(&self, content: &str) -> String {
        let lower = content.to_lowercase();

        // 错误相关
        if lower.contains("error") || lower.contains("exception") || lower.contains("failed") {
            return "error".to_string();
        }

        // 解决方案相关
        if lower.contains("solution") || lower.contains("fix") || lower.contains("resolve") {
            return "solution".to_string();
        }

        // 文档相关
        if lower.contains("document") || lower.contains("readme") || lower.contains("guide") {
            return "documentation".to_string();
        }

        // 模式识别
        if lower.contains("pattern") || lower.contains("approach") || lower.contains("method") {
            return "pattern".to_string();
        }

        "general".to_string()
    }

    /// 提取摘要
    fn extract_summary(&self, content: &str) -> Option<String> {
        // 简单的摘要提取：取前 100 个字符
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return None;
        }

        let summary_len = trimmed.len().min(100);
        let summary = trimmed.chars().take(summary_len).collect::<String>();
        
        if summary.len() < trimmed.len() {
            Some(format!("{}...", summary))
        } else {
            Some(summary)
        }
    }

    /// 获取总收集数量
    pub fn get_total_collected(&self) -> usize {
        self.total_collected
    }

    /// 获取上次收集时间
    pub fn get_last_collection_time(&self) -> Option<i64> {
        self.last_collection_time
    }

    /// 更新配置
    pub fn update_config(&mut self, config: CollectionConfig) {
        self.config = config;
    }

    /// 获取配置
    pub fn get_config(&self) -> &CollectionConfig {
        &self.config
    }
}
