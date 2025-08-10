//! An API that calculates astronomical times such as sunrise, sunset and
//! twilight times.
//!
//! **Note:** There are times when the algorithms can't calculate proper values
//! for sunrise, sunset and twilight. This is usually caused by trying to
//! calculate times for areas either very far North or South, where sunrise /
//! sunset never happen on that date. This is common when calculating twilight
//! with a deep dip below the horizon for locations as far south of the North
//! Pole as London, in the northern hemisphere. The sun never reaches this dip
//! at certain times of the year. When the calculations encounter this condition
//! they will return `None`.

use chrono::{TimeDelta, prelude::*};
use chrono_tz::Tz;

use crate::util::geolocation::GeoLocation;
use crate::util::math_helper::*;
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
pub fn utc_sunrise(date: &DateTime<Tz>, zenith: f64, geo_location: &GeoLocation) -> Option<f64> {
    noaa_calculator::utc_sunrise(date, geo_location, zenith, true)
}

/// Returns the sunset in UTC time without correction for time
/// zone offset from GMT and without using daylight savings time
pub fn utc_sunset(date: &DateTime<Tz>, zenith: f64, geo_location: &GeoLocation) -> Option<f64> {
    noaa_calculator::utc_sunset(date, geo_location, zenith, true)
}

/// Returns the sunrise in UTC time without correction for
/// elevation, time zone offset from GMT and without using daylight savings
/// time
pub fn utc_sea_level_sunrise(
    date: &DateTime<Tz>,
    zenith: f64,
    geo_location: &GeoLocation,
) -> Option<f64> {
    noaa_calculator::utc_sunrise(date, geo_location, zenith, false)
}

/// Returns the sunset in UTC time without correction for
/// elevation, time zone offset from GMT and without using daylight savings
/// time
pub fn utc_sea_level_sunset(
    date: &DateTime<Tz>,
    zenith: f64,
    geo_location: &GeoLocation,
) -> Option<f64> {
    noaa_calculator::utc_sunset(date, geo_location, zenith, false)
}

/// Returns the elevation-adjusted sunrise time.
///
/// The zenith used for the calculation uses [geometric
/// zenith](GEOMETRIC_ZENITH) of 90&deg;. This is
/// [adjusted](crate::util::zenith_adjustments::adjusted_zenith) to add
/// approximately 50/60 of a degree to account for 34 archminutes of refraction
/// and 16 archminutes for the sun's radius for a total of 90.83333&deg;
pub fn sunrise(date: &DateTime<Tz>, geo_location: &GeoLocation) -> Option<DateTime<Tz>> {
    Some(date_time_from_time_of_day(
        date,
        noaa_calculator::utc_sunrise(date, geo_location, GEOMETRIC_ZENITH, true)?,
        geo_location.timezone,
    ))
}

/// Returns the elevation-adjusted
/// sunset time. The zenith used for the calculation uses
/// [geometric zenith](GEOMETRIC_ZENITH) of 90&deg;. This is
/// [adjusted](crate::util::zenith_adjustments::adjusted_zenith) to add
/// approximately 50/60 of a degree to account for 34 archminutes of refraction
/// and 16 archminutes for the sun's radius for a total of 90.83333&deg;
pub fn sunset(date: &DateTime<Tz>, geo_location: &GeoLocation) -> Option<DateTime<Tz>> {
    Some(date_time_from_time_of_day(
        date,
        noaa_calculator::utc_sunset(date, geo_location, GEOMETRIC_ZENITH, true)?,
        geo_location.timezone,
    ))
}

/// Returns the sunrise without elevation adjustment, i.e. at sea level
pub fn sea_level_sunrise(date: &DateTime<Tz>, geo_location: &GeoLocation) -> Option<DateTime<Tz>> {
    sunrise_offset_by_degrees(date, geo_location, GEOMETRIC_ZENITH)
}

/// Returns the sunset without elevation adjustment, i.e. at sea
/// level
pub fn sea_level_sunset(date: &DateTime<Tz>, geo_location: &GeoLocation) -> Option<DateTime<Tz>> {
    sunset_offset_by_degrees(date, geo_location, GEOMETRIC_ZENITH)
}

