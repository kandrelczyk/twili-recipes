use leptos::*;

use leptos_router::use_navigate;
use recipes_common::{Config, RecipesSource};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use thaw::{use_message, Button, ButtonVariant, Divider, Icon, Input, Spinner, Switch};
use wasm_bindgen::prelude::*;

use crate::{components::Header, error::CommandError};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct Args {
    config: Config,
}

#[component]
pub fn Settings(init: bool) -> impl IntoView {
    let navigate = create_rw_signal(use_navigate());
    let message = use_message();
    let loading = create_rw_signal(false);
    let command_error: RwSignal<Option<CommandError>> = create_rw_signal(None);

    let has_config = create_rw_signal(init);

    let llm_token = create_rw_signal("".to_owned());
    let llm_token_invalid = create_rw_signal(false);

    let cloud_storage = create_rw_signal(false);
    let cloud_uri = create_rw_signal("".to_owned());
    let cloud_uri_invalid = create_rw_signal(false);

    let cloud_username = create_rw_signal("".to_owned());
    let cloud_username_invalid = create_rw_signal(false);

    let cloud_pass = create_rw_signal("".to_owned());
    let cloud_pass_invalid = create_rw_signal(false);

    if !init {
        let listener = leptos::window_event_listener_untyped("popstate", move |_| {
            navigate.get_untracked()("/list", Default::default())
        });
        on_cleanup(|| listener.remove());
        spawn_local(async move {
            match invoke("get_config", JsValue::NULL).await {
                Ok(config) => {
                    let config: Config = from_value(config).unwrap();
                    cloud_storage.set(matches![config.recipes_source, RecipesSource::Cloud]);
                    llm_token.set(config.ai_token);
                    cloud_uri.set(config.cloud_uri);
                    cloud_username.set(config.cloud_username);
                    cloud_pass.set(config.cloud_pass);
                    has_config.set(true);
                }
                Err(err) => command_error.set(Some(from_value::<CommandError>(err).unwrap())),
            };
        });
    }

    let submit = move |_| {
        llm_token_invalid.set(llm_token.get_untracked().is_empty());
        if cloud_storage.get() {
            cloud_uri_invalid.set(cloud_uri.get_untracked().is_empty());
            cloud_username_invalid.set(cloud_username.get_untracked().is_empty());
            cloud_pass_invalid.set(cloud_pass.get_untracked().is_empty());
        } else {
            cloud_uri_invalid.set(false);
            cloud_username_invalid.set(false);
            cloud_pass_invalid.set(false);
        }

        if !llm_token_invalid.get()
            && !cloud_uri_invalid.get()
            && !cloud_username_invalid.get()
            && !cloud_pass_invalid.get()
        {
            loading.set(true);
            spawn_local(async move {
                let args = to_value(&Args {
                    config: Config {
                        ai_token: llm_token.get(),
                        cloud_uri: cloud_uri.get(),
                        cloud_username: cloud_username.get(),
                        cloud_pass: cloud_pass.get(),
                        recipes_source: match cloud_storage.get() {
                            true => RecipesSource::Cloud,
                            false => RecipesSource::Local,
                        },
                        ..Default::default()
                    },
                })
                .unwrap();
                match invoke("save_config", args).await {
                    Ok(_) => {
                        navigate.get_untracked()("/", Default::default());
                    }
                    Err(err) => command_error.set(Some(from_value::<CommandError>(err).unwrap())),
                };
                message.create(
                    "Saved".to_owned(),
                    thaw::MessageVariant::Success,
                    Default::default(),
                );
            });
        }
    };

    view! {
        <main class="flex flex-col h-full w-full items-center justify-start">
            <Header
                button=move || {
                    if init {
                        view! { "" }.into_view()
                    } else {
                        view! {
                            <Button
                                class="ml-1 absolute"
                                variant=ButtonVariant::Text
                                round=true
                                on:click=move |_| {
                                    navigate.get_untracked()("/list", Default::default())
                                }
                            >

                                <Icon
                                    width="1.5em"
                                    height="1.5em"
                                    icon=icondata_bi::BiChevronLeftSolid
                                />
                            </Button>
                        }
                            .into_view()
                    }
                }

                title=move || { if init { "Initial setup" } else { "Settings" } }
            />
            {move || {
                if init || has_config.get() {
                    view! {
                        <div class="flex flex-col items-center h-full w-full bg-[url('/public/background.png')]">
                            <div class="p-2 w-full max-w-xl h-full">
                                <div id="api_token" class="p-1 text-sm w-full">
                                    ChatGPT API Token
                                    <Input
                                        value=llm_token
                                        disabled=loading
                                        invalid=llm_token_invalid
                                    />
                                </div>
                                <Divider/>
                                <div id="recipes_source" class="p-1 mt-4 text-sm w-full flex flex-col gap-1">
                                    "Store in NextCloud"
                                    <Switch value=cloud_storage />
                                </div>
                                <Show when=move || cloud_storage.get()>
                                <div id="cloud_uri" class="p-1 mt-4 text-sm w-full">
                                    Nextcloud URI
                                    <Input
                                        value=cloud_uri
                                        disabled=loading
                                        invalid=cloud_uri_invalid
                                    />
                                </div>
                                <div id="cloud_username" class="p-1 mt-4 text-sm w-full">
                                    Nextcloud username
                                    <Input
                                        value=cloud_username
                                        disabled=loading
                                        invalid=cloud_username_invalid
                                    />
                                </div>
                                <div id="cloud_pass" class="p-1 mt-4 text-sm w-full">
                                    Nextcloud password
                                    <Input
                                        value=cloud_pass
                                        disabled=loading
                                        invalid=cloud_pass_invalid
                                    />
                                </div>
                                </Show>
                            </div>
                            <div class="grow"></div>
                            <Button on:click=submit loading class="m-4">
                                Save
                            </Button>
                        </div>
                    }
                        .into_view()
                } else {
                    view! {
                        <div class="flex flex-col h-full justify-center">
                            <Spinner/>
                        </div>
                    }
                        .into_view()
                }
            }}

        </main>
    }
}
