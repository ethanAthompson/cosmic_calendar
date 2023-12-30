pub mod largescreen;
pub mod smallscreen;

// use crate::LagLinks as NormalLink

use leptos::*;
use crate::components::footer::largescreen::LargeScreenLinks;
use crate::components::footer::smallscreen::SmallScreenLinks;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="hard-bg left-0 bottom-0 w-full fixed justify-center items-center py-0">
            <LargeScreenLinks/>
            // <SmallScreenLinks/>
        </footer>
    }
}
