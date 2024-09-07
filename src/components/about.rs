use leptos::prelude::*;
use leptos::spawn::spawn_local;
use leptos_router::hooks::use_navigate;
use serde_wasm_bindgen::from_value;
use thaw::{Button, ButtonAppearance, ButtonShape, Icon, Text};
use wasm_bindgen::prelude::*;

use crate::components::Header;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn About() -> impl IntoView {
    let navigate = RwSignal::new(use_navigate());

    let version = RwSignal::new("".to_owned());

    spawn_local(async move {
        version.set(from_value::<String>(invoke("get_version", JsValue::NULL).await).unwrap());
    });
    view! {
        <div class="w-full h-full">
            <Header
                button=move || {
                    view! {
                        <Button
                            class="ml-1 absolute"
                            appearance=ButtonAppearance::Subtle
                            shape=ButtonShape::Circular
                            icon=icondata_bi::BiChevronLeftSolid
                            on:click=move |_| {
                                navigate.get_untracked()("/list", Default::default())
                            }
                        ></Button>
                    }
                        .into_view()
                }

                title=move || "About".to_owned()
            />
            <div class="flex flex-col mt-8 gap-3 items-center justify-center">
                <Text class="text-xl mb-8">"Twili Recipes version "{move || version.get()}</Text>

                <Icon width="2em" height="2em" icon=icondata_bi::BiGitlab />
                <Text>
                    "Code available on "
                    <a
                        class="text-blue-500"
                        target="_blank"
                        href="https://gitlab.com/cristofa/twili-recipes"
                    >
                        gitlab
                    </a>
                </Text>
                <Text class="mt-8">
                    "Logo based on "
                    <a
                        class="text-blue-500"
                        target="_blank"
                        href="https://github.com/atisawd/boxicons"
                    >
                        BoxIcons
                    </a>
                </Text>
            </div>
        </div>
    }
}
