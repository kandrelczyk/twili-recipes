use async_trait::async_trait;
use recipes_common::{ListEntry, Recipe};
use tauri::Wry;
use tauri_plugin_store::Store;

use super::{error::RecipesError, RecipesProvider};

pub struct LocalClient {
    pub store: Store<Wry>,
}

impl LocalClient {
    fn get_recipes(&self) -> Result<Vec<Recipe>, RecipesError> {
        let recipes: Vec<Recipe> = match self.store.get("recipes").cloned() {
            None => Vec::new(),
            Some(recipes) => serde_json::from_value(recipes)?,
        };

        Ok(recipes)
    }
}

#[async_trait]
impl RecipesProvider for LocalClient {
    async fn list_recipes(&self) -> Result<Vec<ListEntry>, RecipesError> {
        let recipes = self.get_recipes()?;

        Ok(recipes
            .into_iter()
            .map(|r| ListEntry {
                name: r.name.unwrap(),
                filename: r.id.unwrap(),
            })
            .collect())
    }

    async fn save_recipe(&mut self, mut recipe: Recipe) -> Result<(), RecipesError> {
        let mut recipes = self.get_recipes()?;
        match recipe.id.clone() {
            None => {
                recipe.id = Some(uuid::Uuid::new_v4().to_string());
            }
            Some(id) => {
                recipes.retain(|r| *r.id.as_ref().expect("ID missing") != id);
            }
        }
        recipes.push(recipe);
        self.store
            .insert("recipes".to_string(), serde_json::to_value(recipes)?)?;
        self.store.save()?;
        Ok(())
    }

    async fn delete_recipe(&mut self, id: String) -> Result<(), RecipesError> {
        let mut recipes = self.get_recipes()?;
        recipes.retain(|r| *r.id.as_ref().expect("ID missing") != id);
        self.store
            .insert("recipes".to_string(), serde_json::to_value(recipes)?)?;
        self.store.save()?;
        Ok(())
    }

    async fn get_recipe(&self, id: String) -> Result<Recipe, RecipesError> {
        let recipe = self
            .get_recipes()?
            .into_iter()
            .find(|r| r.id.as_ref().expect("Recipe's ID is None") == &id)
            .expect("Recipe not found");
        Ok(recipe)
    }
}
