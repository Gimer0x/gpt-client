mod helpers;

use reqwest::{Client, header};
use std::{env, time::Duration,io::{self, Write}};
use serde::{Deserialize};
use serde_json::from_str;
use dotenvy::dotenv;

use crate::helpers::traits::{Message, GptRequest, AIProvider, OpenAIConfig};
use async_trait::async_trait;

pub struct OpenAIClient {
    client: Client,
    config: OpenAIConfig,
}

#[derive(Deserialize)]
struct ChatResponse {
    output: Vec<OutputItem>,
}
 
#[derive(Deserialize)]
struct OutputItem {
    content: Vec<ContentItem>,
}
 
#[derive(Deserialize)]
struct ContentItem {
    #[serde(default)]
    text: Option<String>,
}

#[derive(Deserialize)]
struct ApiErrorResponse {
    error: ApiError,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
}

#[async_trait]
impl AIProvider for OpenAIClient {
    fn new(config: OpenAIConfig) -> Self {

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &format!("Bearer {}", config.api_key)
            ).unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .timeout(config.timeout)
            .build()
            .unwrap();

        Self { client, config }
    }

    
    async fn gpt_chat(
        &self,
        messages: Vec<Message>,
    ) -> Result<String, Box<dyn std::error::Error>> {

        let body = GptRequest {
            model: self.config.default_model.clone(),
            input: messages,
        };

        let response = self.client
            .post("https://api.openai.com/v1/responses")
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        let status = response.status();
        let body = response.text().await?;

        
        if !status.is_success() {
            let msg = from_str::<ApiErrorResponse>(&body)
            .map(|e| e.error.message)
            .unwrap_or(body);

            return Err(
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other, 
                    format!("API Error ({}): {}", status, msg))));
        }

        let res: ChatResponse = serde_json::from_str(&body).unwrap();
        let text = res.output[0].content[0].text.as_ref().unwrap();
        Ok(text.to_string())
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let open_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let client = OpenAIClient::new(OpenAIConfig {
        api_key: open_api_key,
        default_model: "gpt-4o-mini".to_string(),
        timeout: Duration::from_secs(10),
    });

    loop {
        let content = prompt("\nEnter your message (type 'exit' to quit): ");

        if content.is_empty() {
            break;
        }
        match content.to_lowercase().as_str() {
            "exit" | "quit" | "q" | "bye" | "goodbye" | "end" | "stop" | "terminate" => break,
            _ => {
                let messages = vec![
                    Message {
                        role: "user".to_string(),
                        content: content.clone(),
                    },
                ];

                let response = client.gpt_chat(messages).await?;
                println!("{}", response);
            },
        }
        
    }
    Ok(())
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}