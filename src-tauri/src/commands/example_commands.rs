use std::sync::OnceLock;
use tauri::Manager; // brings app.emit() into scope
use template_common::CustomError;

#[tauri::command]
pub async fn command(error: bool) -> Result<String, CustomError> {
    std::thread::sleep(std::time::Duration::from_secs(1));
    if !error {
        Ok("Ok Response".to_owned())
    } else {
        Err(CustomError {
            reason: "Error Response".to_owned(),
        })
    }
}

static SCHEDULED: OnceLock<bool> = OnceLock::new();

#[tauri::command]
pub async fn start_events(app: tauri::AppHandle) {
    if SCHEDULED.get().is_none() {
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            log::info!("Sending event");
            app.emit("custom_event", ()).expect("To emit event");
        });
        SCHEDULED.set(true).expect("To set scheduled flag")
    }
}
