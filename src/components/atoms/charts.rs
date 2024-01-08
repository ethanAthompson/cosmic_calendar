//
use crate::components::molecules::navbars::Component as Navbar;
use crate::components::molecules::sidebars::Component as DashboardSidebar;

use crate::interfaces::ceramics::traditional::{
    CalendarIcon, DashboardLeftIcon, DashboardRightIcon, ImStackIcon, InfoIcon, MenuGridIcon,
    SettingIcon, TimeFiveIcon,
};
use charming::{component::*, datatype::*, element::*, series::*, *};
use chrono::prelude::*;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::{OffsetComponents, OffsetName, Tz, TZ_VARIANTS, *};
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

pub fn update_gauge_timer(
    input_time: RwSignal<String>,
    input_time_middleman: RwSignal<[u32; 3]>,
    local_time: RwSignal<String>,
    hour: RwSignal<u32>,
    minute: RwSignal<u32>,
    second: RwSignal<u32>,
) {
    set_interval(
        move || {
            hour.update(|v| {
                if input_time.get().is_empty() {
                    *v = Local::now().hour12().1;
                } else {
                    *v = input_time_middleman.get()[0];
                }
            });

            minute.update(|v| {
                if input_time.get().is_empty() {
                    *v = Local::now().minute();
                } else {
                    *v = input_time_middleman.get()[1];
                }
            });

            second.update(|v| {
                if input_time.get().is_empty() {
                    *v = Local::now().second();
                } else {
                    *v = input_time_middleman.get()[2];
                }
            });

            local_time.update(|t| *t = Local::now().format("%d/%m/%Y %r %Z").to_string());
        },
        std::time::Duration::from_secs(1),
    );
}

// For custom theming -> https://leptos-use.rs/browser/use_css_var.html
pub fn spawn_clocks_with_theme(hour: RwSignal<u32>, minute: RwSignal<u32>, second: RwSignal<u32>) {
    // create_effect(move |_| match window().local_stroage().unwrap().unwrap(),get_item("theme").unwrap().as_str() {
    create_effect(move |_| {
        let h = Clock::default().hour(
            hour,
            "Hour Hand".to_string(),
            "cornflowerblue".to_string(),
            "white".to_string(),
        );
        let m = Clock::default().minute(
            minute,
            "Minute Hand".to_string(),
            "darkorchid".to_string(),
            "white".to_string(),
        );
        let s = Clock::default().second(
            second,
            "Second Hand".to_string(),
            "darksalmon".to_string(),
            "white".to_string(),
        );

        Clock::default().paint((250, 250), "hour", h);
        Clock::default().paint((250, 250), "minute", m);
        Clock::default().paint((250, 250), "second", s);
    });
}

// move to charts/gauge/clock
struct Clock {}

impl Default for Clock {
    fn default() -> Self {
        Self {}
    }
}

trait Hands {
    fn paint(self, size: (u32, u32), id: &str, chart: Chart) -> ();
    fn make_hand(
        self,
        max: u32,
        data: RwSignal<u32>,
        label: String,
        color: String,
        text_color: String,
    ) -> Chart;
    fn hour(self, data: RwSignal<u32>, label: String, color: String, text_color: String) -> Chart;
    fn minute(self, data: RwSignal<u32>, label: String, color: String, text_color: String)
        -> Chart;
    fn second(self, data: RwSignal<u32>, label: String, color: String, text_color: String)
        -> Chart;
}

impl Hands for Clock {
    fn paint(self, size: (u32, u32), id: &str, chart: Chart) {
        WasmRenderer::new(size.0, size.1)
            .render(id, &chart)
            .expect("Chart to paint");
    }

    fn make_hand(
        self,
        max: u32,
        data: RwSignal<u32>,
        label: String,
        color: String,
        text_color: String,
    ) -> Chart {
        Chart::new()
            .background_color(Color::Value("transparent".to_string()))
            .tooltip(Tooltip::new().formatter("Counting: {c}"))
            // .title(
            //     Title::new()
            //         .show(true)
            //         .text_align(TextAlign::Center)
            //         .left(CompositeValue::Number(
            //             charming::datatype::NumericValue::Integer(225),
            //         ))
            //         .shadow_blur(2.5)
            //         .shadow_offset_x(2.5)
            //         .shadow_offset_y(2.5)
            //         .text(&label)
            //         .text_style(TextStyle::new().color(Color::Value(text_color.clone())))
            //         .background_color(Color::Value("inherit".to_string()))
            //         .shadow_color(Color::Value("revert".to_string()))
            //         .border_radius(5.0)
            //         // may cause problems?
            //         .target(LinkTarget::Blank),
            // )
            .series(
                charming::series::Gauge::new()
                    // .radius("50%")
                    .anchor(
                        Anchor::new()
                            .show(true)
                            .size(14)
                            .item_style(ItemStyle::new().border_color("#000").border_width(2)),
                    )
                    .pointer(
                        Pointer::new()
                            .item_style(ItemStyle::new().color(Color::Value(text_color.clone()))),
                    )
                    .progress(
                        GaugeProgress::new()
                            .show(true)
                            .overlap(true)
                            .round_cap(true)
                            .item_style(ItemStyle::new().color(Color::Value(color.clone()))),
                    )
                    .color_by(ColorBy::Data)
                    .min(0)
                    .max(max)
                    .split_line(SplitLine::new().show(true).distance(2.0))
                    .split_number(12.0)
                    .axis_label(
                        AxisLabel::new()
                            .show(true)
                            .color(Color::Value(text_color.clone())),
                    )
                    // .title(GaugeTitle::new().show(true).offset_center(("0", "-50%")))
                    .detail(
                        GaugeDetail::new()
                            .value_animation(true)
                            // .precision(1.0)
                            .precision(0.0)
                            .show(true)
                            .formatter("{value}")
                            .color(Color::Value(color.clone())),
                    )
                    .data(vec![DataPoint::Item(
                        DataPointItem::new(CompositeValue::Number(
                            charming::datatype::NumericValue::Integer(data.get() as i64),
                        ))
                        // .name(&label)
                        .name(" ".to_string())
                        .item_style(ItemStyle::new().color(Color::Value("white".to_string()))),
                    )]),
            )
    }

    fn hour(self, data: RwSignal<u32>, label: String, color: String, text_color: String) -> Chart {
        self.make_hand(12, data, label, color, text_color)
    }

    fn minute(
        self,
        data: RwSignal<u32>,
        label: String,
        color: String,
        text_color: String,
    ) -> Chart {
        self.make_hand(60, data, label, color, text_color)
    }

    fn second(
        self,
        data: RwSignal<u32>,
        label: String,
        color: String,
        text_color: String,
    ) -> Chart {
        self.make_hand(60, data, label, color, text_color)
    }
}
