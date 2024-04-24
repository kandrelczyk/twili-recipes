use leptos::*;

use leptos_router::{Route, Router, Routes};
use thaw::{MessagePlacement, MessageProvider};

use crate::components::{recipes::List, Welcome};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="h-screen">
            <MessageProvider placement=MessagePlacement::Top>
                <Router>
                <Routes>
                    <Route path="/" view=Welcome/>
                    <Route path="/list" view=List/>
                </Routes>
                </Router>
            </MessageProvider>
        </main>
    }
}
