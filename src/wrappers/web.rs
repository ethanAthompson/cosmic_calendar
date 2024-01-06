use leptos::*;

use crate::components::tools::local_storage::SavedData;
use leptos_use::storage::use_local_storage;
use leptos_use::storage::JsonCodec;
use web_sys::{Element, MediaQueryList, Storage};

pub fn storage() -> Storage {
    window().local_storage().unwrap().unwrap()
}

pub fn storage_theme() -> String {
    storage().get_item("theme").unwrap().unwrap()
}

pub fn media() -> MediaQueryList {
    window()
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
}

pub fn update_dom_el(id: &'static str, class: &'static str) {
    document()
        .get_element_by_id(id)
        .unwrap()
        .set_class_name(class);
}

pub fn dom_el_bool(id: &'static str) -> Element {
    document().get_element_by_id(id).unwrap()
}

pub fn all_items(class: &'static str, element: &'static str) -> web_sys::NodeList {
    document()
        .get_element_by_id(class)
        .unwrap()
        .query_selector_all(element)
        .unwrap()
}

// Just allows me to dynamically update each key => (get, set, delete)
pub fn save_data() -> (
    Signal<SavedData>,
    WriteSignal<SavedData>,
    impl (Fn()) + Clone,
) {
    // let (state, set_state, del_state) =use_local_storage::<SavedData, JsonCodec>("user_data");
    return use_local_storage::<SavedData, JsonCodec>("user_data");
}

