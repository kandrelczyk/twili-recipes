pub mod error;
pub mod ncclient;

use async_trait::async_trait;
use recipes_common::Recipe;

use self::error::RecipesError;

#[async_trait]
pub trait RecipesProvider: Send + Sync {
    async fn list_recipes(&self) -> Result<Vec<String>, RecipesError>;
    async fn save_recipe(&self, recipe: Recipe) -> Result<(), RecipesError>;
}
