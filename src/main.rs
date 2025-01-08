mod components;
mod error;

use components::App;
use leptos::prelude::*;
use thaw::{ConfigDirection, ConfigProvider};

fn main() {
    console_error_panic_hook::set_once();

    _ = console_log::init_with_level(log::Level::Debug);

    let dir = RwSignal::new(ConfigDirection::Ltr);
    mount_to_body(move || {
        view! {
            <ConfigProvider dir=dir>
                <App />
            </ConfigProvider>
        }
    })
}
