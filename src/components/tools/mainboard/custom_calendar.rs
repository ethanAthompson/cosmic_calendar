use icu_calendar::{AsCalendar, indian::Indian, islamic::IslamicCivil, Iso, chinese::Chinese, Gregorian, Date};
use leptos::{NodeRef, html::{Input, Select}, RwSignal, leptos_dom::logging::console_log, *};

use leptos_router::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::wrappers::date::date_symphony_v2;

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
