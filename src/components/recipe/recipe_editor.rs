use crate::{components::Header, error::CommandError};
use leptos::prelude::*;
use leptos::{spawn::spawn_local, web_sys};
use recipes_common::Recipe;
use serde::Serialize;
use serde_json::{from_str, to_string_pretty};
use serde_wasm_bindgen::{from_value, to_value};
use thaw::*;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct Args {
    recipe: Recipe,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn RecipeEditor(
    recipe: Recipe,
    #[prop(into)] on_back: Callback<web_sys::MouseEvent>,
    #[prop(into)] on_save: Callback<()>,
) -> impl IntoView {
    let saving = RwSignal::new(false);
    let save_error = RwSignal::new(None);
    let show_error = RwSignal::new(false);
    let recipe_json =
        RwSignal::new(to_string_pretty(&recipe).expect("Failed to deserialize recipe"));
    let invalid_json =
        Signal::derive(move || from_str::<Recipe>(recipe_json.get().as_str()).is_err());

    let save_disabled = Signal::derive(move || saving() || invalid_json());

    let title = recipe.name.clone();

    let save_callback = move |_| {
        saving.set(true);
        let original_id = recipe.id.clone();
        let original_name = recipe.name.clone();
        spawn_local(async move {
            let parsed_recipe: Recipe = serde_json::from_str(recipe_json.get_untracked().as_str())
                .expect("Failed to parse recipe");

            let args = to_value(&Args {
                recipe: Recipe {
                    id: original_id.clone(),
                    name: original_name.clone(),
                    ingredients: parsed_recipe.ingredients,
                    steps: parsed_recipe.steps,
                },
            })
            .unwrap();

            match invoke("save_recipe", args).await {
                Ok(_) => on_save.run(()),
                Err(error) => {
                    save_error.set(Some(
                        from_value::<CommandError>(error).expect("Failed to parse CommandError"),
                    ));
                    show_error.set(true);
                }
            }
        });
    };
    view! {
        <main class="flex flex-col h-full w-full items-center justify-start">
            <Header
                button=move || {
                    view! {
                        <Button
                            on_click=move |e| on_back.run(e)
                            icon=icondata_bi::BiChevronLeftSolid
                            class="ml-1 absolute"
                            appearance=ButtonAppearance::Subtle
                            shape=ButtonShape::Circular
                            disabled=saving
                        />
                    }
                        .into_any()
                }

                title=move || title.clone()
            />
            <Dialog open=show_error>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>"Save Error"</DialogTitle>
                        <DialogContent>
                            <p class="text-md mb-4">Failed to save recipe.</p>
                            <Accordion collapsible=true>
                                <AccordionItem value="error">
                                    <AccordionHeader slot>"Error details"</AccordionHeader>
                                    <p class="text-sm break-all text-wrap">
                                        {move || save_error.get().unwrap().reason}
                                    </p>
                                </AccordionItem>
                            </Accordion>
                        </DialogContent>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
            <div class="flex flex-col items-center w-full h-full p-4">
                <Textarea class="w-full h-full" attr:style="resize:none" value=recipe_json />
                // invalid=invalid_json
                <Button
                    on:click=save_callback
                    disabled=save_disabled
                    appearance=ButtonAppearance::Primary
                    class="mt-4"
                >
                    Save
                </Button>
            </div>
        </main>
    }
}
