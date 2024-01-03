use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SavedData {
    pub earth: HashMap<String, chrono_tz::Tz>,
    // celestial timezones a somewhat comprehensive list of them
    // pub celestial: HashMap<String, rust_solar::Ctz>,
    pub celestial: HashMap<String, String>,
}

impl Default for SavedData {
    fn default() -> Self {
        Self {
            earth: HashMap::new(),
            celestial: HashMap::new(),
        }
    }
}
