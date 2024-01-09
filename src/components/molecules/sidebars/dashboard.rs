// the dashboard sidebar
use crate::interfaces::ceramics::traditional::{
    CalendarIcon, DashboardLeftIcon, DashboardRightIcon, ImStackIcon, InfoIcon, MenuGridIcon,
    SettingIcon, TimeFiveIcon, HomeIcon,
};
use charming::{
    datatype::{CompositeValue, DataPoint, DataPointItem},
    element::{Color, ItemStyle},
    *,
};
use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::A;
use leptos_use::use_element_hover;
use web_sys::MouseEvent;

#[component]
pub fn Component<T, K>(
    is_sidebar: RwSignal<bool>,
    mini_sidebar: T,
    toggle_sidebar: K,
) -> impl IntoView
where
    T: Fn() -> HtmlElement<html::Div> + 'static,
    K: Fn(MouseEvent) + 'static + Copy,
{
    // context?
    view! {
        <Show when=move || is_sidebar.get() fallback=mini_sidebar>
            <div class="w-fit bg-accent-2/80 rounded-s-xl">
                <section class="grid grid-rows-auto justify-between space-y-4 p-0 w-full">
                    <article class="p-1">
                        <button class="dashboard-btn" on:click=toggle_sidebar>
                            <DashboardRightIcon class="w-10 h-10 dash-item"/>
                        </button>
                    </article>
                    <hr class="my-12 h-0.5 border-t-0 bg-accent-2/80"/>
                    <article class="p-2">
                        <A href="">
                            <button class="dashboard-btn">
                                <HomeIcon class="w-10 h-10"/>
                            </button>
                        </A>
                    </article>        
                    <article class="p-2">
                        <A href="date">
                            <button class="dashboard-btn">
                                <CalendarIcon class="w-10 h-10"/>
                            </button>
                        </A>
                    </article>
                    <article class="p-2">
                        <A href="timezone">
                            <button class="dashboard-btn">
                                <TimeFiveIcon class="w-10 h-10"/>
                            </button>
                        </A>
                    </article>
                    <article class="p-2">
                        <A href="info">
                            <button class="dashboard-btn">
                                <InfoIcon class="w-10 h-10"/>
                            </button>
                        </A>
                    </article>
                    <article class="p-2">
                        <A href="data">
                            <button class="dashboard-btn">
                                <ImStackIcon class="w-10 h-10"/>
                            </button>
                        </A>
                    </article>
                    <article class="p-2">
                        <A href="settings">
                            <button class="dashboard-btn">
                                <SettingIcon class="w-10 h-10"/>
                            </button>
                        </A>
                    </article>
                </section>
            </div>
        </Show>
    }
}
