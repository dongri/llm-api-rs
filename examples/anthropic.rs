use llm_api_rs::{
    core::{ChatCompletionRequest, ChatMessage},
    providers::anthropic::Anthropic,
    LlmProvider,
};

#[tokio::main]
async fn main() {
    let api_key =
        std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY environment variable not set");

    let client = Anthropic::new(api_key);

    let request = ChatCompletionRequest {
        model: "claude-3-5-sonnet-20241022".to_string(),
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
        temperature: Some(0.7),
        max_tokens: Some(50),
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
