use leptos::*;
use recipes_common::Recipe;
use thaw::Card;

#[component]
pub fn PreviewRecipe(recipe: Recipe) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center overflow-hidden h-[70vh]">
            <p class="mt-2 text-lg text-center">Ingredients</p>
            <Card class="mt-2 text-left overflow-y-scroll h-full">
                <div class="flex flex-col">
                    {recipe
                        .ingredients
                        .into_iter()
                        .map(|i| {
                            view! {
                                <li class="text-md">
                                    {format!("{}....{} {}", i.name, i.quantity, i.scale)}
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </Card>
            <div class="grow"></div>
            <p class="mt-6 mb-2 text-lg text-center">Steps</p>
            <div class="mb-4 overflow-y-scroll h-full snap-y max-w-[700px]">
                <div class="flex pb-4 flex-col gap-2 w-fit mx-4">
                    {recipe
                        .steps
                        .into_iter()
                        .map(|s| {
                            view! {
                                <Card class="w-full h-full min-h-[10vh] snap-center">
                                    {s.desc}
                                    {if s.time > 0 {
                                        view! { <p>time: {s.time.to_owned()}</p> }
                                    } else {
                                        view! { <p></p> }
                                    }}
                                </Card>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
