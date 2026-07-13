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

use std::{
    cmp::Ordering,
    ops::{Add, Sub},
};

use jiff::{SignedDuration, Zoned, civil::Date, tz::TimeZone};

use crate::util::{geolocation::GeoLocation, math_helper::HOUR_NANOS, noaa_calculator};

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

/// Returns the elevation-adjusted sunrise as a UTC fractional hour.
///
/// No local timezone or daylight saving adjustment is applied.
#[must_use]
pub fn utc_sunrise(date: Date, zenith: f64, geo_location: &GeoLocation) -> Option<f64> {
    noaa_calculator::utc_sunrise(date, geo_location, zenith, true)
}

/// Returns the elevation-adjusted sunset as a UTC fractional hour.
///
/// No local timezone or daylight saving adjustment is applied.
#[must_use]
pub fn utc_sunset(date: Date, zenith: f64, geo_location: &GeoLocation) -> Option<f64> {
    noaa_calculator::utc_sunset(date, geo_location, zenith, true)
}

/// Returns the sea-level sunrise as a UTC fractional hour.
///
/// No elevation, local timezone, or daylight saving adjustment is applied.
#[must_use]
pub fn utc_sea_level_sunrise(date: Date, zenith: f64, geo_location: &GeoLocation) -> Option<f64> {
    noaa_calculator::utc_sunrise(date, geo_location, zenith, false)
}

