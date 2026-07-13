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

// TODO: replce this whole system in tests with a single czc that changes loc
pub(crate) fn more_locations_czcs(use_elevation: bool) -> [ComplexZmanimCalendar; 9] {
    let elev = if use_elevation {
        UseElevation::All
    } else {
        UseElevation::No
    };

    more_locations().map(|loc| ComplexZmanimCalendar::new(loc, civil::date(2017, 10, 17), elev))
}
