// Finish what you set to do today
// Clean up footer maybe?
// lean up entire rust code, get it looking neat and clean, avoid redundancies.
// You can get the space stuff later.
use crate::components::tools::mainboard::earth_date_session::Card as EarthDateSession;
use crate::components::tools::mainboard::earth_tz_session::Card as EarthTzSession;

use leptos::{
    html::{Input, Select},
    leptos_dom::logging::console_log,
    *,
};
use leptos_icons::*;

use web_sys::{
    DragEvent, HtmlElement, HtmlHeadingElement, HtmlInputElement, InputEvent, KeyboardEvent,
    MouseEvent, Node,
};

#[component]
pub fn Spooler() -> impl IntoView {
    let add_icon = Icon::from(AiIcon::AiPlusOutlined);
    let count = create_rw_signal(0);
    
    let add_tz_session = move |_| {
        let view = view! {
            <span>
                <EarthTzSession/>
            </span>
        };

        document()
            .get_element_by_id("earth-tz-mod")
            .unwrap()
            .append_child(view.as_ref())
            .unwrap();
    };

    let add_date_session = move |_| {
        count.update(move |value| {
            *value += 1;
        });

        let view = view! {
            <span>
                <EarthDateSession session_id=format!("{:?}xSession", count)/>
            </span>
        };

        document()
            .get_element_by_id("earth-date-mod")
            .unwrap()
            .append_child(view.as_ref())
            .unwrap();
    };

    view! {
        <div id="earth-spooler" class="p-4 rounded-xl cursor-pointer">
            <section id="earth-tz-buttons" class="flex space-x-4 items-center ">
                    <div class="relative">
                        <button id="spool-tz" class="p-2 dark:hover:bg-blend-lighten dark:mix-blend-screen w-full text-start cursor-pointer
                        -skew-y-3 scale-75 hover:-translate-y-2 hover:scale-100 focus:-translate-y-2 focus:scale-100
                        ease-in-out duration-300 glitch desktop:text-2xl laptop:text-xl tablet:text-base text-sm
                        flex space-x-4
                    " on:click=add_tz_session>
                            <Icon icon=add_icon class="w-8 h-8" />
                            Timezone
                        </button>
                    </div>
                    <div class="relative">
                        <button id="spool-date" class="p-2 dark:hover:bg-blend-lighten dark:mix-blend-screen w-full text-start cursor-pointer
                        -skew-y-3 scale-75 hover:-translate-y-2 hover:scale-100 focus:-translate-y-2 focus:scale-100 
                        ease-in-out duration-300 glitch desktop:text-2xl laptop:text-xl tablet:text-base text-sm
                        flex space-x-4
                    "
                     on:click=add_date_session
                >
                            <Icon icon=add_icon class="w-8 h-8"/>
                            Date
                        </button>
                    </div>
            </section>

            <div>
                // Where you convert Earth Timezone -> Space Timezone
                <div id="earth-tz-mod" class="py-4 flex flex-col space-y-4"></div>

                // Where you convert Earth Dates -> Space Dates
                <div id="earth-date-mod" class="py-4 flex flex-col space-y-4"></div>
            </div>
        </div>
    }
}
