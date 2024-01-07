use icu_calendar::{chinese::*, gregorian::*, indian::*, islamic::*, iso::*};
use leptos::{html::*, *};

/// A symphony of calendars
///
/// spectator : The <select> tag that "watches" the decides the operation
/// bridge : The text that the user will see
/// statics_date : The text that shows the user, their conversion in gregorian
///
/// vy : the year
/// vm : the month
/// vd : the day
///
pub fn date_symphony_v2(
    spectator: NodeRef<Select>,
    bridge: RwSignal<String>,
    statics_date: RwSignal<String>,
    vy: NodeRef<Input>,
    vm: NodeRef<Input>,
    vd: NodeRef<Input>,
) {
    // I think this can be simplified to be easier
    // you only need to set the bridge and statics date once
    // you could create small functions to convert the calendar
    // and just call that instead of this tedious operation
    let vy = vy.get().expect("<select> to exist").value();
    let vm = vm.get().expect("<select> to exist").value();
    let vd = vd.get().expect("<select> to exist").value();
    let spectator_name = spectator.get().unwrap().value();

    match spectator.get().unwrap().value().as_str() {
        "Gregorian" => {
            let icu = icu_calendar::Date::try_new_gregorian_date(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
            )
            .unwrap()
            .to_calendar(Gregorian::default());

            bridge.set(format!(
                "{:?}: {:?}/{:?}/{:?}",
                spectator_name,
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));

            let icu = icu_calendar::Date::try_new_gregorian_date(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
            )
            .unwrap()
            .to_calendar(Gregorian::default());

            statics_date.set(format!(
                "{:?}/{:?}/{:?}",
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));
        }
        "Chinese" => {
            let icu = icu_calendar::Date::try_new_chinese_date_with_calendar(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
                Chinese::default(),
            )
            .unwrap();

            bridge.set(format!(
                "{:?}, {:?}: {:?}/{:?}/{:?}",
                spectator_name,
                icu.month().code.to_string(),
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));

            let icu = icu_calendar::Date::try_new_chinese_date_with_calendar(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
                Chinese::default(),
            )
            .unwrap()
            .to_calendar(Gregorian::default());

            statics_date.set(format!(
                "{:?}/{:?}/{:?}",
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));
        }
        "Iso" => {
            let icu = icu_calendar::Date::try_new_iso_date(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
            )
            .unwrap()
            .to_calendar(Iso);

            bridge.set(format!(
                "{:?}: {:?}/{:?}/{:?}",
                spectator_name,
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));

            let icu = icu_calendar::Date::try_new_iso_date(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
            )
            .unwrap()
            .to_calendar(Gregorian::default());

            statics_date.set(format!(
                "{:?}/{:?}/{:?}",
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));
        }
        "IslamicCivil" => {
            let icu = icu_calendar::Date::try_new_islamic_civil_date_with_calendar(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
                IslamicCivil::new_always_calculating(),
            )
            .unwrap();

            bridge.set(format!(
                "{:?}: {:?}/{:?}/{:?}",
                spectator_name,
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));
            let icu = icu_calendar::Date::try_new_islamic_civil_date_with_calendar(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
                IslamicCivil::new_always_calculating(),
            )
            .unwrap()
            .to_calendar(Gregorian::default());

            statics_date.set(format!(
                "{:?}/{:?}/{:?}",
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));
        }
        "Indian" => {
            let icu = icu_calendar::Date::try_new_indian_date(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
            )
            .unwrap()
            .to_calendar(Indian::default());

            bridge.set(format!(
                "{:?}: {:?}/{:?}/{:?}",
                spectator_name,
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));

            let icu = icu_calendar::Date::try_new_indian_date(
                vy.parse::<i32>().unwrap(),
                vm.parse::<u8>().unwrap(),
                vd.parse::<u8>().unwrap(),
            )
            .unwrap()
            .to_calendar(Gregorian);

            statics_date.set(format!(
                "{:?}/{:?}/{:?}",
                icu.year().number,
                icu.month().ordinal,
                icu.day_of_month().0
            ));
        }
        _ => {
            bridge.set(format!("Date Not Implemented"));
        }
    };
    // console_log(&format!("Chosen Calendar: {:?}", spectator_name.as_str()));
}
