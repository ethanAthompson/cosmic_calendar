use leptos::{leptos_dom::logging::console_log, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

use crate::wrappers::web::{media, storage, update_dom_el};

/// The lightswitch component
pub mod switch;

/// Error handlers for the theme
pub enum ZoneError {
    Light,
    Dark,
    System,
    None,
}

/// Enum for entire Theme
pub enum ZoneTheme {
    Light,
    Dark,
    System,
}

/// Error specification
impl fmt::Display for ZoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZoneError::Light => write!(f, "Light theme has a bug"),
            ZoneError::Dark => write!(f, "Dark theme has a bug"),
            ZoneError::System => write!(f, "System theme has a bug"),
            ZoneError::None => write!(f, "There is no bug with the App Theme"),
        }
    }
}

/// Methods for theme
impl ZoneTheme {
    pub fn set_theme(&self) {
        match self {
            ZoneTheme::Light => {
                storage().set_item("theme", "light").unwrap();
                update_dom_el("html-theme", "light");

                if storage().get_item("light").is_ok() {
                    console_log(format!("{}", ZoneError::None).as_str());
                } else {
                    console_log(format!("{}", ZoneError::Light).as_str());
                }
            }
            ZoneTheme::Dark => {
                storage().set_item("theme", "dark").unwrap();
                update_dom_el("html-theme", "dark");
            }
            ZoneTheme::System => {
                storage().remove_item("theme").unwrap();
                update_dom_el("html-theme", "system");

                // Text does need to know if the theme is dark or light
                if media().matches() {
                    storage().set_item("theme", "dark").unwrap();
                    update_dom_el("html-theme", "dark");
                } else {
                    storage().set_item("theme", "light").unwrap();
                    update_dom_el("html-theme", "light");
                }
            }
        }
    }

    /// Executes the closure then sets theme
    pub fn set_theme_opt<F>(&self, f: F, mode: u8)
    where
        F: FnOnce(),
    {
        f();

        match mode {
            0 => ZoneTheme::set_theme(&ZoneTheme::Light),
            1 => ZoneTheme::set_theme(&ZoneTheme::Dark),
            2 => ZoneTheme::set_theme(&ZoneTheme::System),
            _ => {}
        }
    }
}
