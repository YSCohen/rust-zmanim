//! Geolocation struct, with some math for [local mean
//! time](crate::astronomical_calculator::local_mean_time)
use crate::util::math_helper::HOUR_MINUTES;
use jiff::tz::TimeZone;

#[derive(Debug, Clone, PartialEq)]
/// A struct that contains location information such as latitude and longitude
/// required for astronomical calculations. The elevation field may be ignored
/// by calculations that do not account for elevation.
///
/// Construct with [`GeoLocation::new`], which validates the coordinates.
pub struct GeoLocation {
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) elevation: f64,
    pub(crate) timezone: TimeZone,
}

/// An invalid [`GeoLocation`] parameter, returned by [`GeoLocation::new`]. The
/// contained value is the rejected input.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum GeoLocationError {
    /// Latitude was NaN or outside `[-90.0, 90.0]`
    InvalidLatitude(f64),
    /// Longitude was NaN or outside `[-180.0, 180.0]`
    InvalidLongitude(f64),
    /// Elevation was negative, NaN, or infinite
    InvalidElevation(f64),
}

impl core::fmt::Display for GeoLocationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLatitude(lat) => {
                write!(f, "latitude {lat} is not in the range [-90.0, 90.0]")
            }
            Self::InvalidLongitude(long) => {
                write!(f, "longitude {long} is not in the range [-180.0, 180.0]")
            }
            Self::InvalidElevation(elev) => {
                write!(
                    f,
                    "elevation {elev} is not a non-negative finite number of meters"
                )
            }
        }
    }
}

impl std::error::Error for GeoLocationError {}

impl GeoLocation {
    /// Returns a validated `GeoLocation`.
    ///
    /// `latitude` is in the World Geodetic System, or degrees North of the
    /// Equator, and must be in `[-90.0, 90.0]`. `longitude` is in the World
    /// Geodetic System, or degrees East of the IERS Reference Meridian, and
    /// must be in `[-180.0, 180.0]`. `elevation` is in meters above sea level
    /// and must be non-negative and finite.
    ///
    /// # Errors
    ///
    /// Returns a [`GeoLocationError`] describing the first invalid parameter.
    pub fn new(
        latitude: f64,
        longitude: f64,
        elevation: f64,
        timezone: TimeZone,
    ) -> Result<Self, GeoLocationError> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(GeoLocationError::InvalidLatitude(latitude));
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(GeoLocationError::InvalidLongitude(longitude));
        }
        if !elevation.is_finite() || elevation < 0.0 {
            return Err(GeoLocationError::InvalidElevation(elevation));
        }
        Ok(Self {
            latitude,
            longitude,
            elevation,
            timezone,
        })
    }

    /// The latitude in the World Geodetic System, or degrees North of the
    /// Equator
    #[must_use]
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    /// The longitude in the World Geodetic System, or degrees East of the IERS
    /// Reference Meridian
    #[must_use]
    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    /// The elevation in meters above sea level
    #[must_use]
    pub fn elevation(&self) -> f64 {
        self.elevation
    }

    /// The location's time zone
    #[must_use]
    pub fn timezone(&self) -> &TimeZone {
        &self.timezone
    }

    /// Returns the location's local mean time offset from UTC in hours. The
    /// globe is split into 360&deg;, with 15&deg; per hour of the day. For a
    /// location that is at a longitude evenly divisible by 15 (`longitude
    /// % 15 == 0`), at solar noon (with adjustment for the equation of time)
    /// the sun should be directly overhead, so a user who is 1&deg; west of
    /// this will have noon at 4 minutes after standard time noon, and
    /// conversely, a user who is 1&deg; east of the 15&deg; longitude will have
    /// noon at 11:56 AM. Lakewood, N.J., whose longitude is -74.222, is 0.778
    /// away from the closest multiple of 15 at -75&deg;. This is multiplied by
    /// 4 to yield 3 minutes and 7 seconds earlier than standard time. The
    /// offset returned does not account for the daylight saving time offset
    /// since this struct is unaware of dates.
    #[must_use]
    pub fn local_mean_time_offset(&self) -> f64 {
        (self.longitude * 4.0) / HOUR_MINUTES
    }
}
