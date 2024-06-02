use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.keepScreenOn";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_keep-screen-on);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<KeepScreenOn<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_keep-screen-on)?;
  let plugin = KeepScreenOn(handle);

  plugin.ping(PingRequest{value: None});

  Ok(plugin)
}

/// Access to the keep-screen-on APIs.
pub struct KeepScreenOn<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> KeepScreenOn<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }
}
