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

#![doc = include_str!("../README.md")]

pub mod astronomical_calculator;
pub mod complex_zmanim_calendar;
pub mod util;
pub mod zmanim_calculator;

/// A convenience module for glob imports. `use rust_zmanim::prelude::*;`
pub mod prelude {
    pub use chrono::TimeZone; // So `Tz.with_ymd_and_hms()` will work

    pub use crate::astronomical_calculator;
    pub use crate::zmanim_calculator;

    pub use crate::complex_zmanim_calendar::*;
    pub use crate::util::geolocation::GeoLocation;
    pub use zmanim_calculator::ZmanOffset;
}
