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
//! Calculate different astronomical times including sunrise and sunset and
//! Jewish *zmanim* or religious times for prayers and other Jewish religious
//! duties.
//!
//! The [`astronomical_calculator`] provides non-religious astronomical / solar
//! calculations such as sunrise, sunset and twilight.
//!
//! The [`zmanim_calculator`] contains the basics for *zmanim* calculations.
//!
//! The [`complex_zmanim_calendar`] provides a stateful struct with many premade
//! *zmanim* calculations, both built on the [`zmanim_calculator`] API.
//!
//! This project is a port from pinnymz's [python-zmanim
//! project](https://github.com/pinnymz/python-zmanim) and Eliyahu Hershfeld's
//! [KosherJava project](https://github.com/KosherJava/zmanim). Much of the code
//! is ported directly from `python-zmanim` and `KosherJava`, and almost all of
//! the documentation is from `KosherJava`
//!
//! See the [KosherJava site](https://kosherjava.com) for additional information
//! on the original Java project and *zmanim* in general.
//!
//! **Note:** It is important to read the technical notes on top of the
//! [`astronomical_calculator`] documentation.
//!
//! ### Disclaimer
//! I did my best to get accurate results using standardized astronomical
//! calculations. Please use care when using the library for *halacha lemaaseh*
//! applications. **Also**, despite the great *precision* of the returned
//! values, the *accuracy* is nowhere near that. To quote the NOAA, whose
//! algorithm this crate uses, "due to variations in atmospheric composition,
//! temperature, pressure and conditions, observed values may vary from
//! calculations"
//!
//! ## Example (more examples in /examples)
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
    pub use crate::zmanim_calculator;

    pub use crate::complex_zmanim_calendar::*;
    pub use crate::util::geolocation::GeoLocation;
    pub use zmanim_calculator::ZmanOffset;
}
