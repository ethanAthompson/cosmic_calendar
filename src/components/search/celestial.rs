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
            <label> Search for Celestial Bodies </label>
             <div class="relative">
                <input node_ref=timezone_input_el id="celestial-input" on:change=on_change type="text" name="celestial" class="input-box" placeholder="Planet, Moon, or Asteroid Search"/>
                <div class="input-box--search">
                    <Icon icon=search_icon class=""/>
                </div>
            </div>
        </div>
    }
}