/// Returns the sea-level sunset as a UTC fractional hour.
///
/// No elevation, local timezone, or daylight saving adjustment is applied.
#[must_use]
pub fn utc_sea_level_sunset(date: Date, zenith: f64, geo_location: &GeoLocation) -> Option<f64> {
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
pub fn sunrise(date: Date, geo_location: &GeoLocation) -> Option<Zoned> {
    date_time_from_time_of_day(
        date,
        noaa_calculator::utc_sunrise(date, geo_location, GEOMETRIC_ZENITH, true)?,
        geo_location,
        &SolarEvent::Sunrise,
    )
}

/// Returns the elevation-adjusted sunset time.
///
/// The zenith used for the calculation uses
/// [geometric zenith](GEOMETRIC_ZENITH) of 90&deg;. This is
/// [adjusted](crate::util::zenith_adjustments::adjusted_zenith) to add
/// approximately 50/60 of a degree to account for 34 arcminutes of refraction
/// and 16 arcminutes for the sun's radius for a total of 90.83333&deg;
#[must_use]
pub fn sunset(date: Date, geo_location: &GeoLocation) -> Option<Zoned> {
    date_time_from_time_of_day(
        date,
        noaa_calculator::utc_sunset(date, geo_location, GEOMETRIC_ZENITH, true)?,
        geo_location,
        &SolarEvent::Sunset,
    )
}

/// Returns the sunrise without elevation adjustment, i.e. at sea level.
#[must_use]
pub fn sea_level_sunrise(date: Date, geo_location: &GeoLocation) -> Option<Zoned> {
    sunrise_offset_by_degrees(date, geo_location, GEOMETRIC_ZENITH)
}

/// Returns the sunset without elevation adjustment, i.e. at sea level.
#[must_use]
pub fn sea_level_sunset(date: Date, geo_location: &GeoLocation) -> Option<Zoned> {
    sunset_offset_by_degrees(date, geo_location, GEOMETRIC_ZENITH)
}

/// Returns time of an offset by degrees below or above the horizon of sunrise
///
/// A utility function that returns sunrise at an offset zenith.
///
/// The offset is measured from the vertical. For example, to calculate
/// 14&deg; before sunrise, pass `14 + GEOMETRIC_ZENITH` (`104.0`).
#[must_use]
pub fn sunrise_offset_by_degrees(
    date: Date,
    geo_location: &GeoLocation,
    offset_zenith: f64,
) -> Option<Zoned> {
    date_time_from_time_of_day(
        date,
        utc_sea_level_sunrise(date, offset_zenith, geo_location)?,
        geo_location,
        &SolarEvent::Sunrise,
    )
}

/// Returns time of an offset by degrees below or above the horizon of sunset
///
/// A utility function that returns sunset at an offset zenith.
///
/// The offset is measured from the vertical. For example, to calculate
/// 14&deg; after sunset, pass `14 + GEOMETRIC_ZENITH` (`104.0`).
#[must_use]
pub fn sunset_offset_by_degrees(
    date: Date,
    geo_location: &GeoLocation,
    offset_zenith: f64,
) -> Option<Zoned> {
    date_time_from_time_of_day(
        date,
        utc_sea_level_sunset(date, offset_zenith, geo_location)?,
        geo_location,
        &SolarEvent::Sunset,
    )
}

/// Returns a temporal (solar) hour based on the provided sunrise and sunset.
#[must_use]
pub fn temporal_hour(sunrise: &Zoned, sunset: &Zoned) -> SignedDuration {
    sunset.duration_since(sunrise) / 12
}

/// Returns solar noon.
///
/// Solar noon occurs when the Sun transits the celestial meridian and reaches
/// its apparent highest point in the sky.
#[must_use]
pub fn solar_noon(date: Date, geo_location: &GeoLocation) -> Option<Zoned> {
    date_time_from_time_of_day(
        date,
        noaa_calculator::utc_noon(date, geo_location)?,
        geo_location,
        &SolarEvent::Noon,
    )
}

/// Returns solar midnight.
///
/// Solar midnight occurs when the Sun is closest to the nadir (the direction
/// directly below the observer).
#[must_use]
pub fn solar_midnight(date: Date, geo_location: &GeoLocation) -> Option<Zoned> {
    date_time_from_time_of_day(
        date,
        noaa_calculator::utc_midnight(date, geo_location)?,
        geo_location,
        &SolarEvent::Midnight,
    )
}

/// Returns the solar azimuth (in degrees, measured clockwise from due north) of
/// the sun at the given datetime and location.
#[must_use]
pub fn solar_azimuth(instant: &Zoned, geo_location: &GeoLocation) -> f64 {
    noaa_calculator::solar_azimuth(instant, geo_location)
}

/// Returns the solar elevation (in degrees) of the sun at the given datetime
/// and location. The value is negative when the sun is below the horizon, and
/// is based on sea level (not adjusted for altitude).
#[must_use]
pub fn solar_elevation(instant: &Zoned, geo_location: &GeoLocation) -> f64 {
    noaa_calculator::solar_elevation(instant, geo_location)
}

/// Returns the time at which the sun reaches an azimuth of 90&deg; (directly
/// east) or 270&deg; (directly west). Only azimuths of 90&deg; (directly east)
/// and 270&deg; (directly west) are supported; any other value is treated as
/// 270&deg;.
///
/// This is used in polar regions on days when there is no sunrise or sunset,
/// where some *halachic* opinions treat sunrise as the time the sun is directly
/// due east and sunset as the time it is directly due west.
///
/// Returns `None` when the azimuth is never reached for the requested date and
/// location.
#[must_use]
pub fn time_at_azimuth_90_or_270(
    date: Date,
    geo_location: &GeoLocation,
    azimuth: f64,
) -> Option<Zoned> {
    date_time_from_time_of_day(
        date,
        noaa_calculator::utc_time_at_azimuth_90_or_270(date, geo_location, azimuth)?,
        geo_location,
        if azimuth == 90.0 {
            &SolarEvent::Sunrise
        } else {
            &SolarEvent::Sunset
        },
    )
}

/// The type of solar event being calculated, used to anchor a UTC time-of-day
/// to the correct civil date
enum SolarEvent {
    Sunrise,
    Sunset,
    Noon,
    Midnight,
}

/// Returns a `Zoned` datetime in the location's timezone, made from the
/// (floating-point) number of hours in UTC time.
///
/// NOAA returns the UTC time-of-day in `[0, 24)`, but the UTC civil date the
/// event falls on can be the day before/after the requested local civil date,
/// depending on the location's longitude and the event. The UTC anchor date is
/// chosen by the event's apparent solar time, so that e.g. a sunset-type event
/// before local dawn is anchored to the *following* civil day.
fn date_time_from_time_of_day(
    date: Date,
    time_of_day: f64,
    geo_location: &GeoLocation,
    event: &SolarEvent,
) -> Option<Zoned> {
    let anchor = noaa_calculator::antimeridian_adjusted_date(
        &date.to_zoned(geo_location.timezone.clone()).ok()?,
    );

    // apparent solar time of the event, per the longitude's natural offset
    let local_time_hours = geo_location.longitude / 15.0 + time_of_day;
    let anchor = match event {
        SolarEvent::Sunrise if local_time_hours > 18.0 => anchor.yesterday().ok()?,
        SolarEvent::Sunset if local_time_hours < 6.0 => anchor.tomorrow().ok()?,
        SolarEvent::Midnight if local_time_hours < 12.0 => anchor.tomorrow().ok()?,
        SolarEvent::Noon if local_time_hours < 0.0 => anchor.tomorrow().ok()?,
        SolarEvent::Noon if local_time_hours > 24.0 => anchor.yesterday().ok()?,
        _ => anchor,
    };

    // Create UTC datetime at midnight of the anchor date and add the
    // nanosecond conversion of time_of_day
    let total_nanos = (time_of_day * HOUR_NANOS).round() as i64;
    let utc_dt = anchor
        .to_zoned(TimeZone::UTC)
        .ok()?
        .add(SignedDuration::from_nanos(total_nanos));

    Some(utc_dt.with_time_zone(geo_location.timezone.clone()))
}

/// Returns local mean time (LMT) for `hours`  converted to regular clock time.
///
/// The `hours` argument must be in `[0.0, 24.0)`.
///
/// This time is adjusted from standard time to account for the local longitude.
/// The 360&deg; of the globe divided by 24 calculates to 15&deg; per hour with
/// 4 minutes per degree, so at a longitude of 0 , 15, 30 etc... noon is at
/// exactly 12:00 PM. Lakewood, N.J., with a longitude of -74.222, is 0.778&deg;
/// away from -75&deg; (the nearest multiple of 15). This is multiplied by
/// 4 clock minutes per degree to yield about 3 minutes and 7 seconds, for a
/// local noon of approximately 11:56:53 AM. This method is not tied to the
/// theoretical 15&deg; time zones, and it adjusts to the actual timezone and
/// daylight saving time to return LMT.
#[must_use]
pub fn local_mean_time(date: Date, geo_location: &GeoLocation, hours: f64) -> Option<Zoned> {
    if !(0.0..24.0).contains(&hours) {
        return None;
    }
    let time_of_day = hours - geo_location.local_mean_time_offset();
    let total_nanos = (time_of_day * HOUR_NANOS).round() as i64;
    let utc_dt = date
        .to_zoned(TimeZone::UTC)
        .ok()?
        .add(SignedDuration::from_nanos(total_nanos));

    // Unlike the solar events, LMT has no event type to anchor by; re-anchor
    // to the requested local date by exactly 24 absolute hours, i.e. one UTC
    // day (civil-day arithmetic would shift by 23/25 hours across a DST
    // transition)
    let local_dt = utc_dt.with_time_zone(geo_location.timezone.clone());
    match local_dt.date().cmp(&date) {
        Ordering::Less => Some(local_dt.add(SignedDuration::from_hours(24))),
        Ordering::Greater => Some(local_dt.sub(SignedDuration::from_hours(24))),
        Ordering::Equal => Some(local_dt),
    }
}
