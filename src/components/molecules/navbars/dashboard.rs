//
//
use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::*;
use leptos_use::use_element_hover;


#[component]
pub fn Component() -> impl IntoView {
    view! {
        <section class="flex flex-col space-x-4 justify-normal items-center bg-amber-400">        
            <article>
                <h1>Application Right Sidebar</h1>
            </article>
        </section>
    }
}
