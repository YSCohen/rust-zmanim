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

//! # \[WIP\] rust-zmanim
//! The rust-zmanim library is an API that can calculate different astronomical
//! times including sunrise and sunset and Jewish *zmanim* or religious times
//! for prayers and other Jewish religious duties.
//!
//! The [astronomical_calculator] provides non-religious astronomical / solar
//! calculations such as sunrise, sunset and twilight.
//!
//! The [zmanim_calculator] contains the basics for *zmanim* calculations.
//!
//! The [complex_zmanim_calendar] provides a stateful struct and many premade
//! *zmanim* calculations, both merely conveniences built on the
//! [zmanim_calculator] API.
//!
//! This project is a port from pinnymz's [python-zmanim
//! project](https://github.com/pinnymz/python-zmanim) and Eliyahu Hershfeld's
//! [KosherJava project](https://github.com/KosherJava/zmanim). Almost all of
//! the code is from `python-zmanim` and `KosherJava`, and almost all of the
//! documentation is from `KosherJava`
//!
//! See the [KosherJava site](https://kosherjava.com) for additional information
//! on the original Java project and *zmanim* in general.
//!
//! **Note:** It is important to read the technical notes on top of the
//! [astronomical_calculator] documentation.
//!
//! ### Disclaimer
//! I did my best to get accurate results using standardized astronomical
//! calculations. Please use care when using the library for *halacha lemaaseh*
//! applications.
//!
//! ## Example
//! ```rust
//! use chrono_tz::{Asia::Jerusalem, UTC};
//! use rust_zmanim::prelude::*;
//! 
//! // the time in the DateTime will be ignored
//! let dt = Jerusalem.with_ymd_and_hms(2025, 7, 29, 10, 30, 26).unwrap();
//! 
//! let beit_meir = GeoLocation {
//!     latitude: 31.7975,
//!     longitude: 35.0345,
//!     elevation: 526.0,
//!     timezone: Jerusalem,
//! };
//! 
//! // the `zmanim_calculator` lets you make any custom tzais/alos
//! if let Some(tzais_baal_hatanya) =
//!     zmanim_calculator::tzais(&dt, &beit_meir, false, ZmanOffset::Degrees(6.0))
//! {
//!     assert_eq!(
//!         format!("{tzais_baal_hatanya}"),
//!         "2025-07-29 20:05:21.587287 IDT"
//!     );
//! }
//! 
//! // there is also a `ComplexZmanimCalendar` for convenience
//! let czc = ComplexZmanimCalendar {
//!     geo_location: beit_meir,
//!     date: dt,
//!     use_elevation: UseElevation::No,
//! };
//! 
//! if let Some(alos120) = czc.alos_120_minutes() {
//!     assert_eq!(format!("{alos120}"), "2025-07-29 03:53:39.574573 IDT");
//! };
//! 
//! if let Some(sz18) = czc.shaah_zmanis_18_degrees() {
//!     // 1 hour 24 minutes 14 seconds in millis
//!     assert_eq!(sz18, 5054106.0605);
//! }
//! 
//! // the calculations will return `None` if the specified solar event will not occur
//! let north_pole = GeoLocation {
//!     latitude: 90.0,
//!     longitude: 0.0,
//!     elevation: 0.0,
//!     timezone: UTC,
//! };
//! let polar_sunset = zmanim_calculator::shkia(&dt, &north_pole, false);
//! assert!(polar_sunset.is_none());
//! ```

pub mod astronomical_calculator;
pub mod complex_zmanim_calendar;
pub mod util;
pub mod zmanim_calculator;

/// A convenience module for glob imports. `use rust-zmanim::prelude::*;`
pub mod prelude {
    pub use chrono::offset::TimeZone; // So `Tz.with_ymd_and_hms()` will work

    pub use crate::astronomical_calculator;
    pub use crate::zmanim_calculator;

    pub use crate::complex_zmanim_calendar::*;
    pub use crate::util::geolocation::GeoLocation;
    pub use zmanim_calculator::ZmanOffset;
}
