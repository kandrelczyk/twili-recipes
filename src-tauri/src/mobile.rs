#[tauri::mobile_entry_point]
fn main() {
    let app = super::AppBuilder::new().build_app();
    app.run(|_, _| {});
}
