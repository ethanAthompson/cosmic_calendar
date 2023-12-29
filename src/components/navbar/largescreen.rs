use crate::components::links::largescreen::SuperBigLinks;
use crate::components::links::smallscreen::SuperSmallLinks;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::{A, *};
use web_sys::*;

#[component]
pub fn LargeScreenLinks(#[prop(optional)] id: &'static str) -> impl IntoView {
    let with_switch = create_rw_signal(true);

    view! {
        <nav class="flex justify-start items-center py-2">
            <A href="" id="zone-highlight" class="unique-wrap regular-text"> Zone </A>
        </nav>
        <nav class="justify-end hidden desktop:flex laptop:flex tablet:flex">
            <SuperBigLinks with_switch/>
        </nav>
    }
}
