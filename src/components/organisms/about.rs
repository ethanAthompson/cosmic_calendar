use leptos::*;
use leptos_router::A;


use crate::interfaces::ceramics::traditional::DashboardRightIcon;

#[component]
pub fn Page() -> impl IntoView {

    view! {
        <main class="grid grid-cols-1">
            <div class="grid grid-rows-2 p-4 space-y-12">
                <section class="flex p-4">
                    <h1 class="font-rubik screen-1 text-content-1 pointer-events-none"> Welcome to Zone </h1>
                </section>
                <section class="flex items-center justify-start">
                    <A href="dashboard/about" class="enter-btn screen-1 font-rubik px-2">
                        <DashboardRightIcon class="w-10 h-10 text-content-1"/>
                        <figcaption class="screen-1 text-content-1">See Ratings...</figcaption>
                    </A>
                </section>
                <section class="">
                    <figcaption class="pointer-events-none screen-1 text-content-1">Libraries Used</figcaption>
                    // use loop 
                    <ul class="w-fit">
                        <li class="p-4 w-full">
                            <a class="home-link" target="_blank" href="https://leptos-use.rs/introduction.html">Leptos Use</a>
                        </li>
                        <li class="p-4 w-full">
                            <a class="home-link" target="_blank" href="https://github.com/yuankunzhang/charming/tree/main">Charming</a>
                        </li>
                        <li class="p-4 w-full">
                            <a class="home-link" target="_blank" href="https://docs.rs/chrono/latest/chrono/">Chrono</a>
                        </li>
                        <li class="p-4 w-full">
                            <a class="home-link" target="_blank" href="https://docs.rs/chrono-tz/latest/chrono_tz/index.html">Chrono Tz</a>
                        </li>
                        <li class="p-4 w-full">
                            <a class="home-link" target="_blank" href="https://docs.rs/icu_calendar/1.4.0/icu_calendar/index.html">Icu Calendar</a>
                        </li>
                    </ul>
                </section>
            </div>
        </main>
    }
}
