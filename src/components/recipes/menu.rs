use leptos::*;
use leptos_router::use_navigate;
use thaw::{Divider, Icon, Switch};

#[component]
pub fn Menu(
    dark_mode: RwSignal<bool>,
    reload_signal: RwSignal<i32>,
    show_menu: RwSignal<bool>,
) -> impl IntoView {
    let navigate = create_rw_signal(use_navigate());
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

    view! {
        <div class="flex flex-col gap-8 text-lg">
            <div>
                <div class="flex flex-row justify-center w-full">Twili Recipes</div>
                <Divider class="m-2"/>
            </div>
            <div id="settings" class="flex flex-row gap-2 items-center hover:text-blue-400 cursor-pointer" on:click=go_to_settings>
                <Icon icon=icondata_bi::BiCogRegular/>
                Settings
            </div>
            <div id="dark_mode" class="flex flex-row gap-2 items-center">
                <Icon icon=icondata_bi::BiMoonRegular/>
                Dark mode
                <div class="grow"></div>
                <Switch value=dark_mode/>
            </div>
            <div id="refresh" class="flex flex-row gap-2 items-center hover:text-blue-400 cursor-pointer" on:click=reload>
                <Icon icon=icondata_bi::BiRevisionRegular/>
                Refresh
            </div>
            <div id="about" class="flex flex-row gap-2 items-center hover:text-blue-400 cursor-pointer" on:click=go_to_about>
                <Icon icon=icondata_bi::BiInfoCircleRegular/>
                About
            </div>
        </div>
    }
}
