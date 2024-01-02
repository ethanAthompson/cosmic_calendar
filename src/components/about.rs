pub mod description;

use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use crate::components::about::description::Card as DescriptionCard;


#[component]
pub fn Card() -> impl IntoView {
    view! {
        <div class="grid cols-2 items-center justify-center p-4 leading-4">
            <div class="flex items-center justify-center p-4">
                    Zone is a tool made with leptos
            </div>
            <div class="grid-cols-1 flex items-center justify-start p-4 ">
                <DescriptionCard/>
            </div>
        </div>
    }
}
