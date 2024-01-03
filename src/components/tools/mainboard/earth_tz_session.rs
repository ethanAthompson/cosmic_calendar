use leptos::{
    html::{Input, Select},
    leptos_dom::logging::console_log,
    *,
};
use leptos_icons::*;
use leptos_router::create_query_signal;
use strum::IntoEnumIterator;
use strum::*;
use wasm_bindgen::JsCast;
use web_sys::{
    DragEvent, HtmlElement, HtmlHeadingElement, HtmlInputElement, InputEvent, KeyboardEvent,
    MouseEvent, Node,
};

#[component]
pub fn Card() -> impl IntoView {
    view! {
        <div id="earth-sp-s1" class="py-2">
            <section>
                <p>"Earth Timezone -> Space Time"</p>
            </section>
            <div>
                <section class="dark:bg-slate-900 bg-slate-200">
                    <div class="">Drag a timezone here "(:"</div>
                </section>

                <section class="dark:bg-slate-900 bg-slate-200">
                    <div class="">Output here "(:"</div>
                </section>
            </div>
        </div>
    }
}
