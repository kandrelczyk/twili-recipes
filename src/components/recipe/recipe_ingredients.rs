use std::collections::HashMap;

use leptos::prelude::*;
use recipes_common::Recipe;
use thaw::{Button, Card, Divider};

#[component]
pub fn RecipeIngredients(recipe: StoredValue<Recipe>, multiplier: RwSignal<f32>) -> impl IntoView {
    let mut groups = HashMap::<String, Vec<recipes_common::Ingredient>>::new();
    for ingredient in recipe.get_value().ingredients.iter() {
        let group = groups.entry(ingredient.group.clone().unwrap_or_default());
        group
            .and_modify(|g| g.push(ingredient.clone()))
            .or_insert(vec![ingredient.clone()]);
    }

    let grouped_ingredients = groups
                    .into_iter()
                    .map(|(k, v)| {
                        view! {
                            <div>
                                <div class="text-lg sm:text-md">{k.clone().to_uppercase()}</div>
                                {v
                                    .into_iter()
                                    .map(|i| {
                                        view! {
                                            <p class="ml-2">
                                                {i.name()}
                                                <i>
                                                    {move || {
                                                        format!("...{} {}", i.quantity * multiplier.get(), i.scale)
                                                    }}
                                                </i>
                                            </p>
                                        }
                                            .into_view()
                                    })
                                    .collect::<Vec<_>>()}

                            </div>
                        }.into_any()
                    })
                    .intersperse_with(move || view!{<Divider/>}.into_any())
                    .collect::<Vec<AnyView>>();

    view! {
        <div class="text-xl p-4">Ingredients</div>
        <div class="flex flex-row sm:gap-8 gap-6 m-4 justify-center items-center">
            <Button
                icon=icondata_bi::BiMinusRegular
                on_click=move |_| {
                    multiplier.update(|m| if *m <= 1.0 { *m /= 2.0 } else { *m -= 1.0 })
                }
            ></Button>

            <div class="text-xl">Size: {move || multiplier.get()}</div>

            <Button
                icon=icondata_bi::BiPlusRegular
                on_click=move |_| {
                    multiplier.update(|m| if *m < 1.0 { *m *= 2.0 } else { *m += 1.0 })
                }
            ></Button>
        </div>
        <Card class="text-md max-w-md w-[80%] m-2 sm:text-lg overflow-y-auto flex flex-col gap-4 custom-scroll">
            {grouped_ingredients}
        </Card>
    }
}
