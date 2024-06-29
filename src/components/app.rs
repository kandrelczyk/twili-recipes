use leptos::*;

use leptos_router::{Route, Router, Routes};
use thaw::{GlobalStyle, MessagePlacement, MessageProvider, Theme, ThemeProvider};

use crate::components::{
    recipes::{AddRecipe, List},
    About, RecipeView, Settings, Welcome,
};

#[component]
pub fn App() -> impl IntoView {
    let theme = create_rw_signal(Theme::dark());
    let dark_mode = create_rw_signal(true);

    let theme_clb = create_memo(move |_| {
        theme.set(match dark_mode.get() {
            true => Theme::dark(),
            false => Theme::light(),
        });
    });

    view! {
        <main class="h-screen">
            <ThemeProvider theme>
                <GlobalStyle/>
                {move || theme_clb.get()}
                <MessageProvider placement=MessagePlacement::Top>
                    <Router>
                        <Routes>
                            <Route path="/" view=Welcome/>
                            <Route path="/list" view=move || view! { <List dark_mode/>} />
                            <Route path="/item/:filename" view=RecipeView />
                            <Route path="/add" view=AddRecipe/>
                            <Route path="/settings" view=move || view!{<Settings init=false/>}/>
                            <Route path="/initialize" view=move || view!{<Settings init=true/>}/>
                            <Route path="/about" view=move || view!{<About/>}/>
                        </Routes>
                    </Router>
                </MessageProvider>
            </ThemeProvider>
        </main>
    }
}
