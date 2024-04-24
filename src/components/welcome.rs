use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use thaw::Spinner;
use wasm_bindgen::prelude::*;

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
pub fn Welcome() -> impl IntoView {
    let initialized = create_resource(
        || (),
        |_| async move {
            match invoke("initialize", JsValue::NULL).await {
                Ok(success) => {
                    if from_value(success).expect("Wrong response from command") {
                        window().location().set_href("/list").expect("");
                    } else {
                        window().location().set_href("/settings").expect("");
                    }
                    Ok(())
                }
                Err(err) => Err(from_value::<CommandError>(err).unwrap()),
            }
        },
    );

    view! {
        <main class="p-4 flex justify-center items-center w-full h-full">
            <Suspense fallback=move || {
                view! { <Spinner/> }
            }>
                <ErrorBoundary fallback=|errors| {
                    view! {
                        <p class="errors">
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| { e.to_string() })
                                    .collect_view()
                            }}
                        </p>
                    }
                }>
                    {move || initialized}
                </ErrorBoundary>
            </Suspense>

        </main>
    }
}
