use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::A;
use leptos_use::use_element_hover;
use chrono::prelude::*;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::{OffsetComponents, OffsetName, Tz, TZ_VARIANTS, *};


#[component]
pub fn Page() -> impl IntoView {
    view! {
        <span> soon </span>
    }
}
