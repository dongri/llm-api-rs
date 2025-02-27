use crate::error::LlmApiError;
use reqwest::{header::HeaderName, Client};
use serde::{de::DeserializeOwned, Serialize};

pub struct APIClient {
    client: Client,
}

impl Default for APIClient {
    fn default() -> Self {
        Self::new()
    }
}

impl APIClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn send_request<T, U>(
        &self,
        url: String,
        headers: Vec<(HeaderName, String)>,
        request: &T,
    ) -> Result<U, LlmApiError>
    where
        T: Serialize + ?Sized,
        U: DeserializeOwned,
    {
        let mut req = self.client.post(url);

        for (key, value) in headers {
            req = req.header(key, value);
        }

        let response = req
            .json(request)
            .send()
            .await
            .map_err(|e| LlmApiError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .map_err(|e| LlmApiError::DeserializationError(e.to_string()))?;
            let error = LlmApiError::NetworkError(error_text);
            return Err(error);
        }

        response
            .json()
            .await
            .map_err(|e| LlmApiError::DeserializationError(e.to_string()))
    }
}
