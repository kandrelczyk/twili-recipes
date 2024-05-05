use leptos::*;

#[component]
pub fn MenuButton(
    children: Children,
    #[prop(into)] on_click: Callback<ev::MouseEvent>,
) -> impl IntoView {
    view! {
        <div class="flex flex-row gap-2 items-center hover:text-blue-400 cursor-pointer" on:click=on_click>
            {children()}
        </div>
    }
}
