//! Traditional Ceramics are Fired at low temperatures, porous, and often used for pottery and decorative items.
//!
//! These are the icons abstracted
//!

use leptos::*;
use leptos_icons::*;

#[component]
pub fn DashboardRightIcon(class: &'static str) -> impl IntoView {
    view! {<Icon icon=Icon::from(BiIcon::BiRightArrowRegular) class=class />}
}

#[component]
pub fn DashboardLeftIcon(class: &'static str) -> impl IntoView {
    view! {<Icon icon=Icon::from(BiIcon::BiLeftArrowRegular) class=class />}
}

#[component]
pub fn MenuGridIcon(class: &'static str) -> impl IntoView {
    view! {<Icon icon=Icon::from(CgIcon::CgMenuGridR) class=class />}
}
