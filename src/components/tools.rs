pub mod ctz;
pub mod ctz_traits;

use leptos::*;
use leptos_router::*;

// Configuration Part
use crate::components::search::celestial::SearchBar as ConfigurationCelestial;
use crate::components::search::earth::SearchBar as ConfigurationEarth;

// Mainboard Part
use crate::components::controller::celestial::Spooler as MainboardCelestial;
use crate::components::controller::earth::Spooler as MainboardEarth;

// Display Part
use crate::components::card::celestial::Card as DisplayCelestial;
use crate::components::card::earth::Card as DisplayEarth;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SavedData {
    pub earth: HashMap<String, chrono_tz::Tz>,
    // celestial timezones a somewhat comprehensive list of them
    // pub celestial: HashMap<String, rust_solar::Ctz>,
    pub celestial: HashMap<String, String>,
}

impl Default for SavedData {
    fn default() -> Self {
        Self {
            earth: HashMap::new(),
            celestial: HashMap::new(),
        }
    }
}

#[component]
pub fn Card() -> impl IntoView {
    view! {
         <Form action="" method="">
             <main class="grid justify-center items-center desktop:grid-cols-3 laptop:grid-cols-3 grid-rows-1">
                 <section class="justify-center flex items-center p-4 w-full">
                     <div class="p-4">
                         <div class="p-4">
                             <p class="p-2 dark:hover:bg-blend-lighten dark:mix-blend-screen w-full text-start cursor-pointer -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 ease-in-out duration-300 glitch desktop:text-6xl laptop:text-4xl tablet:text-4xl text-xl">
                                 "Configuration"
                             </p>
                         </div>
                         <div class="flex flex-col p-4 dark:mix-blend-screen shadow-earth font-bold">
                             <section>
                                 <label>"Select Earth Timezone"</label>
                                 <div class="p-4">
                                     <ConfigurationEarth/>
                                 </div>
                             </section>
                             <section>
                                 <label>"Select Celestial Body"</label>
                                 <div class="p-4">
                                     <ConfigurationCelestial/>
                                 </div>
                             </section>
                         </div>
                     </div>
                 </section>

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

                 <section class="justify-center flex items-center p-4 w-full">
                     <div class="p-4">
                         <div class="p-4">
                             <p class="p-2 dark:hover:bg-blend-lighten dark:mix-blend-screen w-full text-start cursor-pointer -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 ease-in-out duration-300 glitch desktop:text-6xl laptop:text-4xl tablet:text-4xl text-xl">
                                 "Display"
                             </p>
                         </div>
                         <div class="flex flex-col p-4 dark:mix-blend-screen shadow-earth font-bold">
                             <section class="py-2">
                                 <label>"Earth Timezones"</label>
                                 <div class="p-4">
                                     <DisplayEarth/>
                                 </div>
                             </section>
                             <section class="py-2">
                                 <label>"Celestial Bodies"</label>
                                 <div class="p-4">
                                     <DisplayCelestial/>
                                 </div>
                            </section>
                         </div>
                     </div>
                 </section>
             </main>
         </Form>
         <section class="drop-shadow-lg h-[50vh]">
             <div class="p-4">
                 <p>More Info..</p>
             </div>
         </section>
    }
}
