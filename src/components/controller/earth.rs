use std::{str::FromStr, sync::Arc, time::Duration};

use crate::{
    components::tools::ctz::Ctz,
    wrappers::{
        strings::{filtered_vec, get_initials, matching_left},
        web::{all_items, save_data, update_dom_el},
    },
};
use chrono::prelude::*;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::{OffsetName, Tz, TZ_VARIANTS};
use icu_calendar::{chinese::Chinese, Calendar, *};
use leptos::{
    html::{Input, Select},
    leptos_dom::logging::console_log,
    *,
};
use leptos_icons::*;
use strum::IntoEnumIterator;
use strum::*;
use wasm_bindgen::JsCast;
use web_sys::{
    DragEvent, HtmlElement, HtmlHeadingElement, HtmlInputElement, InputEvent, KeyboardEvent,
    MouseEvent, Node,
};

#[component]
pub fn Spooler() -> impl IntoView {
    let add_icon = Icon::from(AiIcon::AiPlusOutlined);

    let add_tz_session = move |_| {
        let view = view! {
            <span>
                <TzSessionBox/>
            </span>
        };

        document()
            .get_element_by_id("earth-tz-mod")
            .unwrap()
            .append_child(view.as_ref())
            .unwrap();
    };
    let add_date_session = move |_| {
        let view = view! {
            <span>
                <DateSessionBox/>
            </span>
        };

        document()
            .get_element_by_id("earth-date-mod")
            .unwrap()
            .append_child(view.as_ref())
            .unwrap();
    };

    view! {
        <div id="earth-spooler" class="p-4 rounded-xl cursor-pointer">
            <section id="earth-tz-buttons" class="flex space-x-4 items-center ">
                    <div class="relative">
                        <button id="spool-tz" class="p-2 dark:hover:bg-blend-lighten dark:mix-blend-screen w-full text-start cursor-pointer
                        -skew-y-3 scale-75 hover:-translate-y-2 hover:scale-100 focus:-translate-y-2 focus:scale-100
                        ease-in-out duration-300 glitch desktop:text-2xl laptop:text-xl tablet:text-base text-sm
                        flex space-x-4
                    " on:click=add_tz_session>
                            <Icon icon=add_icon class="w-8 h-8" />
                            Timezone
                        </button>
                    </div>
                    <div class="relative">
                        <button id="spool-date" class="p-2 dark:hover:bg-blend-lighten dark:mix-blend-screen w-full text-start cursor-pointer
                        -skew-y-3 scale-75 hover:-translate-y-2 hover:scale-100 focus:-translate-y-2 focus:scale-100 
                        ease-in-out duration-300 glitch desktop:text-2xl laptop:text-xl tablet:text-base text-sm
                        flex space-x-4
                    "
                     on:click=add_date_session
                >
                            <Icon icon=add_icon class="w-8 h-8"/>
                            Date
                        </button>
                    </div>
            </section>

            <div>
                <TzSessionBox/>
                <DateSessionBox/>
                // Where you convert Earth Timezone -> Space Timezone
                // <div id="earth-tz-mod"></div>

                // Where you convert Earth Dates -> Space Dates
                // <div id="earth-date-mod"></div>
            </div>
        </div>
    }
}

#[component]
pub fn Dragger() -> impl IntoView {
    view! {
        <div id="earth-spooler" class="p-4 rounded-xl cursor-pointer">
            <div id="box-a">

            </div>
            <div id="box-z">

            </div>

        </div>
    }
}

#[component]
pub fn TzSessionBox() -> impl IntoView {
    view! {
        <div id="earth-sp-s1" class="py-2">
            <section>
                <p>"Earth Timezone -> Space Time"</p>
            </section>
            <div>
                <section class="dark:bg-slate-900 bg-slate-200">
                    <div class="">Drag a timezone here "(:"</div>
                </section>

                <section class="dark:bg-slate-900 bg-slate-200">
                    <div class="">Output here "(:"</div>
                </section>
            </div>
        </div>
    }
}

