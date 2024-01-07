use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::{A, *};
use std::fmt;
use web_sys::{MediaQueryList, Storage};

#[component]
pub fn Card(
    #[prop(into)] name: RwSignal<String>,
    #[prop(into)] image: RwSignal<String>,
    #[prop(into)] media: RwSignal<String>,
) -> impl IntoView {
    let download_icon = Icon::from(BiIcon::BiDownloadSolid);

    view! {
        <div class="p-4 mix-blend-screen flex flex-col gap-4">
            <section>
                <p>{name}</p>
            </section>
            <section>
                <img src=image class="w-fit h-fit aspect-square p-4" />
            </section>
            <section>
                <button class="border dark:border-amber-200
                    border-amber-800 items-center flex space-x-2
                    p-2 hover:bg-blend-lighten mix-blend-screen w-full text-start cursor-pointer
                    -skew-y-3 scale-50 hover:-translate-y-2 hover:scale-75 focus:-translate-y-2 focus:scale-75 
                    ease-in-out duration-300 glitch desktop:text-6xl laptop:text-4xl tablet:text-4xl text-xl                
                    shadow-md
                " type="download" download=media>
                    <Icon icon=download_icon class="w-8 h-8 dark:text-white text-black"/>
                        <em class="desktop:text-xl text-sm laptop:text-base">{name}</em>
                </button>
            </section>
        </div>
    }
}
