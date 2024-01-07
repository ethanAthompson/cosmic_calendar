pub mod components;
pub mod interfaces;
pub mod wrappers;

use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::components::about::Card as AboutPage;
use crate::components::download::Card as DownloadPage;
use crate::components::home::Card as HomePage;
use crate::components::tools::Card as ToolsPage;
use crate::components::notfound::Card as NotFoundPage;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <div id="root">
            <Router>
                <Navbar/>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/*" view=NotFoundPage/>
                    <Route path="/tool" view=ToolsPage/>
                    <Route path="/about" view=AboutPage/>
                    <Route path="/download" view=DownloadPage/>
                </Routes>
                <Footer/>
            </Router>
        </div>
    }
}
