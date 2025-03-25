mod chatgpt;
mod claude;
mod error;
mod perplexity;
mod free;

use async_trait::async_trait;
pub use chatgpt::*;
pub use claude::*;
pub use error::*;
pub use perplexity::*;
pub use free::*;
use serde_json::Value;

#[async_trait]
pub trait AIClient: Send + Sync {
    async fn parse_recipe(&self, recipe: String) -> Result<String, AIError>;
}


pub fn parse_response(json_str: String) -> Result<String, AIError> {
    let result: Value = serde_json::from_str(&json_str)?;
    let recipe = result["content"]
        .as_array()
        .ok_or(AIError {
            reason: "Invalid response from LLM API".to_owned(),
        })?
    .first()
        .ok_or(AIError {
            reason: "Invalid response from LLM API".to_owned(),
        })?["text"]
    .as_str()
        .ok_or(AIError {
            reason: "Invalid response from LLM API".to_owned(),
        })?
    .to_owned();

    Ok(recipe)
}
