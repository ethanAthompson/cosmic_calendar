use chrono::prelude::*;
use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlElement, HtmlSpanElement, MouseEvent};

use crate::{
    components::tools::innerplanets::earth::{earth_time, EarthTimeZone, SavedData, TimeZone},
    wrappers::{strings::get_initials, web::save_data, date::DATEFORMAT},
};

#[component]
pub fn ChosenTimeZones() -> impl IntoView {

    // INFO! use a transition instead
    // INFO! needs short delay to find id="earth-zones"
    set_timeout(
        move || {
            choose_loaded();
        },
        std::time::Duration::from_millis(500),
    );

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
pub fn LoadedRonEarth(fullname: String, offset: i32) -> impl IntoView {
    let close_icon = Icon::from(BiIcon::BiXCircleRegular);
    let loaded_offset = create_rw_signal(offset);
    let tmp_name = create_rw_signal(get_initials(fullname.clone()));

    // INFO! I pass down the timezone so the async data renders properly
    let customtime = move |_| {
        loaded_offset.set(offset);

        let hour = 3600;
        let fixed_offset = FixedOffset::east_opt(loaded_offset.get_untracked() * hour).unwrap();

        return Utc::now()
            .with_timezone(&fixed_offset)
            .format(DATEFORMAT)
            .to_string();
    };

    let time = create_rw_signal(customtime(loaded_offset.get()));

    set_interval(
        move || {
            time.update(|value| {
                *value = customtime(loaded_offset.get());
            });
        },
        std::time::Duration::from_secs(1),
    );

    let on_remove_loaded_timezone = move |_| {
        if let Some(parent) = document().get_element_by_id(&tmp_name.get()) {
            if let Some(button) =
                document().get_element_by_id(&format!("button-{}", &tmp_name.get()))
            {
                save_data().1.update(move |data| {
                    parent.remove();

                    // removes the key from the map
                    for i in data.earth.clone().into_keys() {
                        console_log(&format!("Keys: {:?}", i));

                        data.earth.remove(&tmp_name.get());
                        // console_log(&format!("removed :{:?}x{:?}", tmp_name.get(),data.earth));
                    }
                });
            }
        };
    };

    view! {
        <p>{fullname.clone()} </p>
        <p>{time}</p>
        <button
            id={format!("button-{}",get_initials(fullname.clone()))}
            class="cursor-grab hover:text-red-400 items-center"
            on:click=on_remove_loaded_timezone
        >
                <Icon icon=close_icon class="w-6 h-6 items-center"/>
        </button>
    }
}

pub fn choose_loaded() {
    // only fills the earth-zones.
    if let Some(parent) = document().get_element_by_id("earth-zones") {
        // fills up each item in earth vector
        for key in save_data().0.get().earth.into_values() {
            // console_log(&format!(
            //     "
            //             Abbr: {:?}
            //             Fullname: {:?}
            //             Offset: {:?}
            //         ",
            //     key.abbr, key.fullname, key.offset
            // ));

            let loaded_node = view! {
                <span class="flex space-x-2" id={key.abbr}>
                    <LoadedRonEarth
                        fullname=key.fullname
                        offset=key.offset
                    />
                </span>
            };

            parent.append_child(&loaded_node).unwrap();
        }
    };
}

#[component]
pub fn RonEarth(name: String) -> impl IntoView {
    let close_icon = Icon::from(BiIcon::BiXCircleRegular);
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

    let on_remove_timezone = move |_| {
        if let Some(parent) = document().get_element_by_id(&tmp_name.get()) {
            if let Some(button) =
                document().get_element_by_id(&format!("button-{}", tmp_name.get()))
            {
                save_data().1.update(move |data| {
                    parent.remove();

                    // removes the key from the map
                    for i in data.earth.clone().into_keys() {
                        console_log(&format!("Keys: {:?}", i));

                        data.earth.remove(&tmp_name.get());
                        // console_log(&format!("removed :{:?}x{:?}", tmp_name.get(),data.earth));
                    }
                });
            }
        }
    };

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
            //INFO! X button goes here, just pops it out of the element list
            <button
                    id={format!("button-{}",get_initials(data.clone().0.to_uppercase()))}
                    class="cursor-grab hover:text-red-400 items-center"
                    on:click=on_remove_timezone
            >
                <Icon icon=close_icon class="w-6 h-6 items-center"/>
            </button>
    </Await>
    }
}

// WARNING! Program crashes when you move this, some visibility reason..
pub async fn get_zones(tmp_name: RwSignal<String>) -> (String, i32, u8) {
    match earth_time()
        .await
        .map
        .get(tmp_name.get_untracked().clone().to_lowercase().as_str())
        .unwrap()
    {
        TimeZone { name, offset, dst } => {
            // let debug = format!("{:?}, {:?}, {:?}", name, offset, dst);
            // console_log(debug.as_str());

            // zone_name.set(name.to_string());
            // zone_offset.set(*offset);
            // zone_dst.set(*dst);

            return (name.to_string(), *offset, *dst);
        }
    }
}
