mod chatgpt;
mod error;
mod perplexity;

use async_trait::async_trait;
pub use chatgpt::*;
pub use error::*;
pub use perplexity::*;

#[async_trait]
pub trait AIClient: Send + Sync {
    async fn parse_recipe(&self, recipe: String) -> Result<String, AIError>;
}
