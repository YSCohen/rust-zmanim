// calculate the beginning of the second ashmura, the correct time to start
// slichos according to some opinions
use jiff::{Span, civil, tz::TimeZone};
use rust_zmanim::prelude::*;
use std::ops::Add;

fn main() {
    let new_york = TimeZone::get("America/New_York").unwrap();
    // first night of slichos
    let date = civil::date(2025, 9, 13);
    let yu = GeoLocation {
        latitude: 40.8506041,
        longitude: -73.9297205,
        elevation: 0.0,
        timezone: new_york,
    };

    // beginning of the night
    let sunset = zmanim_calculator::shkia(&date, &yu, false).unwrap();

    // add a day to calculate sunrise the next day
    let tomorrow = date.add(Span::new().days(1));
    // end of the night
    let sunrise = zmanim_calculator::hanetz(&tomorrow, &yu, false).unwrap();

    // each ashmura is 1/3 of the night
    let ashmura = sunrise.duration_since(&sunset) / 3;
    // first chiluf mishmaros is one ashmura into the night
    let chiluf1 = sunset.add(ashmura).strftime("%Y-%m-%d %H:%M:%S %Z");
    println!("{chiluf1}");
}
