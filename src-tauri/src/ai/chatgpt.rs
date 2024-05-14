use ::serde_json::Value;
use async_trait::async_trait;
use reqwest_dav::re_exports::serde_json;
use serde_json::json;

use crate::ai::AIClient;

use super::AIError;

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
                "model": "gpt-3.5-turbo",
                "temperature": 0.2,
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
            let result: Value = serde_json::from_str(&json_str)?;
            let recipe = result["choices"]
                .as_array()
                .ok_or(AIError {
                    reason: "Invalid response from openai API".to_owned(),
                })?
                .first()
                .ok_or(AIError {
                    reason: "Invalid response from openai API".to_owned(),
                })?["message"]
                .as_object()
                .ok_or(AIError {
                    reason: "Invalid response from openai API".to_owned(),
                })?["content"]
                .as_str()
                .ok_or(AIError {
                    reason: "Invalid response from openai API".to_owned(),
                })?
                .to_owned();

            Ok(recipe)
        } else {
            Err(AIError {
                reason: format!("Received error response from ChatGPT API: {:?}", res),
            })
        }
    }
}