#[component]
pub fn DateSessionBox() -> impl IntoView {
    let close_icon = Icon::from(BiIcon::BiXCircleRegular);
    let date_el: NodeRef<Input> = create_node_ref();
    let cal_el: NodeRef<Select> = create_node_ref();
    let output = create_rw_signal("Drop a Celestial Body Here!".to_string());
    let date_input = create_rw_signal("".to_string());    
    let middleman_input = create_rw_signal("".to_string());
    let calendar_input = create_rw_signal("gregorian".to_string());
    let celestial_input = create_rw_signal("".to_string());

    let remove_sesion = move |ev: MouseEvent| {
        console_log("Ok");
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

    let read_date = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);

        // Reads date and puts it into chrono
        let parse_from_str = NaiveDate::parse_from_str;
        let chrono_str = parse_from_str(&value, "%Y-%m-%d").unwrap();
        let year = chrono_str.year();
        let month = chrono_str.month();
        let day = chrono_str.day();
        let has_leapyear = chrono_str.leap_year();
        let chrono_value = chrono_str.format("%A, %-d %B, %C%y").to_string();

        // sets & some tests
        date_input.set(chrono_value.clone());
        // This middleman allows Back & Forth Conversion with the calendar
        middleman_input.set(chrono_value.clone());
        console_log(&format!(
            "Calendar Reading: {:?} | Leap Year: {:?} | ",
            chrono_value.clone(),
            has_leapyear
        ));
    };

    let read_calendar = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        // if calendar changes then change the date_input
        match event_target_value(&ev).as_str() {
            "gregorian" => {
                let original = middleman_input.get_untracked();

                date_input.set(original);
                // converts value of date_input to gregorian date
                console_log("gregorian");
            }
            "chinese" => {
                // converts value of date_input to chinese date
                let chrono_date = date_input.get_untracked();
                let parse_from_str = NaiveDate::parse_from_str;
                let chrono_str = parse_from_str(&chrono_date, "%A, %-d %B, %C%y").unwrap();
                let year = chrono_str.year();
                let month = chrono_str.month();
                let day = chrono_str.day();

                let mut icu =
                    icu_calendar::Date::try_new_iso_date(year as i32, month as u8, day as u8)
                        .unwrap();

                let to_chinese = icu.to_calendar(chinese::Chinese::default());

                let chinese_date = format!(
                    "Cyclic {:?}, {:?}/{:?}/{:?}",
                    to_chinese.year().cyclic.expect("Cyclic to be present"),
                    to_chinese.year().number,
                    to_chinese.month().ordinal,
                    to_chinese.day_of_month().0,
                );

                date_input.set(chinese_date);
                // console_log(&format!("{:?} POI!", to_chinese));
                // console_log(chinese_date.as_str());

                console_log("chinese");
            }
            "islamic" => {
                console_log("islamic");
            }
            "indian" => {
                console_log("indian");
            }
            _ => {
                console_log("not implemented");
            }
        };

        calendar_input.set(value);
        // console_log(&format!("Reading: {:?}", value));
    };

    let scan_date = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        // console_log(&format!("Scanning: {:?}", value));
    };

    // takes on the id of the timezone dropped, if duplicates then append increment a number adjancently
    view! {
        <div id="earth-sp-s2" class="p-4 dark:bg-slate-900 bg-slate-200 rounded-xl relative">
            <div class="">
                <p class="text-base">Earth Date to Space Date</p>
            </div>
            <div id="remove-session" class="">
                <button class="
                    absolute inset-y-0 z-20 end-0 cursor-pointer dark:bg-red-800 bg-red-200 rounded-xl
                    hover:bg-rose-800 focus:bg-red-600
                    " on:click=remove_sesion>
                    <p class="h-px w-4">" "</p>
                </button>
            </div>
            <section class="p-4 grid grid-rows-3 items-center gap-2">
                <article class="flex space-x-2 w-full">
                    <div class="p-2 flex space-x-2">
                        <select
                        node_ref=cal_el on:input=read_calendar
                        id="earth-cal" class="px-2 dark:bg-slate-900 bg-slate-200 cursor-pointer"
                        oninput="this.form.requestSubmit()">
                             <option class="" value="gregorian">Gregorian</option>
                             <option class="" value="indian">Indian</option>
                             <option class="" value="islamic">Islamic</option>
                             <option class="" value="chinese">Chinese</option>
                        </select>
                        <input node_ref=date_el on:input=read_date on:change=scan_date
                            type="date" class="p-2 hover:bg-amber-200 dark:text-white text-black dark:bg-slate-900 bg-slate-200 rounded-xl" />
                    </div>
                </article>
                <div class="hover:bg-amber-200 dark:bg-slate-800 bg-slate-100 rounded-xl w-full text-center items-center p-4">
                    <span on:drop=absorb_item on:dragover=allow_absorbtion class="p-4">{output}</span>
                </div>
                <div class="dark:bg-slate-800 bg-slate-100 rounded-xl leading-relaxed w-full text-center items-center p-4">
                    <span id="earth-date-output" class="p-2 flex space-x-2">
                        <p>{date_input}</p>
                        <p>{celestial_input}</p>
                    </span>
                </div>
            </section>
        </div>
    }
}
