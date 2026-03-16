/**
 * 知识图谱节点
 */

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub node_type: NodeType,
    pub properties: serde_json::Value,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Code,
    Document,
    Concept,
    Person,
    Organization,
    Location,
    Event,
    Other(String),
}

impl NodeType {
    pub fn color(&self) -> String {
        match self {
            NodeType::Code => "#4F46E5".to_string(),      // 靛蓝
            NodeType::Document => "#10B981".to_string(),   // 绿色
            NodeType::Concept => "#F59E0B".to_string(),     // 橙色
            NodeType::Person => "#EF4444".to_string(),      // 红色
            NodeType::Organization => "#8B5CF6".to_string(), // 紫色
            NodeType::Location => "#06B6D4".to_string(),   // 青色
            NodeType::Event => "#EC4899".to_string(),       // 粉色
            NodeType::Other(_) => "#6B7280".to_string(),    // 灰色
        }
    }
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Other(String::from("unknown"))
    }
}
