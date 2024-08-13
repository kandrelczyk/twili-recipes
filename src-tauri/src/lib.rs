mod ai;
mod commands;
mod recipes;

use std::sync::{Arc, OnceLock};

use ai::AIClient;
use commands::{
    delete_recipe, get_config, get_recipe, get_version, initialize, list_recipes, parse_recipe,
    save_config, save_recipe,
};
use recipes::RecipesProvider;
use tauri::{async_runtime::Mutex, App};
use tauri_plugin_cli::CliExt;
#[cfg(not(debug_assertions))]
use tauri_plugin_log::{Target, TargetKind};

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }

    pub fn build_app(self) -> App {
        let setup = self.setup;

        let provider: Mutex<Option<Box<dyn RecipesProvider>>> = Mutex::new(None);
        let ai_parser: Mutex<Option<Box<dyn AIClient>>> = Mutex::new(None);
        let config_file: Arc<OnceLock<String>> = Arc::new(OnceLock::new());

        let mut builder = tauri::Builder::default()
            .plugin(tauri_plugin_shell::init())
            .plugin(tauri_plugin_store::Builder::new().build())
            .plugin(tauri_plugin_keep_screen_on::init())
            .manage(ai_parser)
            .manage(provider)
            .manage(config_file.clone())
            .plugin(tauri_plugin_cli::init())
            .plugin(tauri_plugin_store::Builder::default().build())
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }
                match app.cli().matches() {
                    Ok(matches) => {
                        if let Some(cfg_flag) = matches.args.get("config") {
                            if let Some(value) = cfg_flag.value.as_str() {
                                config_file
                                    .set(value.to_owned())
                                    .expect("Failed to set settings file");
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("failed to parse cli arguments: {}", err.to_string());
                    }
                };
                if config_file.get().is_none() {
                    config_file
                        .set(".settings.dat".to_owned())
                        .expect("Failed to set settings file");
                }
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                initialize,
                save_recipe,
                list_recipes,
                get_recipe,
                delete_recipe,
                parse_recipe,
                get_config,
                save_config,
                get_version
            ]);

        #[cfg(debug_assertions)]
        {
            let devtools = tauri_plugin_devtools::init();
            builder = builder.plugin(devtools);
        }
        #[cfg(not(debug_assertions))]
        {
            builder = builder.plugin(
                tauri_plugin_log::Builder::default()
                    .clear_targets()
                    .targets([
                        Target::new(TargetKind::Webview),
                        Target::new(TargetKind::Stdout),
                    ])
                    .level(log::LevelFilter::Warn)
                    .build(),
            );
        }

        builder
            .build(tauri::generate_context!())
            .expect("To build tauri app")
    }
}
