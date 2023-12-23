use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;

#[component]
pub fn ChosenCalendars() -> impl IntoView {
    view! {
        <div class="py-2 font-light">
            <label> Calendar Chosen </label>
            <div id="calendar-chosen" class="flex flex-col p-2 result-bgk roudned-lg">
                // <p> Gregorian </p>
            </div>
        </div>
    }
}
