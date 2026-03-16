/**
 * 知识图谱数据
 */

use super::node::{GraphNode, NodeType};
use super::edge::{GraphEdge, EdgeType};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub metadata: GraphMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMetadata {
    pub node_count: usize,
    pub edge_count: usize,
    pub created_at: String,
    pub updated_at: String,
    pub version: String,
}

impl GraphData {
    pub fn new() -> Self {
        let now = chrono::Utc::now().to_rfc3339();

        GraphData {
            nodes: Vec::new(),
            edges: Vec::new(),
            metadata: GraphMetadata {
                node_count: 0,
                edge_count: 0,
                created_at: now.clone(),
                updated_at: now,
                version: "0.1.0".to_string(),
            },
        }
    }

    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.push(node);
        self.metadata.node_count = self.nodes.len();
        self.update_timestamp();
    }

    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.push(edge);
        self.metadata.edge_count = self.edges.len();
        self.update_timestamp();
    }

    pub fn find_node(&self, id: &str) -> Option<&GraphNode> {
        self.nodes.iter().find(|node| node.id == id)
    }

    pub fn find_edges_by_source(&self, source_id: &str) -> Vec<&GraphEdge> {
        self.edges
            .iter()
            .filter(|edge| edge.source == source_id)
            .collect()
    }

    pub fn find_edges_by_target(&self, target_id: &str) -> Vec<&GraphEdge> {
        self.edges
            .iter()
            .filter(|edge| edge.target == target_id)
            .collect()
    }

    pub fn find_neighbors(&self, node_id: &str) -> Vec<&GraphNode> {
        let mut neighbors = Vec::new();

        // 查找从该节点出发的边
        for edge in &self.edges {
            if edge.source == node_id {
                if let Some(node) = self.find_node(&edge.target) {
                    neighbors.push(node);
                }
            } else if edge.target == node_id {
                if let Some(node) = self.find_node(&edge.source) {
                    neighbors.push(node);
                }
            }
        }

        neighbors
    }

    pub fn find_shortest_path(&self, start: &str, end: &str) -> Option<Vec<String>> {
        // 使用 BFS 查找最短路径
        let mut queue: Vec<Vec<String>> = vec![vec![start.to_string()]];
        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
        visited.insert(start.to_string());

        while !queue.is_empty() {
            let path = queue.remove(0);
            let current = path.last()?;

            if *current == end {
                return Some(path);
            }

            // 获取邻居节点
            for edge in &self.edges {
                let neighbor = if edge.source == *current {
                    Some(edge.target.clone())
                } else if edge.target == *current {
                    Some(edge.source.clone())
                } else {
                    None
                };

                if let Some(neighbor_id) = neighbor {
                    if !visited.contains(&neighbor_id) {
                        visited.insert(neighbor_id.clone());
                        let mut new_path = path.clone();
                        new_path.push(neighbor_id);
                        queue.push(new_path);
                    }
                }
            }
        }

        None
    }

    pub fn calculate_degree(&self, node_id: &str) -> usize {
        self.edges
            .iter()
            .filter(|edge| edge.source == node_id || edge.target == node_id)
            .count()
    }

    pub fn calculate_centrality(&self) -> HashMap<String, f64> {
        let mut centrality = HashMap::new();

        for node in &self.nodes {
            let degree = self.calculate_degree(&node.id);
            let total_edges = self.edges.len();

            if total_edges > 0 {
                let value = degree as f64 / total_edges as f64;
                centrality.insert(node.id.clone(), value);
            } else {
                centrality.insert(node.id.clone(), 0.0);
            }
        }

        centrality
    }

    pub fn find_cliques(&self, min_size: usize) -> Vec<Vec<String>> {
        // 简化的团检测算法
        let mut cliques = Vec::new();

        // 对于每个节点，尝试构建团
        for i in 0..self.nodes.len() {
            for j in (i + 1)..self.nodes.len() {
                if self.is_connected(&self.nodes[i].id, &self.nodes[j].id) {
                    let mut clique = vec![self.nodes[i].id.clone(), self.nodes[j].id.clone()];

                    for k in (j + 1)..self.nodes.len() {
                        if clique.iter().all(|node_id| self.is_connected(node_id, &self.nodes[k].id)) {
                            clique.push(self.nodes[k].id.clone());
                        }
                    }

                    if clique.len() >= min_size {
                        cliques.push(clique);
                    }
                }
            }
        }

        cliques
    }

    fn is_connected(&self, node1: &str, node2: &str) -> bool {
        self.edges.iter().any(|edge| {
            (edge.source == node1 && edge.target == node2) ||
            (edge.source == node2 && edge.target == node1)
        })
    }

    pub fn update_timestamp(&mut self) {
        self.metadata.updated_at = chrono::Utc::now().to_rfc3339();
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.metadata.node_count = 0;
        self.metadata.edge_count = 0;
        self.update_timestamp();
    }
}

impl Default for GraphData {
    fn default() -> Self {
        Self::new()
    }
}
