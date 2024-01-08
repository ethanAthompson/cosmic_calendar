use crate::components::atoms::tooltips::{ExitFullScreen, FullScreen};
use crate::components::molecules::navbars::Component as Navbar;
use crate::components::molecules::sidebars::Component as DashboardSidebar;
use crate::interfaces::ceramics::traditional::{DashboardLeftIcon, MenuGridIcon};
use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::*;
use leptos_use::use_element_hover;

#[component]
pub fn Page() -> impl IntoView {
    let button_el = create_node_ref::<Button>();
    let is_sidebar = create_rw_signal(false);
    let is_fullscreen = create_rw_signal(false);
    let is_tooltip_hovered = use_element_hover(button_el);

    let toggle_fullscreen = move |_| {
        is_fullscreen.update(|value| *value = !*value);
    };

    let toggle_sidebar = move |_| {
        is_sidebar.update(|value| *value = !*value);
    };

    let mini_sidebar = move || {
        view! {
            <div class="relative m-0 w-fit flex flex-col justify-start items-center bg-accent-2/80 rounded-s-xl">
                <section class="z-20 absolute left-0">
                    <button on:click=toggle_sidebar><DashboardLeftIcon class="dash-icon"/></button>
                </section>
            </div>
        }
    };

    view! {
        <Navbar is_fullscreen=is_fullscreen />
        <main class=move || if is_fullscreen.get() {"dashboard-container-2"} else {"dashboard-container-1"}>
            <DashboardSidebar is_sidebar=is_sidebar mini_sidebar=mini_sidebar toggle_sidebar=toggle_sidebar/>
            <div class=move || if is_sidebar.get() {"dashboard-card-2"} else {"dashboard-card-1"}>
                <div class="absolute right-0" on:click=toggle_fullscreen>
                    <Show when=move || is_tooltip_hovered.get() fallback=move || view!{ <span></span>}>
                        <Show when=move || is_fullscreen.get() == false fallback=move || view!{<ExitFullScreen/>}>
                            <FullScreen/>
                        </Show>
                    </Show>
                    <button node_ref=button_el class="">
                        <MenuGridIcon class="dash-icon"/>
                    </button>
                </div>
                <div class="bg-accent-2 p-24 rounded-xl">
                    <Outlet />
                </div>
            </div>
        </main>
    }
}
