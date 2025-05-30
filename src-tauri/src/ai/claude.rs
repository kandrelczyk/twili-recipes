use async_trait::async_trait;
use serde_json::json;

use crate::ai::AIClient;

use super::{parse_claude_response, AIError};

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
            .header("x-api-key", self.token.clone())
            .header("anthropic-version", "2023-06-01")
            .json(&json!(
            {
                "model": "claude-3-7-sonnet-20250219",
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
            let recipe = parse_claude_response(json_str)?;
            Ok(recipe)
        } else {
            Err(AIError {
                reason: format!("Received error response from anthropic API: {:?}", res),
            })
        }
    }
}
