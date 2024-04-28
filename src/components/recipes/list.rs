use leptos::*;
use recipes_common::Recipe;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use crate::error::CommandError;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Args {
    recipe: Recipe,
}

#[component]
pub fn List() -> impl IntoView {
    let resource = create_resource(
        || (),
        move |_| async move {
            let args = to_value(&Args {
                recipe: Recipe {
                    name: Some("test".to_owned()),
                    ingredients: Vec::new(),
                    steps: Vec::new(),
                },
            })
            .unwrap();
            match invoke("save_recipe", args).await {
                Ok(_) => Ok("Ok Response".to_owned()),
                Err(error) => Err(from_value::<CommandError>(error).expect("To parse CustomError")),
            }
        },
    );

    view! {
        <main class="p-4">
            Return error:
                        <Suspense fallback=move || {
                            view! {
                                <p>
                                    Loading...
                                </p>
                            }
                        }>
                            <ErrorBoundary fallback=|errors| {
                                view! {<p>
                                        {move || {
                                            errors
                                                .get()
                                                .into_iter()
                                                .map(|(_, e)| {e.to_string()})
                                                .collect_view()
                                        }}
                                    </p>
                                }
                            }>

                                {move || {
                                   resource
                                        .and_then(|response| { view!{ <p>{response}</p>}})
                                    }
                                }
                            </ErrorBoundary>
                        </Suspense>

        </main>
    }
}
