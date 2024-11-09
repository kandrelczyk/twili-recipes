use ::serde_json::Value;
use async_trait::async_trait;
use reqwest_dav::re_exports::serde_json;
use serde_json::json;

use crate::ai::AIClient;

use super::AIError;

pub struct ClaudeClient {
    pub token: String,
    pub prompt: String,
}

impl ClaudeClient {
    pub fn new(token: String, prompt: String) -> ClaudeClient {
        ClaudeClient { token, prompt }
    }
}

#[async_trait]
impl AIClient for ClaudeClient {
    async fn parse_recipe(&self, recipe: String) -> Result<String, AIError> {
        let client = reqwest::Client::new();

        let res = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", format!("{}", self.token))
            .header("anthropic-version", "2023-06-01")
            .json(&json!(
            {
                "model": "claude-3-5-sonnet-20240620",
                "max_tokens": 3000,
                "temperature": 0.0,
                "messages": [
                {
                    "role": "user",
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
            let recipe = result["content"]
                .as_array()
                .ok_or(AIError {
                    reason: "Invalid response from anthropic API".to_owned(),
                })?
                .first()
                .ok_or(AIError {
                    reason: "Invalid response from anthropic API".to_owned(),
                })?["text"]
                .as_str()
                .ok_or(AIError {
                    reason: "Invalid response from anthropic API".to_owned(),
                })?
                .to_owned();

            Ok(recipe)
        } else {
            Err(AIError {
                reason: format!("Received error response from anthropic API: {:?}", res),
            })
        }
    }
}
