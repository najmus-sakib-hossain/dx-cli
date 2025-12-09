use anyhow::{Context, Result};
use reqwest::Client;

use super::models::{GeminiRequest, GeminiResponse, GenerationConfig, Message};

const DEFAULT_API_KEY: &str = "AIzaSyDEYnJZwGLIvN1qf8R_Hx_3TqY9K4vVwXo"; // Placeholder - user should replace
const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";

pub struct GeminiClient {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiClient {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.unwrap_or_else(|| DEFAULT_API_KEY.to_string()),
            model: "gemini-1.5-flash".to_string(), // Free model
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub async fn generate_content(&self, messages: Vec<Message>) -> Result<String> {
        let url = format!(
            "{}/{}:generateContent?key={}",
            GEMINI_API_URL, self.model, self.api_key
        );

        let request = GeminiRequest {
            contents: messages,
            generation_config: Some(GenerationConfig {
                temperature: Some(0.7),
                max_output_tokens: Some(2048),
            }),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Gemini API error: {}", error_text);
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .context("Failed to parse Gemini response")?;

        if let Some(candidate) = gemini_response.candidates.first() {
            Ok(candidate.content.text())
        } else {
            anyhow::bail!("No response from Gemini")
        }
    }

    pub async fn chat(&self, message: &str, history: &[Message]) -> Result<String> {
        let mut messages = history.to_vec();
        messages.push(Message::user(message));
        self.generate_content(messages).await
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }
}

impl Default for GeminiClient {
    fn default() -> Self {
        Self::new(None)
    }
}
