use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_use::storage::use_local_storage;
use thaw::{Divider, Icon, Switch, Theme};

#[component]
pub fn AppMenu(reload_signal: RwSignal<i32>, show_menu: RwSignal<bool>) -> impl IntoView {
    let navigate = RwSignal::new(use_navigate());
    let reload = move |_| {
        show_menu.set(false);
        reload_signal.set(reload_signal.get() + 1);
    };

    let go_to_settings = move |_| {
        show_menu.set(false);
        navigate.get()("/settings", Default::default());
    };

    let go_to_about = move |_| {
        show_menu.set(false);
        navigate.get()("/about", Default::default());
    };

    let (dark, set_dark, _) = use_local_storage::<bool, FromToStringCodec>("dark_mode");

    let is_dark = RwSignal::new(dark.get());
    let theme = Theme::use_rw_theme();

    Effect::new(move |_| {
        if is_dark() {
            theme.set(Theme::dark());
            set_dark.set(true);
        } else {
            theme.set(Theme::light());
            set_dark.set(false);
        }
    });

    view! {
        <div class="flex flex-col gap-8 text-lg w-full">
            <div>
                <div class="flex flex-row justify-center w-full pt-1">Twili Recipes</div>
                <Divider class="m-2" />
            </div>
            <div
                id="settings"
                class="flex flex-row gap-2 items-center hover:text-blue-400 cursor-pointer ml-4"
                on:click=go_to_settings
            >
                <Icon icon=icondata_bi::BiCogRegular />
                Settings
            </div>
            <div id="dark_mode" class="flex flex-row gap-2 items-center ml-4 mr-4">
                <Icon icon=icondata_bi::BiMoonRegular />
                Dark mode
                <div class="grow"></div>
                 <Switch checked=is_dark />
            </div>
            <div
                id="refresh"
                class="flex flex-row gap-2 items-center hover:text-blue-400 cursor-pointer ml-4"
                on:click=reload
            >
                <Icon icon=icondata_bi::BiRevisionRegular />
                Refresh
            </div>
            <div
                id="about"
                class="flex flex-row gap-2 items-center hover:text-blue-400 cursor-pointer ml-4"
                on:click=go_to_about
            >
                <Icon icon=icondata_bi::BiInfoCircleRegular />
                About
            </div>
        </div>
    }
}
