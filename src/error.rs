use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlmApiError {
    #[error("Core error: {0}")]
    CoreError(#[from] crate::core::CoreError),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}
