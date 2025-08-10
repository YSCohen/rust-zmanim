use rust_zmanim::prelude::*;

fn main() {
    // the time in the DateTime will be ignored in zmanim calculations
    let dt = chrono_tz::Asia::Jerusalem
        .with_ymd_and_hms(2025, 7, 29, 10, 30, 26)
        .unwrap();

    // your location here
    let beit_meir = GeoLocation {
        latitude: 31.7975,
        longitude: 35.0345,
        elevation: 526.0,
        timezone: chrono_tz::Asia::Jerusalem,
    };

    // the `zmanim_calculator` lets you make any custom tzais, alos, etc
    if let Some(tzais_pi_degrees) = zmanim_calculator::tzais(
        &dt,
        &beit_meir,
        false,
        &ZmanOffset::Degrees(std::f64::consts::PI),
    ) {
        assert_eq!(
            format!("{tzais_pi_degrees}"),
            "2025-07-29 19:50:30.090272 IDT"
        );
    }

    // there is also a `ComplexZmanimCalendar` struct which stores the date and
    // location, convenient for getting many zmanim for the same point in 4D space.
    // It also has many common zmanim pre-made
    let czc = ComplexZmanimCalendar {
        geo_location: beit_meir,
        date: dt,
        use_elevation: UseElevation::No,
    };

    if let Some(alos120) = czc.alos_120_minutes() {
        assert_eq!(format!("{alos120}"), "2025-07-29 03:53:39.574573 IDT");
    };

    if let Some(sz18) = czc.shaah_zmanis_18_degrees() {
        // 01:24:14.1060605 in minutes
        assert_eq!(sz18, 84.23510100833333);
    }

    // the calculations will return `None` if the specified solar event will not
    // occur
    let north_pole = GeoLocation {
        latitude: 90.0,
        longitude: 0.0,
        elevation: 0.0,
        timezone: chrono_tz::UTC,
    };
    let polar_sunset = zmanim_calculator::shkia(&dt, &north_pole, false);
    assert!(polar_sunset.is_none());
}
