use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use recipes_common::ListEntry;
use serde_wasm_bindgen::from_value;
use thaw::*;
use wasm_bindgen::prelude::*;

use crate::components::recipes::{AppMenu, ListItem};
use crate::components::Header;
use crate::error::CommandError;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn List() -> impl IntoView {
    let search = RwSignal::new(String::from(""));
    let show_menu = RwSignal::new(false);
    let reload_count = RwSignal::new(0);
    let navigate = use_navigate();

    let add_recipe = move |_| navigate("/add", Default::default());

    let recipes = AsyncDerived::new_unsync(move || async move {
        reload_count.get();
        match invoke("list_recipes", JsValue::NULL).await {
            Ok(list) => {
                Ok(from_value::<Vec<ListEntry>>(list).expect("Failed to parse Vec<ListEntry>"))
            }
            Err(error) => {
                Err(from_value::<CommandError>(error).expect("Failed to parse CustomError"))
            }
        }
    });

    view! {
        <div class="flex flex-col h-full w-full items-center justify-start">
            <Header
                button=move || {
                    view! {
                        <Button
                            class="ml-1 absolute"
                            shape=ButtonShape::Circular
                            on_click=move |_| show_menu.set(true)
                            icon=icondata_bi::BiMenuRegular
                        ></Button>
                    }
                }

                title=move || {
                    view! { <Input value=search class="w-1/2" placeholder="Search..." /> }
                }
            />

            <OverlayDrawer
                class="sm:w-2/5 w-4/5 max-w-sm"
                open=show_menu
                position=DrawerPosition::Left
            >
                <AppMenu reload_signal=reload_count show_menu />
            </OverlayDrawer>

            <Suspense fallback=move || {
                view! {
                    <div class="w-full mt-[45vh] flex flex-row justify-center items-center">
                        <Spinner />
                    </div>
                }
            }>
                <ErrorBoundary fallback=move |errors| {
                    view! {
                        <div class="flex max-w-4xl p-4 flex-col text-wrap break-all mt-[40vh] justify-center">
                            <MessageBar
                                intent=MessageBarIntent::Error
                                layout=MessageBarLayout::Multiline
                            >
                                <MessageBarBody>
                                    <MessageBarTitle>
                                        <p class="text-lg">"Failed to load recipes"</p>
                                    </MessageBarTitle>
                                    <p>
                                        {move || {
                                            errors
                                                .get()
                                                .into_iter()
                                                .map(|(_, e)| { e.to_string() })
                                                .collect_view()
                                        }}

                                    </p>
                                </MessageBarBody>
                            </MessageBar>
                            <Button
                                class="w-32 mt-2 mx-auto"
                                on_click=move |_| reload_count.set(reload_count.get() + 1)
                                icon=icondata_bi::BiRevisionRegular
                            >
                                Try again
                            </Button>
                        </div>
                    }
                }>
                    <div class="h-screen w-full overflow-y-auto pb-12 recipe-list">
                        <div class="w-full flex flex-row justify-center p-4">
                            <div class="flex flex-row flex-wrap gap-4 pt-4 justify-around">
                                {move || Suspend::new(async move {
                                    recipes
                                        .await
                                        .map(|response| {
                                            if response.is_empty() {
                                                view! { <p>"You don't have any recipes yet."</p> }
                                                    .into_any()
                                            } else {
                                                response
                                                    .iter()
                                                    .filter(|recipe| {
                                                        recipe
                                                            .name
                                                            .to_lowercase()
                                                            .contains(&search.get().to_lowercase())
                                                    })
                                                    .map(|recipe| view! { <ListItem item=recipe.clone() /> })
                                                    .collect_view()
                                                    .into_any()
                                            }
                                        })
                                })}
                            </div>
                        </div>
                    </div>
                </ErrorBoundary>
            </Suspense>
            <Button
                on:click=add_recipe
                shape=ButtonShape::Circular
                icon=icondata_bi::BiPlusRegular
                appearance=ButtonAppearance::Primary
                size=ButtonSize::Large
                class="fixed bottom-8 right-8"
            />
        </div>
    }
}
