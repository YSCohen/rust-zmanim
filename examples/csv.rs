//! Make a CSV of whichever zmanim you prefer.
//! Good for making a chart - format in a spreadsheet editor
use jiff::{civil, tz::TimeZone};
use rust_zmanim::prelude::*;

fn main() {
    let new_york = TimeZone::get("America/New_York").unwrap();

    let yeshiva = GeoLocation {
        latitude: 40.8506041,
        longitude: -73.9297205,
        elevation: 0.0,
        timezone: new_york,
    };

    println!(
        "Date, Alos 19.8°, Alos 18°, Misheyakir 6.5°, Hanetz, SZKS, SZT, Chatzos, MG, MK, Plag, Shkia, Tzeis 6°"
    );

    let mut date = civil::date(2027, 1, 1);
    let end = civil::date(2027, 12, 30);
    while date <= end {
        let czc = ComplexZmanimCalendar {
            geo_location: yeshiva.clone(),
            date,
            use_elevation: UseElevation::No,
        };

        println!(
            "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            date,
            czc.alos_19_8_degrees().unwrap().strftime("%H:%M:%S"),
            czc.alos_18_degrees().unwrap().strftime("%H:%M:%S"),
            czc.alos(&ZmanOffset::Degrees(6.5))
                .unwrap()
                .strftime("%H:%M:%S"),
            czc.hanetz().unwrap().strftime("%H:%M:%S"),
            czc.sof_zman_shema_gra().unwrap().strftime("%H:%M:%S"),
            czc.sof_zman_tefila_gra().unwrap().strftime("%H:%M:%S"),
            czc.chatzos().unwrap().strftime("%H:%M:%S"),
            czc.mincha_gedola_gra().unwrap().strftime("%H:%M:%S"),
            czc.mincha_ketana_gra().unwrap().strftime("%H:%M:%S"),
            czc.plag_gra().unwrap().strftime("%H:%M:%S"),
            czc.shkia().unwrap().strftime("%H:%M:%S"),
            czc.tzeis_baal_hatanya().unwrap().strftime("%H:%M:%S"),
        );

        date = date.tomorrow().unwrap();
    }
}
