// Agent 监控器
// 监控 Agent 的资源使用、成本和性能

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};

/// 资源指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub agent_id: String,
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: f64,
    pub memory_usage_percent: f64,
    pub disk_io_read_mb: f64,
    pub disk_io_write_mb: f64,
    pub network_bytes_sent: u64,
    pub network_bytes_received: u64,
    pub active_connections: u32,
    pub timestamp: i64,
}

impl ResourceMetrics {
    pub fn new(agent_id: String) -> Self {
        Self {
            agent_id,
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0.0,
            memory_usage_percent: 0.0,
            disk_io_read_mb: 0.0,
            disk_io_write_mb: 0.0,
            network_bytes_sent: 0,
            network_bytes_received: 0,
            active_connections: 0,
            timestamp: Utc::now().timestamp(),
        }
    }
}

/// 成本指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostMetrics {
    pub agent_id: String,
    pub total_cost: f64,
    pub api_call_count: u32,
    pub token_usage: TokenUsage,
    pub model_costs: std::collections::HashMap<String, f64>, // model_name -> cost
    pub daily_costs: Vec<DailyCost>,
    pub timestamp: i64,
}

impl CostMetrics {
    pub fn new(agent_id: String) -> Self {
        Self {
            agent_id,
            total_cost: 0.0,
            api_call_count: 0,
            token_usage: TokenUsage::default(),
            model_costs: std::collections::HashMap::new(),
            daily_costs: vec![],
            timestamp: Utc::now().timestamp(),
        }
    }

    pub fn add_cost(&mut self, model: &str, input_tokens: u32, output_tokens: u32, cost: f64) {
        self.total_cost += cost;
        self.api_call_count += 1;
        
        self.token_usage.input_tokens += input_tokens;
        self.token_usage.output_tokens += output_tokens;
        self.token_usage.total_tokens += input_tokens + output_tokens;
        
        *self.model_costs.entry(model.to_string()).or_insert(0.0) += cost;
        
        self.timestamp = Utc::now().timestamp();
        
        // 更新每日成本
        self.update_daily_cost(cost);
    }

    fn update_daily_cost(&mut self, cost: f64) {
        let today = Utc::now().format("%Y-%m-%d").to_string();
        
        if let Some(daily_cost) = self.daily_costs.iter_mut().find(|d| d.date == today) {
            daily_cost.cost += cost;
        } else {
            self.daily_costs.push(DailyCost {
                date: today,
                cost,
            });
        }
    }
}

/// Token 使用统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}

impl Default for TokenUsage {
    fn default() -> Self {
        Self {
            input_tokens: 0,
            output_tokens: 0,
            total_tokens: 0,
        }
    }
}

/// 每日成本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCost {
    pub date: String,
    pub cost: f64,
}

/// 性能指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub agent_id: String,
    pub task_count: u32,
    pub success_count: u32,
    pub failure_count: u32,
    pub average_task_duration_ms: f64,
    pub max_task_duration_ms: i64,
    pub min_task_duration_ms: i64,
    pub throughput_tasks_per_minute: f64,
    pub timestamp: i64,
}

impl PerformanceMetrics {
    pub fn new(agent_id: String) -> Self {
        Self {
            agent_id,
            task_count: 0,
            success_count: 0,
            failure_count: 0,
            average_task_duration_ms: 0.0,
            max_task_duration_ms: 0,
            min_task_duration_ms: i64::MAX,
            throughput_tasks_per_minute: 0.0,
            timestamp: Utc::now().timestamp(),
        }
    }

    pub fn record_task(&mut self, duration_ms: i64, success: bool) {
        self.task_count += 1;
        
        if success {
            self.success_count += 1;
        } else {
            self.failure_count += 1;
        }

        // 更新持续时间统计
        self.max_task_duration_ms = self.max_task_duration_ms.max(duration_ms);
        self.min_task_duration_ms = self.min_task_duration_ms.min(duration_ms);

        // 计算平均持续时间
        if self.task_count > 0 {
            let total = self.average_task_duration_ms * (self.task_count - 1) as f64;
            self.average_task_duration_ms = (total + duration_ms as f64) / self.task_count as f64;
        }

        self.timestamp = Utc::now().timestamp();
    }
}

/// 监控器
pub struct AgentMonitor {
    agent_id: String,
    is_monitoring: bool,
    resource_metrics: Arc<Mutex<ResourceMetrics>>,
    cost_metrics: Arc<Mutex<CostMetrics>>,
    performance_metrics: Arc<Mutex<PerformanceMetrics>>,
    task_durations: Arc<Mutex<Vec<i64>>>, // 用于计算吞吐量
    monitoring_start_time: Option<i64>,
}

