use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;

#[component]
pub fn ChosenTimeZones() -> impl IntoView {
    view! {
        <div class="py-2 font-light">
            <label> Earth Timezones </label>
            <div id="earth-zones" class="flex flex-col p-2 result-bgk rounded-lg">
            </div>
        </div>
    }
}
