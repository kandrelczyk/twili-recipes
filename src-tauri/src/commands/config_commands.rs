use confy;

use std::sync::Mutex;

use recipes_common::Config;


#[tauri::command]
pub fn get_config(app: tauri::AppHandle, state: tauri::State<Mutex<Config>>) -> Result<Config, AlarmError> {
    let mut stored_config = state.lock().unwrap();
    let config : Config = confy::load("twili-recipes", None)?;

    *stored_config = config.clone();
    Ok(config)
}

#[tauri::command]
pub fn save_config(app: tauri::AppHandle, config: Config, state: tauri::State<Mutex<Config>>) {

    let mut stored_config = state.lock().unwrap();

    *stored_config = config.clone();

    confy::store("weather-alarms", None, config).expect("Failed to store configuration");
}


