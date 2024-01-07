use leptos::{html::Canvas, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::{f64::consts::PI, fmt};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MediaQueryList, MouseEvent, Storage};

#[component]
pub fn Card() -> impl IntoView {
    view! {
        <div>
            <p class="desktop:text-6xl laptop:text-4xl tablet:text-2xl text-xl p-2"> Welcome to the Zone! </p>
        </div>
    }
}
