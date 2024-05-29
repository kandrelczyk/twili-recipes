use recipes_common::Config;
use tauri::{async_runtime::Mutex, Wry};
use tauri_plugin_store::StoreCollection;

use crate::{
    ai::{AIClient, ChatGTPClient},
    commands::error::CommandError,
    recipes::{ncclient::NCClient, RecipesProvider},
};

use super::get_stored_or_default_config;

#[tauri::command]
pub async fn initialize(
    app_handle: tauri::AppHandle,
    store: tauri::State<'_, StoreCollection<Wry>>,
    manager: tauri::State<'_, Mutex<Option<Box<dyn RecipesProvider>>>>,
    ai_client: tauri::State<'_, Mutex<Option<Box<dyn AIClient>>>>,
) -> Result<bool, CommandError> {
    let config: Config = get_stored_or_default_config(app_handle, store);

    if config.all_present() {
        let mut m = manager.lock().await;
        let m2: Box<dyn RecipesProvider> = Box::new(NCClient::new(
            config.cloud_uri,
            config.cloud_username,
            config.cloud_pass,
        ));
        *m = Some(m2);

        let mut ai = ai_client.lock().await;
        let ai2: Box<dyn AIClient> =
            Box::new(ChatGTPClient::new(config.ai_token, config.ai_prompt));
        *ai = Some(ai2);

        Ok(true)
    } else {
        Ok(false)
    }
}
