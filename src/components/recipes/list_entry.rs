use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use recipes_common::ListEntry;
use thaw::Card;

#[component]
pub fn ListItem(item: ListEntry) -> impl IntoView {
    let navigate = RwSignal::new(use_navigate());

    let on_click = move |_| {
        navigate.get()(&format!("/item/{}", item.filename), Default::default());
    };

    view! {
        <Card on:click=on_click class="cursor-pointer shadow-md w-[150px] min-h-[130px]"><div class="w-full min-h-[130px] text-lg text-center flex flex-row justify-center items-center">{item.name}</div></Card>
    }
}
