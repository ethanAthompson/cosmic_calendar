/// Links specifically for footer; may be long
pub mod links;

use crate::components::footer::links::FooterLinks;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="left-0 bottom-0 w-full fixed justify-center items-center py-2 shadow-inner shadow-2xl">
            <section class="hidden laptop:grid desktop:grid grid-cols-2 items-center ">
                <nav class="order-2 flex justify-end px-2">
                    <FooterLinks/>
                </nav>

                <nav class="order-1 flex justify-start px-2 text-black dark:text-white ">
                    "© 2023 Zone™. All Rights Reserved."
                </nav>
            </section>
        </footer>
    }
}
