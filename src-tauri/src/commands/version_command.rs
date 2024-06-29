use tauri::AppHandle;

#[tauri::command]
pub fn get_version(app: AppHandle) -> String {
    app.config().version.clone().unwrap()
}
