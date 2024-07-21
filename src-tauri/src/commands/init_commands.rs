use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use recipes_common::{Config, RecipesSource};
use tauri::{async_runtime::Mutex, Wry};
use tauri_plugin_store::StoreCollection;

use crate::{
    ai::{AIClient, ChatGTPClient},
    commands::error::CommandError,
    recipes::{local::LocalClient, ncclient::NCClient, RecipesProvider},
};

use super::get_stored_or_default_config;

#[tauri::command]
pub async fn initialize(
    app_handle: tauri::AppHandle,
    store: tauri::State<'_, StoreCollection<Wry>>,
    manager: tauri::State<'_, Mutex<Option<Box<dyn RecipesProvider>>>>,
    ai_client: tauri::State<'_, Mutex<Option<Box<dyn AIClient>>>>,
    config_file: tauri::State<'_, Arc<OnceLock<String>>>,
) -> Result<bool, CommandError> {
    let config: Config =
        get_stored_or_default_config(app_handle.clone(), store.clone(), config_file).await;

    if config.all_present() {
        let mut m = manager.lock().await;

        let m2: Box<dyn RecipesProvider> = match config.recipes_source {
            RecipesSource::Cloud => Box::new(NCClient::new(
                config.cloud_uri,
                config.cloud_username,
                config.cloud_pass,
            )),
            RecipesSource::Local => Box::new(LocalClient {
                app_handle: app_handle.clone(),
                path: PathBuf::from("recipes"),
            }),
        };

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
