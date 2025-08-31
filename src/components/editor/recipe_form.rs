use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_use::use_media_query;
use recipes_common::{Ingredient, Recipe, Step};
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use thaw::*;

use crate::components::utils::group_ingredients;
use crate::components::{invoke, GroupForm, Header, StepForm};
use crate::error::CommandError;

#[derive(Serialize)]
struct Args {
    recipe: Recipe,
}

#[derive(Clone)]
pub struct Group {
    pub name: RwSignal<String>,
    pub ingredients: RwSignal<Vec<IngredientValue>>,
}

#[derive(Clone)]
pub struct StepValue {
    pub desc: RwSignal<String>,
    pub time: RwSignal<u32>,
}

#[derive(Clone)]
pub struct IngredientValue {
    pub name: RwSignal<String>,
    pub scale: RwSignal<String>,
    pub quantity: RwSignal<f32>,
}

#[component]
pub fn RecipeForm(
    recipe: Recipe,
    #[prop(into)] on_back: Callback<web_sys::MouseEvent>,
    #[prop(into)] on_save: Callback<()>,
) -> impl IntoView {
    let saving = RwSignal::new(false);
    let save_error = RwSignal::<Option<CommandError>>::new(None);
    let show_error = RwSignal::new(false);

    let title = recipe.name.clone();

    let is_large_screen = use_media_query("(min-width: 600px)");

    let add_button_class = Signal::derive(move || {
        is_large_screen.with(|large| {
            if *large {
                "flex flex-col items-center w-full gap-2 p-4"
            } else {
                "flex flex-col items-start w-full gap-2 p-4"
            }
        })
    });

    let groups = RwSignal::new(
        group_ingredients(&recipe)
            .into_iter()
            .map(|(k, v)| Group {
                name: RwSignal::new(k),
                ingredients: RwSignal::new(
                    v.into_iter()
                        .map(|i| IngredientValue {
                            name: RwSignal::new(i.name),
                            quantity: RwSignal::new(i.quantity),
                            scale: RwSignal::new(i.scale),
                        })
                        .collect(),
                ),
            })
            .collect::<Vec<Group>>(),
    );
    let steps = RwSignal::new(
        recipe
            .steps
            .into_iter()
            .map(|s| StepValue {
                desc: RwSignal::new(s.desc),
                time: RwSignal::new(s.time),
            })
            .collect::<Vec<StepValue>>(),
    );

    let delete_group = move |index| {
        groups.update(|g| {
            g.remove(index);
        });
    };

    let delete_step = move |index| {
        steps.update(|s| {
            s.remove(index);
        });
    };

    let add_group = move |_| {
        groups.update(|g| {
            g.push(Group {
                name: RwSignal::new("".to_owned()),
                ingredients: RwSignal::new(vec![]),
            });
        });
    };

    let add_step = move |_| {
        steps.update(|s| {
            s.push(StepValue {
                desc: RwSignal::new("".to_owned()),
                time: RwSignal::new(0),
            });
        });
    };

    let groups_form = move || {
        groups
            .get()
            .into_iter()
            .enumerate()
            .map(|(i, g)| {
                view! { <GroupForm group=g on_delete=move || delete_group(i) /> }.into_any()
            })
            .collect::<Vec<AnyView>>()
    };

    let steps_form = move || {
        steps
            .get()
            .into_iter()
            .enumerate()
            .map(|(i, s)| {
                view! { <StepForm index=i step=s on_delete=move || delete_step(i) /> }.into_any()
            })
            .collect::<Vec<AnyView>>()
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
            <FieldContextProvider>
                <div class=add_button_class>
                    {groups_form} <Button on:click=add_group icon=icondata_bi::BiPlusRegular>
                        Add group
                    </Button> <div class="flex items-center mt-4 gap-1">
                        <Icon class="min-w-[16px]" icon=icondata_bi::BiInfoCircleRegular />
                        <Text>
                            <i>
                                "Write '[ingredient]' in step body to show correct quantity when viewing the recipe"
                            </i>
                        </Text>
                    </div> {steps_form} <Button on:click=add_step icon=icondata_bi::BiPlusRegular>
                        Add step
                    </Button>
                    <Button
                        on_click={
                            let field_context = FieldContextInjection::expect_context();
                            move |_| {
                                saving.set(true);
                                if field_context.validate() {
                                    let original_id = recipe.id.clone();
                                    let original_name = recipe.name.clone();
                                    spawn_local(async move {
                                        let args = to_value(
                                                &Args {
                                                    recipe: Recipe {
                                                        id: original_id.clone(),
                                                        name: original_name.clone(),
                                                        ingredients: groups
                                                            .get()
                                                            .iter()
                                                            .flat_map(|g| {
                                                                g.ingredients
                                                                    .get()
                                                                    .into_iter()
                                                                    .map(|i| Ingredient {
                                                                        group: Some(g.name.get()).filter(|name| name.is_empty()),
                                                                        name: i.name.get(),
                                                                        quantity: i.quantity.get(),
                                                                        scale: i.scale.get(),
                                                                    })
                                                                    .collect::<Vec<Ingredient>>()
                                                            })
                                                            .collect(),
                                                        steps: steps
                                                            .get()
                                                            .iter()
                                                            .map(|s| {
                                                                Step {
                                                                    desc: s.desc.get(),
                                                                    time: s.time.get(),
                                                                }
                                                            })
                                                            .collect::<Vec<Step>>(),
                                                    },
                                                },
                                            )
                                            .unwrap();
                                        groups
                                            .get_untracked()
                                            .into_iter()
                                            .for_each(|g| {
                                                log::info!("name: {}", g.name.get_untracked())
                                            });
                                        match invoke("save_recipe_break", args).await {
                                            Ok(_) => on_save.run(()),
                                            Err(error) => {
                                                save_error
                                                    .set(
                                                        Some(
                                                            from_value::<CommandError>(error)
                                                                .expect("Failed to parse CommandError"),
                                                        ),
                                                    );
                                                show_error.set(true);
                                            }
                                        }
                                    });
                                } else {
                                    saving.set(false);
                                }
                            }
                        }
                        shape=ButtonShape::Circular
                        appearance=ButtonAppearance::Primary
                        class="fixed bottom-4 right-4"
                    >
                        Save
                    </Button>
                </div>
            </FieldContextProvider>
        </main>
    }
}
