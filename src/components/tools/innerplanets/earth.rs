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

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SavedData {
    // name, offset, fullname
    pub earth: Vec<(String, i32, String)>,
    // celestial needs earth so it can do proper calculation on save point
    pub celestial: Vec<String>,
    pub calendar: Vec<String>,
}

impl Default for SavedData {
    fn default() -> Self {
        Self {
            earth: Vec::new(),
            celestial: Vec::new(),
            calendar: Vec::new(),
            // earth: vec![("Eastern Standard Time".to_owned(), -5), ("Omsk Time".to_owned(), 6)],
            // celestial: vec!["MARS".to_owned(), "TITAN".to_owned()],
            // calendar: vec!["Gregorian".to_owned(), "Solar".to_owned()],
        }
    }
}

/// Returns the earth time .ron file data via serde + ron help
pub async fn earth_time() -> TimeZoneFormat {
    let format: TimeZoneFormat = RonData::new("/data/earth_timezones.ron").await;
    return format;
}

