pub mod smallscreen;
pub mod largescreen;

use leptos::*;
use crate::components::navbar::largescreen::LargeScreenLinks;
use crate::components::navbar::smallscreen::SmallScreenLinks;
use crate::components::navbar::smallscreen::SmallScreenMenu;
use crate::wrappers::web::update_dom_el;

#[component]
pub fn Navbar() -> impl IntoView {
    let menu_state = create_rw_signal(false);

    let toggle_menu = move |_| {
        menu_state.update(|flag: &mut bool| {
            // boolean toggle
            *flag = !*flag;

            // Toggles the items when clicked
            if *flag {
                update_dom_el("mobile-links", "hidden");
            } else {
                update_dom_el("mobile-links", "");
            }
        })
    };

    view! {
        <header class="bg-inherit shadow-lg py-0 grid grid-cols-2 w-full">
            <LargeScreenLinks />
            <SmallScreenMenu on_click=toggle_menu />
        </header>
        <SmallScreenLinks id="mobile-links" on_click=toggle_menu />
    }
}

