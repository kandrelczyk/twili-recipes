use leptos::*;
use recipes_common::Recipe;
use thaw::{Button, Icon, Slider};

use crate::components::{RecipeIngredients, RecipeStep};

#[component]
pub fn RecipePanels(recipe: Recipe) -> impl IntoView {
    let page = create_rw_signal(0.0);
    let page_count = recipe.steps.len() as f64 + 1.0;

    let multiplier = create_rw_signal(1.0);

    let recipe = store_value(recipe);
    let first_page = Signal::derive(move || page.get() == 0.0);
    let last_page = Signal::derive(move || page.get() as usize == recipe().steps.len());

    let step = create_memo(move |_| {
        let mut step = recipe().steps[page.get() as usize - 1].desc.clone();
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
                    if page.get() == 0.0 {
                        view! { <RecipeIngredients recipe multiplier=multiplier/> }.into_view()
                    } else {
                        view! {
                            <div class="w-full text-xl text-center p-4">
                                Step {page} / {page_count - 1.0}
                            </div>
                            <RecipeStep step=step.get()/>
                        }
                            .into_view()
                    }
                }}

            </div>
            <div class="grow"></div>
            <div class="flex flex-row m-4 justify-center items-center">
                <div class="grow"></div>
                <Button class="m-2" on_click=move |_| page.update(|p| *p -= 1.0) disabled=first_page>
                    <Icon width="1.5em" height="1.5em" icon=icondata_bi::BiChevronLeftSolid/>
                </Button>
                <Slider step=1.0 max={page_count - 1.0} value=page class="w-full m-4"/>
                <Button class="m-2" on_click=move |_| page.update(|p| *p += 1.0) disabled=last_page>
                    <Icon width="1.5em" height="1.5em" icon=icondata_bi::BiChevronRightSolid/>
                </Button>
                <div class="grow"></div>
            </div>
        </div>
    }
}
