use chrono::prelude::*;
use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlElement, HtmlSpanElement, MouseEvent};

use crate::{
    components::tools::innerplanets::earth::{earth_time, SavedData, TimeZone},
    wrappers::{strings::get_initials, web::save_data},
};

#[component]
pub fn ChosenTimeZones() -> impl IntoView {
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
    let loaded_offset = create_rw_signal(0);
    let tmp_name = create_rw_signal(get_initials(fullname.clone()));
    // INFO! I pass down the timezone so the async data renders properly
    let customtime = move |offset: i32| {
        loaded_offset.set(offset);

        let hour = 3600;
        let fixed_offset = FixedOffset::east_opt(loaded_offset.get_untracked() * hour).unwrap();

        return Utc::now()
            .with_timezone(&fixed_offset)
            .format("%d/%m/%Y %r")
            .to_string();
    };

    let time = create_rw_signal(customtime(loaded_offset.get()));

    set_interval(
        move || {
            time.update(|value| {
                // console_log(&*value.as_str());
                *value = customtime(loaded_offset.get());
            });
        },
        std::time::Duration::from_secs(1),
    );

    // BUG! FIx offsets, they are off
    // BUG! offsets are not used
    let on_remove_loaded_timezone = move |fullname: String| {
        console_log(&fullname);

        save_data().1.update(move |data| {
            // gets the id of the selected timezone
            if let Some(parent) = document().get_element_by_id(&get_initials(fullname.clone())) {
                let button_id = format!("button-{}", &get_initials(fullname.clone()));

                // gets the id of the selected button
                if let Some(button) = document().get_element_by_id(&button_id) {
                    parent.remove();

                    // INFO! id must delete the certain neighbor
                    for i in 0..data.earth.len() {                    
                        data.earth.remove(i);
                    }
                }
            };
        });
    };

    view! {
        <p>{fullname.clone()} </p>
        // BUG! the old way does react to the interval
        // <p>{move || customtime(offset)}</p>

        // INFO! this new way reacts to the interval
        <p>{time}</p>
        <button
            // WARNING! keep for deletion
            id={format!("button-{}",get_initials(fullname.clone()))}
            class="cursor-grab hover:text-red-400 items-center"
            on:click=move |_| on_remove_loaded_timezone(fullname.clone())
        >
                <Icon icon=close_icon class="w-6 h-6 items-center"/>
        </button>
    }
}

pub fn choose_loaded() {
    // spawns default data
    // save_data().1.set(SavedData::default());

    // only fills the earth-zones.
    if let Some(parent) = document().get_element_by_id("earth-zones") {
        // fills up each item in earth vector
        for i in 0..save_data().0.get().earth.len() {
            // console_log("Meow!!");
            let loaded_node = view! {
                // WARNING! don't remove identifier
                <span class="flex space-x-2" id={save_data().0.get().earth[i].0.clone()}>
                    <LoadedRonEarth
                        // displays fullname
                        fullname=save_data().0.get().earth[i].2.clone()
                        offset=save_data().0.get().earth[i].1.clone()
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
        // console_log(tmp_name.get().as_str());

        // gets the id of the selected timezone
        if let Some(parent) = document().get_element_by_id(tmp_name.get().as_str()) {
            let button_id = format!("button-{}", tmp_name.get());

            // gets the id of the selected button
            if let Some(button) = document().get_element_by_id(button_id.as_str()) {
                // let ids = format!("{:?}, {:?}", parent.id().as_str(), button.id().as_str());
                // console_log(ids.as_str());

                // INFO! removes span that matches the parnt
                parent.remove();
                save_data().1.update(move |data| {
                    // WARNING! Linear Search w/ Data Removal
                    // ensures each value of the earth vector
                    for item in 0..data.earth.len() {
                        // if the selected name matched name from the vector
                        if tmp_name.get() == data.earth[item].0 {
                            // remove that current session
                            data.earth.remove(item);
                        }
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
