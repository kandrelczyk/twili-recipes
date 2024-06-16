use leptos::*;
use leptos_router::{use_navigate, use_params, Params};
use recipes_common::Recipe;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use thaw::{
    use_message, Alert, AlertVariant, Button, ButtonColor, ButtonVariant, Icon, Modal, Popover,
    PopoverTrigger, PopoverTriggerType, Spinner,
};
use wasm_bindgen::prelude::*;

use crate::{
    components::{ActionsSlot, Header, RecipeEditor, RecipePanels},
    error::CommandError,
};

#[derive(Params, Clone, PartialEq)]
struct RecipeParams {
    filename: String,
}

#[derive(Serialize)]
struct RecipeArgs {
    filename: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn RecipeView() -> impl IntoView {
    let navigate = create_rw_signal(use_navigate());
    let message = use_message();
    let params = use_params::<RecipeParams>();

    let show_modal = create_rw_signal(false);
    let show_error_modal = create_rw_signal(false);
    let delete_error = create_rw_signal(None::<String>);
    let show_editor = create_rw_signal(false);

    let filename = create_rw_signal(
        params
            .get_untracked()
            .expect("Missing param")
            .filename
            .clone(),
    );
    let recipe = create_resource(params, move |f| async move {
        let args = to_value(&RecipeArgs {
            filename: f.expect("Missing filename param").filename,
        })
        .expect("Failed to create params");

        match invoke("get_recipe", args).await {
            Ok(recipe) => Ok(from_value::<Recipe>(recipe).expect("Failed to parse Recipe")),
            Err(error) => {
                Err(from_value::<CommandError>(error).expect("Failed to parse CommandError"))
            }
        }
    });
    let listener = leptos::window_event_listener_untyped("popstate", move |_| {
        if show_editor.get() {
            show_editor.set(false);
        } else {
            navigate.get_untracked()("/list", Default::default())
        }
    });

    on_cleanup(|| listener.remove());

    let delete_recipe = create_action(move |file: &String| {
        let filename = file.clone();
        async move {
            let args = to_value(&RecipeArgs { filename }).expect("Failed to create args");

            match invoke("delete_recipe", args).await {
                Ok(_) => {
                    message.create(
                        "Recipe deleted".to_owned(),
                        thaw::MessageVariant::Success,
                        Default::default(),
                    );
                    navigate.get_untracked()("/list", Default::default());
                }
                Err(error) => {
                    show_error_modal.set(true);
                    delete_error.set(Some(format!(
                        "{:?}",
                        from_value::<CommandError>(error).expect("Failed to parse CommandError")
                    )));
                }
            };
            show_modal.set(false);
        }
    });

    view! {
        <main class="flex flex-col h-full w-full items-center justify-start">
            <Show
                fallback=move || {
                    view! {
                        <RecipeEditor
                            on_back=move |_| show_editor.set(false)
                            on_save=Callback::new(move |_| {show_editor.set(false); recipe.refetch()})
                            recipe=recipe
                            .get()
                            .expect("Recipe is None")
                            .expect("Failed to get recipe")/>
                    }
                }
                when=move || !show_editor.get() || recipe.get().is_none()
            >
                <Header
                    button=move || {
                        view! {
                            <Button
                                class="ml-1"
                                variant=ButtonVariant::Text
                                round=true
                                on:click=move |_| navigate
                                    .get_untracked()("/list", Default::default())
                            >
                                <Icon
                                    width="1.5em"
                                    height="1.5em"
                                    icon=icondata_bi::BiChevronLeftSolid
                                />
                            </Button>
                        }
                            .into_view()
                    }

                    title=move || view! { {recipe.and_then(|r| r.name.clone())} }
                >
                    <ActionsSlot slot>
                        <Show fallback=|| view!{} when=move || recipe.get().is_some()>
                            <Popover
                                class="m-2"
                                trigger_type=PopoverTriggerType::Click
                                placement=thaw::PopoverPlacement::BottomEnd
                            >
                                <PopoverTrigger slot>
                                    <Button class="mr-1" variant=ButtonVariant::Text round=true>
                                        <Icon
                                            width="1.5em"
                                            height="1.5em"
                                            icon=icondata_bi::BiDotsVerticalRegular
                                        />
                                    </Button>
                                </PopoverTrigger>
                                <div class="flex flex-col gap-4 text-lg">
                                    <div
                                        on:click=move |_| show_editor.set(true)
                                        id="settings"
                                        class="flex flex-row gap-2 items-center hover:text-blue-400 cursor-pointer"
                                    >
                                        <Icon icon=icondata_bi::BiEditAltSolid/>
                                        Edit JSON
                                    </div>
                                    <div
                                        on:click=move |_| show_modal.set(true)
                                        id="settings"
                                        class="flex flex-row gap-2 items-center hover:text-blue-400 cursor-pointer"
                                    >
                                        <Icon icon=icondata_bi::BiTrashRegular/>
                                        Delete
                                        <Modal
                                            class="max-w-lg w-[80%]"
                                            title="Are you sure?"
                                            mask_closeable=false
                                            close_on_esc=false
                                            closable=false
                                            show=show_modal
                                        >
                                            <div class="flex px-2 sm:px-8 gap-2">
                                                <Button
                                                    on_click=move |_| show_modal.set(false)
                                                    variant=ButtonVariant::Outlined
                                                >
                                                    Cancel
                                                </Button>
                                                <div class="flex-grow"></div>
                                                <Button
                                                    loading=delete_recipe.pending()
                                                    on:click=move |_| {
                                                        delete_recipe.dispatch(filename.get_untracked())
                                                    }
                                                    color=ButtonColor::Error
                                                >
                                                    Delete
                                                </Button>
                                            </div>
                                        </Modal>
                                        <Modal
                                            class="max-w-lg w-[80%]"
                                            show=show_error_modal
                                            title="Failed to delete recipe"
                                        >
                                            <div class="flex px-2 sm:px-8 gap-2">
                                                <Alert variant=AlertVariant::Error>
                                                    <p>{move || delete_error.get()}</p>
                                                </Alert>
                                            </div>
                                        </Modal>
                                    </div>
                                </div>
                            </Popover>
                        </Show>
                    </ActionsSlot>
                </Header>

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
                                    on:click=move |_| recipe.refetch()
                                    icon=icondata_bi::BiRevisionRegular
                                    variant=ButtonVariant::Outlined
                                >
                                    Try again
                                </Button>
                            </div>
                        }
                    }>
                        {move || {
                            recipe
                                .and_then(|recipe| {
                                    view! { <RecipePanels recipe=recipe.clone()/> }
                                })
                        }}

                    </ErrorBoundary>
                </Suspense>
            </Show>
        </main>
    }
}
