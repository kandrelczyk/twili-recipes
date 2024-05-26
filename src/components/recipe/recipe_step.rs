use leptos::*;
use recipes_common::Step;

#[component]
pub fn RecipeStep(step: String) -> impl IntoView {
    view! { <div class="p-4 sm:text-xl text-lg overflow-y-scroll">{step}</div> }
}
