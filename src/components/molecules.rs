//! Molecules a group of atoms bonded together, 
//! representing the smallest fundamental unit of a chemical compound 
//! that can take part in a chemical reaction. 
//!
//! These are the combination of atoms together within the atomic hierarchy
//!
//! HydrogenHome => the H2O home page
//! CarbonDashboard => the CO2 dashboard
//!


/// The molecular page you first start on
pub mod home;

/// The molecular page you manage charts, themes, and settings: allow user to save specific data, shows timezones and space stuff from tool sessions
pub mod dashboard;

/// The molecular page where the 1st tool lives
/// ===
/// ```markdown
/// 1. Supports Earth Timezone -> Celestial Body Timezone
/// 2. Supports Celestial Body Timezone -> Earth Timezone
///
/// ```

pub mod timezone;

/// The molecular page where the 2nd tool lives
/// ===
/// ```markdown
/// 1. Supports Earth Date -> Celestial Body Date
/// 2. Supports Celestial Body Date -> Earth Date
/// 3. Supported Calendars/
///    - [x] Japanese/
///        - [x] Extended
///    - [x] Chinese
///    - [x] Republic of China
///    - [x] Hebrew
///    - [x] Islamic/
///        - [x] Civil
///        - [x] Observational
///        - [x] Tabular
///        - [x] Umm Al Qura
///    - [x] Buddhist
///    - [x] Persian
///    - [x] Iso
///    - [x] Gregorian
///    - [x] Coptic
///    - [x] Julian
///
/// ```
pub mod date;

/// The molecular page that describes the purpose of this website
pub mod about;

/// The molecular page that allows you to download different targets
pub mod download;

/// The error molecularity page occurs 
/// ===
/// ```markdown
/// 1. on wrongpage, 
/// 2. broken setting
/// 3. incorrect localstorage setting
/// ```
pub mod notfound;
