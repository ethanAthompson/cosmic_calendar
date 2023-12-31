use std::sync::Arc;

use crate::{
    components::{
        tools::innerplanets::earth::{earth_time, EarthTimeZone},
    },
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
        <section id="celestial-tz-card" class="p-4 dark:bg-slate-900 bg-slate-200 rounded-xl 
        hover:shadow-amber cursor-pointer
        
        ">
            // <p> {"Valles Marineris M220/20/4"} </p>
            // <p> {"Bayara Vallis V150/9/2"} </p>
            // <p> {"Colles T20/2/28"} </p>
        </section>
    }
}


#[component]
pub fn CelestialDisplay(#[prop(into)] input: RwSignal<String>) -> impl IntoView {
    let close_icon = Icon::from(BiIcon::BiXCircleRegular);

    let remove_item = move |_| {
        let zones = all_items("celestial-tz-card", "span");

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
                on:click=remove_item
            ><Icon icon=close_icon class="w-6 h-6"/></button>
        </div>
    }
}
