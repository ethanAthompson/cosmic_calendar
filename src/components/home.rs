use leptos::{html::Canvas, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::{f64::consts::PI, fmt};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MediaQueryList, MouseEvent, Storage};


#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="w-full p-4 items-center flex flex-col justify-center">
            <Promotion/>
        </div>
    }
}

#[component]
pub fn Promotion() -> impl IntoView {
    view! {
        <div>
            <p class="regular-text p-2"> Welcome to the Zone! </p>
        </div>
    }
}


