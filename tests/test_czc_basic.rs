//! Basic tests for
//! [rust_zmanim::complex_zmanim_calendar::ComplexZmanimCalendar]

use chrono::TimeZone;
use chrono_tz::Asia::Jerusalem;
use rust_zmanim::{
    complex_zmanim_calendar::*, util::geolocation::GeoLocation, zmanim_calculator::ZmanOffset,
};

#[test]
fn test_fixed_local_chatzos() {
    let loc1 = GeoLocation {
        latitude: 31.79388,
        longitude: 35.03684,
        elevation: 586.19,
        timezone: Jerusalem,
    };
    let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
    let czc1 = ComplexZmanimCalendar {
        geo_location: loc1,
        date: date1,
        use_elevation: UseElevation::No,
    };
    let chatzos1 = czc1.fixed_local_chatzos().unwrap().to_string();
    assert_eq!(chatzos1, "2025-08-04 12:39:51.158400 IDT"); // off by < 1 ms

    let loc2 = GeoLocation {
        latitude: 31.0,
        longitude: 35.0,
        elevation: 586.19,
        timezone: Jerusalem,
    };
    let date2 = Jerusalem.with_ymd_and_hms(2025, 1, 4, 0, 0, 0).unwrap();
    let czc2 = ComplexZmanimCalendar {
        geo_location: loc2,
        date: date2,
        use_elevation: UseElevation::No,
    };
    let chatzos2 = czc2.fixed_local_chatzos().unwrap().to_string();
    assert_eq!(chatzos2, "2025-01-04 11:40:00 IST");
}

#[test]
fn test_alos_120_minutes() {
    let loc = GeoLocation {
        latitude: 31.79388,
        longitude: 35.03684,
        elevation: 586.19,
        timezone: Jerusalem,
    };

    let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
    let czc1 = ComplexZmanimCalendar {
        geo_location: loc.clone(),
        date: date1,
        use_elevation: UseElevation::No,
    };
    let alos1 = czc1.alos_120_minutes().unwrap().to_string();
    assert_eq!(alos1, "2025-08-04 03:57:34.359480109 IDT");

    let date2 = Jerusalem.with_ymd_and_hms(2025, 1, 26, 0, 0, 0).unwrap();
    let czc2 = ComplexZmanimCalendar {
        geo_location: loc.clone(),
        date: date2,
        use_elevation: UseElevation::No,
    };
    let alos2 = czc2.alos_120_minutes().unwrap().to_string();
    assert_eq!(alos2, "2025-01-26 04:36:28.393758421 IST");

    let date3 = Jerusalem.with_ymd_and_hms(2005, 5, 15, 0, 0, 0).unwrap();
    let czc3 = ComplexZmanimCalendar {
        geo_location: loc.clone(),
        date: date3,
        use_elevation: UseElevation::No,
    };
    let alos3 = czc3.alos_120_minutes().unwrap().to_string();
    assert_eq!(alos3, "2005-05-15 03:43:01.021454229 IDT"); // off by 200 ms (?!)

    let date4 = Jerusalem.with_ymd_and_hms(2025, 5, 15, 0, 0, 0).unwrap();
    let czc4 = ComplexZmanimCalendar {
        geo_location: loc,
        date: date4,
        use_elevation: UseElevation::No,
    };
    let alos4 = czc4.alos_120_minutes().unwrap().to_string();
    assert_eq!(alos4, "2025-05-15 03:42:56.506814904 IDT");
}

