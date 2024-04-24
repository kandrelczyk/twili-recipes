mod chatgpt;

use async_trait::async_trait;
pub use chatgpt::*;

use recipes_common::Recipe;

#[async_trait]
pub trait AIClient : Send + Sync {
    fn parse_recipe(self, recipe : &str) -> Recipe;
}

