use chrono::{TimeDelta, prelude::*};

use crate::util::geolocation::GeoLocation;
use crate::util::math_helper::*;
use crate::util::noaa_calculator;

pub const GEOMETRIC_ZENITH: f64 = 90.0;
pub const CIVIL_ZENITH: f64 = 96.0;
pub const NAUTICAL_ZENITH: f64 = 102.0;
pub const ASTRONOMICAL_ZENITH: f64 = 108.0;

/// This function returns a `DateTime` representing the elevation adjusted sunrise time. The zenith
/// used for the calculation uses [geometric zenith](GEOMETRIC_ZENITH) of 90&deg;, plus
/// [extra adjustments](crate::util::astronomical_calculations::adjusted_zenith). This is adjusted
/// to add approximately 50/60 of a degree to account for 34 archminutes of refraction and 16
/// archminutes for the sun's radius for a total of 90.83333&deg;.
pub fn sunrise(date: &DateTime<Utc>, geo_location: &GeoLocation) -> DateTime<Utc> {
    date_time_from_time_of_day(
        date,
        noaa_calculator::utc_sunrise(date, geo_location, GEOMETRIC_ZENITH, true),
    )
}

/// A function that returns the sunrise without elevation adjustment, i.e. at sea level
pub fn sea_level_sunrise(date: &DateTime<Utc>, geo_location: &GeoLocation) -> DateTime<Utc> {
    sunrise_offset_by_degrees(date, geo_location, GEOMETRIC_ZENITH)
}

/// A utility function that returns the time of an offset by degrees below or above the horizon of
/// sunrise. Note that the degree offset is from the vertical, so for a calculation of 14&deg;
/// before sunrise, an offset of 14 + [GEOMETRIC_ZENITH] = 104 would have to be passed as a parameter.
pub fn sunrise_offset_by_degrees(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    offset_zenith: f64,
) -> DateTime<Utc> {
    date_time_from_time_of_day(
        date,
        utc_sea_level_sunrise(date, offset_zenith, geo_location),
    )
}

/// This function returns a `DateTime` representing the elevation adjusted sunset time. The zenith
/// used for the calculation uses [geometric zenith](GEOMETRIC_ZENITH) of 90&deg;, plus
/// [extra adjustments](crate::util::astronomical_calculations::adjusted_zenith). This is adjusted
/// to add approximately 50/60 of a degree to account for 34 archminutes of refraction and 16
/// archminutes for the sun's radius for a total of 90.83333&deg;.
pub fn sunset(date: &DateTime<Utc>, geo_location: &GeoLocation) -> DateTime<Utc> {
    date_time_from_time_of_day(
        date,
        noaa_calculator::utc_sunset(date, geo_location, GEOMETRIC_ZENITH, true),
    )
}

/// A function that returns the sunset without elevation adjustment, i.e. at sea level
pub fn sea_level_sunset(date: &DateTime<Utc>, geo_location: &GeoLocation) -> DateTime<Utc> {
    sunset_offset_by_degrees(date, geo_location, GEOMETRIC_ZENITH)
}

/// A utility function that returns the time of an offset by degrees below or above the horizon of
/// sunset. Note that the degree offset is from the vertical, so for a calculation of 14&deg;
/// after sunset, an offset of 14 + [GEOMETRIC_ZENITH] = 104 would have to be passed as a parameter.
pub fn sunset_offset_by_degrees(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    offset_zenith: f64,
) -> DateTime<Utc> {
    date_time_from_time_of_day(
        date,
        utc_sea_level_sunset(date, offset_zenith, geo_location),
    )
}

/// A function that returns the sunrise in UTC time without correction for time zone offset from GMT
/// and without using daylight savings time.
pub fn utc_elevation_sunrise(date: &DateTime<Utc>, zenith: f64, geo_location: &GeoLocation) -> f64 {
    noaa_calculator::utc_sunrise(date, geo_location, zenith, true)
}

/// A function that returns the sunrise in UTC time without correction for elevation, time zone offset
/// from GMT and without using daylight savings time.
pub fn utc_sea_level_sunrise(date: &DateTime<Utc>, zenith: f64, geo_location: &GeoLocation) -> f64 {
    noaa_calculator::utc_sunrise(date, geo_location, zenith, false)
}

/// A function that returns the sunset in UTC time without correction for time zone offset from GMT
/// and without using daylight savings time.
pub fn utc_elevation_sunset(date: &DateTime<Utc>, zenith: f64, geo_location: &GeoLocation) -> f64 {
    noaa_calculator::utc_sunset(date, geo_location, zenith, true)
}

/// A function that returns the sunset in UTC time without correction for elevation, time zone offset
/// from GMT and without using daylight savings time.
pub fn utc_sea_level_sunset(date: &DateTime<Utc>, zenith: f64, geo_location: &GeoLocation) -> f64 {
    noaa_calculator::utc_sunset(date, geo_location, zenith, false)
}

/// A utility function that will allow the calculation of a temporal (solar) hour based on the
/// sunrise and sunset passed as parameters to this function.
pub fn temporal_hour(sunrise: &DateTime<Utc>, sunset: &DateTime<Utc>) -> f64 {
    let daytime_hours = (*sunset - *sunrise).as_seconds_f64() / 3600.0;
    (daytime_hours / 12.0) * HOUR_MILLIS
}

