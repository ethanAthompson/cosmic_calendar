use leptos::*;
use leptos_router::A;
use web_sys::MouseEvent;

use crate::components::theme::switch::{MobileThemeSwitch, ThemeSwitch};

/// A component that inherits an onclick, cleaner
#[component]
pub fn SmallLinks<F>(
    // Takes in a mouse event closure that lives long and can be distributed
    on_click: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static + Copy,
{
    view! {
     <div class="text-black dark:text-white flex flex-col items-center justify-center desktop:text-4xl laptop:text-2xl text-xl">
        <A on:click=on_click.clone() href="tool" class="py-4 rounded-md default-item-select "> "Tool" </A>
        <A on:click=on_click.clone() href="about" class="py-4 rounded-md default-item-select "> "About" </A>
        <A on:click=on_click.clone() href="download" class="py-4 rounded-md default-item-select"> "Download" </A>
        <MobileThemeSwitch on_click=on_click.clone() />
    </div>
    }
}

#[component]
pub fn BigLinks() -> impl IntoView {
    view! {
        <div class="text-black dark:text-white flex-nowrap flex space-x-12 items-center justify-end text-xl desktop:text-2xl laptop:text-xl text-sm">
            <A href="tool" class="py-4 rounded-md default-item-select"> "Tool" </A>
            <A href="about" class="py-4 rounded-md default-item-select "> "About" </A>
            <A href="download" class="py-4 rounded-md default-item-select"> "Download" </A>
            <p class="py-4 rounded-md text-2xl default-item-select"> <ThemeSwitch/></p>
        </div>
    }
}

#[component]
pub fn SuperBigLinks() -> impl IntoView {
    view! {
        // <div class="text-black dark:text-white flex-nowrap flex space-x-12 items-center justify-end text-xl desktop:text-2xl laptop:text-xl text-sm">
        <div class="text-black dark:text-white flex-nowrap flex space-x-12 items-center justify-end text-xl desktop:text-2xl laptop:text-xl text-sm">
            <span class="unique-wrap">
                <A href="tool" class="py-4 rounded-md default-item-select"> "Tool" </A>
            </span>
            <span class="unique-wrap">
                <A href="about" class="py-4 rounded-md default-item-select "> "About" </A>
            </span>
            <span class="unique-wrap">
                <A href="download" class="py-4 rounded-md default-item-select"> "Download" </A>
            </span>
            <span class="unique-theme w-24 h-24">
                <p class="py-4 rounded-md text-2xl default-item-select"> <ThemeSwitch/></p>
            </span>
        </div>
    }
}

/// A component that inherits an onclick, cleaner
#[component]
pub fn SuperSmallLinks<F>(
    // Takes in a mouse event closure that lives long and can be distributed
    on_click: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static + Copy,
{
    view! {
     <div class="text-black dark:text-white flex flex-col items-center justify-center desktop:text-4xl laptop:text-2xl text-xl">
        <span class="sm-unique-wrap">
            <A on:click=on_click.clone() href="tool" class="py-4 rounded-md default-item-select "> "Tool" </A>
        </span>
        <span class="sm-unique-wrap">
            <A on:click=on_click.clone() href="about" class="py-4 rounded-md default-item-select "> "About" </A>
        </span>
        <span class="sm-unique-wrap">
            <A on:click=on_click.clone() href="download" class="py-4 rounded-md default-item-select"> "Download" </A>
        </span>
        <span class="sm-unique-wrap">
            <MobileThemeSwitch on_click=on_click.clone() />
        </span>
    </div>
    }
}
