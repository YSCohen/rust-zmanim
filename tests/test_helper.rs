// Each `tests/*.rs` file compiles as a separate crate, so any helper not used
// by *every* test crate would otherwise warn as dead code.
#![allow(dead_code)]

use jiff::{civil, tz::TimeZone};
use rust_zmanim::{complex_zmanim_calendar::*, util::geolocation::GeoLocation};

fn tz(name: &str) -> TimeZone {
    TimeZone::get(name).unwrap()
}

fn lakewood() -> GeoLocation {
    GeoLocation::new(40.0721087, -74.2400243, 15.0, tz("America/New_York")).unwrap()
}

fn jerusalem() -> GeoLocation {
    GeoLocation::new(31.7781161, 35.233804, 740.0, tz("Asia/Jerusalem")).unwrap()
}

fn los_angeles() -> GeoLocation {
    GeoLocation::new(34.0201613, -118.6919095, 71.0, tz("America/Los_Angeles")).unwrap()
}

fn tokyo() -> GeoLocation {
    GeoLocation::new(35.6733227, 139.6403486, 40.0, tz("Asia/Tokyo")).unwrap()
}

fn arctic_nunavut() -> GeoLocation {
    GeoLocation::new(81.7449398, -64.7945858, 127.0, tz("America/Toronto")).unwrap()
}

fn samoa() -> GeoLocation {
    GeoLocation::new(-13.8599098, -171.8031745, 1858.0, tz("Pacific/Apia")).unwrap()
}

fn fiji() -> GeoLocation {
    GeoLocation::new(-17.633056, 178.016667, 1324.0, tz("Pacific/Fiji")).unwrap()
}

fn honolulu() -> GeoLocation {
    GeoLocation::new(21.466667, -157.966667, 10.0, tz("America/Adak")).unwrap()
}

fn niue() -> GeoLocation {
    GeoLocation::new(-19.053006, -169.859199, 75.0, tz("Pacific/Niue")).unwrap()
}

pub(crate) fn basic_locations() -> [GeoLocation; 6] {
    [
        lakewood(),
        jerusalem(),
        los_angeles(),
        tokyo(),
        arctic_nunavut(),
        samoa(),
    ]
}

pub(crate) fn more_locations() -> [GeoLocation; 9] {
    [
        lakewood(),
        jerusalem(),
        los_angeles(),
        tokyo(),
        arctic_nunavut(),
        samoa(),
        fiji(),
        honolulu(),
        niue(),
    ]
}

/// Labels for [`more_locations`], in the same order, for assert messages.
pub(crate) fn location_labels() -> [&'static str; 9] {
    ["LW", "JM", "LA", "TK", "AN", "SM", "FJ", "HU", "NI"]
}

/// The dates exercised by every generated test, in the same order as
/// `Helpers.SAMPLE_DATES` in the test generator.
pub(crate) fn sample_dates() -> [civil::Date; 7] {
    [
        civil::date(2024, 2, 29),  // leap day
        civil::date(2025, 10, 17), // autumn mid-season
        civil::date(2025, 11, 2),  // US DST fall-back
        civil::date(2025, 12, 21), // winter solstice
        civil::date(2026, 3, 8),   // US DST spring-forward
        civil::date(2026, 3, 20),  // spring equinox
        civil::date(2026, 6, 21),  // summer solstice
    ]
}

/// A single calendar to be re-pointed at each location/date under test via
/// `set_geo_location` / `set_date`; the initial values are arbitrary.
pub(crate) fn single_czc(use_elevation: bool) -> ComplexZmanimCalendar {
    let elev = if use_elevation {
        UseElevation::All
    } else {
        UseElevation::No
    };

    ComplexZmanimCalendar::new(lakewood(), sample_dates()[0], elev)
}
