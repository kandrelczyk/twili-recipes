use std::collections::HashSet;

use leptos::*;
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use thaw::{Button, ButtonVariant, Collapse, CollapseItem, Icon, Modal, TextArea};
use wasm_bindgen::prelude::*;

use crate::{
    components::{recipes::EditRecipe, Header},
    error::CommandError,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct Args {
    recipe: String,
}

#[component]
pub fn AddRecipe() -> impl IntoView {
    let recipe_str = create_rw_signal("".to_owned());

    let error: RwSignal<Option<CommandError>> = create_rw_signal(None);
    let show_error = Signal::derive(move || error.get().is_some());
    let collapse = create_rw_signal(HashSet::from(["".to_string()]));
    let recipe: RwSignal<Option<String>> = create_rw_signal(None);

    let recipe_invalid = create_rw_signal(false);
    let loading = create_rw_signal(false);

    let navigate = create_rw_signal(use_navigate());

    let submit = move |_| {
        recipe_invalid.set(recipe_str.get().is_empty());

        if !recipe_invalid.get() {
            loading.set(true);
            error.set(None);
            spawn_local(async move {
                let args = to_value(&Args {
                    recipe: recipe_str.get_untracked(),
                })
                .unwrap();
                match invoke("parse_recipe", args).await {
                    Ok(result) => {
                        recipe.set(Some(from_value(result).unwrap()));
                    }
                    Err(err) => error.set(Some(from_value::<CommandError>(err).unwrap())),
                };
                loading.set(false);
            });
        }
    };

    let listener = leptos::window_event_listener_untyped("popstate", move |_| {
        if recipe.get().is_none() {
            navigate.get_untracked()("/list", Default::default())
        } else {
            recipe.set(None);
        }
    });

    on_cleanup(|| listener.remove());

    view! {
        <Show
            when=move || recipe.get().is_none()
            fallback=move || {
                view! {
                    <EditRecipe
                        recipe_json=recipe.get().unwrap()
                        go_back=move |_| recipe.set(None)
                    />
                }
            }
        >

            <Modal title="LLM Error" width="300px" show=show_error.get()>
                <p class="text-md mb-4">
                    Failed to call LLM service. Check your configuration or try again later
                </p>
                <Collapse value=collapse>
                    <CollapseItem title="Error details" key="error">
                        <p class="text-sm break-all text-wrap">
                            {move || error.get().unwrap().reason}
                        </p>
                    </CollapseItem>
                </Collapse>
            </Modal>
            <main class="flex flex-col items-center h-full">
                <Header
                    button=move || {
                        view! {
                            <Button
                                class="absolute"
                                variant=ButtonVariant::Text
                                round=true
                                on:click=move |_| navigate.get()("/list", Default::default())
                            >
                                <Icon
                                    width="1.5em"
                                    height="1.5em"
                                    icon=icondata_bi::BiChevronLeftSolid
                                />
                            </Button>
                        }
                    }

                    title=move || {
                        view! { Add Recipe }
                    }
                />
                <div class="flex flex-col items-center h-full w-full bg-[url('/public/background.png')]">
                    <div class="p-2 w-full max-w-xl h-full">
                        <div class="p-1 text-sm w-full h-[95%]">
                            Recipe
                            <TextArea
                                class="h-full"
                                value=recipe_str
                                disabled=loading
                                invalid=recipe_invalid
                            />
                        </div>
                    </div>
                    <div class="grow"></div>
                    <Button on:click=submit loading class="m-4">
                        Process
                    </Button>
                </div>
            </main>
        </Show>
    }
}