#[test]
fn test_tzais_72_minutes_zmanis() {
    let loc = GeoLocation {
        latitude: 31.79388,
        longitude: 35.03684,
        elevation: 586.19,
        timezone: Jerusalem,
    };

    let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
    let czc1 = ComplexZmanimCalendar {
        geo_location: loc.clone(),
        date: date1,
        use_elevation: UseElevation::No,
    };
    let alos1 = czc1.tzais_72_minutes_zmanis().unwrap().to_string();
    assert_eq!(alos1, "2025-08-04 20:55:33.614725027 IDT"); // off by < 1 ms

    let date2 = Jerusalem.with_ymd_and_hms(2025, 1, 26, 0, 0, 0).unwrap();
    let czc2 = ComplexZmanimCalendar {
        geo_location: loc.clone(),
        date: date2,
        use_elevation: UseElevation::No,
    };
    let alos2 = czc2.tzais_72_minutes_zmanis().unwrap().to_string();
    assert_eq!(alos2, "2025-01-26 18:11:54.813293573 IST"); // off by < 1 ms

    let date3 = Jerusalem.with_ymd_and_hms(2005, 5, 15, 0, 0, 0).unwrap();
    let czc3 = ComplexZmanimCalendar {
        geo_location: loc.clone(),
        date: date3,
        use_elevation: UseElevation::No,
    };
    let alos3 = czc3.tzais_72_minutes_zmanis().unwrap().to_string();
    assert_eq!(alos3, "2005-05-15 20:52:25.429886381 IDT");

    let date4 = Jerusalem.with_ymd_and_hms(2025, 5, 15, 0, 0, 0).unwrap();
    let czc4 = ComplexZmanimCalendar {
        geo_location: loc,
        date: date4,
        use_elevation: UseElevation::No,
    };
    let alos4 = czc4.tzais_72_minutes_zmanis().unwrap().to_string();
    assert_eq!(alos4, "2025-05-15 20:52:34.967135271 IDT"); // off by > 2 ms
}

#[test]
fn test_hanetz() {
    let loc = GeoLocation {
        latitude: 31.79388,
        longitude: 35.03684,
        elevation: 586.19,
        timezone: Jerusalem,
    };

    let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
    let czc1 = ComplexZmanimCalendar {
        geo_location: loc.clone(),
        date: date1,
        use_elevation: UseElevation::HanetzShkia,
    };
    let alos1 = czc1.hanetz().unwrap().to_string();
    assert_eq!(alos1, "2025-08-04 05:53:38.656214524 IDT");

    let date2 = Jerusalem.with_ymd_and_hms(2025, 1, 26, 0, 0, 0).unwrap();
    let czc2 = ComplexZmanimCalendar {
        geo_location: loc.clone(),
        date: date2,
        use_elevation: UseElevation::HanetzShkia,
    };
    let alos2 = czc2.hanetz().unwrap().to_string();
    assert_eq!(alos2, "2025-01-26 06:32:32.666982290 IST");

    let date3 = Jerusalem.with_ymd_and_hms(2005, 5, 15, 0, 0, 0).unwrap();
    let czc3 = ComplexZmanimCalendar {
        geo_location: loc,
        date: date3,
        use_elevation: UseElevation::HanetzShkia,
    };
    let alos3 = czc3.hanetz().unwrap().to_string();
    assert_eq!(alos3, "2005-05-15 05:39:02.117825643 IDT");
}

#[test]
fn test_use_elevation_to_bool() {
    // Test all variants with both hanetz_or_shkia true and false
    assert!(!UseElevation::No.to_bool(true));
    assert!(!UseElevation::No.to_bool(false));

    assert!(UseElevation::HanetzShkia.to_bool(true));
    assert!(!UseElevation::HanetzShkia.to_bool(false));

    assert!(UseElevation::All.to_bool(true));
    assert!(UseElevation::All.to_bool(false));
}

#[test]
pub fn test_extreme_latitude_none_returns() {
    // Test at North Pole during winter (no sunrise/sunset)

    let czc = ComplexZmanimCalendar {
        geo_location: GeoLocation {
            latitude: 89.0,
            longitude: 0.0,
            elevation: 0.0,
            timezone: chrono_tz::Etc::GMT,
        },
        date: chrono_tz::Etc::GMT
            .with_ymd_and_hms(2017, 10, 17, 0, 0, 0)
            .unwrap(),
        use_elevation: UseElevation::No,
    };

    // These should return None because alos/tzais can't be calculated at extreme
    // latitudes
    assert!(czc.alos(&ZmanOffset::Degrees(18.0)).is_none());
    assert!(czc.shkia().is_none());
    assert!(czc.mincha_gedola_gra_greater_than_30_minutes().is_none());
    assert!(czc.chatzos().is_some());
}
