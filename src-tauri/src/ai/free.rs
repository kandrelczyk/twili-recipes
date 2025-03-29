use async_trait::async_trait;
use reqwest_dav::re_exports::serde_json;
use serde_json::json;

use crate::ai::AIClient;

use super::{parse_claude_response, AIError};

pub struct FreeClient {
    pub prompt: String,
}

impl FreeClient {
    pub fn new(prompt: String) -> FreeClient {
        FreeClient { prompt }
    }
}

#[async_trait]
impl AIClient for FreeClient {
    async fn parse_recipe(&self, recipe: String) -> Result<String, AIError> {
        let client = reqwest::Client::new();

        let res = client
            .post("https://lemmy.curiana.net/twili/recipe")
            .json(&json!(
            {
                "prompt": self.prompt,
                "recipe": recipe
            }))
            .send()
            .await?;

        if res.status().is_success() {
            let json_str = res.text().await?;
            let recipe = parse_claude_response(json_str)?;
            Ok(recipe)
        } else {
            Err(AIError {
                reason: res.text().await?,
            })
        }
    }
}
