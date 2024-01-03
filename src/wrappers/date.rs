use icu_calendar::{
    chinese::Chinese, indian::Indian, islamic::IslamicCivil, AsCalendar, Date, Gregorian, Iso,
};
use leptos::{
    html::{Input, Select},
    leptos_dom::logging::console_log,
    NodeRef, RwSignal,SignalSet,
};
use tracing::Instrument;

use crate::{
    components::card::earth::EarthDisplay,
    wrappers::{
        strings::{filtered_vec, get_initials, matching_left},
        web::{all_items, save_data, update_dom_el},
    },
};
use chrono::prelude::*;
use leptos_icons::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlHeadingElement, HtmlInputElement, KeyboardEvent, MouseEvent, Node};

pub struct ZoneCalendar;

/// This is just a basic representation
impl ZoneCalendar {
    pub fn set<T>(yymmdd: [NodeRef<Input>; 3], sig: RwSignal<String>, core: T)
    where
        T: AsCalendar,
    {
        // Converts input to NaiveDate which allows formating
        let year = yymmdd[0]
            .get()
            .unwrap()
            .value()
            .parse::<i32>()
            .unwrap_or_default();
        let month = yymmdd[1]
            .get()
            .unwrap()
            .value()
            .parse::<u8>()
            .unwrap_or_default();
        let day = yymmdd[2]
            .get()
            .unwrap()
            .value()
            .parse::<u8>()
            .unwrap_or_default();

        // WARNING! Must be real date: Gregorian is default
        let icu = Date::try_new_gregorian_date(year, month, day).expect("A Date to Exist");
        // let icu = Date::try_new_gregorian_date(year, month, day).expect("A Date to Exist");

        // sets the appropiate calendar from core
        let calendar = icu.to_calendar(core);

        // *value = format!(
        let result = format!(
            "{:?}, {:?} {:?}, {:?} {:?}",
            calendar.day_of_month().0,
            calendar.month().code.0.to_string().as_str(),
            calendar.month().ordinal,
            calendar.year().number,
            calendar.year().cyclic.expect("A Cyclic to be present"),
        );

        console_log("HI");
        sig.set(result);
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
