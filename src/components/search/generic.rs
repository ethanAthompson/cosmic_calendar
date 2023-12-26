use crate::{
    components::{card::earth::RonEarth, tools::innerplanets::earth::earth_time},
    wrappers::{
        strings::get_initials,
        web::{all_items, update_dom_el},
    },
};
use chrono::prelude::*;
use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlHeadingElement, HtmlInputElement, KeyboardEvent, MouseEvent, Node};


// INFO! The classes are different and the async data fetched too. celestial_time(), earth_time(), ...
// INFO! the calender will also be in a ron file, so its easier to add more calendars as time goes on.
//
#[component]
pub fn GenericSearchBar() -> impl IntoView {

    view!{
        
    }
}


#[component]
pub fn GenericSearchItems(
    // represents the list of timezones you can search
    #[prop(into)] vector: RwSignal<Vec<String>>,
    // represents the id of the timezones which is dynamically used
    #[prop(into)] input: RwSignal<String>,
) -> impl IntoView {

    view! {
        
    }
}
