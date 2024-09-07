use leptos::prelude::*;

use leptos_router::path;
use leptos_router::components::{Route, Router, Routes};
use thaw::{ToastPosition, ToasterProvider, ConfigProvider, ConfigDirection};

use crate::components::{Welcome, Settings, recipes::{List, AddRecipe}, About, RecipeView};

#[component]
pub fn App() -> impl IntoView {

    let dir = RwSignal::new(ConfigDirection::Ltr);
    view! {
        <ConfigProvider dir=dir>
            <main class="h-full min-h-screen bg-[url('/public/background.png')]">
                        <ToasterProvider position=ToastPosition::Top>
                            <Router>
                                <Routes fallback=|| "404">
                                    <Route path=path!("/") view=Welcome/>
                                    <Route path=path!("/settings") view=move || view!{<Settings init=false/>}/>
                                    <Route path=path!("/initialize") view=move || view!{<Settings init=true/>}/>
                                    <Route path=path!("/list") view=List/>
                                    <Route path=path!("/item/:filename") view=RecipeView />
                                    <Route path=path!("/add") view=AddRecipe/>
                                    <Route path=path!("/about") view=move || view!{<About/>}/>
                                </Routes>
                            </Router>
                        </ToasterProvider>
            </main>
        </ConfigProvider>
    }
}
