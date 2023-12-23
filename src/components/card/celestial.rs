use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;

#[component]
pub fn ChosenCelestials() -> impl IntoView {
    view! {
        <div class="py-2 font-light">
            <label> Celestial Bodies Chosen </label>
            <div id="body-chosen" class="flex flex-col p-2 result-bgk rounded-lg">
                // <p> Asteroid: Titan </p>
                // <p> Inner Planet: Mars </p>
                // <p> Outer Planet: Neptune </p>
            </div>
        </div>
    }
}
