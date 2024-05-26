pub mod error;
pub mod ncclient;

use async_trait::async_trait;
use recipes_common::{ListEntry, Recipe};

use self::error::RecipesError;

#[async_trait]
pub trait RecipesProvider: Send + Sync {
    async fn list_recipes(&self) -> Result<Vec<ListEntry>, RecipesError>;
    async fn save_recipe(&self, recipe: Recipe) -> Result<(), RecipesError>;
    async fn get_recipe(&self, filename: String) -> Result<Recipe, RecipesError>;
}
