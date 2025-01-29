// XAI API provider
// https://docs.x.ai/docs/guides/chat
// https://console.x.ai/

use crate::core::client::APIClient;
use crate::core::{ChatChoice, ChatCompletionRequest, ChatCompletionResponse, ChatMessage};
use crate::error::LlmApiError;
use async_trait::async_trait;
use reqwest::header;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct XaiChatRequest {
    messages: Vec<XaiMessage>,
    model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct XaiMessage {
    role: String,
    content: Vec<XaiContent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct XaiContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct XaiChatResponse {
    id: String,
    model: String,
    choices: Vec<XaiChoice>,
}

#[derive(Debug, Deserialize)]
struct XaiChoice {
    message: XaiMessageResponse,
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct XaiMessageResponse {
    role: String,
    content: String,
}

pub struct XAI {
    domain: String,
    api_key: String,
    client: APIClient,
}

impl XAI {
    pub fn new(api_key: String) -> Self {
        Self {
            domain: "https://api.x.ai".to_string(),
            api_key,
            client: APIClient::new(),
        }
    }

    fn convert_messages(messages: Vec<ChatMessage>) -> Vec<XaiMessage> {
        messages
            .into_iter()
            .map(|msg| XaiMessage {
                role: msg.role,
                content: vec![XaiContent {
                    content_type: "text".to_string(),
                    text: msg.content,
                }],
            })
            .collect()
    }
}

#[async_trait]
impl crate::providers::LlmProvider for XAI {
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmApiError> {
        let url = format!("{}/v1/chat/completions", self.domain);

        let req = XaiChatRequest {
            messages: Self::convert_messages(request.messages),
            model: request.model,
            temperature: request.temperature,
            max_tokens: request.max_tokens,
        };
        let headers = vec![(header::AUTHORIZATION, format!("Bearer {}", self.api_key))];
        let res: XaiChatResponse = self.client.send_request(url, headers, &req).await?;
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
