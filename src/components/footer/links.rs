use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

#[component]
pub fn FooterLinks() -> impl IntoView {
    view! {
        <div class="flex justify-end space-x-12 items-center px-4 text-2xl">
            <A href="tool" class="py-4 rounded-md default-item-select "> "Tool" </A>
            <A href="about" class="py-4 rounded-md default-item-select "> "About" </A>
            <A href="download" class="py-4 rounded-md default-item-select "> "Download" </A>
        </div>
    }
}
