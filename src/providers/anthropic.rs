// Anthropic API provider
// https://docs.anthropic.com/en/api/getting-started
// https://docs.anthropic.com/en/api/versioning
// https://console.anthropic.com/dashboard

use crate::core::client::APIClient;
use crate::core::{ChatChoice, ChatCompletionRequest, ChatCompletionResponse, ChatMessage};
use crate::error::LlmApiError;
use reqwest::header::HeaderName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct AnthropicChatCompletionRequest {
    messages: Vec<ChatMessage>,
    model: String,
    max_tokens: u32,
    temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct AnthropicChatCompletionResponse {
    content: Vec<AnthropicContent>,
    id: String,
    model: String,
    role: String,
    stop_reason: String,
    usage: AnthropicUsage,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    text: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

pub struct Anthropic {
    domain: String,
    api_key: String,
    client: APIClient,
}

impl Anthropic {
    pub fn new(api_key: String) -> Self {
        Self {
            domain: "https://api.anthropic.com".to_string(),
            api_key,
            client: APIClient::new(),
        }
    }
}

#[async_trait::async_trait]
impl super::LlmProvider for Anthropic {
    async fn chat_completion<'a>(
        &'a self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmApiError> {
        let url = format!("{}/v1/messages", self.domain);

        let headers = vec![
            (HeaderName::from_static("x-api-key"), self.api_key.clone()),
            (
                HeaderName::from_static("anthropic-version"),
                "2023-06-01".to_string(),
            ),
        ];

        let req = AnthropicChatCompletionRequest {
            messages: request.messages,
            model: request.model,
            max_tokens: request.max_tokens.unwrap_or(1000),
            temperature: request.temperature,
        };

        let res: AnthropicChatCompletionResponse =
            self.client.send_request(url, headers, &req).await?;

        Ok(ChatCompletionResponse {
            id: res.id,
            model: res.model,
            choices: res
                .content
                .into_iter()
                .map(|content| ChatChoice {
                    message: ChatMessage {
                        role: res.role.clone(),
                        content: content.text,
                    },
                    finish_reason: res.stop_reason.clone(),
                })
                .collect(),
            usage: Some(crate::core::ChatUsage {
                input_tokens: Some(res.usage.input_tokens),
                output_tokens: Some(res.usage.output_tokens),
            }),
        })
    }
}
