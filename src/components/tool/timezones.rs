use crate::components::tool::DateLength;
use crate::components::tool::DATEFORMAT;
use crate::wrappers::time_offsets::*;
use crate::wrappers::web::update_dom_el;
use chrono::prelude::*;
use chrono::Duration;
use leptos::html::Input;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::SubmitEvent;
use web_sys::{MediaQueryList, Storage};

#[component]
pub fn CustomTimeZone(zone: &'static str, abbr: i32, abbr_name: &'static str) -> impl IntoView {
    let customtime = move || {
        let offset = FixedOffset::east_opt(abbr).unwrap();

        return Utc::now()
            .with_timezone(&offset)
            .format(DATEFORMAT)
            .to_string();
    };

    let (time, set_time) = create_signal(customtime());

    set_interval(
        move || {
            set_time.update(|t| {
                *t = customtime();
            })
        },
        std::time::Duration::from_secs(1),
    );

    view! {
        <div class="p-4">
            <h1>{abbr_name}:  {zone}: {time} </h1>
        </div>
    }
}

#[component]
pub fn InnerPlanetTimeZone(
    zone: &'static str,
    length: DateLength,
    abbr_name: &'static str,
) -> impl IntoView {
    let planet_length = DateLength {
        hour: length.hour,
        min: length.min,
        sec: length.sec,
        year_discovered: length.year_discovered,
        month: length.month,
        day: length.day,
    };

    // sets abbreviations which hold the offset calculation
    let (super_abbr, set_super_abbr) = create_signal(UTC);

    let customtime = move || {
        let chosen_timezone = FixedOffset::east_opt(super_abbr.get_untracked()).unwrap();

        return Utc::now()
            // Below is how the time additions or subtractions are implemented
            // .checked_sub_signed().expect("")
            .checked_add_signed(Duration::hours(planet_length.hour as i64))
            .expect("hour not found")
            .checked_add_signed(Duration::minutes(planet_length.min as i64))
            .expect("minute not found")
            .checked_add_signed(Duration::seconds(planet_length.sec as i64))
            .expect("second not found")
            .checked_add_signed(Duration::days(planet_length.day as i64))
            .expect("day not found")
            .with_timezone(&chosen_timezone)
            .format(DATEFORMAT)
            .to_string();
    };

    let (time, set_time) = create_signal(customtime());

    set_interval(
        move || {
            create_effect(move |_| {
                set_super_abbr.update(|sup| {
                    // This is how I set the timezones
                    let timezone = match document()
                        .get_element_by_id("timezone")
                        .expect("That Timezone does not exist")
                        .get_attribute("data-timezone")
                        .expect("data-timezone not existhj")
                        .as_str()
                    {
                        "EST" => EST,
                        "PSTP" => PSTP,
                        "PST" => PST,
                        "NUT" => NUT,
                        _ => UTC,
                    };

                    *sup = timezone;

                    // let timezone_str = format!("timezone: {}", timezone);
                    // console_log(timezone_str.as_str());
                });
            });

            set_time.update(|t| {
                *t = customtime();
            })
        },
        std::time::Duration::from_secs(1),
    );

    view! {
        <div class="p-4 flex  gap-2">
            <h1>{abbr_name}: {zone}: {time} </h1>
            <ChooseTimeZone />
        </div>
    }
}

#[component]
pub fn ChooseTimeZone() -> impl IntoView {
    let (panel, set_panel) = create_signal(false);
    let (class, set_class) = create_signal("hidden");

    let update_panel = move || {
        set_panel.update(|toggle| {
            if *toggle {
                set_class.set("hidden");
                update_dom_el("zones", class.get());
            } else {
                set_class.set("");
                update_dom_el("zones", class.get());
            }

            *toggle = !*toggle
        })
    };

    // Button toggles panel by default
    let toggle_panel = move |_| {
        update_panel();
    };

    view! {

        <main class="relative inline-block text-left shadow-lg border border-blue-200 rounded-lg">
            <button id="time-btn" on:click=toggle_panel type="button" class="dark:bg-blue-800 bg-blue-400 rounded-lg px-2 inline-flex justify-center items-center">
            </button>
        </main>
    }
}

#[component]
pub fn TimeZoneConfig() -> impl IntoView {
    let (zone, set_zone) = create_signal("UTC".to_string());

    create_effect(move |_| {
        document()
            .get_element_by_id("time-btn")
            .expect("<button> to exist")
            .set_inner_html(format!("Timezone: {}", zone.get()).as_str());
    });

    let get_zone = move |ev| {
        let value = event_target_value(&ev);

        set_zone.set(value.clone());

        // console_log(value.clone().as_str());
    };

    let button_type = "submit";

    let common_th =
        " px-6 py-3 border-r border-l border-t border-slate-200 text-black dark:text-white ";
    let common_tr = "px-6 py-3 border-r border-l border-b border-t border-slate-200 text-black dark:text-white ";
    let common_td = " px-6 py-3 border-r border-slate-200 hover:bg-slate-200 hover:text-slate-800 dark:hover:text-black text-black dark:text-white ";
    let common_input =
        " cursor-pointer transition hover:scale-125 delay-150 duration-75 ease-in-out text-xl";
    view! {
    <div id="zones" class="hidden">
         <table class="w-full text-sm text-left rounded-lg">
            <thead class="text-xs uppercase bg-transparent">
                <tr>
                    <th class=common_th>Earth </th>
                    <th class=common_th>Inner Planets</th>
                    <th class=common_th>Outer Planets</th>
                    <th class=common_th>Significant Asteroids</th>
                </tr>
            </thead>
            <tbody>
                <tr class=common_tr>
                    <td class=common_td>
                        <input name="timezone" value="EST" type=button_type on:click=get_zone class=common_input />
                    </td>
                    <td class=common_td>
                        <input name="timezone" value="MRS" type=button_type on:click=get_zone class=common_input />
                    </td>
                    <td class=common_td>
                        <input name="timezone" value="WIP" type=button_type on:click=get_zone class=common_input />
                    </td>
                    <td class=common_td>
                        <input name="timezone" value="WIP" type=button_type on:click=get_zone class=common_input />
                    </td>
                </tr>
                <tr class=common_tr>
                    <td class=common_td >
                        <input name="timezone" value="PSTP" type=button_type on:click=get_zone class=common_input />
                    </td>
                </tr>
                <tr class=common_tr>
                    <td class=common_td>
                        <input name="timezone" value="PST" type=button_type on:click=get_zone class=common_input />
                    </td>
                </tr>
                <tr class=common_tr>
                    <td class=common_td>
                        <input name="timezone" value="UTC" type=button_type on:click=get_zone class=common_input />
                    </td>
                </tr>
            </tbody>
            <caption class="caption-bottom py-2 text-black dark:text-white ">
                Adjustable UTC Offsets configurations for Mars Standard Time
                <p class="hidden" id="timezone" attr:data-timezone=zone for=zone>{zone}</p>
            </caption>
        </table>
    </div>
    }
}
