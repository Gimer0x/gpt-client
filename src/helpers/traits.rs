use serde::Serialize;
use async_trait::async_trait;
use std::time::Duration;

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct GptRequest {
    pub model: String,
    pub input: Vec<Message>,
}

pub struct OpenAIConfig {
    pub api_key: String,
    pub default_model: String,
    pub timeout: Duration,
}

#[async_trait]
pub trait AIProvider {
    fn new(config: OpenAIConfig) -> Self;

    async fn gpt_chat(&self, messages: Vec<Message>)
        -> Result<String, Box<dyn std::error::Error>>;
}