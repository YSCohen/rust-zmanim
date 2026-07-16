//! Example of using [`astronomical_calculator`] to calculate sunrise, noon,
//! solar position, etc

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

    // where the sun is right now
    let now = Zoned::now();
    let azimuth = astronomical_calculator::solar_azimuth(&now, &kosel);
    let elevation = astronomical_calculator::solar_elevation(&now, &kosel);

    println!(
        "
dawn:     {dawn}
sunrise:  {sunrise}
noon:     {noon}

solar azimuth now:   {azimuth:.2} deg
solar elevation now: {elevation:.2} deg
"
    )
}
