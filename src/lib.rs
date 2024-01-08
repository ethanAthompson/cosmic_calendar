pub mod components;
pub mod interfaces;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::interfaces::composites::media_queries::Card as DebugScreen;
use crate::components::organisms::home::Page as HomePage;
use crate::components::organisms::notfound::Page as NotFoundPage;
use crate::components::organisms::dashboard::Page as DashboardPage;
use crate::components::molecules::dashboards::home::Page as DashboardHomePage;
use crate::components::molecules::dashboards::data::Page as DataPage;
use crate::components::molecules::dashboards::timezone::Page as TimezonePage;
use crate::components::molecules::dashboards::date::Page as DatePage;
use crate::components::molecules::dashboards::info::Page as InfoPage;
use crate::components::molecules::dashboards::settings::Page as SettingsPage;


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <div id="root">
            <Router>
                <Routes>
                    <Route path="/" view=HomePage/>        
                    <Route path="/*any" view=NotFoundPage/>        
                    <Route path="/dashboard" view=DashboardPage>        
                        <Route path="date" view=DatePage />
                        <Route path="timezone" view=TimezonePage/>
                        <Route path="info" view=InfoPage />
                        <Route path="data" view=DataPage />
                        <Route path="settings" view=SettingsPage />
                        <Route path="" view=DashboardHomePage />
                    </Route>
                </Routes>
            </Router>
        </div>
        // <DebugScreen/>
    }
}
