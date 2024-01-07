use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

#[component]
pub fn Card() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center items-center">
            <div class="py-2">
                <section class="py-2">
                    <p class="desktop:text-6xl laptop:text-4xl tablet:text-2xl text-xl">This page is not implemented</p>
                </section>
            </div>
            <div class="py-4 w-full text-black dark:text-white">
                <section class="py-4 w-full items-center">
                    <button class="p-4 cursor-pointer w-full text-start -skew-y-3 scale-75 hover:-translate-y-2 hover:scale-100 focus:-translate-y-2 focus:scale-100 ease-in-out duration-300 glitch rounded-none ">
                        <A href="/">Go to Home</A>
                    </button>
                </section>
                <section class="py-4 items-center w-full">
                    <button class="p-4 cursor-pointer w-full text-start -skew-y-3 scale-75 hover:-translate-y-2 hover:scale-100 focus:-translate-y-2 focus:scale-100 ease-in-out duration-300 glitch rounded-none">
                        <A href="tool">Go to Tool</A>
                    </button>
                </section>
            </div>
        </div>
    }
}
