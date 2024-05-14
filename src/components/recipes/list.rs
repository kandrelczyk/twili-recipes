use leptos::*;
use leptos_router::use_navigate;
use recipes_common::{ListEntry, Recipe};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use thaw::{
    Alert, AlertVariant, Button, ButtonVariant, Drawer, DrawerMount, DrawerPlacement, Icon, Input,
    Spinner,
};
use wasm_bindgen::prelude::*;

use crate::components::recipes::{Header, ListItem, Menu};
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
pub fn List(dark_mode: RwSignal<bool>) -> impl IntoView {
    let search = create_rw_signal(String::from(""));
    let show_menu = create_rw_signal(false);
    let reload_count = create_rw_signal(0);
    let navigate = use_navigate();

    let add_recipe = move |_| navigate("/add", Default::default());

    let recipes = create_resource(reload_count, move |_| async move {
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
        <main class="flex flex-col h-full w-full items-center justify-start">
            <Header
                button=move || {
                    view! {
                        <Button
                            class="ml-1 absolute"
                            variant=ButtonVariant::Text
                            round=true
                            on_click=move |_| show_menu.set(true)
                        >
                            <Icon width="1.5em" height="1.5em" icon=icondata_bi::BiMenuRegular/>
                        </Button>
                    }
                }

                title=move || {
                    view! {
                        <Input value=search class="w-1/2" placeholder="Search..."/>
                        <Drawer
                            class="sm:w-2/5 w-4/5 max-w-sm"
                            show=show_menu
                            mount=DrawerMount::None
                            placement=DrawerPlacement::Left
                        >
                            <Menu dark_mode reload_signal=reload_count show_menu/>
                        </Drawer>
                    }
                }
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
                                on_click=move |_| reload_count.set(reload_count.get() + 1)
                                icon=icondata_bi::BiRevisionRegular
                                variant=ButtonVariant::Outlined
                            >
                                Try again
                            </Button>
                        </div>
                    }
                }>
                    <div class="h-full w-full bg-[url('/public/background.png')]">
                        <div class="w-full flex flex-row justify-center p-4">
                            <div class="flex flex-row flex-wrap gap-4 pt-4 justify-around">
                                {move || {
                                    recipes
                                        .and_then(|response| {
                                            if response.is_empty() {
                                                view!(<p>"You don't have any recipes yet." </p>).into_view()
                                            } else {
                                            response
                                                .iter()
                                                .filter(|recipe| {
                                                    recipe
                                                        .name
                                                        .to_lowercase()
                                                        .contains(&search.get().to_lowercase())
                                                })
                                                .map(|recipe| view! { <ListItem item=recipe.clone()/> })
                                                .collect_view()
                                            }
                                        })
                                }}

                            </div>
                        </div>
                    </div>
                </ErrorBoundary>
            </Suspense>
            <Button
                on:click=add_recipe
                circle=true
                icon=icondata_bi::BiPlusRegular
                class="fixed bottom-0 right-0 m-6 text-4xl p-4"
            />
        </main>
    }
}