/// A utility function that returns the time of an offset by degrees below or
/// above the horizon of sunrise. Note that the degree offset is from the
/// vertical, so for a calculation of 14&deg; before sunrise, an offset of 14 +
/// [GEOMETRIC_ZENITH] = 104 would have to be passed as a parameter
pub fn sunrise_offset_by_degrees(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    offset_zenith: f64,
) -> Option<DateTime<Tz>> {
    Some(date_time_from_time_of_day(
        date,
        utc_sea_level_sunrise(date, offset_zenith, geo_location)?,
        geo_location.timezone,
    ))
}

/// A utility function that returns the time of an offset by degrees below or
/// above the horizon of sunset. Note that the degree offset is from the
/// vertical, so for a calculation of 14&deg; after sunset, an offset of 14 +
/// [GEOMETRIC_ZENITH] = 104 would have to be passed as a parameter
pub fn sunset_offset_by_degrees(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    offset_zenith: f64,
) -> Option<DateTime<Tz>> {
    Some(date_time_from_time_of_day(
        date,
        utc_sea_level_sunset(date, offset_zenith, geo_location)?,
        geo_location.timezone,
    ))
}

/// A utility function that will allow the calculation of a temporal (solar)
/// hour in minutes based on the sunrise and sunset passed as parameters to this
/// function
pub fn temporal_hour(sunrise: &DateTime<Tz>, sunset: &DateTime<Tz>) -> f64 {
    let daytime_hours = (*sunset - *sunrise).as_seconds_f64() / 3_600.0;
    (daytime_hours / 12.0) * HOUR_MINUTES
}

/// Returns solar noon. It occurs when the Sun is transiting the celestial
/// meridian, the apparent highest point in the sky
pub fn solar_noon(date: &DateTime<Tz>, geo_location: &GeoLocation) -> Option<DateTime<Tz>> {
    Some(date_time_from_time_of_day(
        date,
        noaa_calculator::utc_noon(date, geo_location)?,
        geo_location.timezone,
    ))
}

/// Returns solar midnight. It occurs when the Sun closest to the nadir, or the
/// direction pointing directly below the given location
pub fn solar_midnight(date: &DateTime<Tz>, geo_location: &GeoLocation) -> Option<DateTime<Tz>> {
    Some(date_time_from_time_of_day(
        date,
        noaa_calculator::utc_midnight(date, geo_location)?,
        geo_location.timezone,
    ))
}

/// A function that creates a `DateTime` from the `f64` of UTC time from
/// [utc_sunrise], etc
pub fn date_time_from_time_of_day(
    date: &DateTime<Tz>,
    time_of_day: f64,
    timezone: Tz,
) -> DateTime<Tz> {
    let total_seconds = time_of_day * HOUR_SECONDS;
    let hour = (total_seconds / HOUR_SECONDS).floor() as u32;
    let remainder = total_seconds % HOUR_SECONDS;
    let minute = (remainder / MINUTE_SECONDS).floor() as u32;
    let remainder = remainder % MINUTE_SECONDS;
    let second = (remainder).floor() as u32;
    let microsecond = (remainder.fract() * SECOND_MICROS).round() as i64;

    let (year, month, day) = (date.year(), date.month(), date.day());

    Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
        .unwrap()
        .with_timezone(&timezone)
        + TimeDelta::microseconds(microsecond)
}

