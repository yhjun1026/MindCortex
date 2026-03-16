/**
 * 知识图谱构建器
 * 从代码和文档中提取实体和关系，构建知识图谱
 */

use super::graph_data::GraphData;
use super::node::{GraphNode, NodeType};
use super::edge::{GraphEdge, EdgeType};
use std::collections::HashMap;

pub struct GraphBuilder {
    graph: GraphData,
    node_index: HashMap<String, String>, // label -> id
    id_counter: usize,
}

impl GraphBuilder {
    pub fn new() -> Self {
        GraphBuilder {
            graph: GraphData::new(),
            node_index: HashMap::new(),
            id_counter: 0,
        }
    }

    /// 从代码分析结果创建节点
    pub fn add_code_node(
        &mut self,
        name: &str,
        file_path: &str,
        node_type: NodeType,
        properties: serde_json::Value,
    ) -> &mut Self {
        let id = format!("code_{}", self.id_counter);
        self.id_counter += 1;

        let node = GraphNode {
            id: id.clone(),
            label: name.to_string(),
            node_type,
            properties,
            x: None,
            y: None,
        };

        self.graph.add_node(node);
        self.node_index.insert(name.to_string(), id);

        self
    }

    /// 添加文档节点
    pub fn add_document_node(
        &mut self,
        title: &str,
        path: &str,
        content: &str,
        properties: serde_json::Value,
    ) -> &mut Self {
        let id = format!("doc_{}", self.id_counter);
        self.id_counter += 1;

        let node = GraphNode {
            id: id.clone(),
            label: title.to_string(),
            node_type: NodeType::Document,
            properties,
            x: None,
            y: None,
        };

        self.graph.add_node(node);
        self.node_index.insert(title.to_string(), id);

        self
    }

    /// 添加概念节点
    pub fn add_concept_node(
        &mut self,
        concept: &str,
        description: &str,
        properties: serde_json::Value,
    ) -> &mut Self {
        let id = format!("concept_{}", self.id_counter);
        self.id_counter += 1;

        let node = GraphNode {
            id: id.clone(),
            label: concept.to_string(),
            node_type: NodeType::Concept,
            properties,
            x: None,
            y: None,
        };

        self.graph.add_node(node);
        self.node_index.insert(concept.to_string(), id);

        self
    }

    /// 添加边
    pub fn add_edge(
        &mut self,
        source_label: &str,
        target_label: &str,
        edge_type: EdgeType,
        weight: Option<f64>,
        properties: Option<serde_json::Value>,
    ) -> &mut Self {
        if let (Some(source_id), Some(target_id)) = (
            self.node_index.get(source_label),
            self.node_index.get(target_label),
        ) {
            let edge_id = format!("edge_{}_{}_{}", source_id, target_id, self.id_counter);
            self.id_counter += 1;

            let edge = GraphEdge {
                id: edge_id,
                source: source_id.clone(),
                target: target_id.clone(),
                edge_type,
                label: None,
                weight,
                properties,
            };

            self.graph.add_edge(edge);
        }

        self
    }

    /// 从代码导入关系创建边
    pub fn add_import_edge(&mut self, source: &str, target: &str) -> &mut Self {
        self.add_edge(
            source,
            target,
            EdgeType::Uses,
            Some(EdgeType::Uses.default_weight()),
            None,
        )
    }

    /// 从类继承关系创建边
    pub fn add_inheritance_edge(&mut self, child: &str, parent: &str) -> &mut Self {
        self.add_edge(
            child,
            parent,
            EdgeType::Extends,
            Some(EdgeType::Extends.default_weight()),
            None,
        )
    }

    /// 从接口实现关系创建边
    pub fn add_implementation_edge(&mut self, class: &str, interface: &str) -> &mut Self {
        self.add_edge(
            class,
            interface,
            EdgeType::Implements,
            Some(EdgeType::Implements.default_weight()),
            None,
        )
    }

    /// 从函数调用关系创建边
    pub fn add_call_edge(&mut self, caller: &str, callee: &str) -> &mut Self {
        self.add_edge(
            caller,
            callee,
            EdgeType::References,
            Some(EdgeType::References.default_weight()),
            None,
        )
    }

    /// 从文档引用关系创建边
    pub fn add_reference_edge(&mut self, source: &str, target: &str) -> &mut Self {
        self.add_edge(
            source,
            target,
            EdgeType::References,
            Some(EdgeType::References.default_weight()),
            None,
        )
    }

    /// 分析代码并提取实体
    pub fn analyze_code(&mut self, code: &str, language: &str) -> &mut Self {
        match language {
            "typescript" | "javascript" => self.analyze_javascript_code(code),
            "rust" => self.analyze_rust_code(code),
            "python" => self.analyze_python_code(code),
            _ => self,
        }
    }

