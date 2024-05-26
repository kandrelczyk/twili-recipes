use leptos::*;
use leptos_router::use_navigate;
use recipes_common::ListEntry;
use thaw::Card;

#[component]
pub fn ListItem(item: ListEntry) -> impl IntoView {
    let navigate = create_rw_signal(use_navigate());

    let on_click = move |_| {
        navigate.get()(&format!("/item/{}", item.filename), Default::default());
    };

    view! {
        <Card on:click=on_click class="cursor-pointer shadow-md w-[150px] min-h-[150px]"><div class="w-full min-h-[150px] text-lg text-center flex flex-row justify-center items-center">{item.name}</div></Card>
    }
}
