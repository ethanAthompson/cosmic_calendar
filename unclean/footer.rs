// Please organize code when you finish the 1st draft of the website
// More reactivity
// this file is redundant because largeScreenlinks is basically the footer
// please organize this


pub mod largescreen;
pub mod smallscreen;

// use crate::LagLinks as NormalLink

use leptos::*;
use crate::components::footer::largescreen::LargeScreenLinks;
use crate::components::footer::smallscreen::SmallScreenLinks;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        // <footer class="hard-bg left-0 bottom-0 w-full fixed justify-center items-center py-0 bg-inherit">
        <footer class="fixed hard-bg left-0 bottom-0 w-full justify-center items-center py-0 bg-inherit">
            <LargeScreenLinks/>
            // <SmallScreenLinks/>
        </footer>
    }
}
