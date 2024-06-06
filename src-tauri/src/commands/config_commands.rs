use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use recipes_common::Config;
use tauri::Wry;
use tauri_plugin_store::{with_store, StoreCollection};

use super::error::CommandError;

pub async fn get_stored_or_default_config(
    app_handle: tauri::AppHandle,
    store: tauri::State<'_, StoreCollection<Wry>>,
    config_file: tauri::State<'_, Arc<OnceLock<String>>>,
) -> Config {
    let path = PathBuf::from(config_file.get().unwrap().as_str());
    let stored_config = with_store(app_handle, store, path, |store| {
        match store.get("config").cloned() {
            None => Ok(Config::default()),
            Some(config) => Ok(serde_json::from_value(config)?),
        }
    });

    stored_config.expect("Failed to read config")
}

#[tauri::command]
pub async fn get_config(
    app_handle: tauri::AppHandle,
    store: tauri::State<'_, StoreCollection<Wry>>,
    config_file: tauri::State<'_, Arc<OnceLock<String>>>,
) -> Result<Config, CommandError> {
    let config = get_stored_or_default_config(app_handle, store, config_file).await;

    Ok(config)
}

#[tauri::command]
pub async fn save_config(
    app_handle: tauri::AppHandle,
    store: tauri::State<'_, StoreCollection<Wry>>,
    config: Config,
    config_file: tauri::State<'_, Arc<OnceLock<String>>>,
) -> Result<(), CommandError> {
    let path = PathBuf::from(config_file.get().unwrap().as_str());
    with_store(app_handle, store, path, |store| {
        store.insert("config".to_owned(), serde_json::to_value(config)?)?;
        store.save()?;
        Ok(())
    })?;

    Ok(())
}
