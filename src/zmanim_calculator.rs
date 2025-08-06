//! An API that can calculate sunrise, sunset and Jewish *zmanim* (religious
//! times) for prayers and other Jewish religious duties.
//!
//! This module contains the main functionality of the [rust-zmanim](crate)
//! library. **Elevation based *zmanim* (even sunrise and sunset) should not be
//! used *lekula* without the guidance of a *posek***. According to Rabbi Dovid
//! Yehudah Bursztyn in his *Zmanim Kehilchasam*, 7th edition chapter 2, section
//! 7 (pages 181-182) and section 9 (pages 186-187), no *zmanim* besides sunrise
//! and sunset should use elevation. However, Rabbi Yechiel Avrahom Zilber in
//! the *Birur Halacha* Vol. 6 Ch. 58 Pages 34 and 42 is of the opinion that
//! elevation should be accounted for in *zmanim* calculations. Related to this,
//! Rabbi Yaakov Karp in *Shimush Zekeinim*, Ch. 1, page 17 states that
//! obstructing horizons should be factored into *zmanim* calculations.
//!
//! When a *zman* will not occur these functions return `None`. See note in
//! [astronomical_calculator]

use chrono::{TimeDelta, prelude::*};
use chrono_tz::Tz;

use crate::astronomical_calculator;
use crate::util::geolocation::GeoLocation;
use crate::util::math_helper::*;

/// Returns *alos hashachar* (dawn) based on either declination of the sun below
/// the horizon, a fixed time offset, or a minutes *zmaniyos* (temporal minutes)
/// offset before sunrise
pub fn alos(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    use_elevation: bool,
    offset: ZmanOffset,
) -> Option<DateTime<Tz>> {
    match offset {
        ZmanOffset::Degrees(deg) => astronomical_calculator::sunrise_offset_by_degrees(
            date,
            geo_location,
            astronomical_calculator::GEOMETRIC_ZENITH + deg,
        ),
        ZmanOffset::Minutes(min) => Some(offset_by_minutes(
            &elevation_adjusted_sunrise(date, geo_location, use_elevation)?,
            -min,
        )),
        ZmanOffset::MinutesZmaniyos {
            minutes_zmaniyos: minz,
            day_start: start,
            day_end: end,
        } => Some(offset_by_minutes_zmanis(
            &elevation_adjusted_sunrise(date, geo_location, use_elevation)?,
            -minz,
            &start,
            &end,
        )),
    }
}

