use confy;
use recipes_common::Config;
use tauri::async_runtime::Mutex;

use crate::{
    commands::error::CommandError,
    recipes::{ncclient::NCClient, RecipesProvider},
};

#[tauri::command]
pub async fn command(
    error: bool,
    manager: tauri::State<'_, Mutex<Option<Box<dyn RecipesProvider>>>>,
) -> Result<String, CommandError> {
    let m = manager.lock().await;

    println!("{:?}", m.as_ref().unwrap().list_recipes().await.unwrap());

    std::thread::sleep(std::time::Duration::from_secs(1));
    if !error {
        Ok("Ok Response".to_owned())
    } else {
        Err(CommandError {
            reason: "Error Response".to_owned(),
        })
    }
}

#[tauri::command]
pub async fn initialize(
    manager: tauri::State<'_, Mutex<Option<Box<dyn RecipesProvider>>>>,
) -> Result<bool, CommandError> {
    let config: Config = confy::load("twili-recipes", None)?;

    if config.all_present() {
        let mut m = manager.lock().await;
        let m2: Box<dyn RecipesProvider> = Box::new(NCClient::new(
            config.cloud_uri,
            config.cloud_username,
            config.cloud_pass,
        ));
        *m = Some(m2);

        std::thread::sleep(std::time::Duration::from_secs(1));
        Ok(true)
    } else {
        Ok(false)
    }
    //     Err(CommandError {
    //         reason: "Error Response".to_owned(),
    //     })
}
