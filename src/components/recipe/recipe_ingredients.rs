use leptos::*;
use recipes_common::Recipe;
use thaw::{Button, ButtonVariant, Card, Icon};

#[component]
pub fn RecipeIngredients(recipe: StoredValue<Recipe>, multiplier: RwSignal<f32>) -> impl IntoView {
    view! {
        <div class="text-xl p-4">Ingredients</div>
        <div class="flex flex-row sm:gap-4 gap-2 m-4 justify-center items-center">
            <Button
                variant=ButtonVariant::Outlined
                on_click=move |_| multiplier.update(|m| *m /= 2.0)
            >
                <div class="flex items-center">
                    <Icon
                        width="1.5em"
                        height="1.5em"
                        icon=icondata_bi::BiMinusRegular
                    />
                </div>
            </Button>
            <div class="text-xl">Size: {move || multiplier.get()}</div>

            <Button
                variant=ButtonVariant::Outlined
                on:click=move |_| multiplier.update(|m| *m *= 2.0)
            >
                <div class="flex items-center">
                    <Icon
                        width="1.5em"
                        height="1.5em"
                        icon=icondata_bi::BiPlusRegular
                    />
                </div>
            </Button>
        </div>
        <Card class="text-md sm:text-lg overflow-y-scroll">
            {recipe()
                .ingredients
                .into_iter()
                .map(|i| {
                    view! {
                        <li>
                            {i.name.clone()}
                            <i>
                                {move || format!("...{} {}", i.quantity * multiplier.get() , i.scale)}
                            </i>
                        </li>
                    }
                        .into_view()
                })
                .collect::<Vec<View>>()}
        </Card>
    }
}
