use std::{str::FromStr, sync::Arc};

use crate::{
    components::{
        card::earth::EarthDisplay,
    },
    wrappers::{
        strings::{filtered_vec, get_initials, matching_left},
        web::{all_items, save_data, update_dom_el},
    },
};
use chrono::prelude::*;
use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlHeadingElement, HtmlInputElement, KeyboardEvent, MouseEvent, Node};

#[component]
pub fn SearchBar() -> impl IntoView {
    let search_icon = Icon::from(FiIcon::FiSearch);
    let close_icon = Icon::from(BiIcon::BiXRegular);
    let input = create_rw_signal("".to_string());
    let traverse = create_rw_signal(0);
    let filtered_items = create_rw_signal(Vec::new());
    let input_el: NodeRef<Input> = create_node_ref();
    let timezones = create_rw_signal(Vec::new());
    let hide = move || update_dom_el("supported-timezones-container", "hidden");
    let show = move || update_dom_el("supported-timezones-container", " ");

    for variant in chrono_tz::TZ_VARIANTS {
        // console_log(&format!("{:?}", variant));
        timezones.update(move |timezone| {
            timezone.push(format!("{:?}", variant));
        });
    }

    let on_input = move |_| {
        let value = input_el.get().expect("<input> to exist").value();

        let matched = matching_left(&timezones, value.clone());

        input.set(value.clone());

        // filters items based on input
        filtered_vec(&filtered_items, &timezones, &input);

        // you have nothing?
        if matched.is_empty() || value.is_empty() {
            hide();
        }

        // you have something?
        if !matched.is_empty() {
            show();
        }
    };

    let on_focus_out = move |_| {
        traverse.set(0);
        set_timeout(hide, std::time::Duration::from_millis(500));
    };

    let on_focus_in = move |_| {
        hide();
        // show();
    };

    let on_remove = move |ev: MouseEvent| {
        // WARNING! use "" over " " becuase " " adds whitespace
        ev.prevent_default();
        input.set("".to_string());
        traverse.set(0);
        hide();
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
                let zones = all_items("earth-tz-card", "span");

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
                    if let Some(display_bar) = document().get_element_by_id("earth-tz-card") {
                        save_data().1.update(move |data| {

                            let timezone = chrono_tz::Tz::from_str(&input.get()).expect("Timezone appears");

                            data.earth.insert(input.get(), timezone);
                        });

                        let bar_info = view! {
                            <span class="flex space-x-2" id=input.get()>
                                <EarthDisplay input=input.get_untracked() />
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
                        current_element.set_class_name("p-2 dark:bg-slate-900 bg-slate-200");
                        // console_log(current_element.text_content().unwrap().as_str());
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
                        next_element.set_class_name("p-2 dark:bg-slate-800 bg-slate-100");

                        // console_log(next_element.text_content().unwrap().as_str());
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
                                current_element.set_class_name("p-2 dark:bg-slate-900 bg-slate-200");
                            }
                        }
                    }

                    // logs item before Run 2
                    // console_log(&*item.to_string().as_str());

                    // Gracefully moves down
                    if let Some(next_item) = spans.item(*item) {
                        if let Ok(next_element) = next_item.dyn_into::<HtmlElement>() {
                            next_element.set_class_name("p-2 dark:bg-slate-800 bg-slate-100");
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
                        current_element.set_class_name("p-2 dark:bg-slate-900 bg-slate-200");
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
        <main class="flex flex-col py-2">
            <section name="Container" class="flex flex-grow flex-col col-auto z-20 p-0">
                <div name="SearchBar" class="relative">
                    <input
                        on:focusin=on_focus_in on:focusout=on_focus_out on:input=on_input
                        on:keydown=on_keydown
                        node_ref=input_el id="timezone-input" type="text" 
                        placeholder="Timezone Search" prop:value=input maxlength="20" autocomplete="off"
                        class="py-3 px-4 ps-11 block w-full rounded-lg
                             text-base text-gray-900 border border-gray-300
                             bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 
                            dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500
                        "
                    />
                    <div class="absolute inset-y-0 start-0 flex items-center pointer-events-none z-20 ps-4">
                        <Icon icon=search_icon class="dark:text-white text-black"/>
                    </div>
                    <button class="absolute inset-y-0 z-20 end-0 cursor-grab hover:text-red-400 dark:text-white text-black" on:click=on_remove>
                     <Icon icon=close_icon class="w-9 h-9"/>
                    </button>
                </div>
            </section>
            <section name="Container">
                <div id="supported-timezones-container" class="hidden">
                    <div class="
                            flex flex-col focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 
                            dark:text-gray-400 dark:focus:ring-gray-600 py-0 px-0 ps-0 w-full bg-slate-200 border-gray-200 shadow-sm 
                            text-sm focus:z-10 rounded-lg max-h-72
                        ">
                        <section id="supported-timezones" class="flex flex-col overflow-y-auto h-min font-bold overflow-scroll">
                            <Tz_Items vector=filtered_items input=input/>
                        </section>
                    </div>
                </div>
            </section>
        </main>
    }
}

#[component]
pub fn Tz_Items(
    // represents the list of timezones you can search
    #[prop(into)] vector: RwSignal<Vec<String>>,
    // represents the id of the timezones which is dynamically used
    #[prop(into)] input: RwSignal<String>,
) -> impl IntoView {
    let append_input = move |ev: MouseEvent| {
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
            <span on:click=append_input class="cursor-pointer p-2 dark:bg-slate-900 bg-slate-200">{data}</span>
        </For>
    }
}
