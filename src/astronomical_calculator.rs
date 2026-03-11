//! Astronomical calculations for sunrise, sunset, twilight, and solar transit
//! times.
//!
//! This module exposes low-level solar event calculations that power higher
//! level *zmanim* APIs.
//!
//! Most functions accept a [`Date`] and [`GeoLocation`] and return either:
//! - UTC fractional hours (`f64`) for raw event times, or
//! - localized [`Zoned`] datetimes for ergonomic use.
//!
//! Functions that calculate sunrise/sunset generally distinguish between:
//! - elevation-adjusted calculations, and
//! - sea-level calculations.
//!
//! When an event cannot be computed for the requested date/location, functions
//! return `None`.

use std::ops::{Add, Sub};

use jiff::{SignedDuration, Span, Zoned, civil::Date, tz::TimeZone};

use crate::util::geolocation::GeoLocation;
use crate::util::math_helper::HOUR_NANOS;
use crate::util::noaa_calculator;

/// 90&deg; below the vertical. Used as a basis for most calculations since the
/// location of the sun is 90&deg; below the vertical at sunrise and sunset.
///
/// **Note**: for sunrise and sunset the [adjusted
/// zenith](crate::util::zenith_adjustments::adjusted_zenith) is required to
/// account for the radius of the sun and refraction. See the documentation
/// there for more details
pub const GEOMETRIC_ZENITH: f64 = 90.0;

/// Sun's zenith at civil twilight (96&deg;)
pub const CIVIL_ZENITH: f64 = 96.0;

/// Sun's zenith at nautical twilight (102&deg;)
pub const NAUTICAL_ZENITH: f64 = 102.0;

/// Sun's zenith at astronomical twilight (108&deg;)
pub const ASTRONOMICAL_ZENITH: f64 = 108.0;

/// Returns the sunrise in UTC time without correction for time
/// zone offset from GMT and without using daylight savings time
#[must_use]
pub fn utc_sunrise(date: &Date, zenith: f64, geo_location: &GeoLocation) -> Option<f64> {
    noaa_calculator::utc_sunrise(date, geo_location, zenith, true)
}

/// Returns the sunset in UTC time without correction for time
/// zone offset from GMT and without using daylight savings time
#[must_use]
pub fn utc_sunset(date: &Date, zenith: f64, geo_location: &GeoLocation) -> Option<f64> {
    noaa_calculator::utc_sunset(date, geo_location, zenith, true)
}

/// Returns the sunrise in UTC time without correction for
/// elevation, time zone offset from GMT and without using daylight savings
/// time
#[must_use]
pub fn utc_sea_level_sunrise(date: &Date, zenith: f64, geo_location: &GeoLocation) -> Option<f64> {
    noaa_calculator::utc_sunrise(date, geo_location, zenith, false)
}

/// Returns the sunset in UTC time without correction for
/// elevation, time zone offset from GMT and without using daylight savings
/// time
#[must_use]
pub fn utc_sea_level_sunset(date: &Date, zenith: f64, geo_location: &GeoLocation) -> Option<f64> {
    noaa_calculator::utc_sunset(date, geo_location, zenith, false)
}

/// Returns the elevation-adjusted sunrise time.
///
/// The zenith used for the calculation uses [geometric
/// zenith](GEOMETRIC_ZENITH) of 90&deg;. This is
/// [adjusted](crate::util::zenith_adjustments::adjusted_zenith) to add
/// approximately 50/60 of a degree to account for 34 arcminutes of refraction
/// and 16 arcminutes for the sun's radius for a total of 90.83333&deg;
#[must_use]
pub fn sunrise(date: &Date, geo_location: &GeoLocation) -> Option<Zoned> {
    Some(date_time_from_time_of_day(
        date,
        noaa_calculator::utc_sunrise(date, geo_location, GEOMETRIC_ZENITH, true)?,
        &geo_location.timezone,
    ))
}

/// Returns the elevation-adjusted sunset time.
///
/// The zenith used for the calculation uses
/// [geometric zenith](GEOMETRIC_ZENITH) of 90&deg;. This is
/// [adjusted](crate::util::zenith_adjustments::adjusted_zenith) to add
/// approximately 50/60 of a degree to account for 34 arcminutes of refraction
/// and 16 arcminutes for the sun's radius for a total of 90.83333&deg;
#[must_use]
pub fn sunset(date: &Date, geo_location: &GeoLocation) -> Option<Zoned> {
    Some(date_time_from_time_of_day(
        date,
        noaa_calculator::utc_sunset(date, geo_location, GEOMETRIC_ZENITH, true)?,
        &geo_location.timezone,
    ))
}

/// Returns the sunrise without elevation adjustment, i.e. at sea level
#[must_use]
pub fn sea_level_sunrise(date: &Date, geo_location: &GeoLocation) -> Option<Zoned> {
    sunrise_offset_by_degrees(date, geo_location, GEOMETRIC_ZENITH)
}

/// Returns the sunset without elevation adjustment, i.e. at sea
/// level
#[must_use]
pub fn sea_level_sunset(date: &Date, geo_location: &GeoLocation) -> Option<Zoned> {
    sunset_offset_by_degrees(date, geo_location, GEOMETRIC_ZENITH)
}

