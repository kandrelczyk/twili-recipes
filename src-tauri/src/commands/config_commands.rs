use std::path::PathBuf;

use recipes_common::Config;
use tauri::Wry;
use tauri_plugin_store::{with_store, StoreCollection};

use super::error::CommandError;

pub fn get_stored_or_default_config(
    app_handle: tauri::AppHandle,
    store: tauri::State<'_, StoreCollection<Wry>>,
) -> Config {
    let path = PathBuf::from(".settings.dat");
    let stored_config = with_store(app_handle, store, path, |store| {
        match store.get("config").cloned() {
            None => Ok(Config::default()),
            Some(config) => Ok(serde_json::from_value(config)?),
        }
    });

    stored_config.expect("Failed to read config")
}

#[tauri::command]
pub fn get_config(
    app_handle: tauri::AppHandle,
    store: tauri::State<'_, StoreCollection<Wry>>,
) -> Result<Config, CommandError> {
    let config = get_stored_or_default_config(app_handle, store);

    Ok(config)
}

#[tauri::command]
pub fn save_config(
    app_handle: tauri::AppHandle,
    store: tauri::State<'_, StoreCollection<Wry>>,
    config: Config,
) -> Result<(), CommandError> {
    let path = PathBuf::from(".settings.dat");
    with_store(app_handle, store, path, |store| {
        store.insert("config".to_owned(), serde_json::to_value(config)?)?;
        store.save()?;
        Ok(())
    })?;

    Ok(())
}
