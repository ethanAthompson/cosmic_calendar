use leptos::leptos_dom::logging::console_log;
use ron::{
    de::{self, from_reader},
    from_str,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

use crate::wrappers::ron_stuff::RonData;

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

/// Returns the earth time .ron file data via serde + ron help
pub async fn earth_time() -> TimeZoneFormat {
    let format: TimeZoneFormat = RonData::new("/data/earth_timezones.ron").await;

    return format;
}

