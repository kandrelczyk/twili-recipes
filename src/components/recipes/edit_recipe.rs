use std::collections::HashSet;

use leptos::*;
use leptos_router::use_navigate;
use recipes_common::Recipe;
use thaw::{Button, ButtonVariant, Card, Collapse, CollapseItem, Icon};

use crate::error::CommandError;

#[component]
pub fn EditRecipe(recipe_json: String, #[prop(into)] go_back: Callback<(), ()>) -> impl IntoView {
    let navigate = create_rw_signal(use_navigate());
    let loading = create_rw_signal(false);
    let collapse = create_rw_signal(HashSet::from(["".to_string()]));

    let recipe_json = create_rw_signal(recipe_json);

    let parsed_recipe: Signal<Result<Recipe, CommandError>> =
        Signal::derive(move || match serde_json::from_str(&recipe_json.get()) {
            Ok(recipe) => Ok(recipe),
            Err(error) => Err(CommandError {
                reason: format!("Failed to parse JSON: {:?}", error),
            }),
        });

    let has_error = Signal::derive(move || parsed_recipe.get().is_err());
    view! {
        <main class="flex flex-col items-center h-full">
            <div class="p-1 flex flex-row w-full items-center border-b border-slate-500">
                <Button
                    class="absolute"
                    variant=ButtonVariant::Text
                    round=true
                    on:click=move |_| go_back(())
                >
                    <Icon width="1.5em" height="1.5em" icon=icondata_bi::BiChevronLeftSolid/>
                </Button>
                <div class="w-full flex flex-col items-center text-xl">Add recipe</div>
            </div>
            {move || match parsed_recipe.get() {
                Ok(recipe) => view! { <p>{format!("{:?}", recipe)}</p> },
                Err(error) => {
                    view! {
                        <p class="mt-24 p-4 text-center">
                            <Card class="text-md mb-8">
                                LMM returned invalid recipe code and we were not able to parse it.
                            <Collapse class="max-w-sm mt-8" value=collapse>
                                <CollapseItem title="Error details" key="error">
                                    <p class="text-sm text-wrap">{error.reason}</p>
                                </CollapseItem>
                            </Collapse>
                            </Card>
                            <div class="mt-8 flex flex-row m-4">
                                <Button
                                    variant=ButtonVariant::Outlined
                                    on:click=move |_| go_back(())
                                >
                                    Go back
                                </Button>
                                <div class="grow"/>
                                <Button
                                    variant=ButtonVariant::Outlined
                                    on:click=move |_| go_back(())
                                >
                                    Edit manually
                                </Button>
                            </div>
                        </p>
                    }
                }
            }}

            <div class="grow"></div>
            <Button disabled=has_error loading class="m-4">
                Save
            </Button>
        </main>
    }
}
