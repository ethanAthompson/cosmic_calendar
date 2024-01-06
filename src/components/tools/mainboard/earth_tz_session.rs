// When the user clicks off of a card, then pause the time, saving it. memo action
// Organize this code
// Use children() in your props
// make lots of props, reduce rendundancy
// Don't sweat it, you have 70% of the work done
// clean code base before implemnting !!
// actualyl comment your code before moving on
// save to github though
// it'll only take you a week or two to implement your research
//
// header, footer, navbar in one place
// you dont need seperate folders for them
// put pages in a /page folder
// use css to organize your program, to
// keep able to work on it without eros
//
//
//

use std::{str::FromStr, sync::Arc, time::Duration};

use crate::components::search::calendar::SelectBar as CalendarSelection;
use crate::components::tools::mainboard::custom_calendar::MultiCalendarDatePicker;
use crate::wrappers::date::date_symphony_v2;
use crate::{
    components::tools::ctz::Ctz,
    components::tools::mainboard::earth_date_session::MultiRemover,
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
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use strum::IntoEnumIterator;
use strum::*;
use wasm_bindgen::JsCast;
use web_sys::{
    DragEvent, FocusEvent, HtmlElement, HtmlHeadingElement, HtmlInputElement, InputEvent,
    KeyboardEvent, MouseEvent, Node,
};

#[component]
pub fn Card() -> impl IntoView {
    let tz_output = create_rw_signal("".to_string());
    let ctz_output = create_rw_signal("".to_string());
    let tz_time = create_rw_signal("".to_string());
    let ctz_time = create_rw_signal("".to_string());
    let middleman = create_rw_signal("".to_string());
    let celestial_input = create_rw_signal("".to_string());
    let static_time = create_rw_signal("".to_string());
    let static_celestial = create_rw_signal("".to_string());
    // This allows the user to select a celestial body and a timezone
    // and recieve backwards compatability and forwards comapatibility.
    // mars time in earth time, and earth time in mars time.
    let swapped = create_rw_signal(false);
    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn(
        move || {
            // if tz has a timezone dropped into it
            if tz_output.get().is_empty() == false {
                if let Ok(timezone) = chrono_tz::Tz::from_str(tz_output.get().as_str()) {
                    let datetime =
                        DateTime::with_timezone(&Utc::now(), &timezone).format("%Y/%m/%d %r %Z");
                    tz_time.set(datetime.to_string());
                    static_time.set(datetime.to_string());

                    // INFO! Temporary
                    static_celestial.set(ctz_output.get());
                }
            }
        },
        1000,
    );

    let resume = resume.clone();
    let pause = pause.clone();

    view! {
        <div class="">
            <p class="text-base">"Earth Timezone to Space Time"</p>
        </div>
        <MultiRemover id="tz-remover"/>
        // <div on:mouseout=move |_| pause() on:mouseover=move |_| resume() id="detect-tz-parent" class="px-12 py-2 grid grid-rows-3 items-center gap-2">
        <div id="detect-tz-parent" class="px-12 py-2 grid grid-rows-3 items-center gap-2">
            <Show when=move || is_active.get() == true fallback=move || view! {<HiddenCard/>}>
                <article class="flex space-x-2 w-full">
                    <div class="p-2">
                        <figcaption class="text-base text-start font-light">Drop a timezone</figcaption>
                        <section class="p-2 first:">
                             <Dropper matcher=ZoneChoices::EarthTimezones output=tz_output class="hover:bg-amber-200 dark:bg-slate-800 bg-slate-100 rounded-xl w-full text-center items-center p-4" />
                        </section>
                        <section class="p-2 first:">
                             <Dropper matcher=ZoneChoices::CelestialTimezones output=ctz_output class="hover:bg-amber-200 dark:bg-slate-800 bg-slate-100 rounded-xl w-full text-center items-center p-4" />
                        </section>
                        <div class="p-2">
                            <article class="p-2">
                                // This button renders a componet
                                <button 
                                    on:click=move |_| swapped.update(|v| *v = !*v) 
                                    class="p-2 outline hover:bg-inherit"
                                >
                                    Swap
                                </button>
                            </article>
                            <Show when=move || swapped.get() fallback=move || view!{<Unswapped static_time=static_time static_celestial=static_celestial/>}>
                                <article class="p-2">
                                    <section class="p-2">
                                        <p> In Celestial Time: <em class="px-2">{static_celestial}</em> </p>
                                    </section>
                                    <section class="p-2">
                                        <p> In Earth Time: <em class="px-2">{static_time}</em></p>
                                    </section>
                                </article>
                            </Show>
                        </div>
                   </div>
                </article>
                // </Show>
                <div class="dark:bg-slate-800 bg-slate-100 rounded-xl leading-relaxed w-full text-center items-center p-4">
                    <span id="earth-date-output" class="p-2 flex space-x-2">
                        <p>{middleman}</p>
                        <p>{celestial_input}</p>
                    </span>
                </div>
            </Show>
        </div>

    }
}

#[component]
pub fn Unswapped(
    #[prop(into)] static_time: RwSignal<String>,
    #[prop(into)] static_celestial: RwSignal<String>,
) -> impl IntoView {
    // Do Different Calculation, you can pass down signals from here if you want
    view! {
        <article class="p-2">
            <section class="p-2">
                <p> In Earth Time: <em class="px-2">{static_time}</em></p>
            </section>  
            <section class="p-2">
                <p> In Celestial Time: <em class="px-2">{static_celestial}</em> </p>
            </section>
        </article>
    }
}
#[component]
pub fn HiddenCard() -> impl IntoView {
    view! {
        <div class="p-4 w-full h-full bg-gray-500">
            <p class="text-base"> Inactive for performance reasons </p>
            <figcaption class="text-base"> Select me ??? </figcaption>
        </div>
    }
}

#[component]
pub fn DisplaySingleCelestialTime(#[prop(into)] input: RwSignal<String>) -> impl IntoView {
    let time = create_rw_signal("Tz will be soon".to_string());
    // let timezone =
    //     chrono_tz::Tz::from_str(&input.get()).expect("an earth timezone to make it through!");
    // let datetime = DateTime::with_timezone(&Utc::now(), &timezone).format("%Y/%m/%d %r %Z");
    // time.set(datetime.to_string());

    // set_interval(
    //     move || {
    //         let timezone = chrono_tz::Tz::from_str(&input.get())
    //             .expect("an earth timezone to make it through!");
    //         let datetime = DateTime::with_timezone(&Utc::now(), &timezone).format("%Y/%m/%d %r %Z");
    //         time.set(datetime.to_string());
    //     },
    //     std::time::Duration::from_millis(1000),
    // );

    view! {
        <p class="text-base">{input} <em class="px-1">{time}</em></p>
    }
}

#[component]
pub fn DisplaySingleEarthTime(#[prop(into)] input: RwSignal<String>) -> impl IntoView {
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
    view! {
        <p class="text-base">{input} <em class="px-1">{time}</em></p>
    }
}

#[component]
pub fn Dropper(
    class: &'static str,
    output: RwSignal<String>,
    matcher: ZoneChoices,
) -> impl IntoView {
    let absorb_item = move |ev: web_sys::DragEvent| {
        ev.prevent_default();

        let data = ev.data_transfer().unwrap().get_data("text").unwrap();

        match matcher {
            ZoneChoices::EarthTimezones => {
                for variant in chrono_tz::TZ_VARIANTS {
                    if data == variant.to_string() {
                        output.set(data.clone());
                    }
                }
            }
            ZoneChoices::CelestialTimezones => {
                for variant in Ctz::iter() {
                    let item = variant
                        .get_str("Name")
                        .expect("Name to be on enum")
                        .to_string();

                    if data == item {
                        output.set(data.clone());
                        // let proper_data = match data.clone() {
                        //     _ => "Tz Soon..".to_string(),
                        // };

                        // output.set(proper_data);
                        break;
                    }
                }
            }
        }
    };

    let allow_absortion = |ev: DragEvent| {
        ev.prevent_default();
    };

    view! {
        <div class=class>
            <span on:drop=absorb_item on:dragover=allow_absortion class="p-4">{output}</span>
        </div>
    }
}

pub enum ZoneChoices {
    EarthTimezones,
    CelestialTimezones,
}
