use std::sync::Arc;

use crate::{
    components::{
        card::earth::{get_zones, RonEarth},
        tools::innerplanets::earth::{earth_time, EarthTimeZone},
    },
    wrappers::{
        strings::get_initials,
        web::{all_items, save_data, update_dom_el},
    },
};
use chrono::prelude::*;
use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlHeadingElement, HtmlInputElement, KeyboardEvent, MouseEvent, Node};
// this will be generic after I do this first one
#[component]
pub fn SearchBar() -> impl IntoView {
    let search_icon = Icon::from(FiIcon::FiSearch);
    let close_icon = Icon::from(BiIcon::BiXRegular);

    let input = create_rw_signal("".to_string());
    let traverse = create_rw_signal(0);
    let items = create_rw_signal(Vec::new());
    let loaded_items = create_rw_signal(Vec::new());
    let filtered_items = create_rw_signal(Vec::new());
    let input_el: NodeRef<Input> = create_node_ref();

    let async_data = create_resource(
        move || items,
        move |_| async move {
            // for user's non-loaded data
            for item in earth_time().await.map.into_values() {
                // console_log(item.name.as_str());

                items.update(move |time| {
                    time.push(get_initials(item.name));
                });
            }

            // for user's loaded data
            for loaded_item in earth_time().await.map.into_values() {
                loaded_items.update(move |time| {
                    time.push((loaded_item.name, loaded_item.offset));
                });
            }
        },
    );

    filtered_items.update(|item| {
        let matched: Vec<String> = items
            .get_untracked()
            .into_iter()
            .filter(|item| item.contains(input.get().clone().as_str()).to_owned())
            .collect();

        *item = matched;
    });

    let on_input = move |_| {
        let value = input_el.get().expect("<input> to exist").value();

        // value is set to uppercase here
        let evalue = value.to_uppercase().clone();

        input.set(evalue.clone());

        filtered_items.update(|item| {
            let matched: Vec<String> = items
                .get_untracked()
                .into_iter()
                .filter(|item| item.contains(input.get().clone().as_str()).to_owned())
                .collect();

            *item = matched;
        });

        let matched: Vec<String> = items
            .get()
            .into_iter()
            .filter(|item| item.contains(evalue.clone().as_str()).to_owned())
            .collect();

        // you have nothing?
        if matched.is_empty() || evalue.is_empty() {
            update_dom_el("supported-timezones-container", "hidden");
        }

        // you have something?
        if !matched.is_empty() {
            update_dom_el("supported-timezones-container", " ");
        }
    };

    let on_focus_out = move |_| {
        traverse.set(0);
        // hides on delay
        set_timeout(
            move || {
                update_dom_el("supported-timezones-container", "hidden");
            },
            std::time::Duration::from_millis(300),
        );
    };

    let on_focus_in = move |_| {
        update_dom_el("supported-timezones-container", "hidden");
    };

    let on_remove = move |ev: MouseEvent| {
        ev.prevent_default();
        // WARNING! use "" over " " becuase " " adds whitespace
        input.set("".to_string());
        traverse.set(0);

        update_dom_el("supported-timezones-container", "hidden");
    };

    let on_keydown = move |ev: KeyboardEvent| {
        let key = ev.key_code();

        match key {
            // [A-Z]
            65..=90 => {}
            // Backspace
            8 => {}
            // Tab
            9 => {}
            // Enter
            13 => {
                let spans = all_items("supported-timezones", "span");
                let zones = all_items("earth-zones", "span");

                // INFO! for the user if lazy
                if spans.length() == 1 {
                    if let Some(first_span) = spans.item(0) {
                        if let Ok(first_item) = first_span.dyn_into::<HtmlElement>() {
                            if let Some(first_item_text) = first_item.text_content() {
                                input.set(first_item_text);
                            }
                        }
                    }
                } else {
                    // INFO! keep here so it doesn't triumph the neighboring if clauses
                    // WARNING! base case: you enter something and it nots from the list with span showing
                    if !filtered_items.get().contains(&input.get()) {
                        input.set("".to_string());
                    }
                }

                // WARNING! base case: if you have the same timezone in the results
                for zone in 0..zones.length() {
                    let item = zones.item(zone).unwrap().dyn_into::<HtmlElement>().unwrap();
                    // console_log(&zones.length().to_string());
                    // console_log(&zones.length().to_string());
                    // console_log(&item.id().to_string());

                    // if a timezone is already added
                    // then don't add that same timezone
                    if item.id() == input.get() {
                        input.set("".to_string());
                    }
                }

                // WARNING! base case: if you enter something thats not from the list, it doesn't send it empty
                if spans.length() == 0 {
                    input.set("".to_string());
                }

                // if your input isn't empty then it must match on of the timezones
                if !input.get().is_empty() {
                    if let Some(display_bar) = document().get_element_by_id("earth-zones") {
                        // get it to display full name
                        save_data().1.update(move |data| {
                            // let dbgitems = format!("{:?}", loaded_items.get());
                            // console_log(dbgitems.as_str());

                            for i in 0..loaded_items.get().len() {
                                // BUG: the input is hard-coded to initials: EST, GMT so I have to match it from that
                                if input.get() == get_initials(loaded_items.get()[i].0.clone()) {
                                    // loads proper data into storage
                                    let earth_example: EarthTimeZone = EarthTimeZone {
                                        abbr: get_initials(loaded_items.get()[i].0.clone()),
                                        offset: loaded_items.get()[i].1,
                                        fullname: loaded_items.get()[i].0.clone(),
                                    };

                                    // for hashmap
                                    data.earth.insert(earth_example.abbr.clone(), earth_example);
                                }
                            }
                        });

                        let bar_info = view! {
                            <span class="flex space-x-2" id=input.get()>
                                <RonEarth name=input.get_untracked() />
                            </span>
                        };

                        display_bar
                            .append_child(bar_info.as_ref())
                            .expect("Bar failed to be appended");
                    }
                }
            }

            // Up
            38 => {
                let spans = all_items("supported-timezones", "span");

                // Run 1: Hides currently Selected item
                if let Some(current_item) = spans.item(traverse.get() - 1) {
                    if let Ok(current_element) = current_item.dyn_into::<HtmlElement>() {
                        current_element.set_class_name("span-item");
                        console_log(current_element.text_content().unwrap().as_str());
                    }
                }

                traverse.update(move |item: &mut u32| {
                    // console_log(&item.to_string().as_str());

                    // Run 1: Adjusts to 0-index
                    if *item - 1 == 0 {
                        *item = spans.length();
                    } else {
                        *item -= 1
                    }
                });

                let spans = all_items("supported-timezones", "span");

                // Run 2: Selects Previous item which is the above the curently selected
                if let Some(next_item) = spans.item(traverse.get() - 1) {
                    if let Ok(next_element) = next_item.dyn_into::<HtmlElement>() {
                        next_element.set_class_name("span-trav");

                        console_log(next_element.text_content().unwrap().as_str());
                        input.set(
                            next_element
                                .text_content()
                                .unwrap_or_else(|| String::from(" ")),
                        );
                    }
                };
            }

            // Down: Works!
            40 => {
                let spans = all_items("supported-timezones", "span");

                traverse.update(move |item: &mut u32| {
                    // Run 1: Re adjusts the item tracker
                    if *item > spans.length() {
                        *item = 0;
                    }
                    // Run 1: If item is last, remember to clear it before Run 2!
                    if *item == spans.length() {
                        *item = 0;

                        // clears last item: subtracts by 1 because to adjust to 0-index
                        if let Some(current_item) = spans.item(spans.length() - 1) {
                            if let Ok(current_element) = current_item.dyn_into::<HtmlElement>() {
                                current_element.set_class_name("span-item");
                            }
                        }
                    }

                    // logs item before Run 2
                    // console_log(&*item.to_string().as_str());

                    // Gracefully moves down
                    if let Some(next_item) = spans.item(*item) {
                        if let Ok(next_element) = next_item.dyn_into::<HtmlElement>() {
                            next_element.set_class_name("span-trav");
                            // console_log("Down: Show");

                            input.set(
                                next_element
                                    .text_content()
                                    .unwrap_or_else(|| String::from(" ")),
                            );
                        }
                    };

                    // increments to Run 2
                    if *item == spans.length() {
                        *item = 0;
                    } else {
                        *item += 1;
                    }
                });

                let spans = all_items("supported-timezones", "span");

                // Run 2: gets the previous item and clears it for Run 1!
                if let Some(current_item) = spans.item(traverse.get() - 2) {
                    if let Ok(current_element) = current_item.dyn_into::<HtmlElement>() {
                        current_element.set_class_name("span-item");
                        // Runs: 0 -> none, 1 -> hides 0, 2 -> hides 1, ...
                        // let travel = format!("Travel -> {:?}", traverse.get() - 2 );
                        // console_log("Down: Hide");
                        // console_log(travel.as_str());
                    }
                }
            }
            _ => {
                ev.prevent_default();
            }
        }
    };

    view! {
        <div class="py-2 font-light">
            // manage a list of selected timezones, adds on enter, an X appears when you want to remove it.
            <label> Search for Earth + Mars Timezones </label>
            <div class="relative">
                <input
                    on:focusin=on_focus_in on:focusout=on_focus_out on:input=on_input
                    on:keydown=on_keydown
                    node_ref=input_el id="timezone-input" type="text" name="timezone"
                    class="input-box" placeholder="Timezone Search" prop:value=input
                    maxlength="4" autocomplete="off"

                />
                <div class="input-box--search">
                    <Icon icon=search_icon class=""/>
                </div>
                <button class="absolute inset-y-0 z-20 end-0 cursor-grab hover:text-red-400" on:click=on_remove>
                    <Icon icon=close_icon class="w-10 h-10"/>
                </button>
            </div>

            <div id="supported-timezones-container" class="hidden">
                <div class="flex flex-col result-bgk rounded-b-lg max-h-32">
                    <section id="supported-timezones" class="flex flex-col overflow-y-auto h-min font-bold">
                        <SearchItems vector=filtered_items input=input/>
                    </section>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SearchItems(
    // represents the list of timezones you can search
    #[prop(into)] vector: RwSignal<Vec<String>>,
    // represents the id of the timezones which is dynamically used
    #[prop(into)] input: RwSignal<String>,
) -> impl IntoView {
    // adds the chosen option to the search input
    let on_add = move |ev: MouseEvent| {
        // gets the text content: increases security and performs js casting to properly direct it

        if let Some(event) = ev.target() {
            if let Ok(target) = event.dyn_into::<HtmlElement>() {
                input.set(target.text_content().unwrap());
            }
        }

        // hides the options after you set it.
        update_dom_el("supported-timezones-container", "hidden");
    };

    view! {
        <For
            each=move || vector.get()
            key=|item| item.clone()
            let:data
        >
            <span class="span-item" on:click=on_add > {data} </span>
        </For>
    }
}
