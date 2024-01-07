use leptos::*;

use crate::components::tools::local_storage::SavedData;
use leptos_use::storage::use_local_storage;
use leptos_use::storage::JsonCodec;
use web_sys::{MediaQueryList, Storage};


/// shortcut to access localstorage
pub fn storage() -> Storage {
    window().local_storage().unwrap().unwrap()
}

/// Returns the theme value from localstorage
pub fn storage_theme() -> String {
    storage().get_item("theme").unwrap().unwrap()
}

/// Returns basic media query methods
pub fn media() -> MediaQueryList {
    window()
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
}

/// Overwrites the class of a dom element by a specific id
///
pub fn update_dom_el(id: &'static str, class: &'static str) {
    document()
        .get_element_by_id(id)
        .unwrap()
        .set_class_name(class);
}

// pub fn dom_el_bool(id: &'static str) -> Element {
//     document().get_element_by_id(id).unwrap()
// }


/// Returns an iterator of element items associated with a class
///
/// class: the parent element received by id
/// element: the element that is the child of the parent from id
///
/// WARNING! This must be included to perform casting
///
/// use wasm_bindgen::JsCast;
/// # Model
/// ```html
/// <div id="parent"> 
///    <span id="1"></span>
///    <span id="2"></span>
///    <span id="3"></span>
/// </div> 
/// ```
///
/// # Example
/// ```rust
///  # Represents spans from id 1->3
///  let spans = all_items("parent", "span");
/// 
///  for span in 0..spans.length() {
///      let each_span = spans
///                      .item(span).unwrap()
///                      .dyn_into::<HtmlElement>()
///                      .unwrap();
///
///      console_log(&format!("div-parent/span-{:?}", each_span.id()));
///
///  }
///
///```
pub fn all_items(class: &'static str, element: &'static str) -> web_sys::NodeList {
    document()
        .get_element_by_id(class)
        .unwrap()
        .query_selector_all(element)
        .unwrap()
}

/// Returns (get, set, delete)
///
/// get -> Signal<SavedData>
/// set -> WriteSignal<SavedData>
/// delete -> impl (Fn()) + Clone
///
/// # Example
///
/// save_data().0 => .get(), ...
/// save_data().1 => .update(), ...
/// save_data().2 => ...
///
pub fn save_data() -> (
    Signal<SavedData>,
    WriteSignal<SavedData>,
    impl (Fn()) + Clone,
) {
    return use_local_storage::<SavedData, JsonCodec>("user_data");
}

