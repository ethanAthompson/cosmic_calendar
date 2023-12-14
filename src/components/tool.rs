pub mod timezones;

use chrono::prelude::*;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::{fmt, time::Duration};
use web_sys::{MediaQueryList, Storage};

use crate::components::tool::timezones::CustomTimeZone;
use crate::components::tool::timezones::InnerPlanetTimeZone;
use crate::components::tool::timezones::TimeZoneConfig;

use crate::wrappers::time_offsets::*;

/// format str
pub const DATEFORMAT: &'static str = "%A %d/%m/%Y %r";
/// A data structure that collects the datelength
pub struct DateLength {
    pub hour: u32,
    pub min: u32,
    pub sec: u32,
    pub year_discovered: u32,
    pub month: u32,
    pub day: u32,
}

#[component]
pub fn Tool() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center p-4">
            <div class="p-4 ring-2 ring-inherit rounded-lg">
                <section class="p-4 text-black dark:text-white ">
                    <h1> Current Supported Timezones </h1>
                    <p class="h-px dark:bg-slate-200 bg-white w-full"> </p>
                </section>

                <section class="shadow-lg gap-4 flex flex-col ring-2 ring-orange-300 rounded-lg p-4">
                    <section>
                        <p class="text-black dark:text-white "> Earth Timezones </p>
                        <article class="shadow-lg text-black dark:text-white ">
                            <CustomTimeZone zone="Eastern Standard Time" abbr=EST abbr_name="EST"/>
                            <CustomTimeZone zone="Philippine Standard Time" abbr=PSTP abbr_name="PSTP" />
                            <CustomTimeZone zone="Pacific Standard Time" abbr=PST abbr_name="PST" />
                            <CustomTimeZone zone="Niue Standard Time" abbr=NUT abbr_name="NUT" />
                        </article>
                    </section>
                    <section>
                        <p class="text-black dark:text-white "> Inner Planet Timezones </p>
                        <article class="shadow-lg text-black dark:text-white ">
                            <InnerPlanetTimeZone zone="Mars Standard Time" length=MARSLENGTH abbr_name="MST"/>
                        </article>
                    </section>
                </section>
            </div>
            <div class="px-2"></div>
            <div class="mr-2 p-4 ring-2 ring-inherit rounded-lg">
                 <section class="p-4 text-black dark:text-white ">
                    <h1> Click the Timezone button to get started</h1>
                    <p class="h-px dark:bg-slate-200 bg-white w-full"></p>
                </section>

                <section class="relative overflow-x-auto">
                    <TimeZoneConfig/>
                </section>
            </div>
        </div>
    }
}

const MARSLENGTH: DateLength = DateLength {
    hour: 24,
    min: 37,
    sec: 22,
    year_discovered: 1610,
    // custom
    month: 0,
    day: 0,
};
