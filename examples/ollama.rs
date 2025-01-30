// Examples for using the Ollama provider.

// Prepare the environment:
// $ ollama run deepseek-r1:7b

// https://github.com/ollama/ollama/blob/main/docs/api.md#generate-a-chat-completion

use llm_api_rs::{
    core::{ChatCompletionRequest, ChatMessage},
    providers::ollama::Ollama,
    LlmProvider,
};

#[tokio::main]
async fn main() {

    let domain = "http://localhost:11434";

    let client = Ollama::new(domain.to_string());

    let request = ChatCompletionRequest {
        model: "deepseek-r1:7b".to_string(),
        messages: vec![
            ChatMessage {
                role: "assistant".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: "Hello!".to_string(),
            },
        ],
        temperature: None,
        max_tokens: None,
    };

    match client.chat_completion(request).await {
        Ok(response) => {
            for choice in response.choices {
                println!("Response: {}", choice.message.content);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
