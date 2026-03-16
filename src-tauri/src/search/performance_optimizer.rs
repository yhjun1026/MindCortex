// 混合检索性能优化
// 实现性能监控和优化建议

use super::HybridEngine;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// 性能配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceConfig {
    /// 启用并发检索
    pub enable_concurrent_search: bool,
    /// 最大并发数
    pub max_concurrent: usize,
    /// 查询缓存大小
    pub query_cache_size: usize,
    /// �索引预热
    pub enable_index_warmup: bool,
    /// 性能统计周期（秒）
    pub stats_collection_interval: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_concurrent_search: true,
            max_concurrent: 4,
            query_cache_size: 100,
            enable_index_warmup: false,
            stats_collection_interval: 60,
        }
    }
}

/// 性能统计信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceStats {
    pub total_queries: usize,
    pub successful_queries: usize,
    pub failed_queries: usize,
    pub average_query_time_ms: f64,
    pub cache_hits: usize,
    pub concurrent_searches_count: usize,
    pub last_stats_update: i64,
}

/// 性能优化器
pub struct PerformanceOptimizer {
    engine: Arc<HybridEngine>,
    config: PerformanceConfig,
    stats: Arc<std::sync::RwLock<PerformanceStats>>,
    query_cache: Arc<tokio::sync::RwLock<std::collections::HashMap<String, (Vec<super::HybridSearchResult>, Instant)>>>,
}

