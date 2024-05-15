use leptos::*;
use leptos_router::use_navigate;
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
    let navigate = create_rw_signal(use_navigate());

    let initialized = create_resource(
        || (),
        move |_| async move {
            match invoke("initialize", JsValue::NULL).await {
                Ok(success) => {
                    if from_value(success).expect("Wrong response from command") {
                        navigate.get_untracked()("/list", Default::default());
                    } else {
                        navigate.get_untracked()("/initialize", Default::default());
                    }
                    Ok(())
                }
                Err(err) => Err(from_value::<CommandError>(err).unwrap()),
            }
        },
    );

    view! {
        <main class="p-4 flex justify-center items-center w-full h-full">
            <Suspense fallback=|| {
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
