use leptos::prelude::*;

#[slot]
pub struct ActionsSlot {
    children: Children,
}

#[component]
pub fn Header(
    #[prop(into)] button: ViewFn,
    #[prop(into)] title: ViewFn,
    #[prop(optional)] actions_slot: Option<ActionsSlot>,
) -> impl IntoView {
    view! {
            <div
                class="p-1 flex flex-row w-full h-[50px] items-center border-b border-slate-500 sticky top-0 z-40 bg-[--colorNeutralBackground1]"
            >
                {move || button.run()}
                <div class="w-full flex flex-col items-center text-xl">{move || title.run()}</div>
                {if let Some(ActionsSlot {children}) = actions_slot {
                    (children)().into_any()
                } else {
                    ().into_any()
                }}
            </div>
        }
}
