use leptos::prelude::*;
use leptos::spawn::spawn_local;
use leptos_router::hooks::{use_navigate, use_params};
use leptos_router::params::Params;
use recipes_common::Recipe;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use thaw::*;
use wasm_bindgen::prelude::*;

use crate::{
    components::{ActionsSlot, Header, RecipeEditor, RecipePanels},
    error::CommandError,
};

#[derive(Params, Clone, PartialEq)]
struct RecipeParams {
    filename: Option<String>,
}


#[derive(Serialize)]
struct RecipeArgs {
    filename: String,
}

#[derive(Serialize)]
struct KeepScreenOnArgs {
    enable: bool,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn RecipeView() -> impl IntoView {
    let navigate = RwSignal::new(use_navigate());
    let params = use_params::<RecipeParams>();
    let reload_count = RwSignal::new(0);

    let show_modal = RwSignal::new(false);
    let show_error_modal = RwSignal::new(false);
    let delete_error = RwSignal::new(None::<String>);
    let show_editor = RwSignal::new(false);
    let toaster = ToasterInjection::expect_context();

    let filename = RwSignal::new(
        params
            .get_untracked()
            .expect("Missing param")
            .filename
            .clone(),
    );
    let recipe = AsyncDerived::new_unsync(move || async move {
        reload_count.get();
        invoke(
            "plugin:keep-screen-on|keep_screen_on",
            to_value(&KeepScreenOnArgs { enable: true })
                .expect("Failed to serialize KeepScreenOnArgs"),
        )
        .await
        .unwrap();

        let args = to_value(&RecipeArgs {
            filename: filename().unwrap()
        })
        .expect("Failed to create params");

        match invoke("get_recipe", args).await {
            Ok(recipe) => Ok(from_value::<Recipe>(recipe).expect("Failed to parse Recipe")),
            Err(error) => {
                Err(from_value::<CommandError>(error).expect("Failed to parse CommandError"))
            }
        }
    });

    let listener = window_event_listener_untyped("popstate", move |_| {
        if show_editor.get() {
            show_editor.set(false);
        } else {
            navigate.get_untracked()("/list", Default::default())
        }
    });

    on_cleanup(|| {
        spawn_local(async move {
            invoke(
                "plugin:keep-screen-on|keep_screen_on",
                to_value(&KeepScreenOnArgs { enable: false })
                    .expect("Failed to serialize KeepScreenOnArgs"),
            )
            .await
            .unwrap();
        });
        listener.remove();
    });


    let delete_recipe : Action<String, (), SyncStorage> = Action::new_unsync(move |file: &String| {
        let filename = file.clone();
        async move {
            let args = to_value(&RecipeArgs { filename }).expect("Failed to create args");

            match invoke("delete_recipe", args).await {
                Ok(_) => {
                     toaster.dispatch_toast(view! {
                         <Toast>
                             <ToastTitle>"Recipe deleted"</ToastTitle>
                         </Toast>
                     }.into_any(), ToastOptions::default().with_position(ToastPosition::Top));
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

    let on_select = move |key: String| match key.as_str() {
        "edit" => show_editor.set(true),
        "delete" => show_modal.set(true),
        _ => (),
    };

    view! {
        <div class="flex flex-col h-screen w-full items-center justify-start">
            <Show
                fallback=move || {
                    view! {
                        <RecipeEditor
                            on_back=Callback::new(move |_| show_editor.set(false))
                            on_save=Callback::new(move |_| {
                                show_editor.set(false);
                                reload_count.update(|count: &mut i32| *count += 1);
                            })
                            recipe=recipe
                                .get()
                                .expect("Recipe is None")
                                .expect("Failed to get recipe")
                        />
                    }
                }
                when=move || !show_editor.get() || recipe.get().is_none()
            >
                <Header
                    button=move || {
                        view! {
                            <Button
                                class="ml-1"
                                appearance=ButtonAppearance::Subtle
                                shape=ButtonShape::Circular
                                icon=icondata_bi::BiChevronLeftSolid
                                on:click=move |_| navigate
                                    .get_untracked()("/list", Default::default())
                            />
                        }
                            .into_view()
                    }

                    title=move || Suspend::new(async move { recipe.await.map(|r| r.name.clone()) })
                >
                    <ActionsSlot slot>
                        <Show fallback=|| view! {} when=move || recipe.get().is_some()>
                            <Menu on_select position=MenuPosition::BottomEnd>
                                <MenuTrigger slot>
                                    <Button
                                        class="mr-1"
                                        appearance=ButtonAppearance::Subtle
                                        shape=ButtonShape::Circular
                                        icon=icondata_bi::BiDotsVerticalRegular
                                    />
                                </MenuTrigger>
                                <MenuItem value="edit" icon=icondata_bi::BiEditAltSolid>
                                    "Edit JSON"
                                </MenuItem>
                                <MenuItem value="delete" icon=icondata_bi::BiTrashRegular>
                                    "Delete"
                                </MenuItem>
                                <Dialog mask_closeable=false close_on_esc=false open=show_modal>
                                    <DialogSurface class="max-w-lg mx-[10%]">
                                        <DialogBody>
                                            <DialogTitle>"Are you sure?"</DialogTitle>
                                            <DialogContent>
                                                <div class="flex px-2 sm:px-8 gap-2">
                                                    <Button
                                                        disabled=delete_recipe.pending()
                                                        on_click=move |_| { show_modal.set(false) }
                                                    >
                                                        Cancel
                                                    </Button>
                                                    <div class="flex-grow"></div>
                                                    <Button
                                                        appearance=ButtonAppearance::Primary
                                                        disabled=delete_recipe.pending()
                                                        on:click=move |_| {
                                                            delete_recipe.dispatch(filename.get_untracked().unwrap());
                                                        }
                                                    >
                                                        Delete
                                                    </Button>
                                                </div>
                                            </DialogContent>
                                        </DialogBody>
                                    </DialogSurface>

                                </Dialog>
                                <Dialog class="max-w-lg w-[80%]" open=show_error_modal>
                                    <DialogSurface>
                                        <DialogBody>
                                            <DialogTitle>"Failed to delete recipe"</DialogTitle>
                                            <DialogContent>
                                                <div class="flex px-2 sm:px-8 gap-2">
                                                    <MessageBar intent=MessageBarIntent::Error>
                                                        <p>{move || delete_error.get()}</p>
                                                    </MessageBar>
                                                </div>
                                            </DialogContent>
                                        </DialogBody>
                                    </DialogSurface>
                                </Dialog>
                            </Menu>
                        </Show>
                    </ActionsSlot>
                </Header>

                <Transition fallback=move || {
                    view! {
                        <div class="w-full h-full flex flex-row justify-center items-center">
                            <Spinner />
                        </div>
                    }
                }>
                    <ErrorBoundary fallback=move |errors| {
                        view! {
                            <div class="flex max-w-4xl p-4 flex-col text-wrap break-all h-full justify-center">
                                <MessageBar intent=MessageBarIntent::Error>
                                    <MessageBarBody>
                                        <MessageBarTitle>"Failed to load recipes"</MessageBarTitle>
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
                        {move || Suspend::new(async move {
                            recipe
                                .await
                                .map(|recipe| {
                                    view! { <RecipePanels recipe=recipe.clone() /> }
                                })
                        })}

                    </ErrorBoundary>
                </Transition>
            </Show>
        </div>
    }
}
