use recipes_common::{ListEntry, Recipe};
use tauri::async_runtime::Mutex;

use crate::{commands::error::CommandError, recipes::RecipesProvider};

#[tauri::command]
pub async fn save_recipe(
    recipe: Recipe,
    manager: tauri::State<'_, Mutex<Option<Box<dyn RecipesProvider>>>>,
) -> Result<(), CommandError> {
    let m = manager.lock().await;

    Ok(m.as_ref().unwrap().save_recipe(recipe).await?)
}

#[tauri::command]
pub async fn list_recipes(
    manager: tauri::State<'_, Mutex<Option<Box<dyn RecipesProvider>>>>,
) -> Result<Vec<ListEntry>, CommandError> {
    let m = manager.lock().await;

    Ok(m.as_ref().unwrap().list_recipes().await?)
}
