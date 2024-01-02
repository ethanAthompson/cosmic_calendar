use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_router::*;
use web_sys::{Event, InputEvent, KeyboardEvent, MouseEvent};

#[component]
pub fn Card() -> impl IntoView {
    view! {}
}

#[component]
pub fn SelectBar() -> impl IntoView {
    view! {
        <input type="date" class="p-4"/>
    }
}
