// Clean up the code, have 0 warnings when you do Space+D & Space+s
use crate::components::charts::gauge_search::SearchBar as TimezoneSearch;
use crate::components::charts::gauge_search::__Tz_Items;
use crate::wrappers::web::storage_theme;
use charming::component::Toolbox;
use charming::datatype::CompositeValue;
use charming::datatype::DataPoint;
use charming::datatype::DataPointItem;
use charming::element::Anchor;
use charming::element::AxisLabel;
use charming::element::AxisLine;
use charming::element::AxisPointer;
use charming::element::ColorBy;
use charming::element::Formatter;
use charming::element::ItemStyle;
use charming::element::LinkTarget;
use charming::element::TextAlign;
use charming::element::TextStyle;
use charming::element::TriggerOn;
use charming::series::Align;
use charming::{
    component::{Axis, DataZoom, Title},
    datatype::Dataset,
    element::{AxisType, Color, Icon, Padding, Pointer, SplitLine, Tooltip, Trigger},
    series::{Gauge, GaugeDetail, GaugeProgress, GaugeTitle, Line},
    Chart, WasmRenderer,
};
use chrono::prelude::*;
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::{OffsetComponents, OffsetName, Tz, TZ_VARIANTS, *};
use leptos::{
    html::{Input, Select},
    leptos_dom::logging::console_log,
    *,
};
use leptos_icons::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use std::{str::FromStr, sync::Arc};
use wasm_bindgen::JsCast;
use web_sys::{
    Event, HtmlElement, HtmlHeadingElement, HtmlInputElement, KeyboardEvent, MouseEvent, Node,
};

// Have a single effect, that updates your renderer based on a state,
// and have your interval update the state. That's it.
// Effects are a way of saying "when something (state) changes, do this".
// The part of something changing can be anywhere - HTTP request, timer, etc.
// As long as the state changes, the effect will run again
#[component]
pub fn Guage() -> impl IntoView {
    let hour = create_rw_signal(Local::now().hour12().1);
    let minute = create_rw_signal(Local::now().minute());
    let second = create_rw_signal(Local::now().second());

    //
    let local_time = create_rw_signal(Local::now().format("%d/%m/%Y %r %Z").to_string());
    let input_location = create_rw_signal("".to_string());
    let input_time = create_rw_signal("".to_string());
    let input_time_middleman = create_rw_signal([
        hour.get_untracked(),
        minute.get_untracked(),
        second.get_untracked(),
    ]);
    let input_time_displ = create_rw_signal("".to_string());

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

    create_effect(move |_| match storage_theme().as_str() {
            // Better version w/ css variables: reactive to theme for each reduce repeteition
            // let h = Clock::default().hour(
            //     hour,
            //     "--color-primary".to_string(),
            //     "--color-secondary".to_string(),
            //     "--color-highlight".to_string(),
            // );
        "light" => {
            let h = Clock::default().hour(
                hour,
                "Hour Hand".to_string(),
                "dodgerblue".to_string(),
                "black".to_string(),
            );
            let m = Clock::default().minute(
                minute,
                "Minute Hand".to_string(),
                "orchid".to_string(),
                "black".to_string(),
            );
            let s = Clock::default().second(
                second,
                "Second Hand".to_string(),
                "salmon".to_string(),
                "black".to_string(),
            );

            Clock::default().paint((500, 500), "hour", h);
            Clock::default().paint((500, 500), "minute", m);
            Clock::default().paint((500, 500), "second", s);
        }
        "dark" => {
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

            Clock::default().paint((500, 500), "hour", h);
            Clock::default().paint((500, 500), "minute", m);
            Clock::default().paint((500, 500), "second", s);
        }
        _ => {}
    });

    view! {
        <div class="p-4">
            <p> Your Local Time <em class="px-2">{local_time}</em></p>
        </div>
        // <div class="desktop:flex laptop:flex tablet:flex py-2 flex-col">
        <div class="flex flex-col desktop:flex-row laptop:flex-row ">
            <div id="hour"></div>
            <div id="minute"></div>
            <div id="second"></div>
        </div>
        <div class="text-3xl p-4 bg-slate-200 dark:bg-slate-900">
            <p>Chosen Location: <em class="px-2">{input_location}</em></p>
            <p>Time there is : <em class="px-2">{input_time_displ}</em></p>
        </div>
        <div class="p-4">
            <TimezoneSearch input=input_location input_el2=input_time input_el3=input_time_displ input_el4=input_time_middleman/>
        </div>
    }
}

pub fn alter_timezone(tz: &chrono_tz::Tz, clock: [RwSignal<u32>; 3]) {
    let time = Utc::now().with_timezone(tz);
    clock[0].set(time.hour12().1);
    clock[1].set(time.minute());
    clock[2].set(time.second());
}

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
            .title(
                Title::new()
                    .show(true)
                    .text_align(TextAlign::Center)
                    .left(CompositeValue::Number(charming::datatype::NumericValue::Integer(225)))
                    .shadow_blur(2.5)
                    .shadow_offset_x(2.5)
                    .shadow_offset_y(2.5)
                    .text(&label)
                    .text_style(TextStyle::new().color(Color::Value(text_color.clone())))
                    .background_color(Color::Value("inherit".to_string()))
                    .shadow_color(Color::Value("revert".to_string()))
                    .border_radius(5.0)
                    // may cause problems?
                    .target(LinkTarget::Blank),
            )
            .series(
                charming::series::Gauge::new()
                    // .radius("75%")
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
