use std::sync::{Arc, OnceLock};

use recipes_common::Config;
use tauri_plugin_store::StoreExt;

use super::error::CommandError;

pub async fn get_stored_or_default_config(
    app_handle: tauri::AppHandle,
    config_file: tauri::State<'_, Arc<OnceLock<String>>>,
) -> Config {
    let store = app_handle
        .store(config_file.get().unwrap().as_str())
        .unwrap();

    match store.get("config") {
        None => Config::default(),
        Some(config) => serde_json::from_value(config).unwrap(),
    }
}

#[tauri::command]
pub async fn get_config(
    app_handle: tauri::AppHandle,
    config_file: tauri::State<'_, Arc<OnceLock<String>>>,
) -> Result<Config, CommandError> {
    let config = get_stored_or_default_config(app_handle, config_file).await;
    Ok(config)
}

#[tauri::command]
pub async fn save_config(
    app_handle: tauri::AppHandle,
    config: Config,
    config_file: tauri::State<'_, Arc<OnceLock<String>>>,
) -> Result<(), CommandError> {
    let store = app_handle
        .store(config_file.get().unwrap().as_str())
        .unwrap();
    store.set("config", serde_json::to_value(config).unwrap());
    store.save()?;

    Ok(())
}
