use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(mobile)]
mod mobile;

mod error;
mod models;

pub use error::{Error, Result};

#[cfg(mobile)]
use mobile::KeepScreenOn;

#[cfg(desktop)]
use tauri::plugin::PluginHandle;
#[cfg(desktop)]
pub struct KeepScreenOn<R: Runtime>(PluginHandle<R>);
#[cfg(desktop)]
impl<R: Runtime> KeepScreenOn<R> {}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the keep-screen-on APIs.
pub trait KeepScreenOnExt<R: Runtime> {
    fn keep_screen_on(&self) -> &KeepScreenOn<R>;
}

impl<R: Runtime, T: Manager<R>> crate::KeepScreenOnExt<R> for T {
    fn keep_screen_on(&self) -> &KeepScreenOn<R> {
        self.state::<KeepScreenOn<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("keep-screen-on")
        .setup(|_app, _api| {
            #[cfg(mobile)]
            mobile::init(_app, _api)?;
            Ok(())
        })
        .build()
}
