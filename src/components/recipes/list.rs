use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use thaw::Button;

use crate::error::CommandError;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Args {
    error: bool,
}

#[component]
pub fn List() -> impl IntoView {
    let (error, set_error) = create_signal(false);

    let resource = create_resource(error, move |e| async move {
        let args = to_value(&Args { error: e }).unwrap();
        match invoke("command", args).await {
            Ok(result) => Ok(from_value::<String>(result).expect("To parse String")),
            Err(error) => Err(from_value::<CommandError>(error).expect("To parse CustomError")),
        }
    });

    view! {
        <main class="p-4">
            Return error: <Button class="ml-4" on:click=move |_| set_error(!error())> {move || error} </Button>
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