/// Returns time of an offset by degrees below or above the horizon of sunrise
///
/// A utility function that returns the time of an offset by degrees below or
/// above the horizon of sunrise. Note that the degree offset is from the
/// vertical, so for a calculation of 14&deg; before sunrise, an offset of 14 +
/// [`GEOMETRIC_ZENITH`] = 104 would have to be passed as a parameter
#[must_use]
pub fn sunrise_offset_by_degrees(
    date: &Date,
    geo_location: &GeoLocation,
    offset_zenith: f64,
) -> Option<Zoned> {
    Some(date_time_from_time_of_day(
        date,
        utc_sea_level_sunrise(date, offset_zenith, geo_location)?,
        &geo_location.timezone,
    ))
}

/// Returns time of an offset by degrees below or above the horizon of sunset
///
/// A utility function that returns the time of an offset by degrees below or
/// above the horizon of sunset. Note that the degree offset is from the
/// vertical, so for a calculation of 14&deg; after sunset, an offset of 14 +
/// [`GEOMETRIC_ZENITH`] = 104 would have to be passed as a parameter
#[must_use]
pub fn sunset_offset_by_degrees(
    date: &Date,
    geo_location: &GeoLocation,
    offset_zenith: f64,
) -> Option<Zoned> {
    Some(date_time_from_time_of_day(
        date,
        utc_sea_level_sunset(date, offset_zenith, geo_location)?,
        &geo_location.timezone,
    ))
}

/// A utility function that will allow the calculation of a temporal (solar)
/// hour based on the sunrise and sunset passed as parameters to this
/// function
#[must_use]
pub fn temporal_hour(sunrise: &Zoned, sunset: &Zoned) -> SignedDuration {
    sunset.duration_since(sunrise) / 12
}

/// Returns solar noon. It occurs when the Sun is transiting the celestial
/// meridian, the apparent highest point in the sky
#[must_use]
pub fn solar_noon(date: &Date, geo_location: &GeoLocation) -> Option<Zoned> {
    Some(date_time_from_time_of_day(
        date,
        noaa_calculator::utc_noon(date, geo_location)?,
        &geo_location.timezone,
    ))
}

/// Returns solar midnight. It occurs when the Sun closest to the nadir, or the
/// direction pointing directly below the given location
#[must_use]
pub fn solar_midnight(date: &Date, geo_location: &GeoLocation) -> Option<Zoned> {
    let midnight = date_time_from_time_of_day(
        date,
        noaa_calculator::utc_midnight(date, geo_location)?,
        &geo_location.timezone,
    );
    // Compare with noon of the same day
    let noon_hour = date
        .to_zoned(geo_location.timezone.clone())
        .unwrap()
        .with()
        .hour(12)
        .build()
        .unwrap();
    if midnight <= noon_hour {
        Some(midnight.add(Span::new().days(1)))
    } else {
        Some(midnight)
    }
}

/// Returns a `Zoned` datetime with the given timezone, made from the
/// (floating-point) number of hours in UTC time
fn date_time_from_time_of_day(date: &Date, time_of_day: f64, timezone: &TimeZone) -> Zoned {
    // nanosecond conversion of time_of_day
    let total_nanos = (time_of_day * HOUR_NANOS).round() as i64;

    // Create UTC datetime at midnight and add nanoseconds
    // there must be a better way to do this...
    let utc_dt = date
        .to_zoned(TimeZone::UTC)
        .unwrap()
        .add(SignedDuration::from_nanos(total_nanos));

    // Convert to target timezone.
    // NOAA returns UTC time-of-day in [0, 24), but the corresponding UTC date
    // can be the day before/after the target local civil date depending on
    // timezone offset. Re-anchor to the requested local date.
    let local_dt = utc_dt.with_time_zone(timezone.clone());
    if local_dt.date() < *date {
        local_dt.add(Span::new().days(1))
    } else if local_dt.date() > *date {
        local_dt.sub(Span::new().days(1))
    } else {
        local_dt
    }
}

/// Returns local mean time (LMT) converted to regular clock time for the number
/// of hours (0.0 to 23.999...) passed to this function.
///
/// This time is adjusted from standard time to account for the local latitude.
/// The 360&deg; of the globe divided by 24 calculates to 15&deg; per hour with
/// 4 minutes per degree, so at a longitude of 0 , 15, 30 etc... noon is at
/// exactly 12:00pm. Lakewood, N.J., with a longitude of -74.222, is 0.7906 away
/// from the closest multiple of 15 at -75&deg;. This is multiplied by 4 clock
/// minutes (per degree) to yield 3 minutes and 7 seconds for a noon time of
/// 11:56:53am. This method is not tied to the theoretical 15&deg; time zones,
/// but will adjust to the actual time zone and Daylight saving time to return
/// LMT.
#[must_use]
pub fn local_mean_time(date: &Date, geo_location: &GeoLocation, hours: f64) -> Option<Zoned> {
    if (0.0..24.0).contains(&hours) {
        let time_of_day = hours - geo_location.local_mean_time_offset();
        Some(date_time_from_time_of_day(
            date,
            time_of_day,
            &geo_location.timezone,
        ))
    } else {
        None
    }
}
