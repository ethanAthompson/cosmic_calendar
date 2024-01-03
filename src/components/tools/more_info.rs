use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[component]
pub fn Card() -> impl IntoView {
    view! {
         <section class="drop-shadow-lg h-[50vh]">
             <div class="p-4">
                 <p>More Info..</p>
             </div>
         </section>
    }
}
