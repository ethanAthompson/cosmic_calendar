// Finish what you set to do today
// Clean up footer maybe?
// lean up entire rust code, get it looking neat and clean, avoid redundancies.
// You can get the space stuff later.

use std::{str::FromStr, sync::Arc, time::Duration};

use crate::components::search::calendar::SelectBar as CalendarSelection;
use crate::components::tools::mainboard::custom_calendar::MultiCalendarDatePicker;
use crate::wrappers::date::date_symphony_v2;
use crate::{
    components::tools::ctz::Ctz,
    wrappers::{
        date::ZoneCalendar,
        strings::{filtered_vec, get_initials, matching_left},
        web::{all_items, save_data, update_dom_el},
    },
};
use chrono::prelude::*;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos::{html::Select, *};
use leptos_icons::*;
use leptos_router::create_query_signal;
use strum::IntoEnumIterator;
use strum::*;
use wasm_bindgen::JsCast;
use web_sys::{
    DragEvent, HtmlElement, HtmlHeadingElement, HtmlInputElement, InputEvent, KeyboardEvent,
    MouseEvent, Node,
};

#[component]
pub fn Card() -> impl IntoView {
    let select_el: NodeRef<Select> = create_node_ref();
    let output = create_rw_signal("Drop a Celestial Body Here!".to_string());
    let middleman = create_rw_signal("".to_string());
    let calendar_input = create_rw_signal("IslamicCivil".to_string());
    let celestial_input = create_rw_signal("".to_string());
    let static_date = create_rw_signal("".to_string());

    // MM/DD/YY
    let mel: NodeRef<Input> = create_node_ref();
    let del: NodeRef<Input> = create_node_ref();
    let yel: NodeRef<Input> = create_node_ref();

    let remove_session = move |ev: MouseEvent| {
        let x = document().get_element_by_id("remover").unwrap();
        x.parent_element().unwrap().remove();
    };

    let absorb_item = move |ev: web_sys::DragEvent| {
        ev.prevent_default();

        let data = ev.data_transfer().unwrap().get_data("text").unwrap();

        // Only accepts celestial bodies
        for variant in Ctz::iter() {
            if data
                == variant
                    .get_str("Name")
                    .expect("Name to be on enum")
                    .to_string()
            {
                output.set(data.clone());
                let proper_data = match data.clone() {
                    _ => "Tz Soon..".to_string(),
                };

                celestial_input.set(proper_data);
                console_log("Yes");
                break;
            }
        }
    };

    let allow_absorbtion = |ev: DragEvent| {
        ev.prevent_default();
    };

    let register_calendar = move |ev: web_sys::Event| {
        // reset middleman + static_date every calendar change to avoid innacuraccy
        middleman.set("".to_string());
        static_date.set("".to_string());

        // Updates the calender before middle man gets updated
        calendar_input.update(move |calendar| {
            let value = select_el.get().expect("<select> to exist").value();

            *calendar = value.clone();

            console_log(&format!("Registered Calendar: {:?}", *calendar));
        });

        // updates everything along with the middle man's value, and with static's date
        date_symphony_v2(select_el, middleman, static_date, yel, mel, del);
    };

    view! {
        <div class="">
            <p class="text-base">Earth Date to Space Date</p>
        </div>
        <button id="remover" class="absolute inset-y-0 z-20 end-0 cursor-pointer rounded-xl dark:bg-red-800 bg-red-200 hover:animate-pulse focus:animate-pulse"
                on:click=remove_session
        >
            <p class="h-px w-12">" "</p>
        </button>
        <section class="px-12 py-4 grid grid-rows-3 items-center gap-2">
            <article class="flex space-x-2 w-full">
                <div class="p-2">
                    <figcaption class="text-base text-start font-light">Choose a Calendar</figcaption>
                    <section class="p-4">
                        <CalendarSelection reactive_node=select_el on_input=register_calendar />
                    </section>
                </div>
                <div class="p-2">
                    <figcaption class="text-base text-start font-light">Enter a Date</figcaption>
                    <section class="p-2">
                        <MultiCalendarDatePicker spectator=select_el year_el=yel month_el=mel day_el=del bridge=middleman observer=static_date/>
                        <section class="p-2">
                            <p> In Gregorian: {static_date} </p>
                        </section>
                    </section>
                </div>
            </article>
            <div class="hover:bg-amber-200 dark:bg-slate-800 bg-slate-100 rounded-xl w-full text-center items-center p-4">
                <span on:drop=absorb_item on:dragover=allow_absorbtion class="p-4">{output}</span>
            </div>
            <div class="dark:bg-slate-800 bg-slate-100 rounded-xl leading-relaxed w-full text-center items-center p-4">
                <span id="earth-date-output" class="p-2 flex space-x-2">
                    <p>{middleman}</p>
                    <p>{celestial_input}</p>
                </span>
            </div>
        </section>
    }
}
