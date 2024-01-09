use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::*;
use leptos_use::use_element_hover;
use crate::components::molecules::sidebars::dashboard::Component as LeftSideBarMain;
use crate::components::molecules::footers::dashboard::Component as FooterMain;
use crate::components::molecules::headers::dashboard::Component as HeaderMain;
use crate::components::molecules::navbars::dashboard::Component as RightSideBarMain;

#[component]
pub fn Page() -> impl IntoView {

    let is_sidebar = create_rw_signal(true);
    let mini_sidebar = move || {
        view! {
            <div> eggs </div>
        }
    };

    let toggle_sidebar = move |_| {
        is_sidebar.update(|state| {
            *state = !*state;
        })
    };
    
    view! {
        <main class="flex flex-row justify-center items-center p-4">
            <section class="grid grid-cols-1 justify-center items-center">
                <HeaderMain/>
                <section class="flex flex-row jusitfy-center items-center ">
                    // INFO! Custom sidebar positioning in settings..
                    <LeftSideBarMain is_sidebar=is_sidebar mini_sidebar=mini_sidebar toggle_sidebar=toggle_sidebar/>
                    <Outlet />
                    <RightSideBarMain/>        
                </section>
                <FooterMain />
            </section>
        </main>
    }
}
