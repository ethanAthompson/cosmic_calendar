use charming::{
    datatype::{CompositeValue, DataPoint, DataPointItem},
    element::{Color, ItemStyle},
    *,
};
use leptos::{leptos_dom::logging::console_log, *, html::{Button}};
use leptos_router::A;
use leptos_use::use_element_hover;
use crate::interfaces::ceramics::traditional::{DashboardLeftIcon, DashboardRightIcon, MenuGridIcon};

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
            // <div class="relative m-0 w-fit flex flex-col justify-start items-center py-2 bg-accent-2/80 rounded-s-xl" id="sidebar">
                <section class="z-20 absolute left-0">
                    <button on:click=toggle_sidebar><DashboardLeftIcon class="w-10 h-10"/></button>
                </section>
            // </div>
        }
    };

    let apply_start_xl = move || {
        if is_sidebar.get() {
            "grid grid-rows-2 p-4 bg-accent-2/60 w-full rounded-e-xl relative"
        } else {
            "grid grid-rows-2 p-4 bg-accent-2/60 w-full rounded-xl relative"
        }
    };

    let apply_container = move || {
        if is_fullscreen.get() {
            "dashboard-container-2"
        } else {
            "dashboard-container-1"
        }
    };

    view! {
        <Show when=move || is_fullscreen.get() == false fallback=move || view!{<nav></nav>}>
            <header class="p-4 bg-accent-2">
                <nav class="screen-nav justify-between items-center space-x-4">
                    <section class="p-2">
                        <h1 class="nav-item"> Zone </h1>
                    </section>
                    <div class="px-2 flex flex-row justify-between space-x-2">
                        <section class="p-2">
                            <h1 class="nav-item"> About </h1>
                        </section>
                        <section class="p-2">
                            <h1 class="nav-item"> Download </h1>
                        </section>
                    </div>
                </nav>
            </header>
        </Show>
        <main class=apply_container>
            <Show when=move || is_sidebar.get() fallback=mini_sidebar>
                <div class="w-fit py-2 bg-accent-2/80 rounded-s-xl">
                    <section class="flex flex-col justify-evenly space-y-4 p-2">
                    <span class="hidden">Dashboard Items</span>
                        <button on:click=toggle_sidebar>
                            <DashboardRightIcon class="w-10 h-10"/>
                        </button>
                    </section>
                </div>
            </Show>
            <div class=apply_start_xl>
                <div class="absolute right-0" on:click=toggle_fullscreen>
                    <Show when=move || is_tooltip_hovered.get() fallback=move || view!{ <span></span>}>
                        <span class="hidden"><em class="font-light flex space-x-2">Fullscreen Mode</em></span>
                    </Show>
                    <button node_ref=button_el class=""><MenuGridIcon class="w-10 h-10"/></button>
                </div>
                <section class="p-4 grid grid-cols-2">
                    <div class="p-2">
                        <h1 class="font-rubik text-content-1"> Local Timezone </h1>
                        <section class="p-2">
                            <p>EST</p>
                        </section>
                    </div>
                    <div class="p-2">
                        <h1 class="font-rubik text-content-1"> Local Date </h1>
                        <section class="p-2">
                            <p>10/20/2 Saturr day ...</p>
                        </section>
                    </div>
                </section>
                <section class="p-2 grid grid-cols-3">
                        <div class="">
                            <h1 class="font-rubik text-content-1"> Hour Gauge</h1>
                            <SimpleGauge id="1cha"/>
                        </div>
                        <div class="">
                            <h1 class="font-rubik text-content-1"> Minute Gauge</h1>
                            <SimpleGauge id="2cha"/>
                        </div>
                        <div class="">
                            <h1 class="font-rubik text-content-1"> Second Gauge</h1>
                            <SimpleGauge id="3cha"/>
                        </div>
                </section>
            </div>
        </main>
    }
}

#[component]
pub fn SimpleGauge(id: &'static str) -> impl IntoView {
    create_effect(move |_| {
        let chart = Chart::new().series(charming::series::Gauge::new().data(vec![DataPoint::Item(
                        DataPointItem::new(CompositeValue::Number(
                            charming::datatype::NumericValue::Integer(20 as i64),
                        ))
                        .name("Test".to_string())
                        .item_style(ItemStyle::new().color(Color::Value("white".to_string()))),
                    )]));

        WasmRenderer::new(250, 250)
            .render(id, &chart)
            .expect("a chart");
    });

    view! {
        <div id=id></div>
    }
}
