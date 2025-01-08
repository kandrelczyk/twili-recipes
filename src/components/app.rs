use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use std::collections::HashMap;

use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use thaw::{Theme, ToastPosition, ToasterProvider};

use crate::components::{About, AddRecipe, List, RecipeView, Settings, Welcome};

#[derive(Clone)]
pub struct ThemeSetter {
    stored_value: WriteSignal<bool>,
    theme: RwSignal<Theme>,
}

impl ThemeSetter {
    pub fn set_theme(&self, is_dark: bool) {
        let brand_colors = RwSignal::new(HashMap::from([
            (10, "#050202"),
            (20, "#211316"),
            (30, "#381D23"),
            (40, "#4C242E"),
            (50, "#602C39"),
            (60, "#753444"),
            (70, "#8A3B50"),
            (80, "#A1435D"),
            (90, "#B74B69"),
            (100, "#CF5376"),
            (110, "#E65B83"),
            (120, "#EF7192"),
            (130, "#F489A2"),
            (140, "#F89FB2"),
            (150, "#FBB4C2"),
            (160, "#FEC9D3"),
        ]));
        if is_dark {
            self.theme.set(Theme::custom_dark(&brand_colors.get()));
            self.stored_value.set(true);
        } else {
            self.theme.set(Theme::custom_light(&brand_colors.get()));
            self.stored_value.set(false);
        }
    }
}
#[component]
pub fn App() -> impl IntoView {
    let (dark, set_dark, _) = use_local_storage::<bool, FromToStringCodec>("dark_mode");

    let theme_setter = ThemeSetter {
        stored_value: set_dark,
        theme: Theme::use_rw_theme(),
    };
    theme_setter.set_theme(dark.get());
    provide_context(theme_setter);

    view! {
        <main class="h-full min-h-screen bg-[url('/public/background.png')]">
            <ToasterProvider position=ToastPosition::Top>
                <Router>
                    <Routes fallback=|| "404">
                        <Route path=path!("/") view=Welcome />
                        <Route
                            path=path!("/settings")
                            view=move || view! { <Settings init=false /> }
                        />
                        <Route
                            path=path!("/initialize")
                            view=move || view! { <Settings init=true /> }
                        />
                        <Route path=path!("/list") view=List />
                        <Route path=path!("/item/:filename") view=RecipeView />
                        <Route path=path!("/add") view=AddRecipe />
                        <Route path=path!("/about") view=move || view! { <About /> } />
                    </Routes>
                </Router>
            </ToasterProvider>
        </main>
    }
}
