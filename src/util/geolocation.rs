//! Geolocation struct, with some math for [local mean
//! time](crate::astronomical_calculator::local_mean_time)
use crate::util::math_helper::*;

#[derive(Clone, Debug)]
/// A struct that contains location information such as latitude and longitude
/// required for astronomical calculations. The elevation field may not be used
/// by some calculations and would be ignored if set
pub struct GeoLocation {
    /// The latitude in the World Geodetic System, or degrees North of the
    /// Equator
    pub latitude: f64,
    /// The longitude in the World Geodetic System, or degrees East of the IERS
    /// Reference Meridian
    pub longitude: f64,
    /// The elevation in meters above sea level
    pub elevation: f64,
    /// The location's time zone, from [chrono_tz]
    pub timezone: chrono_tz::Tz,
}
impl GeoLocation {
    /// Returns the location's local mean time offset from UTC in hours. The
    /// globe is split into 360&deg;, with 15&deg; per hour of the day. For a
    /// local that is at a longitude that is evenly divisible by 15 (`longitude
    /// % 15 == 0`), at solar noon (with adjustment for the equation of time)
    /// the sun should be directly overhead, so a user who is 1&deg; west of
    /// this will have noon at 4 minutes after standard time noon, and
    /// conversely, a user who is 1&deg; east of the 15&deg; longitude will have
    /// noon at 11:56 AM. Lakewood, N.J., whose longitude is -74.222, is 0.778
    /// away from the closest multiple of 15 at -75&deg;. This is multiplied by
    /// 4 to yield 3 minutes and 10 seconds earlier than standard time. The
    /// offset returned does not account for the Daylight saving time offset
    /// since this struct is unaware of dates.
    pub fn local_mean_time_offset(&self) -> f64 {
        (self.longitude * 4.0) / HOUR_MINUTES
    }
}
