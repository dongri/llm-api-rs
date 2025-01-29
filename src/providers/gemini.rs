// Gemini API provider
// https://ai.google.dev/api/generate-content?hl=en
// https://aistudio.google.com/app/apikey

use crate::core::client::APIClient;
use crate::core::{ChatChoice, ChatCompletionRequest, ChatCompletionResponse, ChatMessage};
use crate::error::LlmApiError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
struct GeminiChatCompletionRequest {
    contents: Vec<GeminiChatCompletionContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GeminiGenerationConfig>,
}

#[derive(Debug, Deserialize)]
struct GeminiChatCompletionResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiChatCompletionContent {
    role: String,
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct GeminiGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxOutputTokens")]
    max_output_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiChatCompletionContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    finish_reason: Option<String>,
}

pub struct Gemini {
    domain: String,
    api_key: String,
    client: APIClient,
}

impl Gemini {
    pub fn new(api_key: String) -> Self {
        Self {
            domain: "https://generativelanguage.googleapis.com".to_string(),
            api_key,
            client: APIClient::new(),
        }
    }
}

#[async_trait::async_trait]
impl super::LlmProvider for Gemini {
    async fn chat_completion<'a>(
        &'a self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmApiError> {
        let url = format!(
            "{}/v1beta/models/{}:generateContent?key={}",
            self.domain, request.model, self.api_key
        );

        let req = GeminiChatCompletionRequest {
            contents: request
                .messages
                .into_iter()
                .map(|msg| GeminiChatCompletionContent {
                    role: msg.role,
                    parts: vec![GeminiPart { text: msg.content }],
                })
                .collect(),
            generation_config: Some(GeminiGenerationConfig {
                temperature: request.temperature,
                max_output_tokens: request.max_tokens,
            }),
        };

        let res: GeminiChatCompletionResponse = self.client.send_request(url, vec![], &req).await?;

        Ok(ChatCompletionResponse {
            id: res.candidates[0].content.parts[0].text.clone(),
            choices: res
                .candidates
                .into_iter()
                .map(|candidate| ChatChoice {
                    message: ChatMessage {
                        role: candidate.content.role.clone(),
                        content: candidate.content.parts[0].text.clone(),
                    },
                    finish_reason: candidate.finish_reason.unwrap_or_default(),
                })
                .collect(),
            model: request.model,
            usage: None,
        })
    }
}
