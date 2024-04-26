mod chatgpt;

use async_trait::async_trait;
pub use chatgpt::*;

use recipes_common::Recipe;

#[async_trait]
pub trait AIClient: Send + Sync {
    async fn parse_recipe(&self, recipe: String) -> Result<Recipe, Box<dyn std::error::Error>>;
}
