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
    let offset = -5;
    let customtime = move || {
        let hour = 3600;
        let offset = FixedOffset::east_opt(offset * hour).unwrap();

        return Utc::now()
            .with_timezone(&offset)
            .format("%d/%m/%Y %r")
            .to_string();
    };

    let (time, set_time) = create_signal(customtime());
    let (tmp_name, set_tmp_name) = create_signal(name.clone());
    
    set_interval(
        move || {
            set_time.update(|t| {
                *t = customtime();
            })
        },
        std::time::Duration::from_secs(1),
    );


    // a resource that displays a view from async data
    let async_data = create_resource(
        || (),
        // every time `count` changes, this will run
        move |_| async move {
            // gets the key of the hashmap that matches the user's input,
            // and displays its hasbrowns

          match earth_time().await.map.get(tmp_name.get().clone().to_lowercase().as_str()).unwrap() {
                TimeZone { name, offset, dst } => {
                    let debug = format!("{:?}, {:?}, {:?}", name, offset, dst);
                    console_log(debug.as_str());
                    // Render a view instead of Logging it to console
                }
            }  
        },
    );

    // INFO! create an x button and dynamically load offset & name
    view! {
        <h1> {name} " Time: " {time} </h1>
    }
}
