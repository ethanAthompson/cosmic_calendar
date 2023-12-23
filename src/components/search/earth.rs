use std::sync::Arc;

use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlHeadingElement, HtmlInputElement, KeyboardEvent, MouseEvent};

use crate::{
    components::tools::innerplanets::earth::earth_time,
    wrappers::{
        strings::get_initials,
        web::{all_items, update_dom_el},
    },
};

// this will be generic after I do this first one
#[component]
pub fn SearchBar() -> impl IntoView {
    let search_icon = Icon::from(FiIcon::FiSearch);
    let close_icon = Icon::from(BiIcon::BiXRegular);

    let input = create_rw_signal("".to_string());
    let items = create_rw_signal(Vec::new());
    let filtered_items = create_rw_signal(Vec::new());

    let input_el: NodeRef<Input> = create_node_ref();

    let async_data = create_resource(
        move || items,
        move |_| async move {
            for item in earth_time().await.map.into_values() {
                items.update(move |time| {
                    time.push(get_initials(item.name));
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
                // maybe use a map to prevent dupes on both sides?
                let spans = all_items("supported-timezones", "span");

                // INFO! for the user if lazy
                if spans.length() == 1 {
                    let first_span = spans
                        .item(0)
                        .unwrap()
                        .dyn_into::<HtmlElement>()
                        .unwrap()
                        .text_content()
                        .unwrap();

                    // console_log(first_span.to_string().as_str());
                    input.set(first_span.to_string());
                }

                // pushes this input its parent list
                document()
                    .get_element_by_id("earth-zones")
                    .unwrap()
                    .append_child(
                        view! {
                            <span> {input.get()}: Etc : Etc </span>
                        }
                        .as_ref(),
                    )
                    .unwrap();
            }
            // Up
            38 => {}
            // Down
            40 => {
                let spans = all_items("supported-timezones", "span");
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
    #[prop(into)] vector: RwSignal<Vec<String>>,
    #[prop(into)] input: RwSignal<String>,
) -> impl IntoView {
    // adds the chosen option to the search input
    let on_add = move |ev: MouseEvent| {
        // gets the id and performs js casting to properly direct it
        let id = ev
            .target()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .expect("Html Element id search not found")
            .id();

        // console_log(id.as_str());

        input.set(id);

        // hides the options after you set it.
        update_dom_el("supported-timezones-container", "hidden");
    };

    view! {
        <For
            each=move || vector.get()
            key=|item| item.clone()
            let:data
        >
            <span id=format!("{data}") class="span-item" on:click=on_add > {data} </span>
        </For>
    }
}
