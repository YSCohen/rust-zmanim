// calculate the beginning of the second ashmura, the correct time to start
// slichos according to some opinions
use chrono::TimeDelta;
use chrono_tz::America::New_York;
use rust_zmanim::prelude::*;

fn main() {
    // first night of slichos
    let mut date = New_York.with_ymd_and_hms(2025, 9, 13, 0, 0, 0).unwrap();
    let yu = GeoLocation {
        latitude: 40.8506041,
        longitude: -73.9297205,
        elevation: 0.0,
        timezone: New_York,
    };

    // beginning of the night
    let sunset = zmanim_calculator::shkia(&date, &yu, false).unwrap();

    // add a day to calculate sunrise the next day
    date += TimeDelta::days(1);
    //end of the night
    let sunrise = zmanim_calculator::hanetz(&date, &yu, false).unwrap();

    // each ashmura is 1/3 of the night
    let ashmura = (sunrise - sunset) / 3;
    // first chiluf mishmaros is one ashmura into the night
    let chiluf1 = sunset + ashmura;
    println!("{chiluf1}");
}
