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
//! ## Notes
//! - Most APIs return `Option<Zoned>`. A result of `None` means the requested
//!   event does not occur for the requested date/location (common in high
//!   latitudes or for deep-twilight calculations).
//! - This crate uses NOAA-based algorithms. Returned values can be highly
//!   precise, but real-world observations vary with atmospheric conditions,
//!   temperature, pressure, etc.
//! - For religious practice (*halacha lemaaseh*), **consult competent halachic
//!   guidance**.
//! - **Elevation based *zmanim* (even sunrise and sunset) should not be
//!   used *lekula* without the guidance of a *posek***. According to Rabbi Dovid
//!   Yehudah Bursztyn in his *Zmanim Kehilchasam*, 7th edition chapter 2, section
//!   7 (pages 181-182) and section 9 (pages 186-187), no *zmanim* besides sunrise
//!   and sunset should use elevation. However, Rabbi Yechiel Avrahom Zilber in
//!   the *Birur Halacha* Vol. 6 Ch. 58 Pages 34 and 42 is of the opinion that
//!   elevation should be accounted for in *zmanim* calculations. Related to this,
//!   Rabbi Yaakov Karp in *Shimush Zekeinim*, Ch. 1, page 17 states that
//!   obstructing horizons should be factored into *zmanim* calculations.
//!
//! ## Background
//! This crate is a Rust port which reuses a lot of code and documentation from:
//! - [KosherJava](https://github.com/KosherJava/zmanim)
//! - [python-zmanim](https://github.com/pinnymz/python-zmanim)
//!
//! See <https://kosherjava.com> for additional background on *zmanim*.
//!
//! ## Example
//! ```rust
//! use jiff::{civil, tz::TimeZone};
//! use rust_zmanim::prelude::*;
//!
//! let date = civil::date(2025, 7, 29);
//!
//! // your location here
//! let beit_meir = GeoLocation {
//!     latitude: 31.7975,
//!     longitude: 35.0345,
//!     elevation: 526.0,
//!     timezone: TimeZone::get("Asia/Jerusalem").unwrap(),
//! };
//!
//! // the zmanim_calculator provides some basic building blocks of zmanim
//! if let Some(sunrise) = zmanim_calculator::hanetz(&date, &beit_meir, false) {
//!     assert_eq!(
//!         sunrise.strftime("%Y-%m-%d %H:%M:%S.%f %Z").to_string(),
//!         "2025-07-29 05:53:39.574572512 IDT"
//!     );
//! }
//!
//! // zmanim_calculator also lets you make any custom tzeis, alos, etc using
//! // ZmanOffset. these may be based on degrees, fixed time, or shaos zmaniyos
//! if let Some(tzeis_degrees) =
//!     zmanim_calculator::tzeis(&date, &beit_meir, false, &ZmanOffset::Degrees(6.13))
//! {
//!     assert_eq!(
//!         tzeis_degrees
//!             .strftime("%Y-%m-%d %H:%M:%S.%f %Z")
//!             .to_string(),
//!         "2025-07-29 20:06:02.501735285 IDT"
//!     );
//! }
//!
//! // there is also a ComplexZmanimCalendar struct which stores the date and
//! // location, convenient for getting many zmanim for the same point in 4D space.
//! // It also has many common zmanim pre-made
//! let czc = ComplexZmanimCalendar {
//!     geo_location: beit_meir,
//!     date,
//!     use_elevation: UseElevation::No,
//! };
//!
//! if let Some(alos120) = czc.alos_120_minutes() {
//!     assert_eq!(
//!         alos120.strftime("%Y-%m-%d %H:%M:%S.%f %Z").to_string(),
//!         "2025-07-29 03:53:39.574572512 IDT"
//!     );
//! }
//!
//! if let Some(sz18) = czc.shaah_zmanis_mga_18_degrees() {
//!     // 01:24:14.106060472
//!     assert_eq!(sz18.as_nanos(), 5_054_106_060_472);
//! }
//!
//! // the calculations will return None if the specified solar event will not
//! // occur
//! let north_pole = GeoLocation {
//!     latitude: 90.0,
//!     longitude: 0.0,
//!     elevation: 0.0,
//!     timezone: TimeZone::UTC,
//! };
//! let polar_sunset = zmanim_calculator::shkia(&date, &north_pole, false);
//! assert!(polar_sunset.is_none());
//! ```

pub mod astronomical_calculator;
pub mod complex_zmanim_calendar;
pub mod util;
pub mod zmanim_calculator;

/// A convenience module for glob imports. `use rust_zmanim::prelude::*;`
pub mod prelude {
    pub use crate::astronomical_calculator;
    pub use crate::zmanim_calculator::{self, ZmanOffset};
    pub use crate::complex_zmanim_calendar::*;
    pub use crate::util::geolocation::GeoLocation;
}