impl PerformanceOptimizer {
    /// 创建性能优化器
    pub fn new(engine: Arc<HybridEngine>) -> Self {
        Self {
            engine,
            config: PerformanceConfig::default(),
            stats: Arc::new(std::sync::RwLock::new(PerformanceStats {
                total_queries: 0,
                successful_queries: 0,
                failed_queries: 0,
                average_query_time_ms: 0.0,
                cache_hits: 0,
                concurrent_searches_count: 0,
                last_stats_update: chrono::Utc::now().timestamp(),
            })),
            query_cache: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 使用自定义配置
    pub fn with_config(engine: Arc<HybridEngine>, config: PerformanceConfig) -> Self {
        Self {
            engine,
            config,
            stats: Arc::new(std::sync::RwLock::new(PerformanceStats {
                total_queries: 0,
                successful_queries: 0,
                failed_queries: 0,
                average_query_time_ms: 0.0,
                cache_hits: 0,
                concurrent_searches_count: 0,
                last_stats_update: chrono::Utc::now().timestamp(),
            })),
            query_cache: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 执行性能优化的搜索
    pub async fn optimized_search(&self, query: &str) -> Result<Vec<super::HybridSearchResult>, String> {
        let start_time = Instant::now();
        
        // 1. 检查缓存
        if let Some(cached) = self.check_cache(query).await {
            return Ok(cached);
        }

        let mut results = vec![];

        // 2. 根据配置执行搜索
        if self.config.enable_concurrent_search {
            results = self.concurrent_search(query).await?;
        } else {
            results = self.engine.search(query).await?;
        }

        let search_time = start_time.elapsed().as_millis() as f64;

        // 3. 更新缓存
        if !results.is_empty() {
            self.update_cache(query, results.clone()).await;
        }

        // 4. 更新统计信息
        self.update_stats(true, search_time, results.len()).await;

        Ok(results)
    }

    /// 检查查询缓存
    async fn check_cache(&self, query: &str) -> Option<Vec<super::HybridSearchResult>> {
        let mut cache = self.query_cache.lock().await;

        // 清理过期缓存
        self.clean_expired_cache(&mut cache).await;

        cache.get(&query.to_string())
            .map(|(results, _timestamp)| results.clone())
    }

    /// 清理过期缓存
    fn clean_expired_cache(&self, cache: &mut std::collections::HashMap<String, (Vec<super::HybridSearchResult>, Instant)>) {
        let max_cache_size = self.config.query_cache_size;
        
        if cache.len() > max_cache_size {
            // 按访问时间排序，删除最旧的
            let mut entries: Vec<(String, Instant)> = cache
                .iter()
                .map(|(key, (_, time))| (key.clone(), *time))
                .collect();
            
            entries.sort_by(|a, b| a.1.cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            
            // 删除最旧的 20%
            let to_remove = max_cache_size * 20 / 100;
            for (key, _) in entries.into_iter().take(to_remove) {
                cache.remove(&key);
            }
        }
    }

    /// 更新缓存
    async fn update_cache(&self, query: &str, results: Vec<super::HybridSearchResult>) {
        let mut cache = self.query_cache.lock().await;
        cache.insert(query.to_string(), (results, Instant::now()));
    }

    /// 并发搜索
    async fn concurrent_search(&self, query: &str) -> Result<Vec<super::HybridSearchResult>, String> {
        let mut handles = vec![];
        let max_concurrent = self.config.max_concurrent;
        
        // 模拟并发搜索不同类型的搜索
        for i in 0..max_concurrent {
            let engine = self.engine.clone();
            let query = query.to_string();
            
            let handle = tokio::spawn(async move {
                engine.search(&query).await
            });
            
            handles.push(handle);
        }

        // 等待所有搜索完成
        let mut all_results = vec![];
        for handle in handles {
            match handle.await {
                Ok(results) => all_results.extend(results),
                Err(e) => eprintln!("Concurrent search failed: {}", e),
            }
        }

        // 合并和去重
        let deduplicated = self.deduplicate_results(all_results)?;
        
        Ok(deduplicated)
    }

    /// 去重搜索结果
    fn deduplicate_results(&self, mut results: Vec<super::HybridSearchResult>)
        -> Result<Vec<super::HybridSearchResult>, String> {

        results.sort_by_key(|r| r.id.clone());
        
        let mut seen_ids = std::collections::HashSet::new();
        let mut deduplicated = vec![];
        
        for result in results {
            if seen_ids.insert(result.id.clone()) {
                deduplicated.push(result);
            }
        }
        
        Ok(deduplicated)
    }

    /// 更新统计信息
    async fn update_stats(&self, success: bool, search_time_ms: f64, result_count: usize) {
        let mut stats = self.stats.lock().await;

        stats.total_queries += 1;

        if success {
            stats.successful_queries += 1;
        } else {
            stats.failed_queries += 1;
        }

        // 计算平均查询时间（移动平均）
        let current_avg = stats.average_query_time_ms;
        stats.average_query_time_ms = (current_avg * (stats.total_queries - 1) as f64 + search_time_ms) / stats.total_queries as f64;

        stats.last_stats_update = chrono::Utc::now().timestamp();

        // lock 会在作用域结束时自动解锁
        drop(stats);
    }

    /// 获取性能统计
    pub fn get_stats(&self) -> PerformanceStats {
        self.stats.lock().unwrap().clone()
    }

    /// 索引预热（后台任务）
    pub fn start_index_warmup(&self) {
        if !self.config.enable_index_warmup {
            return;
        }

        let engine = self.engine.clone();
        
        tokio::spawn(async move {
            // TODO: 实现索引入温
            // 1. 热取常用查询
            // 2. 预热执行搜索
            // 3. 缓存结果
            println!("Index warmup started");
            tokio::time::sleep(Duration::from_secs(60)).await;
        });
    }

    /// 性能分析
    pub fn analyze_performance(&self) -> PerformanceAnalysis {
        let stats = self.get_stats();
        
        let cache_hit_rate = if stats.total_queries > 0 {
            stats.cache_hits as f64 / stats.total_queries * 100.0
        } else {
            0.0
        };
        
        let success_rate = if stats.total_queries > 0 {
            stats.successful_queries as f64 / stats.total_queries * 100.0
        } else {
            0.0
        };
        
        PerformanceAnalysis {
            total_queries: stats.total_queries,
            successful_queries: stats.successful_queries,
            failed_queries: stats.failed_queries,
            average_query_time_ms: stats.average_query_time_ms,
            cache_hit_rate,
            success_rate,
            performance_rating: self.calculate_performance_rating(&stats),
        }
    }

    /// 计算性能评级
    fn calculate_performance_rating(&self, stats: &PerformanceStats) -> PerformanceRating {
        let success_rate = if stats.total_queries > 0 {
            stats.success_queries as f64 / stats.total_queries * 100.0
        } else {
            100.0
        };
        
        let cache_hit_rate = if stats.total_queries > 0 {
            stats.cache_hits as f64 / stats.total_queries * 100.0
        } else {
            0.0
        };
        
        let avg_time_score = if stats.total_queries > 0 {
            let max_acceptable_time = 1000.0; // 1 秒
            let score = 1.0 - (stats.average_query_time_ms / max_acceptable_time).min(1.0);
            score * 100.0
        } else {
            100.0
        };
        
        // 加权评分
        let overall_rating = (success_rate * 0.5 + cache_hit_rate * 0.3 + avg_time_score * 0.2);

        if overall_rating >= 90.0 {
            PerformanceRating::Excellent
        } else if overall_rating >= 75.0 {
            PerformanceRating::Good
        } else if overall_rating >= 60. {
            PerformanceRating::Fair
        } else if overall_rating >= 40.0 {
            PerformanceRating::Poor
        } else {
            PerformanceRating::VeryPoor
        }
    }
}

/// 性能分析
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceAnalysis {
    pub total_queries: usize,
    pub successful_queries: usize,
    pub failed_queries: usize,
    pub average_query_time_ms: f64,
    pub cache_hit_rate: f64,
    pub success_rate: f64,
    pub performance_rating: PerformanceRating,
}

/// 性能评级
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PerformanceRating {
    #[serde(rename = "excellent")]
    Excellent,
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "fair")]
    Fair,
    #[serde(rename = "poor")]
    Poor,
    #[serde(rename = "very_poor")]
    VeryPoor,
}


