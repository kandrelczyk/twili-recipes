mod components;

use components::App;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(move || {
        view! {
            <App/>
        }
    })
}
