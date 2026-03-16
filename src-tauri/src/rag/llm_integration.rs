/**
 * LLM 集成
 * 集成多个大语言模型，提供统一的接口
 */

use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LLMProvider {
    OpenAI,
    Ollama,
    Anthropic,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub provider: LLMProvider,
    pub api_endpoint: String,
    pub api_key: String,
    pub model: String,
    pub temperature: f64,
    pub max_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub content: String,
    pub model: String,
    pub usage: UsageInfo,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageInfo {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

pub struct LLMIntegration {
    config: Option<LLMConfig>,
    client: reqwest::Client,
}

impl LLMIntegration {
    pub fn new() -> Self {
        LLMIntegration {
            config: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn set_config(&mut self, config: LLMConfig) {
        self.config = Some(config);
    }

    pub async fn generate(&self, messages: Vec<LLMMessage>) -> Result<LLMResponse> {
        let config = self.config.as_ref()
            .context("LLM config not set")?;

        match &config.provider {
            LLMProvider::OpenAI => self.generate_openai(messages, config).await,
            LLMProvider::Ollama => self.generate_ollama(messages, config).await,
            LLMProvider::Anthropic => self.generate_anthropic(messages, config).await,
            LLMProvider::Other(_) => Err(anyhow::anyhow!("Unsupported LLM provider")),
        }
    }

    async fn generate_openai(
        &self,
        messages: Vec<LLMMessage>,
        config: &LLMConfig,
    ) -> Result<LLMResponse> {
        let request_body = serde_json::json!({
            "model": config.model,
            "messages": messages,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
        });

        let response = self.client
            .post(&format!("{}/chat/completions", config.api_endpoint))
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to OpenAI")?;

        let response_json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse OpenAI response")?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .context("No content in response")?
            .to_string();

        let usage = response_json["usage"].clone();

        Ok(LLMResponse {
            content,
            model: config.model.clone(),
            usage: UsageInfo {
                prompt_tokens: usage["prompt_tokens"].as_u64().unwrap_or(0) as usize,
                completion_tokens: usage["completion_tokens"].as_u64().unwrap_or(0) as usize,
                total_tokens: usage["total_tokens"].as_u64().unwrap_or(0) as usize,
            },
            finish_reason: response_json["choices"][0]["finish_reason"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
        })
    }

    async fn generate_ollama(
        &self,
        messages: Vec<LLMMessage>,
        config: &LLMConfig,
    ) -> Result<LLMResponse> {
        let request_body = serde_json::json!({
            "model": config.model,
            "messages": messages,
            "stream": false,
        });

        let response = self.client
            .post(&format!("{}/api/chat", config.api_endpoint))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to Ollama")?;

        let response_json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse Ollama response")?;

        let content = response_json["message"]["content"]
            .as_str()
            .context("No content in response")?
            .to_string();

        Ok(LLMResponse {
            content,
            model: config.model.clone(),
            usage: UsageInfo {
                prompt_tokens: 0, // Ollama 不提供详细的 token 使用情况
                completion_tokens: 0,
                total_tokens: 0,
            },
            finish_reason: "stop".to_string(),
        })
    }

    async fn generate_anthropic(
        &self,
        messages: Vec<LLMMessage>,
        config: &LLMConfig,
    ) -> Result<LLMResponse> {
        // 将消息转换为 Anthropic 格式
        let system_message = messages.iter()
            .find(|m| m.role == "system")
            .map(|m| m.content.clone());

        let user_messages: Vec<_> = messages.iter()
            .filter(|m| m.role != "system")
            .collect();

        let request_body = serde_json::json!({
            "model": config.model,
            "max_tokens": config.max_tokens,
            "system": system_message,
            "messages": user_messages,
        });

        let response = self.client
            .post(&format!("{}/v1/messages", config.api_endpoint))
            .header("x-api-key", &config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to Anthropic")?;

        let response_json: serde_json::Value = response
            .json()
            .await
            .context("Failed to parse Anthropic response")?;

        let content = response_json["content"][0]["text"]
            .as_str()
            .context("No content in response")?
            .to_string();

        let usage = response_json["usage"].clone();

        Ok(LLMResponse {
            content,
            model: config.model.clone(),
            usage: UsageInfo {
                prompt_tokens: usage["input_tokens"].as_u64().unwrap_or(0) as usize,
                completion_tokens: usage["output_tokens"].as_u64().unwrap_or(0) as usize,
                total_tokens: usage["input_tokens"].as_u64().unwrap_or(0) as usize
                              + usage["output_tokens"].as_u64().unwrap_or(0) as usize,
            },
            finish_reason: response_json["stop_reason"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
        })
    }

    pub async fn generate_stream(
        &self,
        messages: Vec<LLMMessage>,
        _callback: impl FnMut(String) + Send,
    ) -> Result<()> {
        let config = self.config.as_ref()
            .context("LLM config not set")?;

        // 简化版：实际应用中需要完整的流式处理
        let response = self.generate(messages).await?;
        let mut cb = _callback;
        cb(response.content);

        Ok(())
    }

    pub fn is_configured(&self) -> bool {
        self.config.is_some()
    }

    pub fn get_config(&self) -> Option<&LLMConfig> {
        self.config.as_ref()
    }
}

impl Default for LLMIntegration {
    fn default() -> Self {
        Self::new()
    }
}
