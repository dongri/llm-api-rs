// DeepSeek API provider
// https://api-docs.deepseek.com/
// https://platform.deepseek.com

use crate::core::client::APIClient;
use crate::core::{ChatChoice, ChatCompletionRequest, ChatCompletionResponse, ChatMessage};
use crate::error::LlmApiError;
use async_trait::async_trait;
use reqwest::header;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct DeepSeekChatRequest {
    messages: Vec<DeepSeekMessage>,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChatResponse {
    id: String,
    model: String,
    choices: Vec<DeepSeekChoice>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChoice {
    message: DeepSeekMessage,
    finish_reason: String,
}

pub struct DeepSeek {
    domain: String,
    api_key: String,
    client: APIClient,
}

impl DeepSeek {
    pub fn new(api_key: String) -> Self {
        Self {
            domain: "https://api.deepseek.com".to_string(),
            api_key,
            client: APIClient::new(),
        }
    }

    fn convert_messages(messages: Vec<ChatMessage>) -> Vec<DeepSeekMessage> {
        messages
            .into_iter()
            .map(|msg| DeepSeekMessage {
                role: msg.role,
                content: msg.content,
            })
            .collect()
    }
}

#[async_trait]
impl crate::providers::LlmProvider for DeepSeek {
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmApiError> {
        let url = format!("{}/v1/chat/completions", self.domain);

        let headers = vec![(header::AUTHORIZATION, format!("Bearer {}", self.api_key))];

        let req = DeepSeekChatRequest {
            messages: Self::convert_messages(request.messages),
            model: request.model,
            temperature: request.temperature,
            max_tokens: request.max_tokens,
        };

        let res: DeepSeekChatResponse = self.client.send_request(url, headers, &req).await?;

        Ok(ChatCompletionResponse {
            id: res.id,
            choices: res
                .choices
                .into_iter()
                .map(|choice| ChatChoice {
                    message: ChatMessage {
                        role: choice.message.role,
                        content: choice.message.content,
                    },
                    finish_reason: choice.finish_reason,
                })
                .collect(),
            model: res.model,
            usage: None,
        })
    }
}
