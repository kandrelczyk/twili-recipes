use leptos::prelude::*;
use leptos::reactive::spawn_local;
use recipes_common::Recipe;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use thaw::*;

use crate::components::utils::group_ingredients;
use crate::components::{invoke, GroupForm, Header};
use crate::error::CommandError;

#[derive(Serialize)]
struct Args {
    recipe: Recipe,
}

#[derive(Clone)]
pub struct Group {
    pub name: RwSignal<String>,
    pub ingredients: Vec<IngredientForm>,
}

#[derive(Clone)]
pub struct IngredientForm {
    pub name: RwSignal<String>,
    pub scale: RwSignal<String>,
    pub quantity: RwSignal<String>,
}

#[component]
pub fn RecipeForm(
    recipe: Recipe,
    #[prop(into)] on_back: Callback<web_sys::MouseEvent>,
    #[prop(into)] on_save: Callback<()>,
) -> impl IntoView {
    let saving = RwSignal::new(false);
    let save_error = RwSignal::new(None);
    let show_error = RwSignal::new(false);

    let save_disabled = Signal::derive(saving);

    let title = recipe.name.clone();

    let groups = RwSignal::new(
        group_ingredients(&recipe)
            .into_iter()
            .map(|(k, _)| Group {
                name: RwSignal::new(k),
                ingredients: vec![],
            })
            .collect::<Vec<Group>>(),
    );

    let delete_group = move |index| {
        groups.update(|g| {
            g.remove(index);
        });
    };

    let add_group = move |_| {
        groups.update(|g| {
            g.push(Group {
                name: RwSignal::new("".to_owned()),
                ingredients: vec![],
            });
        });
    };

    let groups_form = move || {
        groups
            .get()
            .into_iter()
            .enumerate()
            .map(|(i, g)| {
                view! {
                    <GroupForm group=g on_delete=move || delete_group(i)/>
                }
                .into_any()
            })
            .collect::<Vec<AnyView>>()
    };

    let save_callback = move |_| {
        saving.set(true);
        let original_id = recipe.id.clone();
        let original_name = recipe.name.clone();

        spawn_local(async move {
            let args = to_value(&Args {
                recipe: Recipe {
                    id: original_id.clone(),
                    name: original_name.clone(),
                    ingredients: vec![],
                    steps: vec![],
                },
            })
            .unwrap();

            groups
                .get_untracked()
                .into_iter()
                .for_each(|g| log::info!("name: {}", g.name.get_untracked()));

            match invoke("save_recipe_break", args).await {
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
        <main class="h-full w-full overflow-y-auto custom-scroll">
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
            <div class="flex flex-col items-center w-full gap-2 p-4">
                { groups_form }
                <Button on:click=add_group icon=icondata_bi::BiPlusRegular>Add group</Button>
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
