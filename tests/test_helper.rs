use chrono_tz::{America, Asia, Pacific};
use rust_zmanim::util::geolocation::GeoLocation;

fn lakewood() -> GeoLocation {
    GeoLocation {
        latitude: 40.0721087,
        longitude: -74.2400243,
        timezone: America::New_York,
        elevation: 15.0,
    }
}

fn samoa() -> GeoLocation {
    GeoLocation {
        latitude: -13.8599098,
        longitude: -171.8031745,
        timezone: Pacific::Apia,
        elevation: 1858.0,
    }
}

fn jerusalem() -> GeoLocation {
    GeoLocation {
        latitude: 31.7781161,
        longitude: 35.233804,
        timezone: Asia::Jerusalem,
        elevation: 740.0,
    }
}

fn los_angeles() -> GeoLocation {
    GeoLocation {
        latitude: 34.0201613,
        longitude: -118.6919095,
        timezone: America::Los_Angeles,
        elevation: 71.0,
    }
}

fn tokyo() -> GeoLocation {
    GeoLocation {
        latitude: 35.6733227,
        longitude: 139.6403486,
        timezone: Asia::Tokyo,
        elevation: 40.0,
    }
}

fn arctic_nunavut() -> GeoLocation {
    GeoLocation {
        latitude: 81.7449398,
        longitude: -64.7945858,
        timezone: America::Toronto,
        elevation: 127.0,
    }
}

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
