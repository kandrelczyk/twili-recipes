use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::*;

use crate::components::Header;

#[component]
pub fn AddRecipeManual() -> impl IntoView {
    let navigate = RwSignal::new(use_navigate());
    let show_editor = RwSignal::new(false);

    let listener = window_event_listener_untyped("popstate", move |_| {
        if show_editor.get() {
            show_editor.set(false);
        } else {
            navigate.get_untracked()("/list", Default::default())
        }
    });

    on_cleanup(|| {
        listener.remove();
    });

    view! {
        <div class="flex flex-col h-screen w-full items-center justify-start">
                <Header
                    button=move || {
                        view! {
                            <Button
                                class="ml-1 absolute"
                                appearance=ButtonAppearance::Subtle
                                shape=ButtonShape::Circular
                                icon=icondata_bi::BiChevronLeftSolid
                                on:click=move |_| {
                                    navigate.get_untracked()("/list", Default::default());
                                }
                            />
                        }
                            .into_view()
                    }

                    title=|| "Add Recipe".to_owned()
                >
                </Header>

             Recipe editor
        </div>
    }
}
