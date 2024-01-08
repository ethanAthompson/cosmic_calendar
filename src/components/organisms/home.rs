use leptos::*;
use leptos_router::A;

use crate::interfaces::ceramics::traditional::DashboardRightIcon;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <main class="grid grid-rows-2">
            <div class="flex p-4">
                <section class="flex flex-col p-2">
                    <h1 class="font-rubik screen-1 text-content-1 pointer-events-none"> Welcome to Zone </h1>
                </section>
                <section class="flex flex-col-reverse">
                </section>
            </div>
            <div class="grid grid-cols-2 p-4">
                <section class="flex items-center">
                    <button class="enter-btn">
                        <DashboardRightIcon class="aspect-square"/>
                        <A href="dashboard" class="font-rubik px-2">Lock in</A>
                    </button>
                </section>
                <section class="flex flex-col">
                </section>
            </div>
        </main>
    }
}
