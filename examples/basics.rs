use jiff::{civil, tz::TimeZone};
use rust_zmanim::prelude::*;

fn main() {
    let date = civil::date(2025, 7, 29);

    // your location here
    let beit_meir = GeoLocation::new(
        31.7975,
        35.0345,
        526.0,
        TimeZone::get("Asia/Jerusalem").unwrap(),
    )
    .unwrap();

    // the zmanim_calculator provides some basic building blocks of zmanim
    if let Some(sunrise) = zmanim_calculator::hanetz(date, &beit_meir, false) {
        println!("Sunrise: {}", sunrise.strftime("%Y-%m-%d %H:%M:%S %Z"));
    }

    // zmanim_calculator also lets you make any custom tzeis, alos, etc using
    // ZmanOffset. these may be based on degrees, fixed time, or shaos zmaniyos
    if let Some(tzeis_degrees) =
        zmanim_calculator::tzeis(date, &beit_meir, false, &ZmanOffset::Degrees(6.13))
    {
        println!(
            "Tzeis (6.13 deg): {}",
            tzeis_degrees.strftime("%Y-%m-%d %H:%M:%S %Z")
        );
    }

    // there is also a ComplexZmanimCalendar struct which stores the date and
    // location, convenient for getting many zmanim for the same point in 4D space.
    // It also has many common zmanim pre-made
    let czc = ComplexZmanimCalendar::new(beit_meir, date, UseElevation::No);

    if let Some(alos120) = czc.alos_120_minutes() {
        println!(
            "Alos (120 min): {}",
            alos120.strftime("%Y-%m-%d %H:%M:%S %Z")
        );
    }

    if let Some(sz18) = czc.shaah_zmanis_mga_18_degrees() {
        println!("Shaah zmanis (MGA 18 deg): {sz18:#}");
    }

    // the calculations will return None if the specified solar event will not
    // occur
    let north_pole = GeoLocation::new(90.0, 0.0, 0.0, TimeZone::UTC).unwrap();
    let polar_sunset = zmanim_calculator::shkia(date, &north_pole, false);
    println!("Sunset at the North Pole in July: {polar_sunset:?}");
}
