use ::serde_json::Value;
use async_trait::async_trait;
use recipes_common::Recipe;
use reqwest_dav::re_exports::serde_json;
use serde_json::json;

use crate::ai::AIClient;

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
    async fn parse_recipe(
        &self,
        recipe: String,
    ) -> Result<recipes_common::Recipe, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&json!(
            {
                "model": "gpt-3.5-turbo",
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

        let json_str = res.text().await?;
        let result: Value = serde_json::from_str(&json_str)?;
        let recipe_json = result["choices"]
            .as_array()
            .expect("Invalid response from openai API")
            .first()
            .expect("Invalid response from openai API")["message"]
            .as_object()
            .expect("Invalid response from openai API")["content"]
            .as_str()
            .expect("Invalid response from openai API");

        let recipe: Recipe = serde_json::from_str(recipe_json)?;

        Ok(recipe)
    }
}
