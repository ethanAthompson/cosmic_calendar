use leptos::*;
use crate::components::links::largescreen::SuperBigLinks;

#[component]
pub fn FooterLinks() -> impl IntoView {
    let with_switch = create_rw_signal(false);
    
    view! {
        <div class="flex justify-end space-x-12 items-center px-4 text-2xl">
            <SuperBigLinks with_switch/>
        </div>
    }
}
