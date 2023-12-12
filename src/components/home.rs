use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="w-full p-4 ">
            Home
        </div>
    }
}
