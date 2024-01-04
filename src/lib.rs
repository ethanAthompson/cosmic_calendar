/// Components for the entire website
pub mod components;

/// Interfaces for the entire website + cross-platform
pub mod interfaces;

/// Wrappers for abstraction
pub mod wrappers;

use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::wrappers::routes::ZoneRoutes;

use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <div id="root">
            <Router>
                <Navbar/>
                <ZoneRoutes/>
                // <Footer/>
            </Router>
        </div>
    }
}


