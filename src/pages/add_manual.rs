use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use recipes_common::Recipe;

use crate::components::RecipeForm;

#[component]
pub fn AddRecipeManual() -> impl IntoView {
    let navigate = RwSignal::new(use_navigate());

    let listener = window_event_listener_untyped("popstate", move |_| {
        navigate.get_untracked()("/list", Default::default())
    });

    on_cleanup(|| {
        listener.remove();
    });

    let recipe = Recipe {
        id: None,
        name: None,
        steps: vec![],
        ingredients: vec![],
    };

    view! {
        <div class="flex flex-col h-screen w-full items-center justify-start">
            <RecipeForm
                on_back=Callback::new(move |_| navigate
                    .get_untracked()("/list", Default::default()))
                on_save=Callback::new(move |_| {
                    navigate.get_untracked()("/list", Default::default());
                })
                recipe=recipe
            />
        </div>
    }
}
