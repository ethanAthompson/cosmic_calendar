//
//
use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::*;
use leptos_use::use_element_hover;


#[component]
pub fn Component() -> impl IntoView {
    view! {
        <section class="flex flex-row space-x-4 justify-normal items-center bg-green-400">        
            <article>
                <h1>Application Header</h1>
        
            </article>

        </section>
    }
}
