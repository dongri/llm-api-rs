// src/providers/groq.rs
// https://console.groq.com/docs/openai

use crate::core::client::APIClient;
use crate::core::{
    ChatChoice, ChatCompletionRequest, ChatCompletionResponse, ChatMessage, ChatUsage,
};
use crate::error::LlmApiError;
use async_trait::async_trait;
use reqwest::header;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GroqChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct GroqChatCompletionResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<ChatUsage>,
}

pub struct Groq {
    domain: String,
    api_key: String,
    client: APIClient,
}

impl Groq {
    pub fn new(api_key: String) -> Self {
        Self {
            domain: "https://api.groq.com".to_string(),
            api_key,
            client: APIClient::new(),
        }
    }
}

#[async_trait]
impl crate::providers::LlmProvider for Groq {
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmApiError> {
        let url = format!("{}/openai/v1/chat/completions", self.domain);

        let headers = vec![(header::AUTHORIZATION, format!("Bearer {}", self.api_key))];

        let req = GroqChatCompletionRequest {
            model: request.model,
            messages: request.messages,
            temperature: request.temperature,
            max_tokens: request.max_tokens,
        };

        let res: GroqChatCompletionResponse =
            self.client.send_request(url, headers, &req).await?;

        Ok(ChatCompletionResponse {
            id: res.id,
            choices: res.choices,
            model: res.model,
            usage: res.usage,
        })
    }
}
