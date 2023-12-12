use chrono::prelude::*;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::{fmt, time::Duration};
use web_sys::{MediaQueryList, Storage};

use crate::wrappers::time_offsets::*;

#[component]
pub fn Tool() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center p-4">
            <div class="p-4 ring-2 ring-inherit rounded-lg">
                <section class="p-4">
                    <h1> You can start calculating below.. </h1>
                    <p class="h-px dark:bg-slate-200 bg-white w-full"> </p>
                </section>

                <section class="shadow-lg gap-4 flex flex-col ring-2 ring-bg-gray-300 rounded-lg p-4">
                    <section>
                        <p> Earth Timezones </p>
                        <article class="shadow-lg">
                            <CustomTimeZone zone="Eastern Standard Time" abbr=EST />
                            <CustomTimeZone zone="Philippine Standard Time" abbr=PSTP />
                            <CustomTimeZone zone="Pacific Standard Time" abbr=PST />
                            <CustomTimeZone zone="Niue Standard Time" abbr=NUT />
                        </article>
                    </section>
                    <section>
                        <p> Inner Planet Timezones </p>
                        <article class="shadow-lg">
                            <InnerPlanetTimeZone zone="Mercury Standard Time" abbr=MYT/>
                        </article>
                    </section>
                </section>
            </div>
        </div>
    }
}

#[component]
pub fn CustomTimeZone(zone: &'static str, abbr: i32) -> impl IntoView {
    let customtime = move || {
        let offset = FixedOffset::east_opt(abbr).unwrap();

        return Utc::now()
            .with_timezone(&offset)
            .format("%d/%m/%Y %r")
            .to_string();
    };

    let (time, set_time) = create_signal(customtime());

    set_interval(
        move || {
            set_time.update(|t| {
                *t = customtime();
            })
        },
        Duration::from_secs(1),
    );

    view! {
        <div class="p-4">
            <h1> {zone} Time: {time} </h1>
        </div>
    }
}

#[component]
pub fn InnerPlanetTimeZone(zone: &'static str, abbr: i32) -> impl IntoView {
    let time = 0;

    let status = "WIP";

    view! {
        <div class="p-4">
            // <h1> Saturn Time: {time} </h1>
            <h1> {zone} : {status} </h1>
        </div>
    }
}
