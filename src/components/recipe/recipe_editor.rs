use std::collections::HashSet;

use crate::{components::Header, error::CommandError};
use leptos::*;
use recipes_common::Recipe;
use serde::Serialize;
use serde_json::{from_str, to_string_pretty};
use serde_wasm_bindgen::{from_value, to_value};
use thaw::{Button, ButtonVariant, Collapse, CollapseItem, Icon, Modal, TextArea};
use wasm_bindgen::prelude::*;

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
pub fn RecipeEditor(
    recipe: Recipe,
    #[prop(into)] on_back: Callback<()>,
    #[prop(into)] on_save: Callback<()>,
) -> impl IntoView {
    let saving = create_rw_signal(false);
    let save_error = create_rw_signal(None);
    let show_error = create_rw_signal(false);
    let recipe_json =
        create_rw_signal(to_string_pretty(&recipe).expect("Failed to deserialize recipe"));
    let invalid_json =
        Signal::derive(move || from_str::<Recipe>(recipe_json.get().as_str()).is_err());
    let collapse = create_rw_signal(HashSet::from(["".to_string()]));

    let title = recipe.name.clone();

    let save_callback = move |_| {
        saving.set(true);
        let original_id = recipe.id.clone();
        let original_name = recipe.name.clone();
        spawn_local(async move {
            let parsed_recipe: Recipe = serde_json::from_str(recipe_json.get_untracked().as_str())
                .expect("Failed to parse recipe");

            let args = to_value(&Args {
                recipe: Recipe {
                    id: original_id.clone(),
                    name: original_name.clone(),
                    ingredients: parsed_recipe.ingredients,
                    steps: parsed_recipe.steps,
                },
            })
            .unwrap();

            match invoke("save_recipe", args).await {
                Ok(_) => {
                    Callable::call(&on_save, ());
                }
                Err(error) => {
                    save_error.set(Some(
                        from_value::<CommandError>(error).expect("Failed to parse CommandError"),
                    ));
                    show_error.set(true);
                }
            }
            saving.set(false);
        });
    };
    view! {
        <main class="flex flex-col h-full w-full items-center justify-start">
            <Header
                button=move || {
                    view! {
                        <Button class="ml-1 absolute" variant=ButtonVariant::Text round=true disabled=saving>
                            <Icon
                                width="1.5em"
                                height="1.5em"
                                icon=icondata_bi::BiChevronLeftSolid
                                on:click=move |_| Callable::call(&on_back, ())
                            />
                        </Button>
                    }
                        .into_view()
                }

                title=move || title.clone()
            />
            <Modal title="Save Error" width="300px" show=show_error>
                <p class="text-md mb-4">
                    Failed to save recipe.
                </p>
                <Collapse value=collapse>
                    <CollapseItem title="Error details" key="error">
                        <p class="text-sm break-all text-wrap">
                            {move || save_error.get().unwrap().reason}
                        </p>
                    </CollapseItem>
                </Collapse>
            </Modal>
            <div class="flex flex-col items-center w-full h-full p-4">
                <TextArea
                    class="w-full h-full"
                    attr:style="resize:none"
                    value=recipe_json
                    invalid=invalid_json
                />
                <Button on:click=save_callback disabled=invalid_json loading=saving class="mt-4">
                    Save
                </Button>
            </div>
        </main>
    }
}
