mod components;
mod error;

use components::App;
use leptos::*;
use thaw::{GlobalStyle, Theme, ThemeProvider};

fn main() {
    console_error_panic_hook::set_once();

    _ = console_log::init_with_level(log::Level::Debug);

    let theme = create_rw_signal(Theme::light());
    mount_to_body(move || {
        view! {
            <ThemeProvider theme>
                <GlobalStyle/>
                <App/>
            </ThemeProvider>
        }
    })
}
