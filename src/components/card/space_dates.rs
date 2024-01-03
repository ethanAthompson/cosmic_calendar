use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::wrappers::web::save_data;

#[component]
pub fn Card() -> impl IntoView {
    view! {}
}

#[component]
pub fn SelectBar() -> impl IntoView {
    view! {
        <div class="flex items-center space-x-2 p-4 dark:bg-slate-900 bg-slate-200 rounded-xl
            hover:shadow-amber cursor-pointer
        ">
            <DateDisplay />
        </div>
    }
}
// Loads options from local storage
#[component]
pub fn DateDisplay() -> impl IntoView {
    let choices = create_rw_signal(Vec::new());

    choices.update(|choice| {
        for item in save_data().0.get_untracked().celestial.into_values() {
            choice.push(item);
        }
    });

    view! {
        <Show
            when=move || (choices.get().len() > 0) == true
            fallback=move || view! {<span>"waiting for date..."</span>}
        >
        <section>
            <p class="font-bold desktop:text-2xl laptop:text-2xl tablet:text-2xl text-xl">
                // {year}/{month}/{day}
            </p>
        </section>
           <select id="calendars" class="px-2 dark:bg-slate-900 bg-slate-200 cursor-pointer dark:hover:bg-blend-lighten dark:mix-blend-screen w-full text-start -skew-y-3 scale-75 hover:-translate-y-2 hover:scale-100 focus:-translate-y-2 focus:scale-100  ease-in-out duration-300 glitch rounded-none desktop:text-2xl laptop:text-2xl tablet:text-2xl text-xl"
            oninput="this.form.requestSubmit()" required="true" name="space_calendar">
                <For
                    each=move || choices.get()
                    key=|item| item.clone()
                    let:data
                >
                 <option class="" value=data.to_lowercase()>{data}</option>
                </For>
           </select>
        </Show>
    }
}
