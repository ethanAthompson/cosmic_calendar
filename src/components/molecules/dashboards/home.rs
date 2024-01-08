//

use crate::components::molecules::navbars::Component as Navbar;
use crate::components::molecules::sidebars::Component as DashboardSidebar;
use crate::components::atoms::charts::{SimpleGauge, update_gauge_timer, spawn_clocks_with_theme};

use crate::interfaces::ceramics::traditional::{
    CalendarIcon, DashboardLeftIcon, DashboardRightIcon, ImStackIcon, InfoIcon, MenuGridIcon,
    SettingIcon, TimeFiveIcon,
};
use charming::{
    datatype::{CompositeValue, DataPoint, DataPointItem},
    element::{Color, ItemStyle},
    *,
};
use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::A;
use leptos_use::use_element_hover;
use chrono::prelude::*;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::{OffsetComponents, OffsetName, Tz, TZ_VARIANTS, *};


#[component]
pub fn Page() -> impl IntoView {
    let hour = create_rw_signal(Local::now().hour12().1);
    let minute = create_rw_signal(Local::now().minute());
    let second = create_rw_signal(Local::now().second());

    let local_time = create_rw_signal(Local::now().format("%d/%m/%Y %r %Z").to_string());
    let input_location = create_rw_signal("".to_string());
    let input_time = create_rw_signal("".to_string());
    let input_time_middleman = create_rw_signal([
        hour.get_untracked(),
        minute.get_untracked(),
        second.get_untracked(),
    ]);
    let input_time_displ = create_rw_signal("".to_string());


    update_gauge_timer(input_time, input_time_middleman, local_time, hour, minute, second);
    spawn_clocks_with_theme(hour, minute, second);
    
    view! {
        <section class="p-4 grid grid-cols-2">
            <label class="dash-title">Home Page</label>
            <div class="p-4">
                <h1 class="dash-text"> Local Time </h1>
                <section class="p-2">
                    <p>{local_time}</p>
                </section>
            </div>
            <div class="p-4">
                <h1 class="dash-text"> Custom Time</h1>
                <section class="p-2">
                    <p>{input_time_displ}</p>
                </section>
            </div>
        </section>
        <section class="p-2 grid grid-cols-3">
                <div class="">
                    <h1 class="dash-text"> Hour Gauge</h1>
                    <div id="hour"></div>
                </div>
                <div class="">
                    <h1 class="dash-text"> Minute Gauge</h1>
                    <div id="minute"></div>
                </div>
                <div class="">
                    <h1 class="dash-text"> Second Gauge</h1>
                    <div id="second"></div>
                </div>
        </section>    
        <section class="p-4 grid grid-cols-1">
            <div class="p-2">
                <h1 class="dash-text"> Search Timezones</h1>
                <section class="grid grid-cols-2 space-x-4 justify-center items-center">
                    <section class="p-2">
                        <input type="search" class="p-4 bg-accent-1 text-content-1 shadow-content-2/80 rounded-lg"/>
                    </section>
                    <section class="p-2">
                        <p>{input_location} example/example/example</p>
                    </section>
                </section>
            </div>
        </section>
    }
}
