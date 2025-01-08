use leptos::prelude::*;
use recipes_common::Recipe;
use thaw::{Button, Icon, Slider};

use crate::components::{RecipeIngredients, RecipeStep};

#[component]
pub fn RecipePanels(recipe: Recipe) -> impl IntoView {
    let page = RwSignal::new(0.0);
    let page_count = recipe.steps.len() as f64 + 1.0;

    let multiplier = RwSignal::new(1.0);

    let recipe = StoredValue::new(recipe);
    let first_page = Signal::derive(move || page.get() == 0.0);
    let last_page = Signal::derive(move || page.get() as usize == recipe.get_value().steps.len());

    let step = Memo::new(move |_| {
        let mut step = recipe.get_value().steps[page.get() as usize - 1]
            .desc
            .clone();
        recipe.get_value().ingredients.iter().for_each(|i| {
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
        <div class="flex flex-col h-[90%] w-full]">
            <div class="w-full flex flex-col overflow-auto items-center">
                {move || {
                    if page.get() == 0.0 {
                        view! { <RecipeIngredients recipe multiplier=multiplier/> }.into_any()
                    } else {
                        view! {
                            <div class="w-full text-xl text-center p-4">
                                Step {page} / {page_count - 1.0}
                            </div>
                            <RecipeStep step=step.get()/>
                        }
                            .into_any()
                    }
                }}

            </div>
            <div class="grow"></div>
            <div class="flex max-w-4xl m-auto flex-row m-4 justify-center items-center">
                <div class="grow"></div>
                <Button icon=icondata_bi::BiChevronLeftSolid class="m-2" on_click=move |_| page.update(|p| *p -= 1.0) disabled=first_page>
                </Button>
                <Slider step=1.0 max={page_count - 1.0} value=page class="w-full m-4"/>
                <Button icon=icondata_bi::BiChevronRightSolid class="m-2" on_click=move |_| page.update(|p| *p += 1.0) disabled=last_page>
                </Button>
                <div class="grow"></div>
            </div>
            {move || if !last_page.get() {
                view!{
                    <div style="opacity: 0.35" on:click=move |_| page.update(|p| *p += 1.0) class="fixed h-full w-[15vw] bg-transparent top-0 right-0 sm:hidden block flex-col justify-center items-center">
                        <Icon class="w-full h-full" width="2em" icon=icondata_bi::BiChevronRightSolid/>
                    </div>}.into_any()
             } else {
               view!{<div></div>}.into_any()
            }}
            {move || if !first_page.get() {
                view!{
                <div style="opacity: 0.35" on:click=move |_| page.update(|p| *p -= 1.0) class="fixed h-full w-[15vw] top-0 left-0 bg-transparent sm:hidden block flex-col justify-center items-center">
                    <Icon class="w-full h-full" width="2em" icon=icondata_bi::BiChevronLeftSolid/>
                </div>}.into_any()
             } else {
               view!{<div></div>}.into_any()
            }}
        </div>
    }
}
