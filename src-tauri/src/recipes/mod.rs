pub mod ncclient;
mod error;

use async_trait::async_trait;

use self::error::RecipesError;


#[async_trait]
pub trait RecipesProvider : Send + Sync {

    async fn list_recipes(&self) -> Result<Vec<String>, RecipesError>;
}
