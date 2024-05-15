mod ai;
mod commands;
mod recipes;

use ai::AIClient;
use commands::{get_config, initialize, list_recipes, parse_recipe, save_config, save_recipe};
use recipes::RecipesProvider;
use tauri::{async_runtime::Mutex, App};
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

        tauri::Builder::default()
            .manage(ai_parser)
            .manage(provider)
            .plugin(
                tauri_plugin_log::Builder::default()
                    .clear_targets()
                    .targets([
                        Target::new(TargetKind::Webview),
                        Target::new(TargetKind::Stdout),
                    ])
                    .level(log::LevelFilter::Info)
                    .build(),
            )
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                initialize,
                save_recipe,
                list_recipes,
                parse_recipe,
                get_config,
                save_config
            ])
            .build(tauri::generate_context!())
            .expect("To build tauri app")
    }
}
