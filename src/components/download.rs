pub mod target;

use crate::components::download::target::Card as Target;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

#[component]
pub fn Card() -> impl IntoView {
    let media = create_rw_signal("the installer or pkg".to_string());
    let image = create_rw_signal("Debian logo".to_string());
    let name = create_rw_signal("Debian Lts 02".to_string());

    view! {
        <div class="w-full p-4 ">
            <div class="grid grid-cols-4 grid-rows-4">
                // <Target name=name image=image media=media />
                // <Target name=name image=image media=media />
                // <Target name=name image=image media=media />
                // <Target name=name image=image media=media />
                // <Target name=name image=image media=media />
                // <Target name=name image=image media=media />
                // <Target name=name image=image media=media />
                // <Target name=name image=image media=media />
            </div>
        </div>
    }
}