impl AgentMonitor {
    pub fn new(agent_id: String) -> Self {
        Self {
            agent_id,
            is_monitoring: false,
            resource_metrics: Arc::new(Mutex::new(ResourceMetrics::new(agent_id.clone()))),
            cost_metrics: Arc::new(Mutex::new(CostMetrics::new(agent_id.clone()))),
            performance_metrics: Arc::new(Mutex::new(PerformanceMetrics::new(agent_id.clone()))),
            task_durations: Arc::new(Mutex::new(Vec::new())),
            monitoring_start_time: None,
        }
    }

    /// 开始监控
    pub fn start(&mut self) {
        if self.is_monitoring {
            return;
        }

        self.is_monitoring = true;
        self.monitoring_start_time = Some(Utc::now().timestamp());
    }

    /// 停止监控
    pub fn stop(&mut self) {
        self.is_monitoring = false;
        self.monitoring_start_time = None;
    }

    /// 记录任务开始
    pub fn start_task(&self, task_id: String) {
        // 记录任务开始时间
        // TODO: 实现任务开始时间跟踪
    }

    /// 记录任务完成
    pub fn record_task(&self, duration_ms: i64, success: bool) {
        {
            let mut perf = self.performance_metrics.lock().unwrap();
            perf.record_task(duration_ms, success);
        }

        {
            let mut durations = self.task_durations.lock().unwrap();
            durations.push(duration_ms);
        }
    }

    /// 记录 API 调用成本
    pub fn record_api_call(&self, model: &str, input_tokens: u32, output_tokens: u32, cost: f64) {
        let mut cost_metrics = self.cost_metrics.lock().unwrap();
        cost_metrics.add_cost(model, input_tokens, output_tokens, cost);
    }

    /// 更新资源指标
    pub fn update_resource_metrics(&self) {
        // 实际实现：从系统获取真实的资源使用情况
        // 这里是示例实现
        let mut metrics = self.resource_metrics.lock().unwrap();
        
        // 模拟资源使用数据
        metrics.cpu_usage_percent = 20.0 + (rand::random::<f64>() * 30.0);
        metrics.memory_usage_mb = 100.0 + (rand::random::<f64>() * 200.0);
        metrics.memory_usage_percent = 10.0 + (rand::random::<f64>() * 20.0);
        metrics.timestamp = Utc::now().timestamp();
    }

    /// 获取资源指标
    pub fn get_metrics(&self) -> Option<ResourceMetrics> {
        self.resource_metrics.lock().ok().map(|m| m.clone())
    }

    /// 获取成本指标
    pub fn get_cost_metrics(&self) -> Option<CostMetrics> {
        self.cost_metrics.lock().ok().map(|m| m.clone())
    }

    /// 获取性能指标
    pub fn get_performance_metrics(&self) -> Option<PerformanceMetrics> {
        self.performance_metrics.lock().ok().map(|m| m.clone())
    }

    /// 获取总成本
    pub fn get_total_cost(&self) -> f64 {
        self.cost_metrics
            .lock()
            .ok()
            .map(|m| m.total_cost)
            .unwrap_or(0.0)
    }

    /// 获取总持续时间
    pub fn get_total_duration(&self) -> i64 {
        self.task_durations
            .lock()
            .ok()
            .map(|d| d.iter().sum())
            .unwrap_or(0)
    }

    /// 获取最后活动时间
    pub fn get_last_activity(&self) -> Option<i64> {
        self.resource_metrics
            .lock()
            .ok()
            .and_then(|m| if m.timestamp > 0 { Some(m.timestamp) } else { None })
    }

    /// 计算吞吐量
    pub fn calculate_throughput(&self) -> f64 {
        let durations = self.task_durations.lock().ok()?;
        
        if durations.is_empty() {
            return 0.0;
        }

        if let Some(start_time) = self.monitoring_start_time {
            let now = Utc::now().timestamp();
            let duration_minutes = (now - start_time) as f64 / 60.0;
            
            if duration_minutes > 0.0 {
                return durations.len() as f64 / duration_minutes;
            }
        }

        0.0
    }

    /// 重置指标
    pub fn reset_metrics(&self) {
        let agent_id = self.agent_id.clone();
        
        *self.resource_metrics.lock().unwrap() = ResourceMetrics::new(agent_id.clone());
        *self.cost_metrics.lock().unwrap() = CostMetrics::new(agent_id.clone());
        *self.performance_metrics.lock().unwrap() = PerformanceMetrics::new(agent_id.clone());
        *self.task_durations.lock().unwrap() = Vec::new();
    }

    /// 导出监控报告
    pub fn export_report(&self) -> serde_json::Value {
        let resource = self.get_metrics();
        let cost = self.get_cost_metrics();
        let performance = self.get_performance_metrics();

        serde_json::json!({
            "agent_id": self.agent_id,
            "is_monitoring": self.is_monitoring,
            "monitoring_start_time": self.monitoring_start_time,
            "resource_metrics": resource,
            "cost_metrics": cost,
            "performance_metrics": performance,
            "throughput_tasks_per_minute": self.calculate_throughput(),
            "exported_at": Utc::now().timestamp(),
        })
    }
}
