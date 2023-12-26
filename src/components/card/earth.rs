use chrono::prelude::*;
use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;

use crate::components::tools::innerplanets::earth::{earth_time, TimeZone};

#[component]
pub fn ChosenTimeZones() -> impl IntoView {
    view! {
        <div class="py-2 font-light">
            <label> Earth Timezones </label>
            <section class="max-h-32 result-bgk flex flex-col rounded-lg p-2">
                <div id="earth-zones" class="overflow-y-auto h-min flex flex-col font-bold ps-4">
                </div>
            </section>
        </div>
    }
}

#[component]
pub fn RonEarth(name: String) -> impl IntoView {
    // BUG! PLEASE KEEP THIS FIRST
    let tmp_name = create_rw_signal(name.clone());
    let zone_offset = create_rw_signal(0);

    // INFO! I pass down the timezone so the async data renders properly
    let customtime = move |offset: i32| {
        zone_offset.set(offset);
        
        let hour = 3600;
        let fixed_offset = FixedOffset::east_opt(zone_offset.get() * hour).unwrap();

        return Utc::now()
            .with_timezone(&fixed_offset)
            .format("%d/%m/%Y %r")
            .to_string();
    };

    let time = create_rw_signal(customtime(zone_offset.get()));

    set_interval(
        move || {
            time.update(|value| {
                *value = customtime(zone_offset.get());
            })
        },
        std::time::Duration::from_secs(1),
    );

    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future=move || get_zones(tmp_name)
            // the data is bound to whatever variable name you provide
            let:data
        >
            // you receive the data by reference and can use it in your view here
            <p> {data.clone().0} </p>
            <p> {customtime(data.clone().1)} </p>

        </Await>
    }
}

pub async fn get_zones(tmp_name: RwSignal<String>) -> (String, i32, u8) {
    match earth_time()
        .await
        .map
        .get(tmp_name.get().clone().to_lowercase().as_str())
        .unwrap()
    {
        TimeZone { name, offset, dst } => {
            let debug = format!("{:?}, {:?}, {:?}", name, offset, dst);
            console_log(debug.as_str());

            // zone_name.set(name.to_string());
            // zone_offset.set(*offset);
            // zone_dst.set(*dst);

            return (name.to_string(), *offset, *dst);
        }
    }
}
