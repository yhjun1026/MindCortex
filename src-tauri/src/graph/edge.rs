/**
 * 知识图谱边
 */

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub edge_type: EdgeType,
    pub label: Option<String>,
    pub weight: Option<f64>,
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EdgeType {
    References,
    Implements,
    Extends,
    Uses,
    Defines,
    Contains,
    RelatedTo,
    Mentions,
    AuthorOf,
    BelongsTo,
    LocatedIn,
    Other(String),
}

impl EdgeType {
    pub fn default_weight(&self) -> f64 {
        match self {
            EdgeType::References => 0.8,
            EdgeType::Implements => 0.9,
            EdgeType::Extends => 0.85,
            EdgeType::Uses => 0.7,
            EdgeType::Defines => 0.75,
            EdgeType::Contains => 0.6,
            EdgeType::RelatedTo => 0.5,
            EdgeType::Mentions => 0.4,
            EdgeType::AuthorOf => 0.8,
            EdgeType::BelongsTo => 0.6,
            EdgeType::LocatedIn => 0.5,
            EdgeType::Other(_) => 0.3,
        }
    }
}

impl Default for EdgeType {
    fn default() -> Self {
        EdgeType::Other(String::from("unknown"))
    }
}
