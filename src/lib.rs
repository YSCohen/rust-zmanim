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
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301
// USA, or connect to: <https://www.gnu.org/licenses/old-licenses/lgpl-2.1.html>

//! The rust-zmanim library is an API that can calculate different astronomical
//! times including sunrise and sunset and Jewish *zmanim* or religious times
//! for prayers and other Jewish religious duties. For non-religious
//! astronomical / solar calculations such as sunrise, sunset and twilight, use
//! the [astronomical_calendar]. The [zmanim_calendar] contains the most
//! commonly used zmanim or religious time calculations.
//!
//! **Note:** It is important to read the technical notes on top of the
//! [astronomical_calendar] documentation.
//!
//! ### Disclaimer
//! I did my best to get accurate results using standardized
//! astronomical calculations. Please use care when using the library for
//! *halacha lemaaseh* applications.

pub mod astronomical_calendar;
pub mod util;
pub mod zmanim_calendar;
pub use util::geolocation::GeoLocation;
