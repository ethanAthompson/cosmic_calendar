use crate::components::links::smallscreen::SuperSmallLinks;
use crate::components::links::largescreen::SuperBigLinks;
use leptos::*;
use leptos_router::{A, *};
use leptos_icons::Icon;
use web_sys::*;

#[component]
pub fn SmallScreenLinks<F>(id: &'static str, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static + Copy,
{
    view! {
        <div id=id>
            <nav class="desktop:hidden laptop:hidden tablet:hidden shadow-lg">
                <SuperSmallLinks on_click=on_click />
            </nav>
        </div>
    }
}

#[component]
pub fn SmallScreenMenu<F>(#[prop(optional)] id: &'static str, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static + Copy,
{
    let menu = Icon::from(leptos_icons::AiIcon::AiMenuOutlined);

    // if menu open show open else show close

    view! {
        // Shows on small screens
        <nav class="flex justify-end px-2 desktop:hidden laptop:hidden tablet:hidden">
            <button class="" on:click=on_click>
                <Icon icon=menu class="cursor-pointer w-7 h-7  default-item-select" />
            </button>
        </nav>
    }
}

