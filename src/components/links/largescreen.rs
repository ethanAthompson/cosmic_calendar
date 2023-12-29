use leptos::*;
use leptos_router::A;
use web_sys::MouseEvent;

use crate::components::theme::smallscreen::Switch as SwitchSm;
use crate::components::theme::largescreen::Switch as SwitchLg;

#[component]
pub fn SuperBigLinks(#[prop(optional)] with_switch: RwSignal<bool>) -> impl IntoView {
    view! {
        // <div class="text-black dark:text-white flex-nowrap flex space-x-12 items-center justify-end text-xl desktop:text-2xl laptop:text-xl text-sm">
        <div class="text-black dark:text-white flex-nowrap flex items-center justify-end text-xl desktop:text-2xl laptop:text-xl text-sm">
            <span class="unique-wrap">
                <A href="tool" class="py-2"> "Tool" </A>
            </span>
            <span class="unique-wrap">
                <A href="about" class="py-2 "> "About" </A>
            </span>
            <span class="unique-wrap">
                <A href="download" class="py-2"> "Download" </A>
            </span>
            <Show
                when=move || with_switch.get()
                fallback=move || view!{<span></span>}
            >
                <span class="unique-theme">
                    <SwitchLg/>
                </span>
            </Show>
        </div>
    }
}
