use crate::wrappers::ron_stuff::RonData;
use leptos::{WriteSignal, Signal, RwSignal, SignalGetUntracked};
use leptos::{document, leptos_dom::logging::console_log};
use leptos_use::storage::use_local_storage;
use leptos_use::storage::JsonCodec;
use ron::{
    de::{self, from_reader},
    from_str,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeZone {
    pub name: String,
    pub offset: i32,
    pub dst: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeZoneFormat {
    pub title: String,
    pub map: HashMap<String, TimeZone>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EarthTimeZone {
    pub abbr: String,
    pub offset: i32,
    pub fullname: String
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SavedData {
    // name, offset, fullname
    // pub earth: HashMap<String, EarthTimeZone>,
    pub earth: HashMap<String, EarthTimeZone>,
    // celestial needs earth so it can do proper calculation on save point
    // pub celestial: HashMap<String>,
    // pub calendar: HashMap<String>,
}

impl Default for SavedData {
    fn default() -> Self {
        Self {
            earth: HashMap::new(),
            // celestial: Vec::new(),
            // calendar: Vec::new(),
        }
    }
}

/// Returns the earth time .ron file data via serde + ron help
pub async fn earth_time() -> TimeZoneFormat {
    let format: TimeZoneFormat = RonData::new("/data/earth_timezones.ron").await;
    return format;
}

