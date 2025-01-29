pub mod anthropic;
pub mod deepseek;
pub mod gemini;
pub mod openai;
pub mod xai;

use crate::core::{ChatCompletionRequest, ChatCompletionResponse};
use crate::error::LlmApiError;
use async_trait::async_trait;

#[async_trait]
pub trait LlmProvider {
    async fn chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmApiError>;
}

pub use anthropic::Anthropic;
pub use deepseek::DeepSeek;
pub use gemini::Gemini;
pub use openai::OpenAI;
pub use xai::XAI;
