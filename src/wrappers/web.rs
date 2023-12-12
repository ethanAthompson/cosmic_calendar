use leptos::{leptos_dom::logging::console_log, *};
use web_sys::{MediaQueryList, Storage};

pub fn storage() -> Storage {
    window().local_storage().unwrap().unwrap()
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
