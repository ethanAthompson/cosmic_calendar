pub mod promotion;

use leptos::{html::Canvas, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::{f64::consts::PI, fmt};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MediaQueryList, MouseEvent, Storage};
use crate::components::home::promotion::Card as PromotionCard;
use crate::components::charts::longitude_gauge::Guage;

#[component]
pub fn Card() -> impl IntoView {
    view! {
        <div class="w-full p-4 items-center flex flex-col justify-center">
            <PromotionCard/>
            <Guage/>
        </div>
    }
}

