use std::collections::HashSet;

use leptos::*;
use leptos_router::use_navigate;
use recipes_common::Recipe;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use thaw::{
    use_message, Alert, AlertVariant, Button, ButtonVariant, Collapse, CollapseItem, Icon, Input,
    MessageVariant, Modal, TextArea,
};
use wasm_bindgen::prelude::*;

use crate::{
    components::{recipes::PreviewRecipe, Header},
    error::CommandError,
};

#[derive(Serialize)]
struct Args {
    recipe: Recipe,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn EditRecipe(
    recipe_json: String,
    #[prop(into)] go_back: Callback<ev::MouseEvent>,
) -> impl IntoView {
    let navigate = create_rw_signal(use_navigate());
    let message = use_message();
    let manual_edit = create_rw_signal(false);
    let collapse = create_rw_signal(HashSet::from(["".to_string()]));

    let json = create_rw_signal(recipe_json.clone());
    let recipe_json = create_rw_signal(recipe_json);

    let name: RwSignal<String> = create_rw_signal("".to_owned());

    let name_invalid = create_rw_signal(false);
    let loading = create_rw_signal(false);
    let error: RwSignal<Option<CommandError>> = create_rw_signal(None);

    let parsed_recipe: Signal<Result<Recipe, CommandError>> =
        Signal::derive(move || Ok(serde_json::from_str(recipe_json.get().as_str())?));

    let has_error = Signal::derive(move || parsed_recipe.get().is_err());

    let save_json = move |_| {
        manual_edit.set(false);
        recipe_json.set(json.get()); //TODO: there's a small bug here showing message on the console
    };

    let save_recipe = move |_| {
        name_invalid.set(name.get().is_empty());

        if !name_invalid.get() {
            loading.set(true);
            spawn_local(async move {
                parsed_recipe.get_untracked().unwrap().name = Some(name.get_untracked());
                let args = to_value(&Args {
                    recipe: Recipe {
                        name: Some(name.get_untracked()),
                        ..parsed_recipe.get_untracked().unwrap()
                    },
                })
                .unwrap();
                match invoke("save_recipe", args).await {
                    Ok(_) => {
                        message.create(
                            "Saved".to_owned(),
                            MessageVariant::Success,
                            Default::default(),
                        );
                        navigate.get()("/list", Default::default());
                    }
                    Err(err) => error.set(Some(from_value::<CommandError>(err).unwrap())),
                };
                loading.set(false);
            });
        }
    };

    view! {
        <main class="flex flex-col h-full">
            <Header
                button=move || {
                    view! {
                        <Button
                            class="absolute"
                            variant=ButtonVariant::Text
                            round=true
                            on_click=go_back
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
                {move || match parsed_recipe.get() {
                    Ok(recipe) => {
                        view! {
                            <p>
                                <PreviewRecipe recipe/>
                            </p>
                        }
                    }
                    Err(error) => {
                        view! {
                            <p class="mt-24 p-4">
                                <Modal width="90%" title="Edit JSON" show=manual_edit>
                                    <div class="flex flex-col items-center">
                                        <TextArea
                                            attr:style="resize:none; height: 300px"
                                            value=json
                                        />
                                        <div class="grow"></div>
                                        <Button class="mt-4" on_click=save_json>
                                            Save
                                        </Button>
                                    </div>
                                </Modal>
                                <Alert variant=AlertVariant::Error class="text-md mb-8">
                                    LMM returned invalid recipe code and we were not able to parse it.
                                    <Collapse class="max-w-sm mt-8" value=collapse>
                                        <CollapseItem title="Error details" key="error">
                                            <p class="text-sm text-wrap">{error.reason}</p>
                                        </CollapseItem>
                                    </Collapse>
                                </Alert>
                                <div class="mt-8 flex flex-row m-4 bg-[--thaw-background]">
                                    <Button variant=ButtonVariant::Outlined on_click=go_back>
                                        Go back
                                    </Button>
                                    <div class="grow"></div>
                                    <Button
                                        variant=ButtonVariant::Outlined
                                        on:click=move |_| manual_edit.set(true)
                                    >
                                        Edit manually
                                    </Button>
                                </div>
                            </p>
                        }
                    }
                }}
                <div class="grow"></div>
                <div class="px-4 text-sm w-full max-w-lg">
                    Name <Input value=name disabled=loading invalid=name_invalid/>
                </div>
                <Button on:click=save_recipe disabled=has_error loading class="m-4">
                    Save
                </Button>
            </div>
        </main>
    }
}
