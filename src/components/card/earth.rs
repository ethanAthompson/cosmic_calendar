use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;

#[component]
pub fn ChosenTimeZones() -> impl IntoView {
    view! {
        <div class="py-2 font-light">
            <label> Earth Timezones </label>
            <section class="max-h-32 result-bgk flex flex-col rounded-lg p-2">
                <div id="earth-zones" class="overflow-y-auto h-min flex flex-col font-bold ps-4">
                </div>
            </section>
        </div>
    }
}
