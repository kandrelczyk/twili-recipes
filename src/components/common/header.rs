use leptos::*;
use thaw::{use_theme, Theme};

#[component]
pub fn Header(#[prop(into)] button: ViewFn, #[prop(into)] title: ViewFn) -> impl IntoView {
    let theme = use_theme(Theme::dark);
    let css_vars = create_memo(move |_| {
        theme.with(|theme| {
            format!(
                "--thaw-background-color: {};",
                theme.common.background_color
            )
        })
    });
    view! {
        <div
            class="p-1 flex flex-row w-full h-[50px] items-center border-b border-slate-500 sticky top-0 z-40 bg-[--thaw-background-color]"
            style=move || css_vars.get()
        >
            {move || button.run()}
            <div class="w-full flex flex-col items-center text-xl">{move || title.run()}</div>
        </div>
    }
}
