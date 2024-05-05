use leptos::*;
use recipes_common::ListEntry;

#[component]
pub fn ListItem(item: ListEntry) -> impl IntoView {
    view! {
        <p>{item.name} {item.filename}</p>
    }
}