/// Returns [sea level
/// sunrise](crate::astronomical_calculator::sea_level_sunrise) if
/// `use_elevation` is false,
/// or [sunrise](crate::astronomical_calculator::sunrise) if
/// it is true. This allows relevant *zmanim* to automatically adjust to the
/// elevation setting
pub fn elevation_adjusted_sunrise(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> Option<DateTime<Tz>> {
    if use_elevation {
        astronomical_calculator::sunrise(date, geo_location)
    } else {
        astronomical_calculator::sea_level_sunrise(date, geo_location)
    }
}

/// An alias for [elevation_adjusted_sunrise]
pub fn hanetz(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> Option<DateTime<Tz>> {
    elevation_adjusted_sunrise(date, geo_location, use_elevation)
}

/// A generic function for calculating the latest *zman krias shema* (time to
/// recite shema in the morning) that is 3 *shaos zmaniyos* (temporal hours)
/// after the start of the day, calculated using the start and end of the day
/// passed to this function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and the latest *zman krias shema* is calculated as 3 of those
/// *shaos zmaniyos* after the beginning of the day
pub fn sof_zman_shema(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> DateTime<Tz> {
    shaos_into_day(day_start, day_end, 3.0)
}

/// A generic function for calculating the latest *zman tefila* (time to recite
/// *shacharis* in the morning) that is 4 *shaos zmaniyos* (temporal hours)
/// after the start of the day, calculated using the start and end of the day
/// passed to this function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and the latest *zman tefila* is calculated as 4 of those *shaos
/// zmaniyos* after the beginning of the day
pub fn sof_zman_tefila(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> DateTime<Tz> {
    shaos_into_day(day_start, day_end, 4.0)
}

/// Returns [Astronomical
/// *chatzos*](crate::astronomical_calculator::sun_transit).
pub fn chatzos(date: &DateTime<Tz>, geo_location: &GeoLocation) -> Option<DateTime<Tz>> {
    astronomical_calculator::sun_transit(date, geo_location)
}

/// A generic function for calculating *mincha gedola* that is 6.5 *shaos
/// zmaniyos* (temporal hours) after the start of the day, calculated using the
/// start and end of the day passed to this function. Mincha gedola is the
/// earliest time one can pray mincha. The Rambam is of the opinion that it is
/// better to delay mincha until mincha ketana while the Rash, Tur, GRA and
/// others are of the opinion that mincha can be prayed lechatchila starting at
/// mincha gedola.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and *mincha gedola* is calculated as 6.5 of those *shaos
/// zmaniyos* after the beginning of the day
pub fn mincha_gedola(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> DateTime<Tz> {
    shaos_into_day(day_start, day_end, 6.5)
}

/// A generic function for calculating *mincha ketana* (preferred earliest time
/// to recite *mincha* in the afternoon) that is 9.5 *shaos zmaniyos* (temporal
/// hours) after the start of the day, calculated using the start and end of the
/// day passed to this function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and *mincha ketana* is calculated as 9.5 of those *shaos
/// zmaniyos* after the beginning of the day
pub fn mincha_ketana(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> DateTime<Tz> {
    shaos_into_day(day_start, day_end, 9.5)
}

/// A generic function for calculating *plag hamincha* (the earliest time that
/// Shabbos can be started) that is halfway between [*mincha
/// gedola*](mincha_gedola) and [*mincha ketana*](mincha_ketana), or 10.75
/// *shaos zmaniyos* (temporal hours) after the start of the day, calculated
/// using the start and end of the day passed to this function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and the latest *zman krias shema* is calculated as 9.5 of those
/// *shaos zmaniyos* after the beginning of the day
pub fn plag_hamincha(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> DateTime<Tz> {
    shaos_into_day(day_start, day_end, 10.75)
}

/// Returns [sea level sunset](crate::astronomical_calculator::sea_level_sunset)
/// if `use_elevation` is false, or
/// [sunset](crate::astronomical_calculator::sunset) if it is true.
/// This allows relevant *zmanim* to automatically adjust to the
/// elevation setting
pub fn elevation_adjusted_sunset(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> Option<DateTime<Tz>> {
    if use_elevation {
        astronomical_calculator::sunset(date, geo_location)
    } else {
        astronomical_calculator::sea_level_sunset(date, geo_location)
    }
}

/// An alias for [elevation_adjusted_sunset]
pub fn shkia(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> Option<DateTime<Tz>> {
    elevation_adjusted_sunset(date, geo_location, use_elevation)
}

/// Returns *tzais* (nightfall) based on either declination of the sun below the
/// horizon, a fixed time offset, or a minutes *zmaniyos* (temporal minutes)
/// offset after sunset
pub fn tzais(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    use_elevation: bool,
    offset: ZmanOffset,
) -> Option<DateTime<Tz>> {
    match offset {
        ZmanOffset::Degrees(deg) => astronomical_calculator::sunset_offset_by_degrees(
            date,
            geo_location,
            astronomical_calculator::GEOMETRIC_ZENITH + deg,
        ),
        ZmanOffset::Minutes(min) => {
            let sunset = elevation_adjusted_sunset(date, geo_location, use_elevation)?;
            Some(offset_by_minutes(&sunset, min))
        }
        ZmanOffset::MinutesZmaniyos {
            minutes_zmaniyos: minz,
            day_start: start,
            day_end: end,
        } => {
            let sunset = elevation_adjusted_sunset(date, geo_location, use_elevation)?;
            Some(offset_by_minutes_zmanis(&sunset, minz, &start, &end))
        }
    }
}

/// An alias for
/// [temporal_hour()](crate::astronomical_calculator::temporal_hour)
pub fn shaah_zmanis(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> f64 {
    astronomical_calculator::temporal_hour(day_start, day_end)
}

/// A generic function for calculating a given number of *shaos zmaniyos*
/// (temporal hours) after the start of the day, calculated using the start and
/// end of the day passed to this function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and the returned `DateTime` is `shaos` of those *shaos zmaniyos*
/// after the beginning of the day
fn shaos_into_day(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>, shaos: f64) -> DateTime<Tz> {
    let shaah_zmanis = astronomical_calculator::temporal_hour(day_start, day_end);
    offset_by_minutes(day_start, (shaah_zmanis / MINUTE_MILLIS) * shaos)
}

/// Returns a `DateTime` which is `minutes` minutes after `time`
pub fn offset_by_minutes(time: &DateTime<Tz>, minutes: f64) -> DateTime<Tz> {
    *time + TimeDelta::microseconds((minutes * MINUTE_MICROS).round() as i64)
}

/// Returns a `DateTime` which is `minutes` minutes *zmaniyos* (temporal
/// minutes) after `time`.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos* and the returned `DateTime` is `minutes` 60ths of those *shaos
/// zmaniyos* after `time`
fn offset_by_minutes_zmanis(
    time: &DateTime<Tz>,
    minutes: f64,
    day_start: &DateTime<Tz>,
    day_end: &DateTime<Tz>,
) -> DateTime<Tz> {
    let shaah_zmanis_skew = shaah_zmanis(day_start, day_end) / HOUR_MILLIS;
    *time + TimeDelta::microseconds((minutes * MINUTE_MICROS * shaah_zmanis_skew).round() as i64)
}

pub enum ZmanOffset {
    Degrees(f64),
    Minutes(f64),
    MinutesZmaniyos {
        minutes_zmaniyos: f64,
        day_start: DateTime<Tz>,
        day_end: DateTime<Tz>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::Asia::Jerusalem;

    #[test]
    fn test_sof_zman_tefila() {
        let start = Jerusalem.with_ymd_and_hms(2025, 7, 29, 6, 00, 00).unwrap();
        let end = Jerusalem.with_ymd_and_hms(2025, 7, 29, 18, 0, 00).unwrap();
        let result1 = sof_zman_tefila(&start, &end).to_string();
        assert_eq!(result1, "2025-07-29 10:00:00 IDT");

        let start = Jerusalem.with_ymd_and_hms(2025, 7, 29, 5, 47, 29).unwrap();
        let end = Jerusalem.with_ymd_and_hms(2025, 7, 29, 19, 15, 42).unwrap();
        let result2 = sof_zman_tefila(&start, &end).to_string();
        assert_eq!(result2, "2025-07-29 10:16:53.333333 IDT");
    }

    #[test]
    fn test_shaos_into_day() {
        let start1 = Jerusalem.with_ymd_and_hms(2025, 7, 29, 6, 00, 00).unwrap();
        let end1 = Jerusalem.with_ymd_and_hms(2025, 7, 29, 18, 0, 00).unwrap();
        let result1 = shaos_into_day(&start1, &end1, 4.34).to_string();
        assert_eq!(result1, "2025-07-29 10:20:24 IDT");

        let start2 = Jerusalem.with_ymd_and_hms(2025, 7, 29, 5, 47, 29).unwrap();
        let end2 = Jerusalem.with_ymd_and_hms(2025, 7, 29, 19, 15, 42).unwrap();
        let result2 = shaos_into_day(&start2, &end2, 4.34).to_string();
        assert_eq!(result2, "2025-07-29 10:39:47.301667 IDT");
    }

    #[test]
    fn test_tzais() {
        let loc = GeoLocation {
            latitude: 31.79388,
            longitude: 35.03684,
            elevation: 586.19,
            timezone: Jerusalem,
        };

        let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
        let tzais1 = format!(
            "{}",
            tzais(&date1, &loc, false, ZmanOffset::Degrees(6.0)).unwrap()
        );
        assert_eq!(tzais1, "2025-08-04 20:00:13.884113 IDT");

        let date2 = Jerusalem.with_ymd_and_hms(2025, 1, 26, 0, 0, 0).unwrap();
        let tzais2 = format!(
            "{}",
            tzais(&date2, &loc, false, ZmanOffset::Degrees(6.0)).unwrap()
        );
        assert_eq!(tzais2, "2025-01-26 17:34:32.830038 IST");

        let date3 = Jerusalem.with_ymd_and_hms(2005, 5, 15, 0, 0, 0).unwrap();
        let tzais3 = format!(
            "{}",
            tzais(&date3, &loc, false, ZmanOffset::Degrees(6.0)).unwrap()
        );
        assert_eq!(tzais3, "2005-05-15 19:56:34.656301 IDT");
    }
}
