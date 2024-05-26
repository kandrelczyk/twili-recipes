use leptos::*;
use leptos_router::{use_navigate, use_params, Params};
use recipes_common::Recipe;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use thaw::{Alert, AlertVariant, Button, ButtonVariant, Icon, Spinner};
use wasm_bindgen::prelude::*;

use crate::{
    components::{Header, RecipePanels},
    error::CommandError,
};

#[derive(Params, Clone, PartialEq)]
struct RecipeParams {
    filename: String,
}

#[derive(Serialize)]
struct GetArgs {
    filename: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn RecipeView() -> impl IntoView {
    let navigate = create_rw_signal(use_navigate());
    let params = use_params::<RecipeParams>();

    let filename = params;
    let recipe = create_resource(filename, move |f| async move {
        let args = to_value(&GetArgs {
            filename: f.expect("Missing filename param").filename,
        })
        .expect("Failed to create params");

        match invoke("get_recipe", args).await {
            Ok(recipe) => Ok(from_value::<Recipe>(recipe).expect("Failed to parse Recipe")),
            Err(error) => {
                Err(from_value::<CommandError>(error).expect("Failed to parse CustomError"))
            }
        }
    });

    view! {
        <main class="flex flex-col h-full w-full items-center justify-start">
            <Header
                button=move || {
                    view! {
                        <Button
                            class="ml-1 absolute"
                            variant=ButtonVariant::Text
                            round=true
                            on:click=move |_| navigate.get_untracked()("/list", Default::default())
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

                title=move || view! { {recipe.and_then(|r| r.name.clone())} }
            />

            <Suspense fallback=move || {
                view! {
                    <div class="w-full h-full flex flex-rowl justify-center items-center bg-[url('/public/background.png')]">
                        <Spinner/>
                    </div>
                }
            }>
                <ErrorBoundary fallback=move |errors| {
                    view! {
                        <div class="flex max-w-4xl p-4 flex-col text-wrap break-all h-full justify-center">
                            <Alert variant=AlertVariant::Error title="Failed to load recipes">
                                <p>
                                    {move || {
                                        errors
                                            .get()
                                            .into_iter()
                                            .map(|(_, e)| { e.to_string() })
                                            .collect_view()
                                    }}

                                </p>
                            </Alert>
                            <Button
                                class="w-32 mt-2 mx-auto"
                                on:click=move |_| recipe.refetch()
                                icon=icondata_bi::BiRevisionRegular
                                variant=ButtonVariant::Outlined
                            >
                                Try again
                            </Button>
                        </div>
                    }
                }>
                    {move || {
                        recipe
                            .and_then(|recipe| {
                                view! { <RecipePanels recipe=recipe.clone()/> }
                            })
                    }}

                </ErrorBoundary>
            </Suspense>
        </main>
    }
}
