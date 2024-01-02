pub mod redirect;

use crate::components::notfound::redirect::Card as RedirectCard;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

#[component]
pub fn Card() -> impl IntoView {
    view! {
        <div class="flex justify-center place-content-center ">
            <RedirectCard/>
        </div>
    }
}
