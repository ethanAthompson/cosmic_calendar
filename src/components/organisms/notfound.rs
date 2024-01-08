//
use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::*;
use leptos_use::use_element_hover;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <main class="w-screen h-screen bg-accent-2 flex flex-col space-y-12 justify-center items-center">
            <div class="shadow-lg p-4 rounded-xl">
                <p class="text-content-1"> You Entered the wrong page </p>
            </div>
            <section class="p-4 flex items-center justify-center rounded-xl">
                <div class="p-4 shadow-lg rounded-xl">
                    <A href="/" class="enter-btn text-content-1 hover:text-content-2"> Lock in? </A>
                </div>
            </section>
        </main>
    }
}
