/**
 * 图谱分析器
 * 提供图谱分析功能，如路径发现、关联推荐、聚类分析
 */

use super::graph_data::GraphData;
use serde::{Serialize, Deserialize};

pub struct GraphAnalyzer {
    graph: GraphData,
}

impl GraphAnalyzer {
    pub fn new(graph: GraphData) -> Self {
        GraphAnalyzer { graph }
    }

    /// 查找两个节点之间的所有路径
    pub fn find_all_paths(&self, start: &str, end: &str, max_depth: usize) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        let mut current_path = vec![start.to_string()];
        let mut visited = std::collections::HashSet::new();
        visited.insert(start.to_string());

        self.dfs_find_paths(start, end, max_depth, &mut current_path, &mut visited, &mut paths);

        paths
    }

    fn dfs_find_paths(
        &self,
        current: &str,
        end: &str,
        max_depth: usize,
        current_path: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
        paths: &mut Vec<Vec<String>>,
    ) {
        if current_path.len() > max_depth + 1 {
            return;
        }

        if current == end {
            paths.push(current_path.clone());
            return;
        }

        for edge in &self.graph.edges {
            let neighbor = if edge.source == current {
                Some(edge.target.clone())
            } else if edge.target == current {
                Some(edge.source.clone())
            } else {
                None
            };

            if let Some(neighbor_id) = neighbor {
                if !visited.contains(&neighbor_id) {
                    visited.insert(neighbor_id.clone());
                    current_path.push(neighbor_id.clone());

                    self.dfs_find_paths(&neighbor_id, end, max_depth, current_path, visited, paths);

                    current_path.pop();
                    visited.remove(&neighbor_id);
                }
            }
        }
    }

    /// 查找节点的关联推荐
    pub fn find_related_nodes(&self, node_id: &str, limit: usize) -> Vec<(String, f64)> {
        let mut scores = std::collections::HashMap::new();

        // 获取直接连接的节点
        let neighbors = self.graph.find_neighbors(node_id);

        for neighbor in &neighbors {
            let base_score = 0.8;

            // 查找邻居的邻居（二度关联）
            let neighbors_of_neighbor = self.graph.find_neighbors(&neighbor.id);
            for n2n in &neighbors_of_neighbor {
                if n2n.id != node_id && n2n.id != neighbor.id {
                    let entry = scores.entry(n2n.id.clone()).or_insert(0.0);
                    *entry += base_score * 0.5;
                }
            }

            // 添加直接连接的节点
            scores.entry(neighbor.id.clone()).or_insert(0.0);
        }

        // 按分数排序
        let mut mut_scores: Vec<(String, f64)> = scores.into_iter().collect();
        mut_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        mut_scores.truncate(limit);
        mut_scores
    }

    /// 聚类分析（简化版 K-Means）
    pub fn cluster_nodes(&self, k: usize) -> Vec<Vec<String>> {
        if self.graph.nodes.is_empty() || k == 0 {
            return Vec::new();
        }

        let node_count = self.graph.nodes.len();
        let cluster_size = (node_count + k - 1) / k;

        self.graph
            .nodes
            .chunks(cluster_size)
            .map(|chunk| chunk.iter().map(|node| node.id.clone()).collect())
            .collect()
    }

    /// 计算图谱密度
    pub fn calculate_density(&self) -> f64 {
        let node_count = self.graph.nodes.len();
        let edge_count = self.graph.edges.len();

        if node_count < 2 {
            return 0.0;
        }

        let max_possible_edges = node_count * (node_count - 1) / 2;
        if max_possible_edges == 0 {
            return 0.0;
        }

        edge_count as f64 / max_possible_edges as f64
    }

    /// 查找桥接节点（删除后会增加连通分量的节点）
    pub fn find_bridge_nodes(&self) -> Vec<String> {
        let mut bridges = Vec::new();

        for node in &self.graph.nodes {
            if self.is_bridge_node(&node.id) {
                bridges.push(node.id.clone());
            }
        }

        bridges
    }

    fn is_bridge_node(&self, node_id: &str) -> bool {
        // 简化的桥接节点检测
        let degree = self.graph.calculate_degree(node_id);
        degree > 1 && degree < self.graph.nodes.len() / 2
    }

    /// 查找关键节点（高中心性节点）
    pub fn find_key_nodes(&self, top_n: usize) -> Vec<(String, f64)> {
        let centrality = self.graph.calculate_centrality();
        let mut mut_centrality: Vec<(String, f64)> = centrality.into_iter().collect();
        mut_centrality.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        mut_centrality.truncate(top_n);
        mut_centrality
    }

    /// 计算子图
    pub fn extract_subgraph(&self, node_ids: &[String]) -> GraphData {
        let node_set: std::collections::HashSet<String> = node_ids.iter().cloned().collect();

        let filtered_nodes: Vec<_> = self.graph.nodes.iter()
            .filter(|node| node_set.contains(&node.id))
            .cloned()
            .collect();

        let filtered_edges: Vec<_> = self.graph.edges.iter()
            .filter(|edge| node_set.contains(&edge.source) && node_set.contains(&edge.target))
            .cloned()
            .collect();

        GraphData {
            nodes: filtered_nodes,
            edges: filtered_edges,
            metadata: self.graph.metadata.clone(),
        }
    }

    /// 查找共同的邻居
    pub fn find_common_neighbors(&self, node1: &str, node2: &str) -> Vec<String> {
        let neighbors1: std::collections::HashSet<String> = self.graph.find_neighbors(node1)
            .iter()
            .map(|node| node.id.clone())
            .collect();

        let neighbors2: std::collections::HashSet<String> = self.graph.find_neighbors(node2)
            .iter()
            .map(|node| node.id.clone())
            .collect();

        neighbors1
            .intersection(&neighbors2)
            .cloned()
            .collect()
    }

    /// 计算节点相似度
    pub fn calculate_similarity(&self, node1: &str, node2: &str) -> f64 {
        // 使用 Jaccard 相似度
        let neighbors1: std::collections::HashSet<String> = self.graph.find_neighbors(node1)
            .iter()
            .map(|node| node.id.clone())
            .collect();

        let neighbors2: std::collections::HashSet<String> = self.graph.find_neighbors(node2)
            .iter()
            .map(|node| node.id.clone())
            .collect();

        let intersection = neighbors1.intersection(&neighbors2).count();
        let union = neighbors1.union(&neighbors2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// 查找相似节点
    pub fn find_similar_nodes(&self, node_id: &str, limit: usize) -> Vec<(String, f64)> {
        let mut similarities = Vec::new();

        for node in &self.graph.nodes {
            if node.id != node_id {
                let similarity = self.calculate_similarity(node_id, &node.id);
                if similarity > 0.0 {
                    similarities.push((node.id.clone(), similarity));
                }
            }
        }

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.truncate(limit);
        similarities
    }

    /// 获取图谱统计信息
    pub fn get_statistics(&self) -> GraphStatistics {
        let node_types: std::collections::HashMap<String, usize> = self.graph.nodes.iter()
            .map(|node| {
                let (t1, t2) = match &node.node_type {
                    super::node::NodeType::Code => ("code", None),
                    super::node::NodeType::Document => ("document", None),
                    super::node::NodeType::Concept => ("concept", None),
                    super::node::NodeType::Person => ("person", None),
                    super::node::NodeType::Organization => ("organization", None),
                    super::node::NodeType::Location => ("location", None),
                    super::node::NodeType::Event => ("event", None),
                    super::node::NodeType::Other(s) => ("other", Some(s)),
                };
                (t1.to_string(), t2)
            })
            .fold(std::collections::HashMap::new(), |mut acc, (t1, t2)| {
                *acc.entry(t1).or_insert(0) += 1;
                if let Some(t2) = t2 {
                    *acc.entry(format!("{}::{}", "other", t2)).or_insert(0) += 1;
                }
                acc
            });

        let edge_types: std::collections::HashMap<String, usize> = self.graph.edges.iter()
            .map(|edge| {
                let t = match &edge.edge_type {
                    super::edge::EdgeType::References => "references",
                    super::edge::EdgeType::Implements => "implements",
                    super::edge::EdgeType::Extends => "extends",
                    super::edge::EdgeType::Uses => "uses",
                    super::edge::EdgeType::Defines => "defines",
                    super::edge::EdgeType::Contains => "contains",
                    super::edge::EdgeType::RelatedTo => "related_to",
                    super::edge::EdgeType::Mentions => "mentions",
                    super::edge::EdgeType::AuthorOf => "author_of",
                    super::edge::EdgeType::BelongsTo => "belongs_to",
                    super::edge::EdgeType::LocatedIn => "located_in",
                    super::edge::EdgeType::Other(s) => s,
                };
                t.to_string()
            })
            .fold(std::collections::HashMap::new(), |mut acc, t| {
                *acc.entry(t).or_insert(0) += 1;
                acc
            });

        let degrees: Vec<usize> = self.graph.nodes.iter()
            .map(|node| self.graph.calculate_degree(&node.id))
            .collect();

        let avg_degree = if !degrees.is_empty() {
            degrees.iter().sum::<usize>() as f64 / degrees.len() as f64
        } else {
            0.0
        };

        let max_degree = degrees.iter().copied().max().unwrap_or(0);
        let min_degree = degrees.iter().copied().min().unwrap_or(0);

        GraphStatistics {
            node_count: self.graph.metadata.node_count,
            edge_count: self.graph.metadata.edge_count,
            node_types,
            edge_types,
            density: self.calculate_density(),
            avg_degree,
            max_degree,
            min_degree,
            connected_components: self.count_connected_components(),
        }
    }

    fn count_connected_components(&self) -> usize {
        let mut visited = std::collections::HashSet::new();
        let mut components = 0;

        for node in &self.graph.nodes {
            if !visited.contains(&node.id) {
                components += 1;
                self.bfs_visit(&node.id, &mut visited);
            }
        }

        components
    }

    fn bfs_visit(&self, start: &str, visited: &mut std::collections::HashSet<String>) {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        while let Some(current) = queue.pop_front() {
            for edge in &self.graph.edges {
                let neighbor = if edge.source == current {
                    Some(edge.target.clone())
                } else if edge.target == current {
                    Some(edge.source.clone())
                } else {
                    None
                };

                if let Some(neighbor_id) = neighbor {
                    if !visited.contains(&neighbor_id) {
                        visited.insert(neighbor_id.clone());
                        queue.push_back(neighbor_id);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStatistics {
    pub node_count: usize,
    pub edge_count: usize,
    pub node_types: std::collections::HashMap<String, usize>,
    pub edge_types: std::collections::HashMap<String, usize>,
    pub density: f64,
    pub avg_degree: f64,
    pub max_degree: usize,
    pub min_degree: usize,
    pub connected_components: usize,
}
