//

use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;

#[component]
pub fn SearchBar() -> impl IntoView {
    let search_icon = Icon::from(FiIcon::FiSearch);

    let timezone_input = create_rw_signal("".to_string());

    let timezone_input_el: NodeRef<Input> = create_node_ref();

    let on_change = move |_| {
        let value = timezone_input_el.get().expect("<input> to exist").value();
        timezone_input.set(value.clone());
        console_log(value.clone().as_str());
    };

    view! {
        <div class="py-2 font-light">
            <label> Search For a Calendar </label>
             <div class="relative">
                <input on:change=on_change node_ref=timezone_input_el id="calendar-input" type="text" name="calendar" class="input-box" placeholder="Calendar Search"/>
                <div class="input-box--search">
                    <Icon icon=search_icon class=""/>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn CheckBox() -> impl IntoView {
    let on_change = move |ev: web_sys::Event| {
        //
        let value = event_target_checked(&ev);

        if value == true {
            console_log("Ok");
            // include earth
        } else {
            console_log("No");
            // don't include earth
        }
    };
    view! {
        <div class="py-2 font-light">
            <div class="flex items-center space-x-2">
                <input  on:change=on_change
                        type="checkbox" name="calendar"
                        class="" value="yes"/>
                <label> Include Earth date? </label>
            </div>
        </div>
    }
}
