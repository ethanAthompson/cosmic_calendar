use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::A;
use leptos_use::use_element_hover;
use chrono::prelude::*;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::{OffsetComponents, OffsetName, Tz, TZ_VARIANTS, *};


#[component]
pub fn Page() -> impl IntoView {
    view! {
        <main class="w-full h-full">
            <section class="py-2 bg-accent-1/40 inline-block">
                <article class="bg-accent-1/50 p-4">
                    <h1> Welcome to the Cosmic Calendar Dashboard </h1>
                    <h2> Click the buttons on the left to use the tools </h2>
                </article>
            </section>
        </main>
    }
}
