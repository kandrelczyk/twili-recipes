#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let app = app_lib::AppBuilder::new().build_app();
    app.run(|_, _| {});
}
