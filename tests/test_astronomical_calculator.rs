//! All expected times were taken from python-zmanim
//! test_astronomical_calendar.py, with added precision (= hopefully added
//! accuracy)

use std::iter::zip;

use rust_zmanim::astronomical_calculator::*;
mod test_helper;
use jiff::civil;

#[test]
fn test_local_mean_time() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 03:56:57.605 EDT",
        "2017-10-17 03:39:03.887 IDT",
        "2017-10-17 03:54:46.058 PDT",
        "2017-10-17 02:41:26.316 JST",
        "2017-10-17 03:19:10.700 EDT",
        "2017-10-17 04:27:12.761 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = civil::date(2017, 10, 17);
        let result = match local_mean_time(&date, &loc, 3.0) {
            Some(dt) => dt.strftime("%Y-%m-%d %H:%M:%S.%3f %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_local_mean_time_invalid_hours() {
    let locations = test_helper::basic_locations();

    for loc in locations {
        let date = civil::date(2017, 10, 17);

        // Test out of range hours
        assert!(local_mean_time(&date, &loc, -0.1).is_none());
        assert!(local_mean_time(&date, &loc, 24.0).is_none());
        assert!(local_mean_time(&date, &loc, 25.0).is_none());
    }
}
