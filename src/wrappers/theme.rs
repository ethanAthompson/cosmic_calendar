use super::web::storage_theme;
use leptos::SignalSet;
use leptos_icons::{Icon, *};
use leptos_use::use_favicon;

/// Returns a class depending on the theme in localstorage
///
pub fn theme_texts() -> &'static str {
    match storage_theme().as_str() {
        "light" => "text-amber-400 hover:text-amber-500 theme-size",
        "dark" => "text-violet-400 hover:text-violet-800 theme-size",
        _ => "text-zinc-400",
    }
}

/// Returns a specific icon relative to localstorage theme
///
pub fn theme_icons() -> Icon {
    match storage_theme().as_str() {
        "light" => Icon::from(BsIcon::BsSun),
        "dark" => Icon::from(BsIcon::BsMoonStars),
        _ => Icon::from(FiIcon::FiMonitor),
    }
}

/// Replaces the favicon on theme changes
/// 
pub fn theme_favicon() {
    let (icon, set_icon) = use_favicon();

    match storage_theme().as_str() {
        "light" => {
            set_icon.set(Some("light.png".to_string()));
        }
        "dark" => {
            set_icon.set(Some("dark.png".to_string()));
        }
        _ => {
            set_icon.set(Some("sys.png".to_string()));
        }
    }
}