    fn analyze_javascript_code(&mut self, code: &str) -> &mut Self {
        // 简化的 JavaScript/TypeScript 代码分析
        // 提取类、函数、导入等

        // 提取类定义
        let class_regex = regex::Regex::new(r"class\s+(\w+)").unwrap();
        for cap in class_regex.captures_iter(code) {
            if let Some(class_name) = cap.get(1) {
                self.add_code_node(
                    class_name.as_str(),
                    "",
                    NodeType::Code,
                    serde_json::json!({
                        "language": "typescript",
                        "type": "class"
                    }),
                );
            }
        }

        // 提取函数定义
        let func_regex = regex::Regex::new(r"(?:function\s+(\w+)|(?:const|let|var)\s+(\w+)\s*=\s*(?:async\s+)?\()").unwrap();
        for cap in func_regex.captures_iter(code) {
            let func_name = cap.get(1).or(cap.get(2));
            if let Some(name) = func_name {
                self.add_code_node(
                    name.as_str(),
                    "",
                    NodeType::Code,
                    serde_json::json!({
                        "language": "typescript",
                        "type": "function"
                    }),
                );
            }
        }

        // 提取 import 语句
        let import_regex = regex::Regex::new(r#"import\s+(?:\{[^}]*\}|\w+)\s+from\s+['"]([^'"]+)['"]"#).unwrap();
        for cap in import_regex.captures_iter(code) {
            if let Some(module_name) = cap.get(1) {
                let module_id = module_name.as_str().replace('/', "::");
                self.add_code_node(
                    &module_id,
                    module_name.as_str(),
                    NodeType::Code,
                    serde_json::json!({
                        "language": "typescript",
                        "type": "module"
                    }),
                );
            }
        }

        self
    }

    fn analyze_rust_code(&mut self, code: &str) -> &mut Self {
        // 简化的 Rust 代码分析

        // 提取结构体
        let struct_regex = regex::Regex::new(r"pub\s+struct\s+(\w+)").unwrap();
        for cap in struct_regex.captures_iter(code) {
            if let Some(struct_name) = cap.get(1) {
                self.add_code_node(
                    struct_name.as_str(),
                    "",
                    NodeType::Code,
                    serde_json::json!({
                        "language": "rust",
                        "type": "struct"
                    }),
                );
            }
        }

        // 提取函数
        let func_regex = regex::Regex::new(r"pub\s+fn\s+(\w+)").unwrap();
        for cap in func_regex.captures_iter(code) {
            if let Some(func_name) = cap.get(1) {
                self.add_code_node(
                    func_name.as_str(),
                    "",
                    NodeType::Code,
                    serde_json::json!({
                        "language": "rust",
                        "type": "function"
                    }),
                );
            }
        }

        // 提取 use 语句
        let use_regex = regex::Regex::new(r"use\s+([^;]+);").unwrap();
        for cap in use_regex.captures_iter(code) {
            if let Some(module_path) = cap.get(1) {
                let module_id = module_path.as_str().replace("::", ".");
                self.add_code_node(
                    &module_id,
                    module_path.as_str(),
                    NodeType::Code,
                    serde_json::json!({
                        "language": "rust",
                        "type": "module"
                    }),
                );
            }
        }

        self
    }

    fn analyze_python_code(&mut self, code: &str) -> &mut Self {
        // 简化的 Python 代码分析

        // 提取类定义
        let class_regex = regex::Regex::new(r"class\s+(\w+)").unwrap();
        for cap in class_regex.captures_iter(code) {
            if let Some(class_name) = cap.get(1) {
                self.add_code_node(
                    class_name.as_str(),
                    "",
                    NodeType::Code,
                    serde_json::json!({
                        "language": "python",
                        "type": "class"
                    }),
                );
            }
        }

        // 提取函数定义
        let func_regex = regex::Regex::new(r"def\s+(\w+)").unwrap();
        for cap in func_regex.captures_iter(code) {
            if let Some(func_name) = cap.get(1) {
                self.add_code_node(
                    func_name.as_str(),
                    "",
                    NodeType::Code,
                    serde_json::json!({
                        "language": "python",
                        "type": "function"
                    }),
                );
            }
        }

        // 提取 import 语句
        let import_regex = regex::Regex::new(r"(?:from\s+(\w+)\s+import|import\s+(\w+))").unwrap();
        for cap in import_regex.captures_iter(code) {
            let module_name = cap.get(1).or(cap.get(2));
            if let Some(name) = module_name {
                self.add_code_node(
                    name.as_str(),
                    "",
                    NodeType::Code,
                    serde_json::json!({
                        "language": "python",
                        "type": "module"
                    }),
                );
            }
        }

        self
    }

    /// 分析文档并提取实体
    pub fn analyze_document(&mut self, content: &str, title: &str) -> &mut Self {
        // 添加文档节点
        self.add_document_node(
            title,
            "",
            content,
            serde_json::json!({
                "content_length": content.len(),
                "word_count": content.split_whitespace().count()
            }),
        );

        // 提取关键词作为概念节点
        let keywords = self.extract_keywords(content);
        for keyword in keywords {
            self.add_concept_node(
                &keyword,
                "",
                serde_json::json!({
                    "source": "document",
                    "source_document": title
                }),
            );

            // 添加文档到概念的边
            self.add_reference_edge(title, &keyword);
        }

        self
    }

    fn extract_keywords(&self, text: &str) -> Vec<String> {
        // 简化的关键词提取
        let words: Vec<String> = text
            .split_whitespace()
            .map(|word| {
                word.chars()
                    .filter(|c| c.is_alphanumeric() || *c == '_')
                    .collect::<String>()
            })
            .filter(|word| word.len() > 3)
            .collect();

        words
    }

    /// 构建并返回图谱
    pub fn build(self) -> GraphData {
        self.graph
    }

    /// 获取当前图谱
    pub fn get_graph(&self) -> &GraphData {
        &self.graph
    }

    /// 清空图谱
    pub fn clear(&mut self) {
        self.graph = GraphData::new();
        self.node_index.clear();
        self.id_counter = 0;
    }
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}
