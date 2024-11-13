use std::path::PathBuf;

use async_trait::async_trait;
use recipes_common::{ListEntry, Recipe};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use super::{error::RecipesError, RecipesProvider};

pub struct LocalClient {
    pub app_handle: AppHandle,
    pub path: PathBuf,
}

impl LocalClient {
    fn get_recipes(&self) -> Result<Vec<Recipe>, RecipesError> {
        let store = self.app_handle.store(self.path.clone())?;
        let recipes: Vec<Recipe> = match store.get("recipes") {
            None => Vec::<Recipe>::new(),
            Some(recipes) => {
                serde_json::from_value(recipes).expect("Failed to deserialize recipes")
            }
        };

        Ok(recipes)
    }

    fn save_recipes(&self, recipes: Vec<Recipe>) -> Result<(), RecipesError> {
        let store = self.app_handle.store(self.path.clone())?;
        store.set(
            "recipes".to_string(),
            serde_json::to_value(recipes).expect("Failed to serialize recipes"),
        );
        store.save()?;

        Ok(())
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

        self.save_recipes(recipes)
    }

    async fn delete_recipe(&mut self, id: String) -> Result<(), RecipesError> {
        let mut recipes = self.get_recipes()?;
        recipes.retain(|r| *r.id.as_ref().expect("ID missing") != id);
        self.save_recipes(recipes)
    }

    async fn rename_recipe(&mut self, id: String, name: String) -> Result<(), RecipesError> {
        let mut recipes = self.get_recipes()?;
        let recipe = recipes
            .iter_mut()
            .find(|r| r.id.as_ref().expect("Recipe's ID is None") == &id)
            .expect("Recipe not found");
        recipe.name = Some(name);
        self.save_recipes(recipes)
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
