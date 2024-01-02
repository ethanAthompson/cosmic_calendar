use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

use crate::components::about::Card as AboutPage;
use crate::components::download::Card as DownloadPage;
use crate::components::home::Card as HomePage;
use crate::components::tools::Card as ToolsPage;
use crate::components::notfound::Card as NotFoundPage;


/// component for holding routes
#[component]
pub fn ZoneRoutes() -> impl IntoView {


    view! {
        <Routes>
            <Route path="/" view=HomePage/>
            <Route path="/*" view=NotFoundPage/>
            <Route path="/tool" view=ToolsPage/>
            <Route path="/about" view=AboutPage/>
            <Route path="/download" view=DownloadPage/>
        </Routes>
    }
}


