use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use template_common::CustomError;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "event"])]
    pub async fn listen(
        event: &str,
        closure: &Closure<dyn Fn(JsValue)>,
    ) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Args {
    error: bool,
}

#[component]
pub fn App() -> impl IntoView {
    let (error, set_error) = create_signal(false);

    let resource = create_resource(error, move |e| async move {
        let args = to_value(&Args { error: e }).unwrap();
        match invoke("command", args).await {
            Ok(result) => Ok(from_value::<String>(result).expect("To parse String")),
            Err(error) => Err(from_value::<CustomError>(error).expect("To parse CustomError")),
        }
    });

    create_local_resource(
        || (),
        move |_| async move {
            invoke("start_events", JsValue::NULL)
                .await
                .expect("To schedule events");

            let cb = Closure::<dyn Fn(JsValue)>::new(move |_| {
                log::info!("Received event"); // will appear in console
            });
            listen("custom_event", &cb)
                .await
                .expect("To create listener");
            cb.forget();
        },
    );

    view! {
        <main class="p-4">
            Return error: <button class="ml-4 rounded outline outline-offset-2 outline-blue-500" on:click=move |_| set_error(!error())> {move || error} </button>
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
