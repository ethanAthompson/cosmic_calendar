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
            <A href="" id="zone-highlight" class="
                -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 
                ease-in-out duration-300 p-2 glitch desktop:text-6xl laptop:text-4xl tablet:text-2xl text-xl
        "> Zone </A>
        </nav>
        <nav class="justify-end hidden desktop:flex laptop:flex tablet:flex">
            <SuperBigLinks with_switch/>
        </nav>
    }
}
