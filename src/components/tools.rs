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
            <main class="grid justify-center items-center desktop:grid-cols-3 laptop:grid-cols-3 grid-rows-1">
                // <Configuration/>
                <MainBoard/>
                // <Display/>
            </main>

            // Sub Content
            <main>
                // <MoreInfo/>
            </main>
         </Form>
    }
}
