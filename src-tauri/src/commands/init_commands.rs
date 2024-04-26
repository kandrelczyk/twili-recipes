use confy;
use recipes_common::Config;
use tauri::async_runtime::Mutex;

use crate::{
    ai::{AIClient, ChatGTPClient},
    commands::error::CommandError,
    recipes::{ncclient::NCClient, RecipesProvider},
};

#[tauri::command]
pub async fn command(
    error: bool,
    manager: tauri::State<'_, Mutex<Option<Box<dyn RecipesProvider>>>>,
    ai_client: tauri::State<'_, Mutex<Option<Box<dyn AIClient>>>>,
) -> Result<String, CommandError> {
    let m = manager.lock().await;

    println!("{:?}", m.as_ref().unwrap().list_recipes().await.unwrap());

    let ai = ai_client.lock().await;
    println!(
        "{:?}",
        ai.as_ref()
            .unwrap()
            .parse_recipe("test".to_owned())
            .await
            .unwrap()
    );

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
    ai_client: tauri::State<'_, Mutex<Option<Box<dyn AIClient>>>>,
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

        let mut ai = ai_client.lock().await;
        let ai2: Box<dyn AIClient> =
            Box::new(ChatGTPClient::new(config.ai_token, config.ai_prompt));
        *ai = Some(ai2);

        Ok(true)
    } else {
        Ok(false)
    }
}
