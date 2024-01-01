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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EarthTimeZone {
    pub abbr: String,
    pub offset: i32,
    pub fullname: String
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SavedData {
    pub earth: HashMap<String, chrono_tz::Tz>,
    // celestial timezones a somewhat comprehensive list of them
    // pub celestial: HashMap<String, rust_solar::Ctz>,
    pub celestial: HashMap<String, chrono_tz::Tz>,
}

impl Default for SavedData {
    fn default() -> Self {
        Self {
            earth: HashMap::new(),
            celestial: HashMap::new(),
        }
    }
}


