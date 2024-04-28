use recipes_common::Recipe;
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
