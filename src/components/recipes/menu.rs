use leptos::*;
use thaw::{Divider, Icon, Switch};

use crate::components::recipes::MenuButton;

#[component]
pub fn Menu(
    dark_mode: RwSignal<bool>,
    reload_signal: RwSignal<i32>,
    show_menu: RwSignal<bool>,
) -> impl IntoView {
    let reload = move |_| {
        show_menu.set(false);
        reload_signal.set(reload_signal.get() + 1);
    };

    let go_to_settings = move |_| {
        show_menu.set(false);
        window().location().set_href("/settings").expect("");
    };

    view! {
        <div class="flex flex-col gap-8 text-lg">
            <div>
                <div class="flex flex-row justify-center w-full">Twili Recipes</div>
                <Divider class="m-2"/>
            </div>
            <MenuButton on_click=go_to_settings>
                <Icon icon=icondata_bi::BiCogRegular/>
                Settings
            </MenuButton>
            <div class="flex flex-row gap-2 items-center">
                <Icon icon=icondata_bi::BiMoonRegular/>
                Dark mode
                <div class="grow"></div>
                <Switch value=dark_mode/>
            </div>
            <MenuButton on_click=reload>
                <Icon icon=icondata_bi::BiRevisionRegular/>
                Refresh
            </MenuButton>
            <MenuButton on_click=|_| log::info!("test")>
                <Icon icon=icondata_bi::BiPowerOffRegular/>
                Quit
            </MenuButton>
        </div>
    }
}
