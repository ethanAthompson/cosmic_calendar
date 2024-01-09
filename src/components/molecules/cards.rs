//! Orgnaic Dashboard pages
//!

/// The molecular home page of the organic dashboard
pub mod home;

/// The molecular settings page of the organic dashboard
pub mod settings;

/// The molecular info page of the organic dashboard
pub mod info;

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
pub mod timezone;

/// The molecular page where the 1st tool lives
/// ===
/// ```markdown
/// 1. Supports Earth Timezone -> Celestial Body Timezone
/// 2. Supports Celestial Body Timezone -> Earth Timezone
///
/// ```
pub mod date;

/// The molecular page where the calculations are shown
pub mod data;
