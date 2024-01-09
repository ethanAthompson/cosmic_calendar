#![allow(warnings)] // makes it easier to spot errors

pub mod components;
pub mod interfaces;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::molecules::cards::data::Page as DataPage;
use crate::components::molecules::cards::date::Page as DatePage;
use crate::components::molecules::cards::home::Page as DashboardHomePage;
use crate::components::molecules::cards::info::Page as InfoPage;
use crate::components::molecules::cards::settings::Page as SettingsPage;
use crate::components::molecules::cards::timezone::Page as TimezonePage;
use crate::components::molecules::navbars::app::Component as Navbar;
use crate::components::organisms::about::Page as AboutPage;
use crate::components::organisms::dashboard::Page as DashboardPage;
use crate::components::organisms::download::Page as DownloadPage;
use crate::components::organisms::home::Page as HomePage;
use crate::components::organisms::notfound::Page as NotFoundPage;
use crate::interfaces::composites::media_queries::Card as DebugScreen;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Global Signals
    let is_fullscreen = create_rw_signal(false);
    let is_mobilenavbar_toggled = create_rw_signal(false);
    let nav_pages = create_rw_signal(vec!["About", "Download", "Dashboard"]);

    // Providers
    provide_context(is_fullscreen);
    provide_context(is_mobilenavbar_toggled);
    provide_context(nav_pages);

    view! {
        <div id="root">
            <Router>
                <Navbar/>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/*any" view=NotFoundPage/>
                    <Route path="/about" view=AboutPage/>
                    <Route path="/download" view=DownloadPage/>
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
