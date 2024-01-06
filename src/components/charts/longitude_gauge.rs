use std::{str::FromStr, sync::Arc};
use crate::components::charts::gauge_search::SearchBar as TimezoneSearch;
use crate::components::charts::gauge_search::__Tz_Items;
use charming::{
    component::{Axis, DataZoom, Title},
    datatype::Dataset,
    element::{AxisType, Color, Icon, Padding, Pointer, SplitLine, Tooltip, Trigger},
    series::{Gauge, GaugeDetail, GaugeProgress, GaugeTitle, Line},
    Chart, WasmRenderer,
};
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono::prelude::*;
use chrono_tz::{OffsetName, Tz, TZ_VARIANTS, OffsetComponents, *};
use leptos::{
    html::{Input, Select},
    leptos_dom::logging::console_log,
    *,
};
use leptos_icons::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
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
    // let select_el = NodeRef::<Select>::new();
    let input_el = NodeRef::<Input>::new();

    set_interval(
        move || hour.update(|v| if *v == 12 { *v = 0 } else { *v += 1 }),
        std::time::Duration::from_secs(3600),
    );

    set_interval(
        move || minute.update(|v| if *v == 60 { *v = 0 } else { *v += 1 }),
        std::time::Duration::from_secs(60),
    );

    set_interval(
        move || {
            second.update(|v| if *v == 60 { *v = 0 } else { *v += 1 });
            local_time.update(|t| *t = Local::now().format("%d/%m/%Y %r %Z").to_string());
            // input_time.update(|t| *t = pick_timezone(select_el.get().unwrap().value().as_str()));
        },
        std::time::Duration::from_secs(1),
    );

    create_effect(move |_| {
        let h = Clock::default().hour(hour, "Hour Hand".to_string());
        let m = Clock::default().minute(minute, "Minute Hand".to_string());
        let s = Clock::default().second(second, "Second Hand".to_string());

        Clock::default().paint((500, 500), "hour", h);
        Clock::default().paint((500, 500), "minute", m);
        Clock::default().paint((500, 500), "second", s);
    });

    let time_change = move |ev: Event| {
        let value = input_el.get().unwrap().value();

        console_log(&value.to_string());

        let timezone =
            chrono_tz::Tz::from_str(&value).expect("an earth timezone to make it through!");
        let datetime = DateTime::with_timezone(&Utc::now(), &timezone).format("%Y/%m/%d %r %Z");
        console_log(datetime.to_string().as_str());
    };

    view! {
        <p> Your Local Time <em class="px-2">{local_time}</em></p>
        <div class="flex space-x-2">
            <div id="hour"></div>
            <div id="minute"></div>
            <div id="second"></div>
        </div>
        <div>
            <p>Chosen Location: <em class="px-2">{input_location}</em></p>
            <p>Time there is : <em class="px-2">{input_time}</em></p>
        </div>
        <div class="p-4">
            <TimezoneSearch input=input_location/>
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
    fn make_hand(self, max: u32, data: RwSignal<u32>, label: String) -> Chart;
    fn hour(self, data: RwSignal<u32>, label: String) -> Chart;
    fn minute(self, data: RwSignal<u32>, label: String) -> Chart;
    fn second(self, data: RwSignal<u32>, label: String) -> Chart;
}

impl Hands for Clock {
    fn paint(self, size: (u32, u32), id: &str, chart: Chart) {
        WasmRenderer::new(size.0, size.1)
            .render(id, &chart)
            .expect("Chart to paint");
    }

    fn make_hand(self, max: u32, data: RwSignal<u32>, label: String) -> Chart {
        Chart::new().series(
            charming::series::Gauge::new()
                .progress(GaugeProgress::new().show(true))
                .min(0)
                .max(max)
                .detail(
                    GaugeDetail::new()
                        .formatter("{value}")
                        .color(Color::Value("white".to_string())),
                )
                .z(20.0)
                .data(vec![(data.get() as i32, label)]),
        )
    }

    fn hour(self, data: RwSignal<u32>, label: String) -> Chart {
        self.make_hand(12, data, label)
    }

    fn minute(self, data: RwSignal<u32>, label: String) -> Chart {
        self.make_hand(60, data, label)
    }

    fn second(self, data: RwSignal<u32>, label: String) -> Chart {
        self.make_hand(60, data, label)
    }
}
