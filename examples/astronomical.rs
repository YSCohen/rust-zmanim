//! Example of using [`astronomical_calculator`] to calculate sunrise, noon, etc

use jiff::{Zoned, tz::TimeZone};
use rust_zmanim::prelude::*;

fn main() {
    let jerusalem = TimeZone::get("Asia/Jerusalem").unwrap();
    let today = Zoned::now().date();

    // your location here
    let kosel = GeoLocation::new(31.777, 35.234, 700.0, jerusalem).unwrap();

    let dawn = astronomical_calculator::sunrise_offset_by_degrees(
        today,
        &kosel,
        astronomical_calculator::ASTRONOMICAL_ZENITH,
    )
    .unwrap();
    let sunrise = astronomical_calculator::sunrise(today, &kosel).unwrap();
    let noon = astronomical_calculator::solar_noon(today, &kosel).unwrap();

    println!(
        "
dawn:     {dawn}
sunrise:  {sunrise}
noon:     {noon}
"
    )
}
