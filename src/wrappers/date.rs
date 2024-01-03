use chrono::{format::format, Datelike, NaiveDate};
use icu_calendar::{
    any_calendar::AnyDateInner, buddhist::Buddhist, chinese::Chinese, coptic::Coptic, dangi::Dangi,
    ethiopian::Ethiopian, gregorian::GregorianDateInner, hebrew::Hebrew, indian::Indian,
    islamic::*, japanese::Japanese, julian::Julian, persian::Persian, roc::Roc, *,
};
use leptos::{
    html::Input, leptos_dom::logging::console_log, NodeRef, RwSignal, SignalGet, SignalSet,
    SignalUpdate,
};

pub struct ZoneCalendar;


/// This is just a basic representation
impl ZoneCalendar {
    pub fn set<T>(yymmdd: [NodeRef<Input>; 3], sig: RwSignal<String>, core: T)
    where
        T: AsCalendar,
    {
        // Converts input to NaiveDate which allows formating
        let year = yymmdd[0].get().unwrap().value().parse::<i32>().unwrap_or_default();
        let month = yymmdd[1].get().unwrap().value().parse::<u8>().unwrap_or_default();
        let day = yymmdd[2].get().unwrap().value().parse::<u8>().unwrap_or_default();

        // WARNING! Must be real date: Gregorian is default
        let icu = Date::try_new_gregorian_date(year, month, day).expect("A Date to Exist");
        // let icu = Date::try_new_gregorian_date(year, month, day).expect("A Date to Exist");

        // sets the appropiate calendar from core
        let calendar = icu.to_calendar(core);

        // *value = format!(
        let result = format!(
            "{:?}, {:?} {:?}, {:?} {:?}",
            calendar.day_of_month().0,
            calendar.month().code.0.to_string().as_str(),
            calendar.month().ordinal,
            calendar.year().number,
            calendar.year().cyclic.expect("A Cyclic to be present"),
        );

        console_log("HI");

        sig.set(result);
    }
}
