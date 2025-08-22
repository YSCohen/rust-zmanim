//! All expected times were taken from python-zmanim
//! test_astronomical_calendar.py, with added precision (= hopefully added
//! accuracy)

use std::iter::zip;

use chrono::TimeZone;
use rust_zmanim::astronomical_calculator::*;
mod test_helper;

#[test]
fn test_utc_sunrise() {
    let locations = test_helper::basic_locations();
    let expected_times = [
        Some(11.153270653847327),
        Some(3.65893933754938),
        Some(14.007081524683),
        Some(20.805701197448574),
        None,
        Some(16.905106884133506),
    ];

    for (loc, etime) in zip(locations, expected_times) {
        let date = loc
            .timezone
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap();
        assert_eq!(utc_sunrise(&date, 90.0, &loc), etime)
    }
}

#[test]
fn test_utc_sunset() {
    let locations = test_helper::basic_locations();
    let expected_times = [
        Some(22.24410902650522),
        Some(15.14635335602205),
        Some(1.3181997867511512),
        Some(8.07962870894405),
        None,
        Some(5.518735324395347),
    ];

    for (loc, etime) in zip(locations, expected_times) {
        let date = loc
            .timezone
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap();
        assert_eq!(utc_sunset(&date, 90.0, &loc), etime)
    }
}

#[test]
fn test_utc_sea_level_sunrise() {
    let locations = test_helper::basic_locations();
    let expected_times = [
        Some(11.16434722717731),
        Some(3.72862262379616),
        Some(14.029265176318292),
        Some(20.822684611510432),
        None,
        Some(17.001584105223422),
    ];

    for (loc, etime) in zip(locations, expected_times) {
        let date = loc
            .timezone
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap();
        assert_eq!(utc_sea_level_sunrise(&date, 90.0, &loc), etime)
    }
}

#[test]
fn test_utc_sea_level_sunset() {
    let locations = test_helper::basic_locations();
    let expected_times = [
        Some(22.233043012013848),
        Some(15.076714287212347),
        Some(1.2960317445001053),
        Some(8.06265870940885),
        None,
        Some(5.422149175301648),
    ];

    for (loc, etime) in zip(locations, expected_times) {
        let date = loc
            .timezone
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap();
        assert_eq!(utc_sea_level_sunset(&date, 90.0, &loc), etime)
    }
}

#[test]
fn test_sunrise() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 07:09:11.774354 EDT",
        "2017-10-17 06:39:32.181615 IDT",
        "2017-10-17 07:00:25.493489 PDT",
        "2017-10-17 05:48:20.524311 JST",
        "None",
        "2017-10-17 06:54:18.384783 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = loc
            .timezone
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap();
        let result = match sunrise(&date, &loc) {
            Some(dt) => dt.to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sunset() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 18:14:38.792495 EDT",
        "2017-10-17 18:08:46.872082 IDT",
        "2017-10-17 18:19:05.519232 PDT",
        "2017-10-17 17:04:46.663352 JST",
        "None",
        "2017-10-17 19:31:07.447168 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = loc
            .timezone
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap();
        let result = match sunset(&date, &loc) {
            Some(dt) => dt.to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sea_level_sunrise() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 07:09:51.650018 EDT",
        "2017-10-17 06:43:43.041446 IDT",
        "2017-10-17 07:01:45.354635 PDT",
        "2017-10-17 05:49:21.664601 JST",
        "None",
        "2017-10-17 07:00:05.702779 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = loc
            .timezone
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap();
        let result = match sea_level_sunrise(&date, &loc) {
            Some(dt) => dt.to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sea_level_sunset() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 18:13:58.954843 EDT",
        "2017-10-17 18:04:36.171434 IDT",
        "2017-10-17 18:17:45.714280 PDT",
        "2017-10-17 17:03:45.571354 JST",
        "None",
        "2017-10-17 19:25:19.737031 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = loc
            .timezone
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap();
        let result = match sea_level_sunset(&date, &loc) {
            Some(dt) => dt.to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sunrise_offset_by_degrees() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 06:10:57.242475 EDT",
        "2017-10-17 05:50:43.409194 IDT",
        "2017-10-17 06:07:22.827474 PDT",
        "2017-10-17 04:53:55.060779 JST",
        "2017-10-17 04:47:28.032047 EDT",
        "2017-10-17 06:13:13.549285 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = loc
            .timezone
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap();
        let result = match sunrise_offset_by_degrees(&date, &loc, 102.0) {
            Some(dt) => dt.to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}

#[test]
fn test_sunset_offset_by_degrees() {
    let locations = test_helper::basic_locations();
    let expected_datetime_strs = [
        "2017-10-17 19:12:49.151362 EDT",
        "2017-10-17 18:57:33.344454 IDT",
        "2017-10-17 19:12:05.406326 PDT",
        "2017-10-17 17:59:08.923403 JST",
        "2017-10-17 19:15:04.565184 EDT",
        "2017-10-17 20:12:15.536080 +14",
    ];

    for (loc, edt) in zip(locations, expected_datetime_strs) {
        let date = loc
            .timezone
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap();
        let result = match sunset_offset_by_degrees(&date, &loc, 102.0) {
            Some(dt) => dt.to_string(),
            None => String::from("None"),
        };
        assert_eq!(result, edt)
    }
}
