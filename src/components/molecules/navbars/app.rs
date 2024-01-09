use crate::components::atoms::links::navbar::{nav_links, Button as MobileButton, mobile_nav_links};
use crate::interfaces::ceramics::traditional::{
    CalendarIcon, DashboardLeftIcon, DashboardRightIcon, FaviconIcon, ImStackIcon, InfoIcon,
    MenuGridIcon, SettingIcon, TimeFiveIcon,
};
use charming::{
    datatype::{CompositeValue, DataPoint, DataPointItem},
    element::{Color, ItemStyle},
    *,
};
use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::A;
use leptos_use::{use_element_hover, use_media_query};

#[component]
pub fn Component() -> impl IntoView {
    let is_fullscreen = use_context::<RwSignal<bool>>().expect("Writer");
    let is_mobile_toggled = use_context::<RwSignal<bool>>().expect("Writer");
    let is_mobile = use_media_query("(min-width: 480px)");
    let pages = use_context::<RwSignal<Vec<&str>>>().expect("Pages");

    view! {
        <div class="grid grid-cols-2 bg-accent-2 shadow-lg">
            <nav class="px-2 py-4 flex justify-start space-x-4 items-center">
                <FaviconIcon class="w-10 h-10 pointer-events-none" />
                <A href="/" class="screen-1 text-content-1 hover:text-content-2"> Zone </A>
            </nav>
            <Show when=move || is_mobile.get() == true fallback=move || view!{<MobileButton/>}>
                <nav class="px-2 py-4 flex justify-end space-x-12 items-center">
                    {nav_links(pages.get_untracked())}
                </nav>
            </Show>
        </div>
        <Show when=move || is_mobile_toggled.get() == true fallback=move || view!{<div></div>}>
            <div class="screen-gridnav grid-rows-1 bg-accent-2 shadow-lg items-center justify-center w-full">
                {mobile_nav_links(pages.get_untracked())}
            </div>
        </Show>
    }
}
