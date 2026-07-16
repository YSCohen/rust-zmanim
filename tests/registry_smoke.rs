//! Exercises every registry entry end-to-end and checks that dispatching a few
//! *zmanim* through the registry matches calling their accessors directly. Not
//! a thorough CZC test (see `tests/tes_czc_generated_*`), just a few sanity
//! checks to make sure the registry is working

use jiff::{civil, tz::TimeZone};
use rust_zmanim::{
    complex_zmanim_calendar::{ZmanValue, find_zman},
    prelude::*,
};

fn jerusalem_calendar() -> ComplexZmanimCalendar {
    let geo = GeoLocation::new(
        31.778,
        35.235,
        754.0,
        TimeZone::get("Asia/Jerusalem").unwrap(),
    )
    .unwrap();
    ComplexZmanimCalendar::new(geo, civil::date(2026, 7, 14), UseElevation::No)
}

#[test]
fn almost_every_entry_returns_some() {
    let czc = jerusalem_calendar();
    let mut some = 0;
    for entry in rust_zmanim::complex_zmanim_calendar::ALL_ZMANIM {
        if (entry.compute)(&czc).is_some() {
            some += 1;
        }
    }

    // At a temperate latitude in July, essentially every zman occurs,
    // aside from the Ben Ish Chai polar zmanim.
    assert!(some > 150, "expected most zmanim to be Some, got {some}");
}

#[test]
fn registry_dispatch_matches_direct_call() {
    let czc = jerusalem_calendar();

    let entry = find_zman("shkia").expect("shkia should be registered");
    match (entry.compute)(&czc) {
        Some(ZmanValue::Time(z)) => assert_eq!(z, czc.shkia().unwrap()),
        other => panic!("expected Time for shkia, got {other:?}"),
    }

    let entry = find_zman("shaah_zmanis_gra").expect("shaah_zmanis_gra should be registered");
    match (entry.compute)(&czc) {
        Some(ZmanValue::Duration(d)) => assert_eq!(d, czc.shaah_zmanis_gra().unwrap()),
        other => panic!("expected Duration for shaah_zmanis_gra, got {other:?}"),
    }

    let entry = find_zman("chatzos_hayom").expect("chatzos_hayom should be registered");
    match (entry.compute)(&czc) {
        Some(ZmanValue::Time(z)) => assert_eq!(z, czc.chatzos_hayom().unwrap()),
        other => panic!("expected Time for chatzos_hayom, got {other:?}"),
    }

    let entry = find_zman("polar_sunset_ben_ish_chai")
        .expect("polar_sunset_ben_ish_chai should be registered");
    if let Some(thing) = (entry.compute)(&czc) {
        panic!("expected None for polar_sunset_ben_ish_chai, got {thing:?}")
    }

    assert!(find_zman("not_a_real_zman").is_none());
}
