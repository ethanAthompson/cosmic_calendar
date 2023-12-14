use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

#[component]
pub fn FooterLinks() -> impl IntoView {
    view! {
        <div class=" text-black dark:text-white flex flex-nowrap space-x-12 items-center justify-end text-xl ">
            <A href="" class="py-4 rounded-md transition text-2xl hover:scale-125 delay-150 duration-75 ease-in-out hover:text-orange-300"> "Home" </A>
            <A href="tool" class="py-4 rounded-md transition text-2xl hover:scale-125 delay-150 duration-75 ease-in-out hover:text-orange-300"> "Tool" </A>
            <A href="about" class="py-4 rounded-md transition text-2xl hover:scale-125 delay-150 duration-75 ease-in-out hover:text-orange-300"> "About" </A>
            <A href="download" class="py-4 rounded-md transition text-2xl hover:scale-125 delay-150 duration-75 ease-in-out hover:text-orange-300"> "Download" </A>
        </div>
    }
}
