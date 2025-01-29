// src/providers/openai.rs
// https://platform.openai.com/docs/api-reference/chat/create
// https://platform.openai.com

use crate::core::client::APIClient;
use crate::core::{
    ChatChoice, ChatCompletionRequest, ChatCompletionResponse, ChatMessage, ChatUsage,
};
use crate::error::LlmApiError;
use async_trait::async_trait;
use reqwest::header;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct OpenAIChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChatCompletionResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<ChatUsage>,
}

pub struct OpenAI {
    domain: String,
    api_key: String,
    client: APIClient,
}

impl OpenAI {
    pub fn new(api_key: String) -> Self {
        OpenAI {
            domain: "https://api.openai.com".to_string(),
            api_key,
            client: APIClient::new(),
        }
    }
}

#[async_trait]
impl crate::providers::LlmProvider for OpenAI {
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmApiError> {
        let url = format!("{}/v1/chat/completions", self.domain);

        let headers = vec![(header::AUTHORIZATION, format!("Bearer {}", self.api_key))];

        let req = OpenAIChatCompletionRequest {
            model: request.model,
            messages: request.messages,
            temperature: request.temperature,
            max_tokens: request.max_tokens,
        };

        let res: OpenAIChatCompletionResponse =
            self.client.send_request(url, headers, &req).await?;

        Ok(ChatCompletionResponse {
            id: res.id,
            choices: res.choices,
            model: res.model,
            usage: res.usage,
        })
    }
}
