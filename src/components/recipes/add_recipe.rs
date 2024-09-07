use leptos::prelude::*;
use leptos::spawn::spawn_local;
use leptos_router::hooks::use_navigate;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use thaw::*;
use wasm_bindgen::prelude::*;

use crate::{
    components::{recipes::EditRecipe, Header},
    error::CommandError,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct Args {
    recipe: String,
}

#[component]
pub fn AddRecipe() -> impl IntoView {
    let recipe_str = RwSignal::new("".to_owned());

    let error: RwSignal<Option<CommandError>> = RwSignal::new(None);
    let show_error = Signal::derive(move || error.get().is_some());
    let recipe: RwSignal<Option<String>> = RwSignal::new(None);

    let recipe_invalid = RwSignal::new(false);
    let loading = RwSignal::new(false);

    let navigate = RwSignal::new(use_navigate());

    let submit = move |_| {
        recipe_invalid.set(recipe_str.get().is_empty());

        if !recipe_invalid.get() {
            loading.set(true);
            error.set(None);
            spawn_local(async move {
                let args = to_value(&Args {
                    recipe: recipe_str.get_untracked(),
                })
                .unwrap();
                match invoke("parse_recipe", args).await {
                    Ok(result) => {
                        recipe.set(Some(from_value(result).unwrap()));
                    }
                    Err(err) => error.set(Some(from_value::<CommandError>(err).unwrap())),
                };
                loading.set(false);
            });
        }
    };

    let listener = window_event_listener_untyped("popstate", move |_| {
        if recipe.get().is_none() {
            navigate.get_untracked()("/list", Default::default())
        } else {
            recipe.set(None);
        }
    });

    on_cleanup(|| listener.remove());

    view! {
        <Show
            when=move || recipe.get().is_none()
            fallback=move || {
                view! {
                    <EditRecipe
                        recipe_json=recipe.get().unwrap()
                        go_back=Callback::new(move |_| recipe.set(None))
                    />
                }
            }
        >

            <Dialog open=show_error.get()>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>"LLM Error"</DialogTitle>
                        <DialogContent>
                            <p class="text-md mb-4">
                                Failed to call LLM service. Check your configuration or try again later
                            </p>
                            <Accordion collapsible=true>
                                <AccordionItem value="error">
                                    <AccordionHeader slot>"Error details"</AccordionHeader>
                                    <p class="text-sm break-all text-wrap">
                                        {move || error.get().unwrap().reason}
                                    </p>
                                </AccordionItem>
                            </Accordion>
                        </DialogContent>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
            <main class="flex flex-col items-center h-screen">
                <Header
                    button=move || {
                        view! {
                            <Button
                                class="absolute"
                                appearance=ButtonAppearance::Subtle
                                shape=ButtonShape::Circular
                                icon=icondata_bi::BiChevronLeftSolid
                                on:click=move |_| navigate.get()("/list", Default::default())
                            ></Button>
                        }
                    }

                    title=move || {
                        view! { Add Recipe }
                    }
                />
                <div class="flex flex-col items-center h-full w-full">
                    <div class="p-2 w-full max-w-xl h-full">
                        <div class="p-1 text-sm w-full h-[95%]">
                            Recipe
                            <Textarea class="h-full block" value=recipe_str disabled=loading />
                        // TODO                                invalid=recipe_invalid
                        </div>
                    </div>
                    <div class="grow"></div>
                    <Button
                        on:click=submit
                        appearance=ButtonAppearance::Primary
                        disabled=loading
                        class="m-4"
                    >
                        Process
                    </Button>
                </div>
            </main>
        </Show>
    }
}