/// Returns local mean time (LMT) converted to regular clock time for the number
/// of hours (0.0 to 23.999...) passed to this function. This time is adjusted
/// from standard time to account for the local latitude. The 360&deg; of the
/// globe divided by 24 calculates to 15&deg; per hour with 4 minutes per
/// degree, so at a longitude of 0 , 15, 30 etc... noon is at exactly 12:00pm.
/// Lakewood, N.J., with a longitude of -74.222, is 0.7906 away from the closest
/// multiple of 15 at -75&deg;. This is multiplied by 4 clock minutes (per
/// degree) to yield 3 minutes and 7 seconds for a noon time of 11:56:53am. This
/// method is not tied to the theoretical 15&deg; time zones, but will adjust to
/// the actual time zone and Daylight saving time to return LMT.
pub fn local_mean_time(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    hours: f64,
) -> Option<DateTime<Tz>> {
    if !(0.0..24.0).contains(&hours) {
        None
    } else {
        Some(
            date_time_from_time_of_day(
                date,
                hours - (geo_location.raw_offset() / HOUR_MILLIS),
                geo_location.timezone,
            ) - TimeDelta::milliseconds(geo_location.local_mean_time_offset()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::Asia::Jerusalem;

    #[test]
    fn test_sea_level_sunrise() {
        let loc = GeoLocation {
            latitude: 31.79388,
            longitude: 35.03684,
            elevation: 586.19,
            timezone: Jerusalem,
        };

        let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
        let rise1 = format!("{}", sea_level_sunrise(&date1, &loc).unwrap());
        assert_eq!(rise1, "2025-08-04 05:57:34.359480 IDT");

        let date2 = Jerusalem.with_ymd_and_hms(2025, 1, 26, 0, 0, 0).unwrap();
        let rise2 = format!("{}", sea_level_sunrise(&date2, &loc).unwrap());
        assert_eq!(rise2, "2025-01-26 06:36:28.393758 IST");

        let date3 = Jerusalem.with_ymd_and_hms(2005, 5, 15, 0, 0, 0).unwrap();
        let rise3 = format!("{}", sea_level_sunrise(&date3, &loc).unwrap());
        assert_eq!(rise3, "2005-05-15 05:43:01.021454 IDT");
    }

    #[test]
    fn test_temporal_hour() {
        let start1 = Jerusalem.with_ymd_and_hms(2025, 7, 29, 6, 00, 00).unwrap();
        let end1 = Jerusalem.with_ymd_and_hms(2025, 7, 29, 18, 0, 00).unwrap();
        assert_eq!(temporal_hour(&start1, &end1), 60.0);

        let start2 = Jerusalem.with_ymd_and_hms(2025, 7, 29, 5, 47, 29).unwrap();
        let end2 = Jerusalem.with_ymd_and_hms(2025, 7, 29, 19, 15, 42).unwrap();
        assert_eq!(temporal_hour(&start2, &end2), 67.35138888888889);
    }

    #[test]
    fn test_sunset_offset_by_degrees() {
        let loc = GeoLocation {
            latitude: 31.79388,
            longitude: 35.03684,
            elevation: 586.19,
            timezone: Jerusalem,
        };

        let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
        let set1 = format!("{}", sunset_offset_by_degrees(&date1, &loc, 98.5).unwrap());
        assert_eq!(set1, "2025-08-04 20:13:13.825504 IDT");

        let date2 = Jerusalem.with_ymd_and_hms(2025, 1, 26, 0, 0, 0).unwrap();
        let set2 = format!("{}", sunset_offset_by_degrees(&date2, &loc, 98.5).unwrap());
        assert_eq!(set2, "2025-01-26 17:46:52.877997 IST");

        let date3 = Jerusalem.with_ymd_and_hms(2005, 5, 15, 0, 0, 0).unwrap();
        let set3 = format!("{}", sunset_offset_by_degrees(&date3, &loc, 98.5).unwrap());
        assert_eq!(set3, "2005-05-15 20:09:52.608705 IDT");
    }

    #[test]
    fn test_local_mean_time() {
        let loc = GeoLocation {
            latitude: 31.79388,
            longitude: 35.03684,
            elevation: 586.19,
            timezone: Jerusalem,
        };

        let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
        let rise1 = format!("{}", local_mean_time(&date1, &loc, 12.0).unwrap());
        assert_eq!(rise1, "2025-08-04 12:39:51.158 IDT"); // off by one millis

        let date2 = Jerusalem.with_ymd_and_hms(2025, 1, 26, 0, 0, 0).unwrap();
        let rise2 = format!("{}", local_mean_time(&date2, &loc, 13.14159).unwrap());
        assert_eq!(rise2, "2025-01-26 12:48:20.882 IST"); // off by one millis

        let rise3 = format!("{}", local_mean_time(&date2, &loc, 6.23456).unwrap());
        assert_eq!(rise3, "2025-01-26 05:53:55.574 IST"); // off by one millis
    }
}
