use crate::{
    components::theme::ZoneTheme,
    wrappers::{
        theme::{theme_icons, theme_texts},
        web::{storage, storage_theme, update_dom_el},
    },
};
use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;

#[component]
pub fn ThemeSwitch() -> impl IntoView {
    let sun: Icon = Icon::from(BsIcon::BsSun);
    let moon: Icon = Icon::from(BsIcon::BsMoonStars);
    let sys: Icon = Icon::from(FiIcon::FiMonitor);

    let panel = create_rw_signal(false);
    let panel_class = create_rw_signal("");
    let icon_class = create_rw_signal(theme_texts());
    let icon_theme = create_rw_signal(theme_icons());

    // updates state theme
    let update_panel = move || {
        panel.update(|toggle| {
            if *toggle {
                panel_class.set("hidden");
                update_dom_el("themes", panel_class.get());
            } else {
                panel_class.set("");
                update_dom_el("themes", panel_class.get());
            }

            *toggle = !*toggle
        })
    };

    // Updates icon depending on theme
    // this way it never flickers due to the way html theme is set up
    let update_icon = move || {
        icon_theme.update(|current| {
            *current = theme_icons();
        })
    };

    // Updates the icons theme by performing wasm, dom element changing
    let update_icon_class = move || {
        create_effect(move |_| {
            update_dom_el("theme-icon", theme_texts());
        })
    };

    // Button toggles panel by default
    let toggle_panel = move |_| {
        update_panel();
    };

    let light_theme = move |_| {
        ZoneTheme::set_theme_opt(&ZoneTheme::Light, update_panel, 0);
        update_icon();
        update_icon_class();
    };

    let dark_theme = move |_| {
        ZoneTheme::set_theme_opt(&ZoneTheme::Dark, update_panel, 1);
        update_icon();
        update_icon_class();
    };

    let system_theme = move |_| {
        ZoneTheme::set_theme_opt(&ZoneTheme::System, update_panel, 2);
        update_icon();
        update_icon_class();
    };

    view! {

        <main class="relative inline-block text-left">
            <button id="theme-icon-button" on:click=toggle_panel type="button" class="inline-flex justify-center items-center px-2 py-2 dark:text-zinc-400 text-zinc-800">
                <span id="theme-icon" class=icon_class.get_untracked() >
                    <Icon icon=icon_theme.to_owned()  />
                </span>
            </button>
            <article id="themes" class="hidden">
                <article class="absolute right-0 z-25 mt-8 p-2 w-fit rounded-lg shadow-lg flex flex-col flex-nowrap origin-top-left bg-slate-800 ">
                    <span class="flex gap-4 py-2 items-center text-2xl hover:text-amber-400 cursor-pointer" on:click=light_theme>
                         <Icon icon=sun class="cursor-pointer"/>
                        Light
                    </span>
                    <hr class="h-px w-full bg-gray"/>
                    <span class="flex gap-4 py-2 items-center text-2xl hover:text-violet-400 cursor-pointer" on:click=dark_theme>
                         <Icon icon=moon class="cursor-pointer"/>
                        Dark
                    </span>
                    <hr class="h-px w-full bg-gray"/>
                    <span class="flex gap-4 py-2 items-center text-2xl hover:text-gray-400 cursor-pointer" on:click=system_theme>
                         <Icon icon=sys class="cursor-pointer"/>
                        System
                    </span>
                </article>
            </article>
        </main>
    }
}

#[component]
pub fn MobileThemeSwitch<F>(on_click: F) -> impl IntoView
where
    // Allows the event function to be inherited
    F: Fn(MouseEvent) + 'static + Copy,
{
    let sun: Icon = Icon::from(BsIcon::BsSun);
    let moon: Icon = Icon::from(BsIcon::BsMoonStars);
    let sys: Icon = Icon::from(FiIcon::FiMonitor);

    // Updates the icons theme by performing wasm, dom element changing
    let update_icon_class = move || {
        create_effect(move |_| {
            update_dom_el("theme-icon", theme_texts());
        })
    };

    let update_panel = move || {
        // console_log("did update");
        update_icon_class();
    };

    let change_icon = move || {
        create_effect(move |_| {
            let theme = document()
                .get_element_by_id("theme-icon")
                .expect("theme icon id");

            let button = document()
                .get_element_by_id("theme-icon-button")
                .expect("theme icon button id");

            let theme_icon = theme_icons();
            let icon_class = theme_texts();

            let new_icon = view! {
                 <span id="theme-icon" class=icon_class >
                    <Icon icon=theme_icon  />
                </span>
            };

            theme.remove();
            button.append_child(&new_icon).expect("New Icon");
        })
    };

    let light_theme = move |_| {
        ZoneTheme::set_theme_opt(&ZoneTheme::Light, move || update_panel(), 0);
        change_icon();
    };

    let dark_theme = move |_| {
        ZoneTheme::set_theme_opt(&ZoneTheme::Dark, move || update_panel(), 1);
        change_icon();
    };

    let system_theme = move |_| {
        ZoneTheme::set_theme_opt(&ZoneTheme::System, move || update_panel(), 2);
        change_icon();
    };

    view! {
        <main class="flex items-center justify-center space-x-4">
            <section>
                <span on:mousedown=light_theme  class="default-item-select flex gap-2 py-2 items-center text-2xl hover:text-amber-400 cursor-pointer" on:click=on_click>
                     <Icon icon=sun class="cursor-pointer"/>
                    Light
                </span>
            </section>

            <section>
                <span on:mousedown=dark_theme class="default-item-select flex gap-2 py-2 items-center text-2xl hover:text-violet-400 cursor-pointer" on:click=on_click>
                     <Icon icon=moon class="cursor-pointer"/>
                    Dark
                </span>
            </section>

            <section>
                <span on:mousedown=system_theme class="default-item-select flex gap-2 py-2 items-center text-2xl hover:text-gray-400 cursor-pointer" on:click=on_click>
                     <Icon icon=sys class="cursor-pointer"/>
                    System
                </span>
            </section>
        </main>
    }
}
