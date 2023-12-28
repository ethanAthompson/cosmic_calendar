/// Asteroids needed
pub mod asteroids;

/// Inner Planets needed
pub mod innerplanets;

/// Moons needed
pub mod moons;

/// Outer Planets needed
pub mod outerplanets;

use crate::components::card::calendar::ChosenCalendars;
use crate::components::card::celestial::ChosenCelestials;
use crate::components::card::earth::ChosenTimeZones;
use crate::components::search::calendar::CheckBox as CalendarCheckBox;
use crate::components::search::calendar::SearchBar as CalendarSearchBar;
use crate::components::search::celestial::SearchBar as CelestialSearchBar;
use crate::components::search::earth::SearchBar as EarthTimeZoneSearchBar;

use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;

#[component]
pub fn Tools() -> impl IntoView {
    view! {
        <main class="grid grid-cols-2">

            // Where the astronaut searches for their stuff
            <section class="p-4 border border-slate-800 ">
                <form>
                    <EarthTimeZoneSearchBar/>
                    // <CelestialSearchBar/>
                    // <CalendarSearchBar/>
                    // <CalendarCheckBox/>
                </form>
            </section>

            // Where the astronaut can see the live dates synced and stuff.
            // INFO! each of the Chosen has its own tmp_name so its easier
            <section class="p-4 border border-slate-800 ">
                 <ChosenTimeZones/>       
                // <ChosenCelestials/>
                // <ChosenCalendars/>
            </section>
        </main>
    }
}
