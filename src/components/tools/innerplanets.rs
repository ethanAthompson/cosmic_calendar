pub mod earth;
pub mod mars;
pub mod mercury;
pub mod venus;

use chrono::*;
use leptos::{html::Input, leptos_dom::logging::console_log, *};
use leptos_icons::*;

use crate::wrappers::date::DATEFORMAT;

/// Mars Date
///
/// Example M220/13/25T748.600 or M216/17/08T123.456+3
///
/// {mir} = Mir as an integer (negative, 0, or positive), with no thousands separators.
/// {month} = Month as a zero-padded 2-digit integer in the range 01–24.
/// {sol} = Sol of the month as a zero-padded 2-digit integer in the range 01–28.
/// {mil} = Millisols as a zero-padded 3-digit integer in the range 000–999.
/// {mic} = Microsols as a zero-padded 3-digit integer in the range 000–999.
/// {offset} = Number of decisols offset for the timezone.
///
#[derive(Clone, PartialEq, Debug, Default)]
pub struct MarsDate {
    mir: i32,   // Year
    month: i32, // Month
    sol: i32,   // Day
    hour: i32,
    minute: i32,
    second: i32,
    timezone: String,
}

trait Gregorian {
    fn with_sol(day: i32) -> f32;
}

/// Mars Date w/ Gregorian
impl Gregorian for MarsDate {
    /// if you enter a day then it should convert it to sol visa-versa
    /// https://marscalendar.com/time-of-the-sol
    fn with_sol(day: i32) -> f32 {

        // dsol is the hours in earth time
        let dsol = 2.5;

        // sol is a day in mars
        let sol = dsol * 10.;

        // 10 sol = earth days in hours 250.0 : 10.41 days
        console_log(&format!("10 sol = earth days in hours {:?}", sol * 10.));
        let gregorian_days = 0.;
        
        console_log(&gregorian_days.to_string());

        // I add 4 because it seems to be how you convert greg to sol properly
        return gregorian_days;
    }
}

/// Mars Timezones
///
/// MTC-5	-180°	-162°	Amazonis Time (AMT)
/// MTC-4	-162°	-126°	Olympus Time (OT)
/// MTC-3	-126°	-90°	Tharsis Time (TT)
/// MTC-2	-90°	-54°	Marineris Time (MT)
/// MTC-1	-54°	-18°	Argyre Time (AGT)
/// MTC	-18°	18°	Noachis Time (NT)
/// MTC+1	18°	54°	Arabia Time (ABT)
/// MTC+2	54°	90°	Hellas Time (HT)
/// MTC+3	90°	126°	Utopia Time (UT)
/// MTC+4	126°	162°	Elysium Time (ET)
/// MTC+5	162°	180°	Arcadia Time (ACT)
///
#[derive(Clone, PartialEq, Debug)]
pub struct MarsTimeZones {}

/// Earth Date
///
/// So expressed in Earth time,
/// on Mars each solar hour is 1 hour 1 minute 39 seconds (1/24 of a sol) long,
/// each solar minute is 61.65 seconds long,
/// and each solar second is 1.0275 seconds long.
///
/// [https://unitconverter.io/seconds/hours/3699]
/// Year => 8979 hours
/// Hour => 1h 1m 39s or 3699 seconds
/// Minute => 61.65s or 61 minutes and 38 seconds
/// second => 1.0275s
#[derive(Clone, PartialEq, Debug)]
pub struct EarthDate {
    year: i32,
    month: i32,
    day: i32,
    timezone: String,
}

/// a simple function to test proper calculation
/// a better way is to calculate from a .ron file
/// a better way would be to make this a method of earthDate and Marsdate
/// returns a mars date
pub fn earth_date_to_mars_date(date: EarthDate) -> MarsDate {
    let earth_date = EarthDate {
        year: date.year,
        month: date.month,
        day: date.day,
        timezone: date.timezone,
    };

    // generates the sol for the 8th day in mars
    let sol = MarsDate::with_sol(date.day);

    MarsDate {
        mir: 0,
        month: 0,
        sol: sol as i32,
        hour: 0,
        minute: 0,
        second: 0,
        timezone: String::from(""),
    }
}

pub fn mars_date_to_earth_date(date: MarsDate) -> EarthDate {
    let mars_date = MarsDate {
        mir: date.mir,
        month: date.month,
        sol: date.sol,
        hour: date.hour,
        minute: date.minute,
        second: date.second,
        timezone: date.timezone,
    };

    // generates sol to earth
    // let sol = MarsDate::with_sol(date.sol);

    EarthDate {
        year: 0,
        month: 0,
        // day: sol as i32,
        day: 0,
        timezone: String::from(""),
    }
}

#[component]
pub fn TestingApp() -> impl IntoView {
    let mars = create_rw_signal(String::new());

    mars.update(|date| {
        let edata = earth_date_to_mars_date(EarthDate {
            year: 2023,
            month: 9,
            day: 2,
            timezone: String::from("EST"),
        });

        let mdata = mars_date_to_earth_date(MarsDate {
            mir: 220,
            month: 23,
            sol: 4,
            hour: 0,
            minute: 0,
            second: 0,
            timezone: String::from("AMT"),
        });

        let eformat = format!(
            "M{:?}/{:?}/{:?} ({:?}/{:?}/{:?}) {:?}",
            edata.mir,
            edata.month,
            edata.sol,
            edata.hour,
            edata.minute,
            edata.second,
            edata.timezone
        );

        let mformat = format!(
            "{:?}/{:?}/{:?} {:?}",
            mdata.year, mdata.month, mdata.day, mdata.timezone
        );

        *date = eformat;
    });

    view! {
        <div class="flex flex-col">
            {mars}
        </div>
    }
}
