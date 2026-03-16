// 结果排序器
// 负责搜索结果的融合、排序和去重

use super::{HybridSearchResult, ResultSource, ResultType, SearchResultMetadata};
use std::collections::{HashMap, HashSet};

/// 结果排序配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResultRankerConfig {
    /// 结果融合策略
    pub fusion_strategy: FusionStrategy,
    /// 去重策略
    pub deduplicate_strategy: DeduplicateStrategy,
    /// 时间权重（0-1）
    pub time_weight: f64,
    /// 相关性权重（0-1）
    pub relevance_weight: f64,
    /// 来源优先级
    pub source_priority: Vec<String>,
}

impl Default for ResultRankerConfig {
    fn default() -> Self {
        Self {
            fusion_strategy: FusionStrategy::WeightedAverage,
            deduplicate_strategy: DeduplicateStrategy::KeepBestScore,
            time_weight: 0.3,
            relevance_weight: 0.7,
            source_priority: vec![
                "agent_session".to_string(),
                "file".to_string(),
                "memory".to_string(),
            ],
        }
    }
}

/// 结果融合策略
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FusionStrategy {
    /// 加权平均
    WeightedAverage,
    /// 取最大值
    MaxScore,
    /// 取最小值
    MinScore,
    /// 取最新
    Newest,
}

/// 去重策略
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DeduplicateStrategy {
    /// 保留最高分
    KeepBestScore,
    /// 保留最新
    KeepNewest,
    /// 保留最旧
    KeepOldest,
}

/// 结果排序器
pub struct ResultRanker {
    config: ResultRankerConfig,
    source_scores: HashMap<String, f64>,
}

impl ResultRanker {
    /// 创建新的结果排序器
    pub fn new() -> Self {
        Self {
            config: ResultRankerConfig::default(),
            source_scores: HashMap::new(),
        }
    }

    /// 使用自定义配置创建
    pub fn with_config(config: ResultRankerConfig) -> Self {
        Self {
            config,
            source_scores: HashMap::new(),
        }
    }

    /// 更新配置
    pub fn update_config(&mut self, config: ResultRankerConfig) {
        self.config = config;
    }

    /// 设置来源权重
    pub fn set_source_score(&mut self, source: String, score: f64) {
        self.source_scores.insert(source, score);
    }

    /// 融合和排序结果
    pub fn rank_and_fuse(&self, mut results: Vec<HybridSearchResult>) 
        -> Vec<HybridSearchResult> {
        
        // 1. 融合重复结果
        results = self.fuse_results(results);

        // 2. 调整分数
        self.adjust_scores(&mut results);

        // 3. 时间加权
        self.apply_time_weight(&mut results);

        // 4. 来源优先级
        self.apply_source_priority(&mut results);

        // 5. 排序
        self.sort_results(&mut results);

        results
    }

