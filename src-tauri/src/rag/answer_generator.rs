/**
 * Answer Generator
 * 基于检索的上下文和查询，生成答案
 */

use super::query_parser::ParsedQuery;
use super::context_builder::QueryContext;
use super::llm_integration::{LLMIntegration, LLMMessage};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedAnswer {
    pub query: String,
    pub answer: String,
    pub citations: Vec<Citation>,
    pub confidence: f64,
    pub metadata: AnswerMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub document_id: String,
    pub title: String,
    pub snippet: String,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerMetadata {
    pub generation_time_ms: u64,
    pub context_size: usize,
    pub model_used: String,
    pub tokens_used: usize,
}

pub struct AnswerGenerator {
    llm: LLMIntegration,
    max_context_length: usize,
}

impl AnswerGenerator {
    pub fn new() -> Self {
        AnswerGenerator {
            llm: LLMIntegration::new(),
            max_context_length: 4000,
        }
    }

    pub fn set_llm_config(&mut self, config: super::llm_integration::LLMConfig) {
        self.llm.set_config(config);
    }

    pub async fn generate_answer(
        &self,
        query: &ParsedQuery,
        context: &QueryContext,
    ) -> Result<GeneratedAnswer, anyhow::Error> {
        let start_time = std::time::Instant::now();

        // 构建提示词
        let system_prompt = self.build_system_prompt(query);
        let user_prompt = self.build_user_prompt(query, context);

        // 准备消息
        let messages = vec![
            LLMMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            LLMMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        // 调用 LLM 生成答案
        let response = self.llm.generate(messages).await?;

        // 提取引用
        let citations = self.extract_citations(context);

        // 计算置信度
        let confidence = self.calculate_confidence(&response, context);

        let generation_time = start_time.elapsed().as_millis() as u64;

        Ok(GeneratedAnswer {
            query: query.original.clone(),
            answer: response.content,
            citations,
            confidence,
            metadata: AnswerMetadata {
                generation_time_ms: generation_time,
                context_size: self.calculate_context_size(context),
                model_used: response.model,
                tokens_used: response.usage.total_tokens,
            },
        })
    }

    fn build_system_prompt(&self, query: &ParsedQuery) -> String {
        let base_prompt = "You are a helpful AI assistant for a knowledge management system. \
        Your task is to answer questions based on the provided context from code and documents.";

        let instruction = match &query.intent {
            super::query_parser::QueryIntent::Search => {
                "Provide relevant search results and explain what was found."
            },
            super::query_parser::QueryIntent::Explain => {
                "Provide a clear explanation of the concept or code, using examples when helpful."
            },
            super::query_parser::QueryIntent::Compare => {
                "Compare the items mentioned in the query, highlighting similarities and differences."
            },
            super::query_parser::QueryIntent::List => {
                "Provide a comprehensive list of the requested items."
            },
            super::query_parser::QueryIntent::Summarize => {
                "Provide a concise summary of the main points."
            },
            super::query_parser::QueryIntent::Analyze => {
                "Provide a detailed analysis, breaking down the components and relationships."
            },
            super::query_parser::QueryIntent::Generate => {
                "Generate the requested code or content based on the patterns found in the context."
            },
            super::query_parser::QueryIntent::Other(_) => {
                "Provide a helpful and informative response based on the context."
            },
        };

        format!("{} {}", base_prompt, instruction)
    }

    fn build_user_prompt(&self, query: &ParsedQuery, context: &QueryContext) -> String {
        let mut prompt = String::new();

        // 添加查询
        prompt.push_str(&format!("Question: {}\n\n", query.original));

        // 添加上下文
        if !context.relevant_documents.is_empty() {
            prompt.push_str("Context:\n");

            for (i, doc) in context.relevant_documents.iter().enumerate() {
                if i < 5 { // 限制文档数量
                    prompt.push_str(&format!(
                        "\n[Document {}]\nTitle: {}\nContent: {}\n\n",
                        i + 1,
                        doc.title,
                        doc.content.chars().take(500).collect::<String>() // 限制内容长度
                    ));
                }
            }
        }

        // 添加代码示例
        if !context.code_examples.is_empty() && context.metadata.include_code {
            prompt.push_str("\nCode Examples:\n");

            for (i, example) in context.code_examples.iter().enumerate().take(3) {
                prompt.push_str(&format!(
                    "\n[Example {}]\nLanguage: {}\nCode:\n```\n{}\n```\n",
                    i + 1,
                    example.language,
                    example.code.chars().take(300).collect::<String>()
                ));
            }
        }

        // 添加请求
        prompt.push_str("\nPlease provide a helpful answer based on the context above. \
        If the context doesn't contain enough information, acknowledge this and suggest what additional information might be needed.");

        prompt
    }

    fn extract_citations(&self, context: &QueryContext) -> Vec<Citation> {
        context
            .relevant_documents
            .iter()
            .map(|doc| Citation {
                document_id: doc.id.clone(),
                title: doc.title.clone(),
                snippet: doc.content.chars().take(200).collect(),
                relevance_score: doc.relevance_score,
            })
            .collect()
    }

    fn calculate_confidence(&self, response: &super::llm_integration::LLMResponse, context: &QueryContext) -> f64 {
        let mut confidence = 0.5; // 基础置信度

        // 基于上下文相关性
        if !context.relevant_documents.is_empty() {
            let avg_relevance: f64 = context.relevant_documents.iter()
                .map(|d| d.relevance_score)
                .sum::<f64>()
                / context.relevant_documents.len() as f64;

            confidence += avg_relevance * 0.3;
        }

        // 基于响应长度（太短可能是信息不足）
        if response.content.len() > 100 {
            confidence += 0.1;
        }

        // 基于完成原因
        if response.finish_reason == "stop" {
            confidence += 0.1;
        }

        confidence.max(0.0).min(1.0)
    }

    fn calculate_context_size(&self, context: &QueryContext) -> usize {
        let mut size = 0;

        for doc in &context.relevant_documents {
            size += doc.content.len();
        }

        for example in &context.code_examples {
            size += example.code.len();
        }

        size
    }

    pub async fn generate_stream(
        &self,
        query: &ParsedQuery,
        context: &QueryContext,
        mut callback: impl FnMut(String) + Send,
    ) -> Result<GeneratedAnswer, anyhow::Error> {
        let start_time = std::time::Instant::now();

        let system_prompt = self.build_system_prompt(query);
        let user_prompt = self.build_user_prompt(query, context);

        let messages = vec![
            LLMMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            LLMMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        // 流式生成
        let full_answer = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let full_answer_clone = full_answer.clone();

        let callback_wrapper = move |chunk: String| {
            callback(chunk.clone());
            let mut answer = full_answer_clone.lock().unwrap();
            answer.push_str(&chunk);
        };

        self.llm.generate_stream(messages, callback_wrapper).await?;

        let answer = full_answer.lock().unwrap().clone();

        let generation_time = start_time.elapsed().as_millis() as u64;

        Ok(GeneratedAnswer {
            query: query.original.clone(),
            answer: answer.clone(),
            citations: self.extract_citations(context),
            confidence: 0.7, // 流式生成的置信度
            metadata: AnswerMetadata {
                generation_time_ms: generation_time,
                context_size: self.calculate_context_size(context),
                model_used: "unknown".to_string(),
                tokens_used: answer.len() / 4, // 估算
            },
        })
    }

    pub fn set_max_context_length(&mut self, length: usize) {
        self.max_context_length = length;
    }

    pub fn is_llm_configured(&self) -> bool {
        self.llm.is_configured()
    }
}

impl Default for AnswerGenerator {
    fn default() -> Self {
        Self::new()
    }
}
