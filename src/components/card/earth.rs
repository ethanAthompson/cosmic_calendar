use std::sync::Arc;

use crate::{
    components::tools::innerplanets::earth::{earth_time, EarthTimeZone},
    wrappers::{
        strings::{filtered_vec, get_initials, matching_left},
        web::{all_items, save_data, update_dom_el},
    },
};
use chrono::prelude::*;
use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlHeadingElement, HtmlInputElement, KeyboardEvent, MouseEvent, Node};

#[component]
pub fn Card() -> impl IntoView {
    view! {
        <section id="earth-tz-card" class="p-4 dark:bg-slate-900 bg-slate-200 rounded-xl
            flex flex-col space-y-4 hover:shadow-amber cursor-pointer
        ">
            // <p> {"New York 12/3/2023"} </p>
            // <p> {"London 4/30/2023"} </p>
            // <p> {"Africa 2/23/2023"} </p>
        </section>
    }
}

#[component]
pub fn EarthDisplay(#[prop(into)] input: RwSignal<String>) -> impl IntoView {
    let close_icon = Icon::from(BiIcon::BiXCircleRegular);

    let remove_timezone = move |_| {
        let zones = all_items("earth-tz-card", "span");

        if let Some(parent) = document().get_element_by_id(&input.get()) {
            if let Some(button) =
                document().get_element_by_id(&format!("button-{}", &input.get()))
            {

                parent.remove();
            }
        }
    };

    let button_id = format!("button-{}", input.get());
    view! {
        <div class="flex space-x-2 items-center">
            <p class="leading-4"> {input} </p>
            <button
                id={button_id} 
                class="hover:text-red-400 text-black"
                on:click=remove_timezone
            ><Icon icon=close_icon class="w-6 h-6"/></button>
        </div>
    }
}
