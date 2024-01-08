use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::*;
use leptos_use::use_element_hover;

#[component]
pub fn FullScreen() -> impl IntoView {
    view! {
        <span>
            <em class="">Enter Fullscreen Mode</em>
        </span>
    }
}

#[component]
pub fn ExitFullScreen() -> impl IntoView {
    view! {
        <span>
            <em class="">Exit Fullscreen Mode</em>
        </span>
    }
}
