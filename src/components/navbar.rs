/// Links specifically for navbar; may be long
pub mod links;

use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

use crate::components::navbar::links::NavbarLinks;
use crate::components::theme::switch::ThemeSwitch;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <header class="shadow-lg p-4 grid grid-cols-2 w-full">
            <nav class="flex justify-start items-center">
                <div class="p-4">
                    <h1 class="text-4xl"> Cosmic Calendar </h1>
                </div>
            </nav>
            <nav class="flex justify-end px-4">
                <NavbarLinks/>
                <p class="px-4 py-4 rounded-md transition text-2xl focus:scale-125 delay-150 duration-75 ease-in-out hover:text-orange-300"> <ThemeSwitch/></p>
            </nav>
        </header>
    }
}
