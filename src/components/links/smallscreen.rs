use leptos::*;
use leptos_router::A;
use web_sys::MouseEvent;

use crate::components::theme::smallscreen::Switch as SwitchSm;
use crate::components::theme::largescreen::Switch as SwitchLg;

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
     <div class="dark:bg-slate-900 bg-slate-200 mix-blend-darken dark:mix-blend-lighten py-0 grid grid-cols-2 text-black dark:text-white items-center justify-center desktop:text-4xl laptop:text-2xl text-xl">
        <nav class="order-1 flex flex-col">
            <span class="dark:hover:text-white hover:text-black -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 ease-in-out duration-300 glitch">
                <A on:click=on_click.clone() href="tool" class="p-1 rounded-md "> "Tool" </A>
            </span>
            <span class="dark:hover:text-white hover:text-black -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 ease-in-out duration-300 glitch">
                <A on:click=on_click.clone() href="about" class="p-1 rounded-md "> "About" </A>
            </span>
            <span class="dark:hover:text-white hover:text-black -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 ease-in-out duration-300 glitch">
                <A on:click=on_click.clone() href="download" class="p-1 rounded-md"> "Download" </A>
            </span>
        </nav>

        <nav class="order-2">
            <span class="">
                <SwitchSm on_click=on_click.clone() />
            </span>
        </nav>
     </div>
    }
}
