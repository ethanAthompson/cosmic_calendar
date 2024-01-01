pub mod asteroids;
pub mod innerplanets;
pub mod moons;
pub mod outerplanets;

use leptos::*;
use leptos_router::*;
use crate::components::search::earth::SearchBar as EarthBar;
use crate::components::search::celestial::SearchBar as CelestialBar;
use crate::components::search::calendar::SelectBar as CalendarBar;
use crate::components::search::calendar::EnterDateBar;
use crate::components::card::earth::Card as EarthCard;
use crate::components::card::celestial::Card as CelestialCard;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap};


#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SavedData {
    pub earth: HashMap<String, chrono_tz::Tz>,
    // celestial timezones a somewhat comprehensive list of them
    // pub celestial: HashMap<String, rust_solar::Ctz>,
    pub celestial: HashMap<String, chrono_tz::Tz>,
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
pub fn Tools() -> impl IntoView {
    view! {
        <Form action="" method="">
            <main class="grid justify-center items-center 
                desktop:grid-cols-2 laptop:grid-cols-2 tablet:grid-cols-2 grid-rows-1
            ">
                <section class="justify-center flex items-center p-4 w-full">
                    <div class="p-4">
                        <div class="p-4">
                            <p class="
                    p-2 hover:bg-blend-lighten mix-blend-screen w-full text-start cursor-pointer
                    -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 
                    ease-in-out duration-300 glitch desktop:text-6xl laptop:text-4xl tablet:text-4xl text-xl        
                 ">"Configuration"</p>
                        </div>
                        <div class="flex flex-col p-4 mix-blend-screen shadow-earth font-bold">
                            <section>
                                <label>"Select Earth Timezone"</label>
                                <div class="p-4">
                                    <EarthBar/>
                                </div>
                            </section>
                            <section>
                                <label>"Select Celestial Body"</label>
                                <div class="p-4">
                                    <CelestialBar/>
                                </div>
                            </section>
                            <section>
                                <label>"Enter Earth Date"</label>
                                <div class="p-2 flex flex-col gap-y-2">
                                    <EnterDateBar/>
                                </div>
                                <figcaption class="font-light
                     p-2 hover:bg-blend-lighten mix-blend-screen w-full text-start cursor-pointer
                    -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 
                    ease-in-out duration-300 glitch desktop:text-6xl laptop:text-4xl tablet:text-4xl text-xl               
        
        ">Default Calendar is Gregorian</figcaption>
                            </section>
                        </div>
                    </div>
                </section>

                <section class="justify-center flex items-center p-4 w-full">
                    <div class="p-4">
                        <div class="p-4">
                            <p class="
                    p-2 hover:bg-blend-lighten mix-blend-screen w-full text-start cursor-pointer
                    -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 
                    ease-in-out duration-300 glitch desktop:text-6xl laptop:text-4xl tablet:text-4xl text-xl        
                ">"Display"</p>
                        </div>
                        <div class="flex flex-col p-4 mix-blend-screen shadow-earth font-bold">
                            <section class="py-2">
                                <label>"Earth Timezones"</label>
                                <div class="p-4">
                                    <EarthCard/>
                                </div>
                            </section>
                            <section class="py-2">
                                <label>"Celestial Bodies"</label>
                                <div class="p-4">
                                    <CelestialCard/>
                                </div>
                           </section>
                            <section class="py-2">
                                <label>"Earth Date"</label>
                                <div class="p-4">
                                    <CalendarBar/>
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

