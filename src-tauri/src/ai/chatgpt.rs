use async_trait::async_trait;
use recipes_common::Recipe;
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

        println!("{:?}", res.text().await?);

        Ok(Recipe {
            name: "text".to_owned(),
        })
    }
}
