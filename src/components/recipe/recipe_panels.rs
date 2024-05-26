use leptos::*;
use recipes_common::Recipe;
use thaw::{Button, Icon};

use crate::components::{RecipeIngredients, RecipeStep};

#[component]
pub fn RecipePanels(recipe: Recipe) -> impl IntoView {
    let page = create_rw_signal(0);
    let page_count = recipe.steps.len();

    let multiplier = create_rw_signal(1.0);

    let recipe = store_value(recipe);
    let first_page = Signal::derive(move || page.get() == 0);
    let last_page = Signal::derive(move || page.get() == recipe().steps.len() - 1);

    let step = create_memo(move |_| {
        let mut step = recipe().steps[page.get()].desc.clone();
        recipe().ingredients.iter().for_each(|i| {
            step = step.replace(
                format!("[{}]", i.name).as_str(),
                format!(
                    "{} ({}{})",
                    i.name,
                    i.quantity * multiplier(),
                    match i.scale.is_empty() {
                        true => "".to_owned(),
                        false => format!(" {}", i.scale),
                    }
                )
                .as_str(),
            )
        });
        step
    });
    view! {
        <div class="flex flex-col h-[90%] w-full bg-[url('/public/background.png')]">
            <div class="w-full flex flex-col overflow-scroll items-center">
                {move || {
                    if page.get() == 0 {
                        view! { <RecipeIngredients recipe multiplier=multiplier/> }.into_view()
                    } else {
                        view! {
                            <div class="w-full text-xl text-center p-4">
                                Step {page} / {page_count - 1}
                            </div>
                            <RecipeStep step=step.get()/>
                        }
                            .into_view()
                    }
                }}

            </div>
            <div class="grow"></div>
            <div class="flex flex-row sm:gap-4 gap-2 m-4 justify-center items-center">
                <Button on_click=move |_| page.update(|p| *p -= 1) disabled=first_page>
                    <Icon width="1.5em" height="1.5em" icon=icondata_bi::BiChevronLeftSolid/>
                </Button>
                {move || {
                    (0..page_count)
                        .map(|p| {
                            view! {
                                <p>

                                    <Icon
                                        width="1em"
                                        height="1em"
                                        icon=if p == page.get() as usize {
                                            icondata_bi::BiCircleSolid
                                        } else {
                                            icondata_bi::BiCircleRegular
                                        }
                                    />

                                </p>
                            }
                                .into_view()
                        })
                        .collect::<Vec<View>>()
                }}

                <Button on_click=move |_| page.update(|p| *p += 1) disabled=last_page>
                    <Icon width="1.5em" height="1.5em" icon=icondata_bi::BiChevronRightSolid/>
                </Button>
            </div>
        </div>
    }
}
