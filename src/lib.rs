pub mod components;
pub mod interfaces;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::interfaces::composites::media_queries::Card as DebugScreen;
use crate::components::organisms::home::Page as HomePage;
use crate::components::organisms::dashboard::Page as DashboardPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <div id="root">
            <Router>
                <Routes>
                    <Route path="/" view=HomePage/>        
                        // Nest timezone & date in the dashboard route /dashboard/timezone, etc..
                    <Route path="/dashboard" view=DashboardPage/>        
                </Routes>
            </Router>
        </div>
        // <DebugScreen/>
    }
}
