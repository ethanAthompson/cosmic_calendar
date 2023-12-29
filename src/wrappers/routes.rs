use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

use crate::components::about::About;
use crate::components::download::Download;
use crate::components::home::Home;
use crate::components::tools::Tools;

/// component for holding routes
#[component]
pub fn ZoneRoutes() -> impl IntoView {    
    view! {
        <Routes>
            <Route path="" view=Home/>
            <Route path="/tool" view=Tools/>
            <Route path="/about" view=About/>
            <Route path="/download" view=Download/>
        </Routes>
    }
}