    /// 融合重复结果
    fn fuse_results(&self, mut results: Vec<HybridSearchResult>) 
        -> Vec<HybridSearchResult> {
        
        // 按内容 ID 分组
        let mut grouped: HashMap<String, Vec<HybridSearchResult>> = HashMap::new();
        
        for result in results {
            grouped
                .entry(result.id.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }

        let mut fused_results = vec![];

        for (id, mut group_results) in grouped {
            match &group_results.len() {
                1 => {
                    // 只有一个结果，直接保留
                    fused_results.push(group_results.pop().unwrap());
                }
                _ => {
                    // 多个结果，根据策略融合
                    let fused = self.fuse_duplicate(id, group_results);
                    fused_results.push(fused);
                }
            }
        }

        fused_results
    }

    /// 融并重复结果
    fn fuse_duplicate(&self, _id: String, results: Vec<HybridSearchResult>)
        -> HybridSearchResult {

        match &self.config.deduplicate_strategy {
            DeduplicateStrategy::KeepBestScore => {
                // 保留最高分的结果
                results.into_iter()
                    .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
                    .unwrap()
            }
            DeduplicateStrategy::KeepNewest => {
                // 保留最新的结果
                results.into_iter()
                    .max_by(|a, b| a.metadata.timestamp.partial_cmp(&b.metadata.timestamp).unwrap())
                    .unwrap()
            }
            DeduplicateStrategy::KeepOldest => {
                // 保留最旧的结果
                results.into_iter()
                    .min_by(|a, b| a.metadata.timestamp.partial_cmp(&b.metadata.timestamp).unwrap())
                    .unwrap()
            }
        }
    }

    /// 调整分数
    fn adjust_scores(&self, results: &mut Vec<HybridSearchResult>) {
        // 计算最大和最小分数
        if let (Some(max_score), Some(min_score)) = (
            results.iter().map(|r| r.score).max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
            results.iter().map(|r| r.score).min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
        ) {
            if max_score != min_score {
                let range = max_score - min_score;
                if range > 0.0 {
                    for result in results {
                        // 归一化分数到 [0, 1]
                        let normalized = if range > 0.0 {
                            (result.score - min_score) / range
                        } else {
                            0.5
                        };
                        result.score = normalized;
                    }
                }
            }
        }
    }

    /// 应用时间权重
    fn apply_time_weight(&self, results: &mut Vec<HybridSearchResult>) {
        if self.config.time_weight <= 0.0 {
            return;
        }

        let now = chrono::Utc::now().timestamp();
        
        // 找到最新的时间戳
        let latest_timestamp = results
            .iter()
            .map(|r| r.metadata.timestamp)
            .max()
            .unwrap_or(now);

        // 计算时间衰减
        let time_decay_days = 7; // 7 天
        let time_decay_seconds = time_decay_days * 24 * 3600;
        
        for result in results {
            let age_seconds = now - result.metadata.timestamp;
            let time_factor = if age_seconds < time_decay_seconds {
                1.0 - (age_seconds as f64) / (time_decay_seconds as f64) * 0.5
            } else {
                0.5
            };
            
            let relevance_factor = result.score;
            
            // 融合时间权重和相关性权重
            result.score = (self.config.time_weight * time_factor 
                          + self.config.relevance_weight * relevance_factor) 
                          / (self.config.time_weight + self.config.relevance_weight);
        }
    }

    /// 应用来源优先级
    fn apply_source_priority(&self, results: &mut Vec<HybridSearchResult>) {
        if self.config.source_priority.is_empty() {
            return;
        }

        for result in results {
            // 查找来源优先级
            if let Some(priority) = self.config.source_priority
                .iter()
                .position(|p| result.metadata.source.contains(p)) 
            {
                // 根据优先级调整分数
                let max_priority = self.config.source_priority.len() as f64;
                let priority_weight = 1.0 - (priority as f64) / max_priority;
                result.score *= 1.0 + priority_weight * 0.2;
            }
        }
    }

    /// 排序结果
    fn sort_results(&self, results: &mut Vec<HybridSearchResult>) {
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    }

    /// 获取结果统计
    pub fn get_statistics(&self, results: &[HybridSearchResult]) -> ResultStatistics {
        let total = results.len();
        let mut source_counts: HashMap<String, usize> = HashMap::new();
        let mut type_counts: HashMap<ResultSource, usize> = HashMap::new();

        for result in results {
            *source_counts.entry(result.metadata.source.clone()).or_insert(0) += 1;
            *type_counts.entry(result.source.clone()).or_insert(0) += 1;
        }

        let average_score = if total > 0 {
            results.iter().map(|r| r.score).sum::<f64>() / total as f64
        } else {
            0.0
        };

        ResultStatistics {
            total,
            source_counts,
            type_counts,
            average_score,
        }
    }
}

/// 结果统计信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResultStatistics {
    pub total: usize,
    pub source_counts: HashMap<String, usize>,
    pub type_counts: HashMap<ResultSource, usize>,
    pub average_score: f64,
}

impl Default for ResultRanker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_result_fusion() {
        let ranker = ResultRanker::new();
        
        // 测试重复结果融合
        let mut results = vec![
            HybridSearchResult {
                id: "test1".to_string(),
                content: "Test content".to_string(),
                score: 0.8,
                source: ResultSource::Keyword,
                result_type: ResultType::KeywordMatch,
                metadata: SearchResultMetadata {
                    source: "test".to_string(),
                    timestamp: chrono::Utc::now().timestamp() - 3600, // 1 小时前
                    session_id: None,
                    message_id: None,
                    file_path: None,
                },
            },
            HybridSearchResult {
                id: "test1".to_string(),
                content: "Test content".to_string(),
                score: 0.7,
                source: ResultSource::Semantic,
                result_type: ResultType::SemanticMatch,
                metadata: SearchResultMetadata {
                    source: "test".to_string(),
                    timestamp: chrono::Utc::now().timestamp() - 7200, // 2 小时前
                    session_id: None,
                    message_id: None,
                    file_path: None,
                },
            },
        ];

        let fused = ranker.fuse_results(results);
        assert_eq!(fused.len(), 1); // 应该保留最高分的
        assert_eq!(fused[0].score, 0.8);
        assert_eq!(fused[0].source, ResultSource::Keyword);
    }
    
    #[test]
    fn test_score_normalization() {
        let ranker = ResultRanker::new();
        
        let mut results = vec![
            HybridSearchResult {
                id: "test1".to_string(),
                content: "Test content 1".to_string(),
                score: 0.3,
                source: ResultSource::Keyword,
                result_type: ResultType::KeywordMatch,
                metadata: SearchResultMetadata {
                    source: "test".to_string(),
                    timestamp: chrono::Utc::now().timestamp(),
                    session_id: None,
                    message_id: None,
                    file_path: None,
                },
            },
            HybridSearchResult {
                id: "test2".to_string(),
                content: "Test content 2".to_string(),
                score: 0.9,
                source: ResultSource::Semantic,
                result_type: ResultType::SemanticMatch,
                metadata: SearchResultMetadata {
                    source: "test".to_string(),
                    timestamp: chrono::Utc::now().timestamp(),
                    session_id: None,
                    message_id: None,
                    file_path: None,
                },
            },
        ];

        ranker.adjust_scores(&mut results);
        
        // 分数应该被归一化
        assert!((results[1].score - results[0].score).abs() < 0.1);
    }
}
