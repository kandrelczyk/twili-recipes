use recipes_common::Recipe;

use crate::ai::AIClient;

pub struct ChatGTPClient {}

impl AIClient for ChatGTPClient {
    fn parse_recipe(self, _recipe: &str) -> recipes_common::Recipe {
        Recipe {
            name: "text".to_owned(),
        }
    }
}
