use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use recipes_common::{Config, RecipesSource, LLM};
use tauri::{async_runtime::Mutex, Manager};

use crate::{
    ai::{AIClient, ChatGPTClient, ClaudeClient, FreeClient, PerplexityClient},
    commands::error::CommandError,
    recipes::{local::LocalClient, ncclient::NCClient, RecipeFile, RecipesProvider},
};

use super::get_stored_or_default_config;

#[tauri::command]
pub async fn initialize(
    app_handle: tauri::AppHandle,
    manager: tauri::State<'_, Mutex<Option<Box<dyn RecipesProvider>>>>,
    ai_client: tauri::State<'_, Mutex<Option<Box<dyn AIClient>>>>,
    config_file: tauri::State<'_, Arc<OnceLock<String>>>,
    recipes_file: tauri::State<'_, Arc<OnceLock<RecipeFile>>>,
) -> Result<bool, CommandError> {
    #[cfg(not(mobile))]
    app_handle
        .get_webview_window("main")
        .unwrap()
        .show()
        .expect("Failed to show window");
    let config: Config = get_stored_or_default_config(app_handle.clone(), config_file).await;

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
                path: PathBuf::from(recipes_file.get().unwrap().0.as_str()),
            }),
        };

        *m = Some(m2);

        let mut ai = ai_client.lock().await;
        let ai2: Box<dyn AIClient> = match config.llm {
            LLM::Free => Box::new(FreeClient::new(config.ai_prompt)),
            LLM::Claude => Box::new(ClaudeClient::new(config.ai_token, config.ai_prompt)),
            LLM::GPT => Box::new(ChatGPTClient::new(config.ai_token, config.ai_prompt)),
            LLM::Perplexity => Box::new(PerplexityClient::new(config.ai_token, config.ai_prompt)),
        };
        *ai = Some(ai2);

        Ok(true)
    } else {
        Ok(false)
    }
}
