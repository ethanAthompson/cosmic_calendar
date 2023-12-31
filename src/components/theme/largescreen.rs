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
pub fn Switch() -> impl IntoView {
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
            <button id="theme-icon-button" on:click=toggle_panel type="button" class="px-2 inline-flex justify-center items-center dark:text-zinc-400 text-zinc-800">
                <span id="theme-icon" class=icon_class.get_untracked() >
                    <Icon icon=icon_theme.to_owned() class="w-7 h-7 desktop:w-12 desktop:h-12" />
                </span>
            </button>
            <article id="themes" class="hidden">
                <article class="absolute right-0 z-25 mt-8 p-2 w-fit rounded-lg shadow-lg flex flex-col flex-nowrap origin-top-left dark:bg-slate-900 bg-slate-200">
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

