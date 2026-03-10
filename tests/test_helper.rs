use jiff::{civil, tz::TimeZone};
use rust_zmanim::{complex_zmanim_calendar::*, util::geolocation::GeoLocation};

fn tz(name: &str) -> TimeZone {
    TimeZone::get(name).unwrap()
}

fn lakewood() -> GeoLocation {
    GeoLocation {
        latitude: 40.0721087,
        longitude: -74.2400243,
        timezone: tz("America/New_York"),
        elevation: 15.0,
    }
}

fn samoa() -> GeoLocation {
    GeoLocation {
        latitude: -13.8599098,
        longitude: -171.8031745,
        timezone: tz("Pacific/Apia"),
        elevation: 1858.0,
    }
}

fn jerusalem() -> GeoLocation {
    GeoLocation {
        latitude: 31.7781161,
        longitude: 35.233804,
        timezone: tz("Asia/Jerusalem"),
        elevation: 740.0,
    }
}

fn los_angeles() -> GeoLocation {
    GeoLocation {
        latitude: 34.0201613,
        longitude: -118.6919095,
        timezone: tz("America/Los_Angeles"),
        elevation: 71.0,
    }
}

fn tokyo() -> GeoLocation {
    GeoLocation {
        latitude: 35.6733227,
        longitude: 139.6403486,
        timezone: tz("Asia/Tokyo"),
        elevation: 40.0,
    }
}

fn arctic_nunavut() -> GeoLocation {
    GeoLocation {
        latitude: 81.7449398,
        longitude: -64.7945858,
        timezone: tz("America/Toronto"),
        elevation: 127.0,
    }
}

fn fiji() -> GeoLocation {
    GeoLocation {
        latitude: -17.633056,
        longitude: 178.016667,
        timezone: tz("Pacific/Fiji"),
        elevation: 1324.0,
    }
}

fn honolulu() -> GeoLocation {
    GeoLocation {
        latitude: 21.466667,
        longitude: -157.966667,
        timezone: tz("America/Adak"),
        elevation: 10.0,
    }
}

fn niue() -> GeoLocation {
    GeoLocation {
        latitude: -19.053006,
        longitude: -169.859199,
        timezone: tz("Pacific/Niue"),
        elevation: 75.0,
    }
}

#[allow(dead_code)]
pub fn basic_locations() -> [GeoLocation; 6] {
    [
        lakewood(),
        jerusalem(),
        los_angeles(),
        tokyo(),
        arctic_nunavut(),
        samoa(),
    ]
}

#[allow(dead_code)]
pub fn more_locations() -> [GeoLocation; 9] {
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

#[allow(dead_code)]
pub fn more_locations_czcs(use_elevation: bool) -> [ComplexZmanimCalendar; 9] {
    let elev = if use_elevation {
        UseElevation::All
    } else {
        UseElevation::No
    };

    more_locations().map(|loc| ComplexZmanimCalendar {
        geo_location: loc.clone(),
        date: civil::date(2017, 10, 17),
        use_elevation: elev,
    })
}
