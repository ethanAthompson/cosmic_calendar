use leptos::*;
use leptos_router::A;
use web_sys::MouseEvent;

use crate::components::theme::smallscreen::Switch as SwitchSm;
use crate::components::theme::largescreen::Switch as SwitchLg;

#[component]
pub fn SuperBigLinks(#[prop(optional)] with_switch: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="text-black dark:text-white flex-nowrap flex items-center justify-end text-xl desktop:text-2xl laptop:text-xl">
            <span class="-skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 ease-in-out duration-300 p-2 glitch">
                <A href="tool" class="py-2"> "Tool" </A>
            </span>
            <span class="-skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 ease-in-out duration-300 p-2 glitch">
                <A href="about" class="py-2 "> "About" </A>
            </span>
            <span class="-skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 ease-in-out duration-300 p-2 glitch">
                <A href="download" class="py-2"> "Download" </A>
            </span>
            <Show
                when=move || with_switch.get()
                fallback=move || view!{<span></span>}
            >
                <span class=" -skew-y-3 hover:-skew-y-6 scale-50 hover:scale-75 focus:-skew-y-6 focus:scale-75 ease-in-out duration-300 px-2">
                    <SwitchLg/>
                </span>
            </Show>
        </div>
    }
}
