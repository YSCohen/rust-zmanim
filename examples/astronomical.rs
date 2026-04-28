//! Example of using [`astronomical_calculator`] to calculate sunrise, noon, etc

use jiff::{tz::TimeZone, Zoned};
use rust_zmanim::prelude::*;

fn main() {
    let jerusalem = TimeZone::get("Asia/Jerusalem").unwrap();
    let today = Zoned::now().date();

    // your location here
    let kosel = GeoLocation {
        latitude: 31.777,
        longitude: 35.234,
        elevation: 700.0,
        timezone: jerusalem,
    };

    let dawn = astronomical_calculator::sunrise_offset_by_degrees(
        &today,
        &kosel,
        astronomical_calculator::ASTRONOMICAL_ZENITH,
    )
    .unwrap();
    let sunrise = astronomical_calculator::sunrise(&today, &kosel).unwrap();
    let noon = astronomical_calculator::solar_noon(&today, &kosel).unwrap();

    println!(
        "
dawn:     {dawn}
sunrise:  {sunrise}
noon:     {noon}
"
    )
}
