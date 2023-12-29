pub mod links;

use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

use crate::components::navbar::links::*;
use crate::wrappers::web::update_dom_el;

#[component]
pub fn Navbar() -> impl IntoView {
    let menu = Icon::from(leptos_icons::AiIcon::AiMenuOutlined);

    // more svelete like
    let menu_state = create_rw_signal(false);

    let toggle_menu = move |_| {
        menu_state.update(|flag: &mut bool| {
            // sets flag to opposite counterpart
            *flag = !*flag;
            // console_log(&*flag.to_string().as_str());

            // Toggles the items when clicked
            create_effect(move |_| {
                if menu_state.get() {
                    update_dom_el("mobile-links", "hidden");
                } else {
                    update_dom_el("mobile-links", "");
                }
            });
        })
    };

    view! {
        <header class="p-4 grid grid-cols-2 w-full shadow-md">
            <nav class="flex justify-start items-center">
                <A href="" class="unique-wrap desktop:text-6xl laptop:text-4xl tablet:text-2xl text-xl"> Zone </A>
            </nav>
            <nav class="justify-end hidden desktop:flex laptop:flex tablet:flex">
                <SuperBigLinks />
            </nav>
            <nav class="flex justify-end px-2 desktop:hidden laptop:hidden tablet:hidden">
                <button class="" on:click=toggle_menu>
                    <Icon icon=menu class="cursor-pointer w-12 h-12  default-item-select" />
                </button>
            </nav>
        </header>

        <div id="mobile-links">
            <nav class="desktop:hidden laptop:hidden tablet:hidden shadow-lg">
                <SmallLinks on_click=toggle_menu />
            </nav>
        </div>
    }
}
