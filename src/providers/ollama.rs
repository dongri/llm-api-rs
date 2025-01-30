// Ollama API provider

use crate::core::client::APIClient;
use crate::core::{ChatChoice, ChatCompletionRequest, ChatCompletionResponse, ChatMessage};
use crate::error::LlmApiError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct OllamaChatCompletionRequest {
    messages: Vec<ChatMessage>,
    model: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaChatCompletionResponse {
    message: MessageContent,
}

#[derive(Debug, Deserialize)]
struct MessageContent {
    role: String,
    content: String,
}

pub struct Ollama {
    domain: String,
    client: APIClient,
}

impl Ollama {
    pub fn new(domain: String) -> Self {
        Self {
            domain,
            client: APIClient::new(),
        }
    }
}

#[async_trait::async_trait]
impl super::LlmProvider for Ollama {
    async fn chat_completion<'a>(
        &'a self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmApiError> {
        let url = format!("{}/api/chat", self.domain);

        let req = OllamaChatCompletionRequest {
            messages: request.messages,
            model: request.model.clone(),
            stream: false,
        };

        let res: OllamaChatCompletionResponse =
            self.client.send_request(url, vec![], &req).await?;

        Ok(ChatCompletionResponse {
            id: "".to_string(),
            model: request.model.clone(),
            choices: vec![ChatChoice {
                message: ChatMessage {
                    role: res.message.role,
                    content: res.message.content,
                },
                finish_reason: "completed".to_string(),
            }],
            usage: None,
        })
    }
}
