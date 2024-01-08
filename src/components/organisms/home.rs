use leptos::*;
use leptos_router::A;

use crate::interfaces::ceramics::traditional::DashboardRightIcon;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <main class="grid grid-rows-2 p-4 space-y-12">
            <section class="flex p-4">
                <h1 class="font-rubik screen-1 text-content-1 pointer-events-none"> Welcome to Zone </h1>
            </section>
            <section class="flex items-center justify-start">
                <A href="dashboard" class="enter-btn screen-1 font-rubik px-2">
                    <DashboardRightIcon class="w-10 h-10 text-content-1"/>
                    <figcaption class="screen-1 text-content-1">Lock into dashboard</figcaption>
                </A>
            </section>
        </main>
    }
}
