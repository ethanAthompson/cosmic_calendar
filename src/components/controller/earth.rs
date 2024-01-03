// Finish what you set to do today
// Clean up footer maybe?
// lean up entire rust code, get it looking neat and clean, avoid redundancies.
// You can get the space stuff later.

use std::{str::FromStr, sync::Arc, time::Duration};

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
use chrono_tz::{OffsetName, Tz, TZ_VARIANTS};
use icu_calendar::{
    any_calendar::AnyDateInner,
    buddhist::Buddhist,
    chinese::Chinese,
    coptic::Coptic,
    dangi::Dangi,
    ethiopian::Ethiopian,
    hebrew::Hebrew,
    indian::Indian,
    islamic::*,
    japanese::{Japanese, JapaneseExtended},
    julian::Julian,
    persian::Persian,
    roc::Roc,
    *,
};
use leptos::{
    html::{Input, Select},
    leptos_dom::logging::console_log,
    *,
};
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
pub fn Spooler() -> impl IntoView {
    let add_icon = Icon::from(AiIcon::AiPlusOutlined);
    let count = create_rw_signal(0);
    
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
        count.update(move |value| {
            *value += 1;
        });

        let view = view! {
            <span>
                <DateSessionBox session_id=format!("{:?}xSession", count)/>
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
                // Where you convert Earth Timezone -> Space Timezone
                <div id="earth-tz-mod" class="py-4 flex flex-col space-y-4"></div>

                // Where you convert Earth Dates -> Space Dates
                <div id="earth-date-mod" class="py-4 flex flex-col space-y-4"></div>
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
pub fn DateSessionBox(session_id: String) -> impl IntoView {
    let close_icon = Icon::from(BiIcon::BiXCircleRegular);
    let input_el: NodeRef<Input> = create_node_ref();
    let select_el: NodeRef<Select> = create_node_ref();
    let output = create_rw_signal("Drop a Celestial Body Here!".to_string());
    let middleman = create_rw_signal("".to_string());
    let calendar_input = create_rw_signal("IslamicCivil".to_string());
    let celestial_input = create_rw_signal("".to_string());
    let final_date = create_rw_signal("".to_string());

    // MM/DD/YY
    let mel: NodeRef<Input> = create_node_ref();
    let del: NodeRef<Input> = create_node_ref();
    let yel: NodeRef<Input> = create_node_ref();

    let remove_sesion = move |ev: MouseEvent| {
        console_log(ev.target().unwrap().dyn_into::<HtmlElement>().unwrap().id().as_str());
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
        // reset middleman every calendar change to avoid innacuraccy
        middleman.set("".to_string());
        let value = select_el.get().expect("<select> to exist").value();
        let vy = yel.get().expect("<select> to exist").value();
        let vm = mel.get().expect("<select> to exist").value();
        let vd = del.get().expect("<select> to exist").value();
        date_symphony_v2(select_el, calendar_input, yel, mel, del);
        calendar_input.set(value.clone());

        console_log(&format!("Registered Calendar: {:?}", value.clone()));
    };

    // takes on the id of the timezone dropped, if duplicates then append increment a number adjancently
    view! {
        <div id=session_id class="p-4 dark:bg-slate-900 bg-slate-200 rounded-xl relative">
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
                    <div class="p-2">
                        <figcaption class="text-base text-start font-light">Choose a Calendar</figcaption>
                        <section class="p-4">
                            <select node_ref=select_el on:input=register_calendar id="earth-cal" class="p-2 dark:bg-slate-900 bg-slate-200 cursor-pointer roudned-xl dark:text-white text-black">
                                 <optgroup label="Islamic">
                                    <option class="" value="IslamicCivil">Islamic Civil</option>
                                    <option class="" value="IslamicTabular">Islamic Tabular</option>
                                    <option class="" value="IslamicObservational">Islamic Observational</option>
                                    <option class="" value="IslamicUmmAlQura">Islamic Umm AlQura</option>
                                 </optgroup>
                                 <optgroup label="Chinese">
                                     <option class="" value="Chinese">Chinese</option>
                                     <option class="" value="RepublicOfChina">Republic of China</option>
                                </optgroup>
                                 <optgroup label="Japanese">
                                     <option class="" value="Japanese">Japanese</option>
                                     <option class="" value="JapaneseExtended">Japanese Extended</option>
                                </optgroup>
                                <optgroup label="Others">
                                     <option class="" value="Gregorian">Gregorian</option>
                                     <option class="" value="Iso">Iso</option>
                                     <option class="" value="Indian">Indian</option>
                                     <option class="" value="Iuddhist">Buddhist</option>
                                     <option class="" value="Coptic">Coptic</option>
                                     <option class="" value="Dangi">Dangi</option>
                                     <option class="" value="Ethiopian">Ethiopian</option>
                                     <option class="" value="Hebrew">Hebrew</option>
                                     <option class="" value="Julian">Julian</option>
                                     <option class="" value="Persian">Persian</option>
                                </optgroup>
                            </select>
                        </section>
                    </div>
                    <div class="p-2">
                        <figcaption class="text-base text-start font-light">Enter a Date</figcaption>
                        <section class="p-2">
                            <MultiCalendarDatePicker spectator=select_el year_el=yel month_el=mel day_el=del bridge=middleman/>
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
        </div>
    }
}

// put in seperate file maybe?
#[component]
pub fn MultiCalendarDatePicker(
    #[prop(into)] year_el: NodeRef<Input>,
    #[prop(into)] month_el: NodeRef<Input>,
    #[prop(into)] day_el: NodeRef<Input>,
    /// The calendar reactivity: did I update? A -> Node Ref
    #[prop(into)]
    spectator: NodeRef<Select>,
    /// Connects middle man
    #[prop(into)]
    bridge: RwSignal<String>,
) -> impl IntoView {
    // actually use tailwind properly.
    let common = "rounded-none dark:text-white text-black dark:bg-slate-700 bg-slate-200";

    let debug_recv = move |_ref: NodeRef<Input>| {
        let value = _ref.get().expect("Value").value();
        let name = _ref.get().unwrap().placeholder();

        console_log(&format!("{:?}: {:?}", name, &value.as_str()));
        console_log(&format!("Watching: {:?}", spectator.get().unwrap().value()));
    };

    let date_symphony = move |guard: NodeRef<Select>| {
        let vy = year_el.get().expect("<select> to exist").value();
        let vm = month_el.get().expect("<select> to exist").value();
        let vd = day_el.get().expect("<select> to exist").value();

        date_symphony_v2(spectator, bridge, year_el, month_el, day_el);
    };

    let recv_year = move |ev: web_sys::Event| {
        debug_recv(year_el);
        date_symphony(spectator);
    };

    let recv_day = move |ev: web_sys::Event| {
        debug_recv(day_el);
        date_symphony(spectator);
    };

    let recv_month = move |ev: web_sys::Event| {
        debug_recv(month_el);
        date_symphony(spectator);
    };

    view! {
        <div class="p-2 grid items-center justify-center
                desktop:grid-cols-3 laptop:grid-cols-3 
                tablet:grid-cols-3 grid-rows-3 phone:space-y-2 watch:space-y-2 space-y-2
                desktop:space-x-2 laptop:space-x-2 tablet:space-x-2" desktop:space-y-0 laptop:space-y-0 tablet:space-y-0
        >
            <input type="text" placeholder="Month" id="Month" on:input=recv_month node_ref=month_el class=common maxlength="2"/>
            <input type="text" placeholder="Day" id="Day" on:input=recv_day node_ref=day_el  class=common maxlength="2"/>
            <input type="text" placeholder="Year" id="Year" on:input=recv_year node_ref=year_el class=common maxlength="4"/>
        </div>
    }
}

/// Your calendar is considered in the calculation of the chosen space dates
/// Only 5 Calendars are supported as of now
/// Chinese + IslamicCivil + ISO + Gregorian + Indian
pub fn date_symphony_v2(
    spectator: NodeRef<Select>,
    bridge: RwSignal<String>,
    vy: NodeRef<Input>,
    vm: NodeRef<Input>,
    vd: NodeRef<Input>,
) {
    let vy = vy.get().expect("<select> to exist").value();
    let vm = vm.get().expect("<select> to exist").value();
    let vd = vd.get().expect("<select> to exist").value();
    let spectator_name = spectator.get().unwrap().value();

    match spectator.get().unwrap().value().as_str() {
        "Gregorian" => {
            let icu = icu_calendar::Date::try_new_gregorian_date(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
            )
            .unwrap()
            .to_calendar(Gregorian::default());

            bridge.set(format!(
                "{:?}: {:?}/{:?}/{:?}",
                spectator_name,
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));
        }
        "Chinese" => {
            let icu = icu_calendar::Date::try_new_chinese_date_with_calendar(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
                Chinese::default(),
            )
            .unwrap();

            bridge.set(format!(
                "{:?}, {:?}: {:?}/{:?}/{:?}",
                spectator_name,
                icu.month().code.to_string(),
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));
        }
        "Iso" => {
            let icu = icu_calendar::Date::try_new_iso_date(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
            )
            .unwrap()
            .to_calendar(Iso);

            bridge.set(format!(
                "{:?}: {:?}/{:?}/{:?}",
                spectator_name,
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));
        }
        "IslamicCivil" => {
            let icu = icu_calendar::Date::try_new_islamic_civil_date_with_calendar(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
                IslamicCivil::new_always_calculating(),
            )
            .unwrap();

            bridge.set(format!(
                "{:?}: {:?}/{:?}/{:?}",
                spectator_name,
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));
        }
        "Inidan" => {
            let icu = icu_calendar::Date::try_new_indian_date(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
            )
            .unwrap()
            .to_calendar(Indian::default());

            bridge.set(format!(
                "{:?}: {:?}/{:?}/{:?}",
                spectator_name,
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));
        }
        _ => {
            bridge.set(format!("Date Not Implemented"));
        }
    };
    console_log(&format!("Chosen Calendar: {:?}", spectator_name.as_str()));
}
