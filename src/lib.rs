// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, see:
// <https://www.gnu.org/licenses/old-licenses/lgpl-2.1.html>

//! # rust-zmanim
//! `rust-zmanim` calculates solar events (such as sunrise, sunset, and
//! twilight) and Jewish *zmanim* built on those events.
//!
//! ## Modules
//! - [`astronomical_calculator`]: Low-level astronomical calculations.
//! - [`zmanim_calculator`]: Core *zmanim* building blocks.
//! - [`complex_zmanim_calendar`]: Stateful convenience API with many predefined
//!   *zmanim*.
//!
//! ## Examples
//!
//! ### `zmanim_calculator`: [*Hanetz*](zmanim_calculator::hanetz) (sunrise)
//! ```rust
//! use jiff::{tz::TimeZone, Zoned};
//! use rust_zmanim::prelude::*;
//!
//! let date = Zoned::now().date();
//! let location = GeoLocation {
//!     latitude: 31.778,
//!     longitude: 35.234,
//!     elevation: 754.0,
//!     timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
//! };
//!
//! let hanetz = zmanim_calculator::hanetz(&date, &location, false).unwrap();
//! println!("Hanetz: {}", hanetz.strftime("%H:%M:%S %Z"));
//! ```
//!
//! ### `zmanim_calculator`: [*Shkia*](zmanim_calculator::shkia) (sunset)
//! ```rust
//! # use jiff::{tz::TimeZone, Zoned};
//! # use rust_zmanim::prelude::*;
//! #
//! # let date = Zoned::now().date();
//! # let location = GeoLocation {
//! #     latitude: 31.778,
//! #     longitude: 35.234,
//! #     elevation: 754.0,
//! #     timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
//! # };
//! #
//! let shkia = zmanim_calculator::shkia(&date, &location, false).unwrap();
//! println!("Shkia: {}", shkia.strftime("%H:%M:%S %Z"));
//! ```
//!
//! ### `zmanim_calculator`: [*Chatzos*](zmanim_calculator::chatzos) (Solar Noon)
//! ```rust
//! # use jiff::{tz::TimeZone, Zoned};
//! # use rust_zmanim::prelude::*;
//! #
//! # let date = Zoned::now().date();
//! # let location = GeoLocation {
//! #     latitude: 31.778,
//! #     longitude: 35.234,
//! #     elevation: 754.0,
//! #     timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
//! # };
//! #
//! let chatzos = zmanim_calculator::chatzos(&date, &location).unwrap();
//! println!("Chatzos: {}", chatzos.strftime("%H:%M:%S %Z"));
//! ```
//!
//! ### `zmanim_calculator`: [*Alos*](zmanim_calculator::alos) with a Minutes Offset
//! ```rust
//! # use jiff::{tz::TimeZone, Zoned};
//! # use rust_zmanim::prelude::*;
//! #
//! # let date = Zoned::now().date();
//! # let location = GeoLocation {
//! #     latitude: 31.778,
//! #     longitude: 35.234,
//! #     elevation: 754.0,
//! #     timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
//! # };
//! #
//! let alos_72 =
//!     zmanim_calculator::alos(&date, &location, false, &ZmanOffset::Minutes(72.0)).unwrap();
//! println!("Alos (72 min): {}", alos_72.strftime("%H:%M:%S %Z"));
//! ```
//!
//! ### `zmanim_calculator`: [*Tzeis*](zmanim_calculator::tzeis) with a Degrees Offset
//! ```rust
//! # use jiff::{tz::TimeZone, Zoned};
//! # use rust_zmanim::prelude::*;
//! #
//! # let date = Zoned::now().date();
//! # let location = GeoLocation {
//! #     latitude: 31.778,
//! #     longitude: 35.234,
//! #     elevation: 754.0,
//! #     timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
//! # };
//! #
//! let tzeis_4_5 =
//!     zmanim_calculator::tzeis(&date, &location, false, &ZmanOffset::Degrees(4.5)).unwrap();
//! println!("Tzeis (4.5 deg): {}", tzeis_4_5.strftime("%H:%M:%S %Z"));
//! ```
//!
//! ### `zmanim_calculator`: [*Sof Zman Shema*](zmanim_calculator::sof_zman_shema) (MGA) using *zmanis*-based *Alos* and *Tzeis*
//! ```rust
//! # use jiff::{tz::TimeZone, Zoned};
//! # use rust_zmanim::prelude::*;
//! #
//! # let date = Zoned::now().date();
//! # let location = GeoLocation {
//! #     latitude: 31.778,
//! #     longitude: 35.234,
//! #     elevation: 754.0,
//! #     timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
//! # };
//! #
//! let hanetz = zmanim_calculator::hanetz(&date, &location, false).unwrap();
//! let shkia = zmanim_calculator::shkia(&date, &location, false).unwrap();
//! // this is a GRA shaah zmanis, based on sunrise and sunset
//! let shaah_zmanis_gra = zmanim_calculator::shaah_zmanis(&hanetz, &shkia);
//!
//! // the offset used for alos and tzeis will be based on shaos zmaniyos
//! let offset_50_zmanis = ZmanOffset::MinutesZmaniyos {
//!     minutes_zmaniyos: 50.0,
//!     shaah_zmanis: shaah_zmanis_gra,
//! };
//! let alos_50_zmanis =
//!     zmanim_calculator::alos(&date, &location, false, &offset_50_zmanis).unwrap();
//! let tzeis_50_zmanis =
//!     zmanim_calculator::tzeis(&date, &location, false, &offset_50_zmanis).unwrap();
//!
//! // calculate Sof Zman Shema according to the Magen Avraham, that it is
//! // calculated from alos to tzeis
//! let szks = zmanim_calculator::sof_zman_shema(&alos_50_zmanis, &tzeis_50_zmanis);
//! println!(
//!     "Sof Zman Shema (MGA 50 min zmaniyos): {}",
//!     szks.strftime("%H:%M:%S %Z")
//! );
//! ```
//!
//! ### [`ComplexZmanimCalendar`](complex_zmanim_calendar::ComplexZmanimCalendar): Build Once, Reuse Many Times
//! ```rust
//! # use jiff::{tz::TimeZone, Zoned};
//! # use rust_zmanim::prelude::*;
//! #
//! # let date = Zoned::now().date();
//! # let location = GeoLocation {
//! #     latitude: 31.778,
//! #     longitude: 35.234,
//! #     elevation: 754.0,
//! #     timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
//! # };
//! #
//! let czc = ComplexZmanimCalendar {
//!     geo_location: location,
//!     date,
//!     use_elevation: UseElevation::No,
//! };
//!
//! println!("Hanetz: {}", czc.hanetz().unwrap().strftime("%H:%M:%S %Z"));
//! println!("Shkia: {}", czc.shkia().unwrap().strftime("%H:%M:%S %Z"));
//! ```
//!
//! ### `ComplexZmanimCalendar`: Premade [72-Minute *Alos*](complex_zmanim_calendar::ComplexZmanimCalendar::alos_72_minutes)
//! ```rust
//! # use jiff::{tz::TimeZone, Zoned};
//! # use rust_zmanim::prelude::*;
//! #
//! # let date = Zoned::now().date();
//! # let location = GeoLocation {
//! #     latitude: 31.778,
//! #     longitude: 35.234,
//! #     elevation: 754.0,
//! #     timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
//! # };
//! #
//! # let czc = ComplexZmanimCalendar {
//! #     geo_location: location,
//! #     date,
//! #     use_elevation: UseElevation::No,
//! # };
//! #
//! println!("Alos (72 min): {}", czc.alos_72_minutes().unwrap().strftime("%H:%M:%S %Z"));
//! ```
//!
//! ### `ComplexZmanimCalendar`: [Generic *Tzeis*](complex_zmanim_calendar::ComplexZmanimCalendar::tzeis) with Custom Offset
//! ```rust
//! # use jiff::{tz::TimeZone, Zoned};
//! # use rust_zmanim::prelude::*;
//! #
//! # let date = Zoned::now().date();
//! # let location = GeoLocation {
//! #     latitude: 31.778,
//! #     longitude: 35.234,
//! #     elevation: 754.0,
//! #     timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
//! # };
//! #
//! # let czc = ComplexZmanimCalendar {
//! #     geo_location: location,
//! #     date,
//! #     use_elevation: UseElevation::No,
//! # };
//! #
//! let tzeis_7_083 = czc.tzeis(&ZmanOffset::Degrees(7.083)).unwrap();
//! println!("Tzeis (7.083 deg): {}", tzeis_7_083.strftime("%H:%M:%S %Z"));
//! ```
//!
//! ### `ComplexZmanimCalendar`: [*Tzeis* 90 Minutes *Zmaniyos* After Sunset](complex_zmanim_calendar::ComplexZmanimCalendar::tzeis_90_minutes_zmanis)
//! ```rust
//! # use jiff::{tz::TimeZone, Zoned};
//! # use rust_zmanim::prelude::*;
//! #
//! # let date = Zoned::now().date();
//! # let location = GeoLocation {
//! #     latitude: 31.778,
//! #     longitude: 35.234,
//! #     elevation: 754.0,
//! #     timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
//! # };
//! #
//! # let czc = ComplexZmanimCalendar {
//! #     geo_location: location,
//! #     date,
//! #     use_elevation: UseElevation::No,
//! # };
//! #
//! let tzeis_90_zmanis = czc.tzeis_90_minutes_zmanis().unwrap();
//! println!(
//!     "Tzeis (90 min zmaniyos): {}",
//!     tzeis_90_zmanis.strftime("%H:%M:%S %Z")
//! );
//! ```
//!
//! ### `ComplexZmanimCalendar`: [Elevation Modes](complex_zmanim_calendar::UseElevation)
//! ```rust
//! # use jiff::{tz::TimeZone, Zoned};
//! # use rust_zmanim::prelude::*;
//! #
//! # let date = Zoned::now().date();
//! # let location = GeoLocation {
//! #     latitude: 31.778,
//! #     longitude: 35.234,
//! #     elevation: 754.0,
//! #     timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
//! # };
//! #
//! let sea_level = ComplexZmanimCalendar {
//!     geo_location: location.clone(),
//!     date,
//!     use_elevation: UseElevation::No,
//! };
//!
//! let elevated = ComplexZmanimCalendar {
//!     geo_location: location,
//!     date,
//!     use_elevation: UseElevation::HanetzShkia,
//! };
//!
//! let sea_level_hanetz = sea_level.hanetz().unwrap();
//! let elevated_hanetz = elevated.hanetz().unwrap();
//! println!("Sea-level hanetz: {}", sea_level_hanetz.strftime("%H:%M:%S %Z"));
//! println!("Elevated hanetz:  {}", elevated_hanetz.strftime("%H:%M:%S %Z"));
//! ```
//!
//! ## Notes
//! - Most APIs return `Option<Zoned>`. A result of `None` means the requested
//!   event does not occur for the requested date/location (common in high
//!   latitudes or for deep-twilight calculations).
//! - This crate uses NOAA-based algorithms. Returned values can be highly
//!   precise, but real-world observations vary with atmospheric conditions,
//!   temperature, pressure, etc.
//! - For religious practice (*halacha lemaaseh*), **consult competent halachic
//!   guidance**.
//! - **Elevation based *zmanim* (even sunrise and sunset) should not be used
//!   *lekula* without the guidance of a *posek***. According to Rabbi Dovid
//!   Yehudah Bursztyn in his *Zmanim Kehilchasam*, 7th edition chapter 2,
//!   section 7 (pages 181-182) and section 9 (pages 186-187), no *zmanim*
//!   besides sunrise and sunset should use elevation. However, Rabbi Yechiel
//!   Avrahom Zilber in the *Birur Halacha* Vol. 6 Ch. 58 Pages 34 and 42 is of
//!   the opinion that elevation should be accounted for in *zmanim*
//!   calculations. Related to this, Rabbi Yaakov Karp in *Shimush Zekeinim*,
//!   Ch. 1, page 17 states that obstructing horizons should be factored into
//!   *zmanim* calculations.
//!
//! ## Background
//! This crate is a Rust port which reuses a lot of code and documentation from:
//! - [KosherJava](https://github.com/KosherJava/zmanim)
//! - [python-zmanim](https://github.com/pinnymz/python-zmanim)
//!
//! See the [KosherJava website](https://kosherjava.com) for additional
//! background on *zmanim*.

pub mod astronomical_calculator;
pub mod complex_zmanim_calendar;
pub mod util;
pub mod zmanim_calculator;

/// A convenience module for glob imports. `use rust_zmanim::prelude::*;`
pub mod prelude {
    pub use crate::astronomical_calculator;
    pub use crate::complex_zmanim_calendar::*;
    pub use crate::util::geolocation::GeoLocation;
    pub use crate::zmanim_calculator::{self, ZmanOffset};
}
