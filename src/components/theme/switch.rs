use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

use crate::{components::theme::ZoneTheme, wrappers::web::update_dom_el};

#[component]
pub fn ThemeSwitch() -> impl IntoView {
    let sun = Icon::from(BsIcon::BsSun);
    let moon = Icon::from(BsIcon::BsMoonStars);
    let system = Icon::from(FiIcon::FiMonitor);

    let (panel, set_panel) = create_signal(false);
    let (class, set_class) = create_signal("hidden");

    let update_panel = move || {
        set_panel.update(|toggle| {
            if *toggle {
                set_class.set("hidden");
                update_dom_el("themes", class.get());
            } else {
                set_class.set("");
                update_dom_el("themes", class.get());
            }

            *toggle = !*toggle
        })
    };

    // Button toggles panel by default
    let toggle_panel = move |_| {
        update_panel();
    };

    // each dropdown-item -> dark, light, system (adjacent to css classes)
    let light_theme = move |_| {
        update_panel();
        ZoneTheme::set_theme(&ZoneTheme::Light);
    };

    let dark_theme = move |_| {
        update_panel();
        ZoneTheme::set_theme(&ZoneTheme::Dark);
    };

    let system_theme = move |_| {
        update_panel();
        ZoneTheme::set_theme(&ZoneTheme::System);
    };

    view! {

        <main class="relative inline-block text-left">
            <button on:click=toggle_panel type="button" class="inline-flex justify-center items-center px-2 py-2">
                <Icon icon=sun class=""/>
            </button>
            <article id="themes" class="hidden">
                <article class="absolute right-0 z-25 mt-8 p-2 w-fit rounded-lg shadow-lg flex flex-col flex-nowrap origin-top-left bg-slate-800 ">
                    <span class="flex gap-4 py-2 items-center text-2xl hover:text-orange-700 cursor-pointer" on:click=light_theme>
                         <Icon icon=sun class="cursor-pointer"/>
                        Light
                    </span>
                    <hr class="h-px w-full bg-gray"/>
                    <span class="flex gap-4 py-2 items-center text-2xl hover:text-violet-700 cursor-pointer" on:click=dark_theme>
                         <Icon icon=moon class="cursor-pointer"/>
                        Dark
                    </span>
                    <hr class="h-px w-full bg-gray"/>
                    <span class="flex gap-4 py-2 items-center text-2xl hover:text-gray-700 cursor-pointer" on:click=system_theme>
                         <Icon icon=system class="cursor-pointer"/>
                        System
                    </span>
                </article>
            </article>
        </main>
    }
}