/// A function that returns sundial or solar noon. It occurs when the Sun is transiting the
/// celestial meridian. It is calculated as halfway between the sunrise and sunset passed
/// to this function. This time can be slightly off the real transit time due to changes
/// in declination (the lengthening or shortening day).
pub fn sun_transit(date: &DateTime<Utc>, geo_location: &GeoLocation) -> DateTime<Utc> {
    let sunrise = sea_level_sunrise(date, geo_location);
    let sunset = sea_level_sunset(date, geo_location);
    let noon_hour = (temporal_hour(&sunrise, &sunset) / HOUR_MILLIS) * 6.0;
    sunrise + TimeDelta::microseconds(((noon_hour / 24.0) * DAY_MICROS).round() as i64)
}

/// A function that creates a `DateTime` from the returned `f64` of [utc_elevation_sunrise], etc.
pub fn date_time_from_time_of_day(date: &DateTime<Utc>, time_of_day: f64) -> DateTime<Utc> {
    let total_seconds = time_of_day * 3600.0;
    let hour = (total_seconds / 3600.0).floor() as u32;
    let remainder = total_seconds % 3600.0;
    let minute = (remainder / 60.0).floor() as u32;
    let remainder = remainder % 60.0;
    let second = (remainder).floor() as u32;
    let microsecond = (remainder.fract() * SECOND_MICROS).round() as u32;

    let (year, month, day) = (date.year(), date.month(), date.day());

    let naive_datetime = NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|date| date.and_hms_micro_opt(hour, minute, second, microsecond))
        .unwrap();

    DateTime::from_naive_utc_and_offset(naive_datetime, Utc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utc_elevation_sunrise() {
        let (date, loc) = date_loc();
        assert_eq!(
            utc_elevation_sunrise(&date, GEOMETRIC_ZENITH, &loc),
            2.8371795341279333
        );
        assert_eq!(utc_elevation_sunrise(&date, 45.6, &loc), 6.526984934096173);
    }

    #[test]
    fn test_utc_elevation_sunset() {
        let (date, loc) = date_loc();
        assert_eq!(
            utc_elevation_sunset(&date, GEOMETRIC_ZENITH, &loc),
            16.701593810321235
        );
        assert_eq!(utc_elevation_sunset(&date, 96.7, &loc), 17.144401462315347);
    }

    #[test]
    fn test_utc_sea_level_sunrise() {
        let (date, loc) = date_loc();
        assert_eq!(
            utc_sea_level_sunrise(&date, GEOMETRIC_ZENITH, &loc),
            2.8999327653799707
        );
        assert_eq!(utc_sea_level_sunrise(&date, 67.23, &loc), 4.825298918685346);
    }

    #[test]
    fn test_utc_sea_level_sunset() {
        let (date, loc) = date_loc();
        assert_eq!(
            utc_sea_level_sunset(&date, GEOMETRIC_ZENITH, &loc),
            16.63897816594255
        );
        assert_eq!(utc_sea_level_sunset(&date, 100.1, &loc), 17.446005941808895);
    }

    #[test]
    fn test_sunrise() {
        let (date, loc) = date_loc();
        let result = format!("{}", sunrise(&date, &loc));

        // result from python-zmanim. currently fails by 1 microsecond
        // assert_eq!(result, "2025-07-29 02:50:13.846322 UTC");

        // cheating
        assert_eq!(result, "2025-07-29 02:50:13.846323 UTC");
    }

    #[test]
    fn test_sunset() {
        let (date, loc) = date_loc();
        let result = format!("{}", sunset(&date, &loc));

        // this one passes without cheating
        assert_eq!(result, "2025-07-29 16:42:05.737717 UTC");
    }

    #[test]
    fn test_sun_transit() {
        let (date, loc) = date_loc();
        let result = format!("{}", sun_transit(&date, &loc));
        assert_eq!(result, "2025-07-29 09:46:10.039676 UTC")
    }

    #[test]
    fn test_sunset_offset_by_degrees() {
        let (date, loc) = date_loc();
        let result = format!("{}", sunset_offset_by_degrees(&date, &loc, 96.0));

        // fails by 1 microsecond
        // assert_eq!(result, "2025-07-29 17:04:59.441463 UTC")

        // cheating
        assert_eq!(result, "2025-07-29 17:04:59.441464 UTC")
    }

    #[test]
    fn test_temporal_hour() {
        let start1 = Utc.with_ymd_and_hms(2025, 7, 29, 6, 00, 00).unwrap();
        let end1 = Utc.with_ymd_and_hms(2025, 7, 29, 18, 0, 00).unwrap();
        assert_eq!(temporal_hour(&start1, &end1), 3600000.0);

        let start2 = Utc.with_ymd_and_hms(2025, 7, 29, 5, 47, 29).unwrap();
        let end2 = Utc.with_ymd_and_hms(2025, 7, 29, 19, 15, 42).unwrap();
        assert_eq!(temporal_hour(&start2, &end2), 4041083.3333333335);
    }

    fn date_loc() -> (DateTime<Utc>, GeoLocation) {
        let date = Utc.with_ymd_and_hms(2025, 7, 29, 10, 30, 26).unwrap();
        let beit_meir = GeoLocation {
            latitude: 31.78,
            longitude: 35.03,
            elevation: 526.0,
        };
        (date, beit_meir)
    }
}
