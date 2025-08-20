//! All expected times were taken from python-zmanim
//! test_astronomical_calendar.py, with added precision (= hopefully added
//! accuracy)

use chrono::TimeZone;
use rust_zmanim::astronomical_calculator::*;
mod test_helper;

#[test]
fn test_utc_sunrise() {
    let expected_times = [
        Some(11.153270653847327),
        Some(3.65893933754938),
        Some(14.007081524683),
        Some(20.805701197448574),
        None,
        Some(16.905106884133506),
    ];
    let places = test_helper::basic_locations();

    for i in 0..6 {
        let loc = &places[i];
        let date = loc
            .timezone
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap();
        assert_eq!(utc_sunrise(&date, 90.0, loc), expected_times[i])
    }
}

#[test]
fn test_utc_sunset() {
    let expected_times = [
        Some(22.24410902650522),
        Some(15.14635335602205),
        Some(1.3181997867511512),
        Some(8.07962870894405),
        None,
        Some(5.518735324395347),
    ];
    let places = test_helper::basic_locations();

    for i in 0..6 {
        let loc = &places[i];
        let date = loc
            .timezone
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap();
        assert_eq!(utc_sunset(&date, 90.0, loc), expected_times[i])
    }
}
