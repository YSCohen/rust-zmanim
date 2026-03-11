use jiff::{civil, tz::TimeZone};
use rust_zmanim::prelude::*;

fn main() {
    let date = civil::date(2025, 7, 29);

    // your location here
    let beit_meir = GeoLocation {
        latitude: 31.7975,
        longitude: 35.0345,
        elevation: 526.0,
        timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
    };

    // the zmanim_calculator provides some basic building blocks of zmanim
    if let Some(sunrise) = zmanim_calculator::hanetz(&date, &beit_meir, false) {
        assert_eq!(
            sunrise.strftime("%Y-%m-%d %H:%M:%S.%f %Z").to_string(),
            "2025-07-29 05:53:39.574572512 IDT"
        );
    }

    // zmanim_calculator also lets you make any custom tzeis, alos, etc using
    // ZmanOffset. these may be based on degrees, fixed time, or shaos zmaniyos
    if let Some(tzeis_degrees) =
        zmanim_calculator::tzeis(&date, &beit_meir, false, &ZmanOffset::Degrees(6.13))
    {
        assert_eq!(
            tzeis_degrees
                .strftime("%Y-%m-%d %H:%M:%S.%f %Z")
                .to_string(),
            "2025-07-29 20:06:02.501735285 IDT"
        );
    }

    // there is also a ComplexZmanimCalendar struct which stores the date and
    // location, convenient for getting many zmanim for the same point in 4D space.
    // It also has many common zmanim pre-made
    let czc = ComplexZmanimCalendar {
        geo_location: beit_meir,
        date,
        use_elevation: UseElevation::No,
    };

    if let Some(alos120) = czc.alos_120_minutes() {
        assert_eq!(
            alos120.strftime("%Y-%m-%d %H:%M:%S.%f %Z").to_string(),
            "2025-07-29 03:53:39.574572512 IDT"
        );
    }

    if let Some(sz18) = czc.shaah_zmanis_mga_18_degrees() {
        // 01:24:14.106060472
        assert_eq!(sz18.as_nanos(), 5_054_106_060_472);
    }

    // the calculations will return None if the specified solar event will not
    // occur
    let north_pole = GeoLocation {
        latitude: 90.0,
        longitude: 0.0,
        elevation: 0.0,
        timezone: TimeZone::UTC,
    };
    let polar_sunset = zmanim_calculator::shkia(&date, &north_pole, false);
    assert!(polar_sunset.is_none());
}
