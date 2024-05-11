use leptos::*;
use recipes_common::ListEntry;
use thaw::Card;

#[component]
pub fn ListItem(item: ListEntry) -> impl IntoView {
    view! {
        <Card class="shadow-md w-[150px] min-h-[150px]"><div class="w-full text-lg text-center">{item.name}</div></Card>
    }
}
