use chrono_tz::{Asia::Jerusalem, UTC};
use rust_zmanim::prelude::*;

fn main() {
    // the time in the DateTime will be ignored
    let dt = Jerusalem.with_ymd_and_hms(2025, 7, 29, 10, 30, 26).unwrap();

    let beit_meir = GeoLocation {
        latitude: 31.7975,
        longitude: 35.0345,
        elevation: 526.0,
        timezone: Jerusalem,
    };

    // the `zmanim_calculator` lets you make any custom tzais/alos
    if let Some(tzais_baal_hatanya) =
        zmanim_calculator::tzais(&dt, &beit_meir, false, ZmanOffset::Degrees(6.0))
    {
        assert_eq!(
            format!("{tzais_baal_hatanya}"),
            "2025-07-29 20:05:21.587287 IDT"
        );
    }

    // there is also a `ComplexZmanimCalendar` for convenience
    let czc = ComplexZmanimCalendar {
        geo_location: beit_meir,
        date: dt,
        use_elevation: UseElevation::No,
    };

    if let Some(alos120) = czc.alos_120_minutes() {
        assert_eq!(format!("{alos120}"), "2025-07-29 03:53:39.574573 IDT");
    };

    if let Some(sz18) = czc.shaah_zmanis_18_degrees() {
        // 1 hour 24 minutes 14 seconds in millis
        assert_eq!(sz18, 5054106.0605);
    }

    // the calculations will return `None` if the specified solar event will not
    // occur
    let north_pole = GeoLocation {
        latitude: 90.0,
        longitude: 0.0,
        elevation: 0.0,
        timezone: UTC,
    };
    let polar_sunset = zmanim_calculator::shkia(&dt, &north_pole, false);
    assert!(polar_sunset.is_none());
}
