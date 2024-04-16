#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    AppHandle, Manager,
};

fn get_maximized_menu(app: &AppHandle) -> Menu<tauri::Wry> {
    Menu::with_items(
        app,
        &[&MenuItem::with_id(app, "quit", "&Quit", true, None::<&str>).unwrap()],
    )
    .unwrap()
}

fn get_minimized_menu(app: &AppHandle) -> Menu<tauri::Wry> {
    Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, "show", "&Show", true, None::<&str>).unwrap(),
            &MenuItem::with_id(app, "quit", "&Quit", true, None::<&str>).unwrap(),
        ],
    )
    .unwrap()
}

fn main() {
    let app = app_lib::AppBuilder::new()
        .setup(move |app| {
            let handle = app.handle();
            let menu = get_maximized_menu(handle);
            let _tray = tauri::tray::TrayIconBuilder::with_id("tray_1")
                .icon(tauri::image::Image::from_bytes(include_bytes!(
                    "../icons/32x32.png"
                ))?)
                .menu(&menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => {
                        app.get_window("main")
                            .unwrap()
                            .show()
                            .expect("To show the window");
                        app.tray_by_id("main")
                            .unwrap()
                            .set_menu(Some(get_maximized_menu(app)))
                            .unwrap();
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .build_app();
    app.hide_menu().unwrap();
    app.run(|app, event| {
        if let tauri::RunEvent::WindowEvent {
            event: tauri::WindowEvent::CloseRequested { api, .. },
            ..
        } = event
        {
            app.tray_by_id("main")
                .unwrap()
                .set_menu(Some(get_minimized_menu(app)))
                .unwrap();
            api.prevent_close();
            app.get_window("main")
                .unwrap()
                .hide()
                .expect("To hide the window");
        }
    });
}
