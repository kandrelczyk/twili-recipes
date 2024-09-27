use leptos::prelude::*;
use leptos::spawn::spawn_local;
use leptos_router::hooks::use_navigate;
use recipes_common::Recipe;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use thaw::*;
use wasm_bindgen::prelude::*;

use crate::{
    components::{recipes::PreviewRecipe, ActionsSlot, Header},
    error::CommandError,
};

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
pub fn EditRecipe(
    recipe_json: String,
    #[prop(into)] go_back: Callback<web_sys::MouseEvent>,
) -> impl IntoView {
    let navigate = RwSignal::new(use_navigate());
    let manual_edit = RwSignal::new(false);

    let json = RwSignal::new(recipe_json.clone());
    let recipe_json = RwSignal::new(recipe_json);

    let name: RwSignal<String> = RwSignal::new("".to_owned());

    let name_invalid = RwSignal::new(false);
    let loading = RwSignal::new(false);
    let error: RwSignal<Option<CommandError>> = RwSignal::new(None);

    let save_disabled = Signal::derive(move || loading() || name_invalid());
    let toaster = ToasterInjection::expect_context();

    let parsed_recipe: Signal<Result<Recipe, CommandError>> =
        Signal::derive(move || Ok(serde_json::from_str(recipe_json.get().as_str())?));

    let _has_error = Signal::derive(move || parsed_recipe.get().is_err());

    let save_json = move |_| {
        manual_edit.set(false);
        recipe_json.set(json.get()); //TODO: there's a small bug here showing message on the console
    };

    let save_recipe = move |_| {
        name_invalid.set(name.get().is_empty());

        if !name_invalid.get() {
            loading.set(true);
            spawn_local(async move {
                parsed_recipe.get_untracked().unwrap().name = Some(name.get_untracked());
                let args = to_value(&Args {
                    recipe: Recipe {
                        name: Some(name.get_untracked()),
                        ..parsed_recipe.get_untracked().unwrap()
                    },
                })
                .unwrap();
                match invoke("save_recipe", args).await {
                    Ok(_) => {
                        toaster.dispatch_toast(
                            move || {
                                view! {
                                    <Toast>
                                        <ToastTitle>"Saved"</ToastTitle>
                                    </Toast>
                                }
                            },
                            ToastOptions::default()
                                .with_position(ToastPosition::Top)
                                .with_intent(ToastIntent::Success),
                        );
                        navigate.get_untracked()("/list", Default::default());
                    }
                    Err(err) => error.set(Some(from_value::<CommandError>(err).unwrap())),
                };
                loading.set(false);
            });
        }
    };

    view! {
        <main class="flex flex-col h-full">
            <Header
                button=move || {
                    view! {
                        <Button
                            class="absolute"
                            appearance=ButtonAppearance::Subtle
                            shape=ButtonShape::Circular
                            icon=icondata_bi::BiChevronLeftSolid
                            on_click=move |e| go_back.run(e)
                        />
                    }
                }

                title=move || {
                    view! { Add Recipe }
                }
            >
                <ActionsSlot slot>
                    <Show when=move || parsed_recipe.get().is_ok()>
                        <Button
                            class="mr-1"
                            appearance=ButtonAppearance::Subtle
                            shape=ButtonShape::Circular
                            icon=icondata_bi::BiEditAltSolid
                            on:click=move |_| manual_edit.set(true)
                        />
                    </Show>
                </ActionsSlot>
            </Header>
            <Dialog open=manual_edit>
                <DialogSurface class="w-[90%]">
                    <DialogBody>
                        <DialogTitle>"Edit JSON"</DialogTitle>
                        <DialogContent>
                            <div class="flex flex-col items-center">
                                <Textarea attr:style="resize:none; height: 300px; width: 100%" value=json />
                                <div class="grow"></div>
                            </div>
                        </DialogContent>
                        <DialogActions>
                            <Button class="mt-4" appearance=ButtonAppearance::Primary on_click=save_json>
                                Save
                            </Button>
                        </DialogActions>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
            <div class="flex flex-col items-center h-full w-full">
                {move || match parsed_recipe.get() {
                    Ok(recipe) => {
                        view! {
                            <p>
                                <PreviewRecipe recipe />
                            </p>
                        }.into_any()
                    }
                    Err(error) => {
                        view! {
                            <p class="mt-24 p-4">
                                <MessageBar  intent=MessageBarIntent::Error class="text-md mb-8">
                                    "LMM returned invalid recipe code and we were not able to parse it."
                                    <Accordion class="max-w-sm mt-8" collapsible=true>
                                        <AccordionItem value="error">
                                            <AccordionHeader slot>
                                                "ErrorDetails"
                                            </AccordionHeader>
                                            <p class="text-sm text-wrap">{error.reason}</p>
                                        </AccordionItem>
                                    </Accordion>
                                </MessageBar>
                                <div class="mt-8 flex flex-row m-4 bg-[--thaw-background]">
                                    <Button on_click=move |e| go_back.run(e)>
                                        Go back
                                    </Button>
                                    <div class="grow"></div>
                                    <Button
                                        on:click=move |_| manual_edit.set(true)
                                    >
                                        Edit manually
                                    </Button>
                                </div>
                            </p>
                        }.into_any()
                    }
                }} <div class="grow"></div>
                <div class="px-4 text-sm w-full max-w-lg">
                    Name
                    <Input value=name class="w-full" disabled=loading />//TODO invalid
                </div>
                <Button on:click=save_recipe appearance=ButtonAppearance::Primary disabled=save_disabled class="m-4">
                    Save
                </Button>
            </div>
        </main>
    }
}
