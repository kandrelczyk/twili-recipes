use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_use::storage::use_local_storage;
use serde_wasm_bindgen::from_value;
use thaw::{Spinner, Theme};
use wasm_bindgen::prelude::*;

use crate::error::CommandError;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn Welcome() -> impl IntoView {
    let navigate = RwSignal::new(use_navigate());

    //let (dark, _, _) = use_local_storage::<bool, FromToStringCodec>("dark_mode");
    //let theme = Theme::use_rw_theme();
    //if dark() {
    //theme.set(Theme::dark())
    //}
    AsyncDerived::new_unsync(move || async move {
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
    });

    view! {
        <main class="p-4 flex justify-center items-center w-full h-screen">

            <Transition fallback=|| view! { <Spinner /> }>
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
                    <ul>
                        {move || Suspend::new(async move {
                            view! { <Spinner /> }
                        })}
                    </ul>
                </ErrorBoundary>
            </Transition>
        </main>
    }
}
