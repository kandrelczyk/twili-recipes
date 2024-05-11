use tauri::async_runtime::Mutex;

use crate::ai::AIClient;

use super::error::CommandError;

#[tauri::command]
pub async fn parse_recipe(
    recipe: String,
    ai_client: tauri::State<'_, Mutex<Option<Box<dyn AIClient>>>>,
) -> Result<String, CommandError> {
    let client = ai_client.lock().await;
    Ok(client.as_ref().unwrap().parse_recipe(recipe).await?)
}
