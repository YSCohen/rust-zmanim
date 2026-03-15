# `rust-zmanim`

Fast, reliable Rust APIs for sunrise/sunset and Jewish zmanim calculations from any date and location.

`rust-zmanim` calculates:

- astronomical times such as sunrise, sunset, solar noon, and twilight
- Jewish zmanim for prayers and other halachic time-based use cases

This crate uses the solar position algorithm implemented by NOAA, based on equations from *Astronomical Algorithms* by Jean Meeus. See [`noaa_calculator`](https://docs.rs/rust-zmanim/latest/rust_zmanim/util/noaa_calculator/index.html) for more details

`rust-zmanim` is ported from  [python-zmanim](https://github.com/pinnymz/python-zmanim) and [KosherJava zmanim](https://github.com/KosherJava/zmanim)

## Quick Start

`rust-zmanim` is [on crates.io](https://crates.io/crates/rust-zmanim). Add it to your project via `Cargo.toml` or by running `cargo add rust-zmanim`.

[`jiff`](https://crates.io/crates/jiff) is also required, as all times, dates, and durations are represented using `jiff` types.

### Example Usage

```rust
use jiff::{tz::TimeZone, Zoned};
use rust_zmanim::prelude::*;

let date = Zoned::now().date();
let location = GeoLocation {
    latitude: 31.778,
    longitude: 35.234,
    elevation: 754.0,
    timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
};

let czc = ComplexZmanimCalendar {
    geo_location: location,
    date,
    use_elevation: UseElevation::No,
};

println!(
    "Sunrise: {}",
    czc.hanetz().unwrap().strftime("%H:%M:%S %Z")
);

println!(
    "Nightfall: {}",
    czc.tzeis_baal_hatanya()
        .unwrap()
        .strftime("%H:%M:%S %Z")
);
```

Most functions return `Option<Zoned>`. `None` is returned when a solar event does not occur for the given date and location — for example, near the poles during parts of summer or winter, or when requesting a dawn/nightfall angle the sun never reaches.

### API Overview

- `astronomical_calculator`: for low-level solar/astronomical calculations
- `zmanim_calculator`: for stateless zmanim calculation functions (you pass `date` and `GeoLocation` each call)
- `ComplexZmanimCalendar`: stateful struct for calculating multiple zmanim for a single date and location, with built-in methods covering both common and uncommon zmanim

## Usage

### 1. One-Off Calculations with `zmanim_calculator`

```rust
use jiff::{tz::TimeZone, Zoned};
use rust_zmanim::prelude::*;

let date = Zoned::now().date();
let location = GeoLocation {
    latitude: 31.778,
    longitude: 35.234,
    elevation: 754.0,
    timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
};

let hanetz = zmanim_calculator::hanetz(&date, &location, false).unwrap();
let shkia = zmanim_calculator::shkia(&date, &location, false).unwrap();
println!("Hanetz: {}", hanetz.strftime("%H:%M:%S %Z"));
println!("Shkia: {}", shkia.strftime("%H:%M:%S %Z"));

let sof_shema_gra = zmanim_calculator::sof_zman_shema(&hanetz, &shkia);
println!(
    "Sof zman shema (GRA): {}",
    sof_shema_gra.strftime("%H:%M:%S %Z")
);

println!(
    "Chatzos: {}",
    zmanim_calculator::chatzos(&date, &location)
        .unwrap()
        .strftime("%H:%M:%S %Z")
);
```

### 2. Custom Offsets

```rust
use jiff::{tz::TimeZone, Zoned};
use rust_zmanim::prelude::*;

let date = Zoned::now().date();
let location = GeoLocation {
    latitude: 31.778,
    longitude: 35.234,
    elevation: 754.0,
    timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
};

let tzeis_4_5 =
    zmanim_calculator::tzeis(&date, &location, false, &ZmanOffset::Degrees(4.5)).unwrap();
println!(
    "Tzeis (4.5 deg): {}",
    tzeis_4_5.strftime("%H:%M:%S %Z")
);

let alos_72 =
    zmanim_calculator::alos(&date, &location, false, &ZmanOffset::Minutes(72.0)).unwrap();
println!(
    "Alos (72 min): {}",
    alos_72.strftime("%H:%M:%S %Z")
);

// this is much easier with ComplexZmanimCalendar - see next example
let hanetz = zmanim_calculator::hanetz(&date, &location, false).unwrap();
let shkia = zmanim_calculator::shkia(&date, &location, false).unwrap();
let shaah_zmanis = zmanim_calculator::shaah_zmanis(&hanetz, &shkia);

let tzeis_90_zmanis = zmanim_calculator::tzeis(
    &date,
    &location,
    false,
    &ZmanOffset::MinutesZmaniyos {
        minutes_zmaniyos: 90.0,
        shaah_zmanis,
    },
)
.unwrap();

println!(
    "Tzeis (90 min zmaniyos): {}",
    tzeis_90_zmanis.strftime("%H:%M:%S %Z")
);
```

### 3. Using `ComplexZmanimCalendar`

```rust
use jiff::{tz::TimeZone, Zoned};
use rust_zmanim::prelude::*;

let date = Zoned::now().date();
let location = GeoLocation {
    latitude: 31.778,
    longitude: 35.234,
    elevation: 754.0,
    timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
};

let czc = ComplexZmanimCalendar {
    geo_location: location,
    date,
    use_elevation: UseElevation::No,
};

let tzeis_4_5 = czc.tzeis(&ZmanOffset::Degrees(4.5)).unwrap();
println!("Tzeis (4.5 deg): {}", tzeis_4_5.strftime("%H:%M:%S %Z"));

let alos_72 = czc.alos_72_minutes().unwrap();
println!("Alos (72 min): {}", alos_72.strftime("%H:%M:%S %Z"));

let tzeis_90_zmanis = czc.tzeis_90_minutes_zmanis().unwrap();
println!(
    "Tzeis (90 min zmaniyos): {}",
    tzeis_90_zmanis.strftime("%H:%M:%S %Z")
);
```

## Elevation Handling

`ComplexZmanimCalendar` has a `use_elevation` setting that controls when elevation adjustments apply.

- `UseElevation::No`: sea-level based zmanim
- `UseElevation::HanetzShkia`: elevation for sunrise/sunset only
- `UseElevation::All`: elevation applied broadly

`zmanim_calculator` functions take a `bool` for whether they should use elevation

See module docs for halachic discussion and caveats.

## Accuracy and Limitations

Results are mathematically precise within the bounds of the underlying model. Observed real-world times may differ due to atmospheric conditions such as pressure, temperature, and refraction. As NOAA notes, calculated and observed values can vary.

Exercise appropriate caution when applying these results to practical halachic decisions.

## Further Reading

See runnable examples in `examples/`

### Documentation

- [crate docs on docs.rs](https://docs.rs/rust-zmanim/latest/rust_zmanim/)
- [`astronomical_calculator`](https://docs.rs/rust-zmanim/latest/rust_zmanim/astronomical_calculator/)
- [`zmanim_calculator`](https://docs.rs/rust-zmanim/latest/rust_zmanim/zmanim_calculator/)
- [`ComplexZmanimCalendar`](https://docs.rs/rust-zmanim/latest/rust_zmanim/complex_zmanim_calendar/struct.ComplexZmanimCalendar.html)