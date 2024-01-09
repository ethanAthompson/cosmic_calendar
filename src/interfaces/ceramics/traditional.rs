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

#[component]
pub fn CalendarIcon(class: &'static str) -> impl IntoView {
    view! {<Icon icon=Icon::from(CgIcon::CgCalendarDates) class=class />}
}

#[component]
pub fn TimeFiveIcon(class: &'static str) -> impl IntoView {
    view! {<Icon icon=Icon::from(BiIcon::BiTimeFiveRegular) class=class />}
}

#[component]
pub fn SettingIcon(class: &'static str) -> impl IntoView {
    view! {<Icon icon=Icon::from(HiIcon::HiCog8ToothOutlineLg) class=class />}
}

#[component]
pub fn InfoIcon(class: &'static str) -> impl IntoView {
    view! {<Icon icon=Icon::from(AiIcon::AiInfoCircleOutlined) class=class />}
}

#[component]
pub fn ImStackIcon(class: &'static str) -> impl IntoView {
    view! {<Icon icon=Icon::from(ImIcon::ImStack) class=class />}
}

#[component]
pub fn HomeIcon(class: &'static str) -> impl IntoView {
    view! {<Icon icon=Icon::from(AiIcon::AiHomeTwotone) class=class />}
}

#[component]
pub fn FullScreenIcon(class: &'static str) -> impl IntoView {
    view! {<Icon icon=Icon::from(AiIcon::AiFullscreenOutlined) class=class />}
}

#[component]
pub fn MinimizeIcon(class: &'static str) -> impl IntoView {
    view! {<Icon icon=Icon::from(FiIcon::FiMinimize) class=class />}
}

#[component]
pub fn FaviconIcon(class: &'static str) -> impl IntoView {
    view! {
        <img src="/public/icons/dark.png" class=class/>
    }
}
