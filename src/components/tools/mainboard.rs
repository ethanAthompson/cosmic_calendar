// Mainboard Part
use crate::components::controller::celestial::Spooler as MainboardCelestial;
use crate::components::controller::earth::Spooler as MainboardEarth;


use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[component]
pub fn Card() -> impl IntoView {
    view! {
        
         <section class="justify-center flex items-center w-full">
             <div class="p-4">
                 <div class="p-4">
                     <p class="p-2 dark:hover:bg-blend-lighten dark:mix-blend-screen w-full text-start cursor-pointer -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 ease-in-out duration-300 glitch desktop:text-6xl laptop:text-4xl tablet:text-4xl text-xl">
                         "Mainboard"
                     </p>
                 </div>
                 <div class="flex flex-col p-4 dark:mix-blend-screen shadow-earth font-bold">
                     <section>
                         <label>"Earth Controller"</label>
                         <div class="p-4">
                             <MainboardEarth/>
                         </div>
                     </section>
                     <section>
                         <label>"Celestial Controller"</label>
                         <div class="p-4">
                             <MainboardCelestial/>
                         </div>
                     </section>
                 </div>
             </div>
         </section>
    }
}
