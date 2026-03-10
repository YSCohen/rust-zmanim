//! All expected times were taken from python-zmanim
//! test_astronomical_calendar.py, with added precision (= hopefully added
//! accuracy)

use std::iter::zip;

use rust_zmanim::astronomical_calculator::*;
mod test_helper;
use jiff::civil;

#[test]
fn test_sunrise() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 07:09:11.774 EDT",
        "2017-10-17 06:39:32.181 IDT",
        "2017-10-17 07:00:25.493 PDT",
        "2017-10-17 05:48:20.524 JST",
        "None",
        "2017-10-17 06:54:18.384 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = civil::date(2017, 10, 17);
        let result = match sunrise(&date, &loc) {
            Some(dt) => dt.strftime("%Y-%m-%d %H:%M:%S.%3f %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sunset() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 18:14:38.792 EDT",
        "2017-10-17 18:08:46.872 IDT",
        "2017-10-17 18:19:05.519 PDT",
        "2017-10-17 17:04:46.663 JST",
        "None",
        "2017-10-17 19:31:07.447 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = civil::date(2017, 10, 17);
        let result = match sunset(&date, &loc) {
            Some(dt) => dt.strftime("%Y-%m-%d %H:%M:%S.%3f %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sea_level_sunrise() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 07:09:51.650 EDT",
        "2017-10-17 06:43:43.041 IDT",
        "2017-10-17 07:01:45.354 PDT",
        "2017-10-17 05:49:21.664 JST",
        "None",
        "2017-10-17 07:00:05.702 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = civil::date(2017, 10, 17);
        let result = match sea_level_sunrise(&date, &loc) {
            Some(dt) => dt.strftime("%Y-%m-%d %H:%M:%S.%3f %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sea_level_sunset() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 18:13:58.954 EDT",
        "2017-10-17 18:04:36.171 IDT",
        "2017-10-17 18:17:45.714 PDT",
        "2017-10-17 17:03:45.571 JST",
        "None",
        "2017-10-17 19:25:19.737 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = civil::date(2017, 10, 17);
        let result = match sea_level_sunset(&date, &loc) {
            Some(dt) => dt.strftime("%Y-%m-%d %H:%M:%S.%3f %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sunrise_offset_by_degrees() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 06:10:57.242 EDT",
        "2017-10-17 05:50:43.409 IDT",
        "2017-10-17 06:07:22.827 PDT",
        "2017-10-17 04:53:55.060 JST",
        "2017-10-17 04:47:28.032 EDT",
        "2017-10-17 06:13:13.549 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = civil::date(2017, 10, 17);
        let result = match sunrise_offset_by_degrees(&date, &loc, 102.0) {
            Some(dt) => dt.strftime("%Y-%m-%d %H:%M:%S.%3f %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sunset_offset_by_degrees() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 19:12:49.151 EDT",
        "2017-10-17 18:57:33.344 IDT",
        "2017-10-17 19:12:05.406 PDT",
        "2017-10-17 17:59:08.923 JST",
        "2017-10-17 19:15:04.565 EDT",
        "2017-10-17 20:12:15.536 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = civil::date(2017, 10, 17);
        let result = match sunset_offset_by_degrees(&date, &loc, 102.0) {
            Some(dt) => dt.strftime("%Y-%m-%d %H:%M:%S.%3f %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

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
fn test_solar_noon() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 12:42:18.881 EDT",
        "2017-10-17 12:24:28.945 IDT",
        "2017-10-17 12:40:05.813 PDT",
        "2017-10-17 11:26:55.034 JST",
        "2017-10-17 12:04:32.300 EDT",
        "2017-10-17 13:12:43.198 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = civil::date(2017, 10, 17);
        let result = match solar_noon(&date, &loc) {
            Some(dt) => dt.strftime("%Y-%m-%d %H:%M:%S.%3f %Z").to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_solar_midnight() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-18 00:42:12.782 EDT",
        "2017-10-18 00:24:22.754 IDT",
        "2017-10-18 00:39:59.751 PDT",
        "2017-10-17 23:26:48.756 JST",
        "2017-10-18 00:04:26.193 EDT",
        "2017-10-18 01:12:36.881 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = civil::date(2017, 10, 17);
        let result = match solar_midnight(&date, &loc) {
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
