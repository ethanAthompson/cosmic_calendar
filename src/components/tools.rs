pub mod configuration;
pub mod ctz;
pub mod ctz_traits;
pub mod display;
pub mod mainboard;
pub mod more_info;
pub mod local_storage;

use crate::components::tools::configuration::Card as Configuration;
use crate::components::tools::display::Card as Display;
use crate::components::tools::mainboard::Card as MainBoard;
use crate::components::tools::more_info::Card as MoreInfo;

use leptos::*;
use leptos_router::*;

#[component]
pub fn Card() -> impl IntoView {
    view! {
         <Form action="" method="">
            // Main Content
            <main class="">
                <section class="flex space-x-2">
                    <Configuration/>
                    <Display/>
                </section>
                <section class="flex space-y-2 pb-2">
                    <MainBoard/>
                </section>
            </main>

            // Sub Content
            <main>
                // <MoreInfo/>
            </main>
         </Form>
    }
}
