use leptos_icons::{Icon, *};

use super::web::storage_theme;

pub fn theme_texts() -> &'static str {
    match storage_theme().as_str() {
        "light" => "text-amber-400 hover:text-amber-500 theme-size",
        "dark" => "text-violet-400 hover:text-violet-800 theme-size",
        _ => "text-zinc-400",
    }
}

pub fn theme_icons() -> Icon {
    let sun: Icon = Icon::from(BsIcon::BsSun);
    let moon: Icon = Icon::from(BsIcon::BsMoonStars);
    let sys: Icon = Icon::from(FiIcon::FiMonitor);

    match storage_theme().as_str() {
        "light" => sun,
        "dark" => moon,
        _ => sys,
    }
}
