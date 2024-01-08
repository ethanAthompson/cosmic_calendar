use crate::interfaces::ceramics::traditional::{
    CalendarIcon, DashboardLeftIcon, DashboardRightIcon, ImStackIcon, InfoIcon, MenuGridIcon,
    SettingIcon, TimeFiveIcon,
};
use charming::{
    datatype::{CompositeValue, DataPoint, DataPointItem},
    element::{Color, ItemStyle},
    *,
};
use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::A;
use leptos_use::use_element_hover;

#[component]
pub fn Component(is_fullscreen: RwSignal<bool>) -> impl IntoView {
    // context?
    view! {
        <Show when=move || is_fullscreen.get() == false fallback=move || view!{<nav></nav>}>
            <header class="p-4 bg-accent-2">
                <nav class="screen-nav justify-between items-center space-x-4">
                    <section class="p-2">
                        <A href="/" class="nav-item"> Zone </A>
                    </section>
                    <div class="px-2 flex flex-row justify-between space-x-2">
                        <section class="p-2">
                            <A href="/about" class="nav-item"> About </A>
                        </section>
                        <section class="p-2">
                            <A href="/download" class="nav-item"> Download </A>
                        </section>
                    </div>
                </nav>
            </header>
        </Show>
    }
}
