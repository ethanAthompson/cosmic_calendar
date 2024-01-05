use std::{str::FromStr, sync::Arc};

use crate::wrappers::{
    strings::{filtered_vec, get_initials, matching_left},
    web::{all_items, save_data, update_dom_el},
};
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::{OffsetName, Tz, TZ_VARIANTS};

use chrono::prelude::*;
use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlHeadingElement, HtmlInputElement, KeyboardEvent, MouseEvent, Node};

#[component]
pub fn Card() -> impl IntoView {
    set_timeout(spawnloaded, std::time::Duration::from_millis(500));

    view! {
        // <section id="earth-tz-card" class="p-4 dark:bg-slate-900 bg-slate-200 rounded-xl
            // flex flex-col space-y-4 hover:shadow-amber cursor-pointer
        // ">
        <section id="earth-tz-card" class="p-4 flex flex-col space-y-4 cursor-pointer">
            // <p> {"New York 12/3/2023"} </p>
            // <p> {"London 4/30/2023"} </p>
            // <p> {"Africa 2/23/2023"} </p>
        </section>
    }
}

#[component]
pub fn EarthDisplay(#[prop(into)] input: RwSignal<String>) -> impl IntoView {
    let close_icon = Icon::from(BiIcon::BiXCircleRegular);
    let time = create_rw_signal("".to_string());
    let timezone =
        chrono_tz::Tz::from_str(&input.get()).expect("an earth timezone to make it through!");
    let datetime = DateTime::with_timezone(&Utc::now(), &timezone).format("%Y/%m/%d %r %Z");
    time.set(datetime.to_string());

    set_interval(
        move || {
            let timezone = chrono_tz::Tz::from_str(&input.get())
                .expect("an earth timezone to make it through!");
            let datetime = DateTime::with_timezone(&Utc::now(), &timezone).format("%Y/%m/%d %r %Z");
            time.set(datetime.to_string());
        },
        std::time::Duration::from_millis(1000),
    );

    let remove_timezone = move |_| {
        let zones = all_items("earth-tz-card", "span");

        if let Some(parent) = document().get_element_by_id(&input.get()) {
            if let Some(button) = document().get_element_by_id(&format!("button-{}", &input.get()))
            {
                save_data().1.update(move |data| {
                    parent.remove();

                    for item in data.earth.clone().into_keys() {
                        console_log(&format!("Key: {:?}", item));

                        data.earth.remove(&input.get());
                    }
                })
            }
        }
    };


    let startdrag = move |ev: web_sys::DragEvent| {
        ev.data_transfer()
            .expect("Data to be here")
            .set_data(
                "text",
                ev.target()
                    .unwrap()
                    .dyn_into::<HtmlElement>()
                    .unwrap()
                    .id()
                    .as_str(),
            )
            .expect("Data to be transferred.");

        console_log("Sending Data");
    };
    let button_id = format!("button-{}", input.get());
    view! {
        <span id=input on:dragstart=startdrag draggable="true" class="flex space-x-2 items-center
                    p-2 dark:hover:bg-blend-lighten dark:mix-blend-screen w-full text-start cursor-pointer
                    -skew-y-3 scale-100 hover:-translate-y-2 hover:scale-12 focus:-translate-y-2 focus:scale-125 
                    ease-in-out duration-300 glitch text-xl                
        ">
            <p class="text-base">{input} <em class="px-1">{time}</em></p>
            <button
                id={button_id}
                class="hover:text-red-400 dark:text-white text-black"
                on:click=remove_timezone
            ><Icon icon=close_icon class="w-6 h-6"/></button>
        </span>
    }
}

#[component]
pub fn LocalEarthDisplay(
    /// The input allows the tz to be properly updated
    #[prop(into)]
    input: RwSignal<String>,
) -> impl IntoView {
    let close_icon = Icon::from(BiIcon::BiXCircleRegular);
    let time = create_rw_signal("".to_string());
    let timezone = Tz::from_str(&input.get()).expect("an earth timezone to make it through!");
    let datetime = DateTime::with_timezone(&Utc::now(), &timezone).format("%Y/%m/%d %r %Z");
    time.set(datetime.to_string());

    set_interval(
        move || {
            let timezone = chrono_tz::Tz::from_str(&input.get())
                .expect("an earth timezone to make it through!");
            let datetime = DateTime::with_timezone(&Utc::now(), &timezone).format("%Y/%m/%d %r %Z");
            time.set(datetime.to_string());
        },
        std::time::Duration::from_millis(1000),
    );

    let remove_timezone = move |_| {
        let zones = all_items("earth-tz-card", "span");

        if let Some(parent) = document().get_element_by_id(&input.get()) {
            if let Some(button) = document().get_element_by_id(&format!("button-{}", &input.get()))
            {
                save_data().1.update(move |data| {
                    parent.remove();

                    for item in data.earth.clone().into_keys() {
                        console_log(&format!("Key: {:?}", item));

                        data.earth.remove(&input.get());
                    }
                })
            }
        }
    };

    let startdrag = move |ev: web_sys::DragEvent| {
        ev.data_transfer()
            .expect("Data to be here")
            .set_data(
                "text",
                ev.target()
                    .unwrap()
                    .dyn_into::<HtmlElement>()
                    .unwrap()
                    .id()
                    .as_str(),
            )
            .expect("Data to be transferred.");

        console_log("Sending Data");
    };

    let button_id = format!("button-{}", input.get());
    view! {
        <span id=input class="flex space-x-2 items-center
                    p-2 dark:hover:bg-blend-lighten dark:mix-blend-screen w-full text-start cursor-pointer
                    -skew-y-3 scale-100 hover:-translate-y-2 hover:scale-12 focus:-translate-y-2 focus:scale-125 
                    ease-in-out duration-300 glitch desktop:text-6xl laptop:text-4xl tablet:text-4xl text-xl                
                "
        on:dragstart=startdrag draggable="true"
        >
            <p class="text-base">{input} <em class="px-1">{time}</em></p>
            <button
                id={button_id}
                class="hover:text-red-400 dark:text-white text-black"
                on:click=remove_timezone
            ><Icon icon=close_icon class="w-6 h-6"/></button>
        </span>
    }
}

/// Spawns in local storage data by iterating a hashmap
pub fn spawnloaded() {
    // only fills the earth-zones.
    if let Some(parent) = document().get_element_by_id("earth-tz-card") {
        // fills up each item in earth vector
        for key in save_data().0.get().earth.into_values() {
            let loaded_node = view! {
                <span class="flex space-x-2" id={key.to_string()}>
                    <LocalEarthDisplay input=key.to_string() />
                </span>
            };

            parent.append_child(&loaded_node).unwrap();
        }
    };
}
