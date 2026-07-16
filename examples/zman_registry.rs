//! Example of using the zman registry (`ALL_ZMANIM` / `find_zman`) to look up
//! zmanim by name and iterate over all of them

use jiff::{Zoned, tz::TimeZone};
use rust_zmanim::prelude::*;

fn main() {
    let jerusalem = TimeZone::get("Asia/Jerusalem").unwrap();
    let today = Zoned::now().date();

    // your location here
    let kosel = GeoLocation::new(31.777, 35.234, 700.0, jerusalem).unwrap();
    let czc = ComplexZmanimCalendar::new(kosel, today, UseElevation::No);

    // look up a single zman by its method name
    let entry = find_zman("sof_zman_shema_gra").unwrap();
    if let Some(ZmanValue::Time(time)) = (entry.compute)(&czc) {
        println!("{}: {}\n", entry.name, time.strftime("%H:%M:%S %Z"));
    }

    // iterate over every zero-argument zman accessor
    for entry in ALL_ZMANIM {
        match (entry.compute)(&czc) {
            Some(ZmanValue::Time(time)) => {
                println!("{}: {}", entry.name, time.strftime("%H:%M:%S %Z"));
            }
            Some(ZmanValue::Duration(duration)) => println!("{}: {duration:#}", entry.name),
            None => println!("{}: does not occur today", entry.name),
        }
    }
}
