# rust-zmanim
The rust-zmanim library is an API that can calculate different astronomical times including sunrise and sunset and Jewish *zmanim* or religious times for prayers and other Jewish religious duties.

The `astronomical_calculator` provides non-religious astronomical / solar calculations such as sunrise, sunset and twilight.

The `zmanim_calculator` contains the most commonly used zmanim or religious time calculations.

**Note:** It is important to read the technical notes on top of the `astronomical_calculator` documentation.

### Disclaimer
I did my best to get accurate results using standardized astronomical calculations. Please use care when using the library for *halacha lemaaseh* applications.

This project is a port from pinnymz's [python-zmanim project](https://github.com/pinnymz/python-zmanim) and Eliyahu Hershfeld's [KosherJava project](https://github.com/KosherJava/zmanim). Almost all of the code is from `python-zmanim` and `KosherJava`, and almost all of the documentation, including some of this README, is from `KosherJava`

See the [KosherJava Zmanim site](https://kosherjava.com) for additional information on the original Java project, and *zmanim* in general.

## Example
```rust
use chrono_tz::Asia::Jerusalem;
use rust_zmanim::prelude::*;

// the time in the DateTime will be ignored
let dt = Jerusalem.with_ymd_and_hms(2025, 7, 29, 10, 30, 26).unwrap();

let beit_meir = GeoLocation {
    latitude: 31.7975,
    longitude: 35.0345,
    elevation: 526.0,
    timezone: Jerusalem,
};
let tzais_baal_hatanya =
    zmanim_calculator::tzais(&dt, &beit_meir, false, ZmanOffset::Degrees(6.0));
assert_eq!(
    format!("{tzais_baal_hatanya}"),
    "2025-07-29 20:05:21.587287 IDT"
)
```
