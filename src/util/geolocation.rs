//! Trivial geolocation struct
use crate::util::math_helper::*;
use chrono::offset::TimeZone;
use chrono_tz::OffsetComponents;

#[derive(Debug)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub timezone: chrono_tz::Tz,
}
impl GeoLocation {
    /// Returns the total offset of `self.timezone` in milliseconds, including
    /// UTC offset and DST offset
    pub fn raw_offset(&self) -> f64 {
        let date = self.timezone.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let offset_components = date.offset();
        (offset_components.dst_offset().num_milliseconds()
            + offset_components.dst_offset().num_milliseconds()) as f64
    }

    /// Returns the location's local mean time offset in
    /// milliseconds from local standard time. The globe is split into 360&deg;,
    /// with 15&deg; per hour of the day. For a local that is at a longitude
    /// that is evenly divisible by 15 `(longitude % 15 == 0)`, at solar
    /// noon (with adjustment for the equation of time) the sun should be
    /// directly overhead, so a user who is 1&deg; west of this will have
    /// noon at 4 minutes after standard time noon, and conversely, a user
    /// who is 1&deg; east of the 15&deg; longitude will have noon at 11:56
    /// AM. Lakewood, N.J., whose longitude is -74.222, is 0.778 away from
    /// the closest multiple of 15 at -75&deg;. This is multiplied by 4 to
    /// yield 3 minutes and 10 seconds earlier than standard time.
    pub fn local_mean_time_offset(&self) -> i64 {
        (self.longitude * 4.0 * MINUTE_MILLIS - self.raw_offset()).round() as i64
    }
}
