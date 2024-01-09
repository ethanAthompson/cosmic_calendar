// navbar links

use leptos::*;
use leptos_router::A;
use crate::interfaces::ceramics::traditional::{
    CalendarIcon, DashboardLeftIcon, DashboardRightIcon, FaviconIcon, ImStackIcon, InfoIcon,
    MenuGridIcon, SettingIcon, TimeFiveIcon,
};

/// Creates a vector of str given (LARGE SCREENS)
pub fn nav_links(links: Vec<&'static str>) -> View {
    links
        .into_iter()
        .map(|name| {
            view! {
                <p>
                    <A class="screen-1 text-center text-content-1 hover:text-content-2" href=name.to_string().to_lowercase()>{name}</A>
                </p>
            }
        })
        .collect_view()
}

/// Creates a vector of str given (MOBILE)
pub fn mobile_nav_links(links: Vec<&'static str>) -> View {
    links
        .into_iter()
        .map(|name| {
            view! {
                <p>
                    <A class="screen-1 text-center text-content-1 w-full hover:text-content-2" href=name.to_string().to_lowercase()>{name}</A>
                </p>
            }
        })
        .collect_view()
}

#[component]
pub fn Button() -> impl IntoView {
    let is_mobile_toggled = use_context::<RwSignal<bool>>().expect("Writer");

    let mobile_toggled = move |_| {
        is_mobile_toggled.update(|state| {
            *state = !*state;
        });    
    };
    
    view! {    
        <nav class="px-2 py-4 flex justify-end items-center">    
            <button class="" on:click=mobile_toggled>
                <MenuGridIcon class="w-10 h-10 text-content-1 hover:text-content-2" />
            </button>
        </nav> 
    }
}
