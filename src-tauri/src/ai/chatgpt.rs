use async_trait::async_trait;
use reqwest_dav::re_exports::serde_json;
use serde_json::json;

use crate::ai::AIClient;

use super::{parse_response, AIError};

pub struct ChatGTPClient {
    pub token: String,
    pub prompt: String,
}

impl ChatGTPClient {
    pub fn new(token: String, prompt: String) -> ChatGTPClient {
        ChatGTPClient { token, prompt }
    }
}

#[async_trait]
impl AIClient for ChatGTPClient {
    async fn parse_recipe(&self, recipe: String) -> Result<String, AIError> {
        let client = reqwest::Client::new();

        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&json!(
            {
                "model": "gpt-4o",
                "temperature": 0.0,
                "messages": [
                {
                    "role": "system",
                    "content": self.prompt
                },
                {
                    "role": "user",
                    "content": recipe
                }
                ]
            }))
            .send()
            .await?;

        if res.status().is_success() {
            let json_str = res.text().await?;
            let recipe = parse_response(json_str)?;
            Ok(recipe)
        } else {
            Err(AIError {
                reason: format!("Received error response from ChatGPT API: {:?}", res),
            })
        }
    }
}
