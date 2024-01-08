//
use crate::components::molecules::navbars::Component as Navbar;
use crate::components::molecules::sidebars::Component as DashboardSidebar;

use crate::interfaces::ceramics::traditional::{
    CalendarIcon, DashboardLeftIcon, DashboardRightIcon, ImStackIcon, InfoIcon, MenuGridIcon,
    SettingIcon, TimeFiveIcon,
};
use charming::{
    datatype::{CompositeValue, DataPoint, DataPointItem},
    element::{Color, ItemStyle},
    *,
};
use leptos::{html::Button, leptos_dom::logging::console_log, *};
use leptos_router::A;
use leptos_use::use_element_hover;

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
