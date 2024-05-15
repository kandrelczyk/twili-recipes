use leptos::*;

#[component]
pub fn Header(#[prop(into)] button: ViewFn, #[prop(into)] title: ViewFn) -> impl IntoView {
    view! {
        <div class="p-1 flex flex-row w-full h-[5%] items-center border-b border-slate-500">
            {move || button.run()}
            <div class="w-full flex flex-col items-center text-xl">{move || title.run()}
            </div>
        </div>
    }
}
