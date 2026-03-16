/**
 * RAG (Retrieval-Augmented Generation) 模块
 * 提供基于检索增强生成的问答功能
 */

pub mod query_parser;
pub mod context_builder;
pub mod llm_integration;
pub mod answer_generator;

pub use query_parser::{QueryParser, ParsedQuery, QueryIntent, QueryFilters, QueryContext, QueryPreferences};
pub use context_builder::{ContextBuilder, QueryContext as ContextBuilderContext, DocumentContext, CodeExample, EntityContext};
pub use llm_integration::{LLMIntegration, LLMConfig, LLMMessage, LLMResponse, LLMProvider, UsageInfo};
pub use answer_generator::{AnswerGenerator, GeneratedAnswer, Citation, AnswerMetadata};
