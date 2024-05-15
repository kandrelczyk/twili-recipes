use recipes_common::Config;

use super::error::CommandError;

#[tauri::command]
pub fn get_config() -> Result<Config, CommandError> {
    let config: Config = confy::load("twili-recipes", None)?;

    Ok(config)
}

#[tauri::command]
pub fn save_config(config: Config) -> Result<(), CommandError> {
    confy::store("twili-recipes", None, config)?;

    Ok(())
}
