use crate::components::molecules::navbars::Component as Navbar;
use crate::components::molecules::sidebars::Component as DashboardSidebar;
use crate::components::atoms::charts::SimpleGauge;

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
pub fn Page() -> impl IntoView {
    view! {
        <section class="p-4 grid grid-cols-2">
            <label class="text-content-1">Timezone Page</label>
            <div class="p-2">
                <h1 class="font-rubik text-content-1"> Local Timezone </h1>
                <section class="p-2">
                    <p>EST</p>
                </section>
            </div>
            <div class="p-2">
                <h1 class="font-rubik text-content-1"> Local Date </h1>
                <section class="p-2">
                    <p>10/20/2 Saturr day ...</p>
                </section>
            </div>
        </section>
        <section class="p-2 grid grid-cols-3">
                <div class="">
                    <h1 class="font-rubik text-content-1"> Hour Gauge</h1>
                    <SimpleGauge id="16cha"/>
                </div>
                <div class="">
                    <h1 class="font-rubik text-content-1"> Minute Gauge</h1>
                    <SimpleGauge id="17cha"/>
                </div>
                <div class="">
                    <h1 class="font-rubik text-content-1"> Second Gauge</h1>
                    <SimpleGauge id="18cha"/>
                </div>
        </section>    
    }
}
