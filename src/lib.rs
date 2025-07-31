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
//! The rust-zmanim library is an API that can calculate different astronomical
//! times including sunrise and sunset and Jewish *zmanim* or religious times
//! for prayers and other Jewish religious duties.
//!
//! **Note:** All of the times are currently provided in UTC
//!
//! The `astronomical_calculator` provides non-religious astronomical / solar
//! calculations such as sunrise, sunset and twilight.
//!
//! The `zmanim_calculator` contains the most commonly used zmanim or religious
//! time calculations.
//!
//! **Note:** It is important to read the technical notes on top of the
//! `astronomical_calculator` documentation.
//!
//! ### Disclaimer
//! I did my best to get accurate results using standardized astronomical
//! calculations. Please use care when using the library for *halacha lemaaseh*
//! applications.
//!
//! This project is a port from pinnymz's [python-zmanim
//! project](https://github.com/pinnymz/python-zmanim), which is in turn a port
//! from Eliyahu Hershfeld's [KosherJava
//! project](https://github.com/KosherJava/zmanim). Almost all of the code is
//! from `python-zmanim`, and almost all of the documentation, including some
//! of this README, (and the original implementation) is from `KosherJava`
//!
//! See the [KosherJava Zmanim site](https://kosherjava.com) for additional
//! information on the original Java project, and *zmanim* in general.
//!
//! ```rust
//! use rust_zmanim::prelude::*;
//!
//! let dt = Utc.with_ymd_and_hms(2025, 7, 29, 10, 30, 26).unwrap();
//! let beit_meir = GeoLocation {
//!     latitude: 31.7975,
//!     longitude: 35.0345,
//!     elevation: 526.0,
//! };
//! let tzais_baal_hatanya =
//!     zmanim_calculator::tzais(&dt, &beit_meir, false, ZmanOffset::Degrees(6.0));
//! assert_eq!(
//!     tzais_baal_hatanya.to_string(),
//!     "2025-07-29 17:05:00.779653 UTC"
//! )
//! ```

pub mod astronomical_calculator;
pub mod util;
pub mod zmanim_calculator;

/// A convenience module for glob imports. `use rust-zmanim::prelude::*;`
pub mod prelude {
    pub use chrono::prelude::*;

    pub use crate::astronomical_calculator;
    pub use crate::zmanim_calculator;

    pub use crate::util::geolocation::GeoLocation;
    pub use zmanim_calculator::ZmanOffset;
}
