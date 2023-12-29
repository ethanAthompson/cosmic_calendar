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
pub fn Switch<F>(on_click: F) -> impl IntoView
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
                    <Icon icon=theme_icon  class="modew" />
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
        <main class="flex flex-col items-center w-full">
            <section class="w-full">
                <span on:mousedown=light_theme  class="flex gap-2 p-1 items-center wing-item-select hover:text-amber-400 cursor-pointer" on:click=on_click>
                     <Icon icon=sun class="cursor-pointer"/>
                    Light
                </span>
            </section>

            <section class="w-full">
                <span on:mousedown=dark_theme class="flex gap-2 p-1 items-center wing-item-select hover:text-violet-400 cursor-pointer" on:click=on_click>
                     <Icon icon=moon class="cursor-pointer"/>
                    Dark
                </span>
            </section>

            <section class="w-full">
                <span on:mousedown=system_theme class="flex gap-2 p-1 items-center wing-item-select hover:text-gray-400 cursor-pointer" on:click=on_click>
                     <Icon icon=sys class="cursor-pointer"/>
                    System
                </span>
            </section>
        </main>
    }
}
