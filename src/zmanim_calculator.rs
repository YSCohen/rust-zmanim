//! An API that can calculate sunrise, sunset and Jewish *zmanim* (religious
//! times) for prayers and other Jewish religious duties.
//!
//! This module contains the main functionality of the [`rust-zmanim`](crate)
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
//! [`astronomical_calculator`]

use chrono::{TimeDelta, prelude::*};
use chrono_tz::Tz;

use crate::astronomical_calculator;
use crate::util::geolocation::GeoLocation;
use crate::util::math_helper::MINUTE_NANOS;

/// Returns *alos hashachar* (dawn) based on either declination of the sun below
/// the horizon, a fixed time offset, or a minutes *zmaniyos* (temporal minutes)
/// offset before sunrise
#[must_use]
pub fn alos(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    use_elevation: bool,
    offset: &ZmanOffset,
) -> Option<DateTime<Tz>> {
    match offset {
        ZmanOffset::Degrees(deg) => astronomical_calculator::sunrise_offset_by_degrees(
            date,
            geo_location,
            astronomical_calculator::GEOMETRIC_ZENITH + deg,
        ),
        ZmanOffset::Minutes(min) => Some(
            hanetz(date, geo_location, use_elevation)?
                + TimeDelta::nanoseconds((-min * MINUTE_NANOS).round() as i64),
        ),
        ZmanOffset::MinutesZmaniyos {
            minutes_zmaniyos: minz,
            shaah_zmanis: shaah,
        } => Some(offset_by_minutes_zmanis(
            &hanetz(date, geo_location, use_elevation)?,
            -minz,
            *shaah,
        )),
    }
}

/// Returns [sea level
/// sunrise](crate::astronomical_calculator::sea_level_sunrise) if
/// `use_elevation` is false,
/// or [sunrise](crate::astronomical_calculator::sunrise) if
/// it is true.
///
/// This allows relevant *zmanim* to automatically adjust to the
/// elevation setting
#[must_use]
pub fn hanetz(
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

/// A generic function for calculating the latest *zman krias shema* (time to
/// recite shema in the morning) that is 3 *shaos zmaniyos* (temporal hours)
/// after the start of the day, calculated using the start and end of the day
/// passed to this function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and the latest *zman krias shema* is calculated as 3 of those
/// *shaos zmaniyos* after the beginning of the day
#[must_use]
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
#[must_use]
pub fn sof_zman_tefila(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> DateTime<Tz> {
    shaos_into_day(day_start, day_end, 4.0)
}

/// A generic function for calculating the latest time for burning *chametz* on
/// *Erev Pesach* that is 5 *shaos zmaniyos* (temporal hours) after the start of
/// the day, calculated using the start and end of the day passed to this
/// function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and the latest *zman tefila* is calculated as 5 of those *shaos
/// zmaniyos* after the beginning of the day
#[must_use]
pub fn sof_zman_biur_chametz(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> DateTime<Tz> {
    shaos_into_day(day_start, day_end, 5.0)
}

/// Returns [Astronomical
/// noon](crate::astronomical_calculator::solar_noon).
#[must_use]
pub fn chatzos(date: &DateTime<Tz>, geo_location: &GeoLocation) -> Option<DateTime<Tz>> {
    astronomical_calculator::solar_noon(date, geo_location)
}

/// Returns [Astronomical
/// midnight](crate::astronomical_calculator::solar_midnight).
#[must_use]
pub fn chatzos_halayla(date: &DateTime<Tz>, geo_location: &GeoLocation) -> Option<DateTime<Tz>> {
    astronomical_calculator::solar_midnight(date, geo_location)
}

/// Returns the local time for fixed *chatzos*.
///
/// This time is noon and midnight
/// adjusted from standard time to account for the local latitude. The 360&deg;
/// of the globe divided by 24 calculates to 15&deg; per hour with 4 minutes per
/// degree, so at a longitude of 0 , 15, 30 etc... *Chatzos* is at exactly 12:00
/// noon. This is the time of *chatzos* according to the *Aruch Hashulchan* in
/// *Orach Chaim* 233:14 and Rabbi Moshe Feinstein in *Igros Moshe Orach Chaim*
/// 1:24 and 2:20. Lakewood, N.J., with a longitude of -74.222, is 0.778 away
/// from the closest multiple of 15 at -75&deg;. This is multiplied by 4 to
/// yield 3 minutes and 7 seconds for a *chatzos* of 11:56:53. This method is
/// not tied to the theoretical 15&deg; time zones, but will adjust to the
/// actual time zone and Daylight saving time.
#[must_use]
pub fn fixed_local_chatzos(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
) -> Option<DateTime<Tz>> {
    astronomical_calculator::local_mean_time(date, geo_location, 12.0)
}

/// Returns *mincha gedola* calculated as 30 minutes after *chatzos* and not 1/2
/// of a *shaah zmanis* after *chatzos* as calculated by [`mincha_gedola`].
///
/// Some use this time to delay the start of mincha in the winter when 1/2 of a
/// *shaah zmanis* is less than 30 minutes. See One should not use this time to
/// start *mincha* before the standard *mincha gedola*. See *Shulchan Aruch
/// Orach Chayim* 234:1 and the *Shaar Hatziyon seif katan ches*.
#[must_use]
pub fn mincha_gedola_30_minutes(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
) -> Option<DateTime<Tz>> {
    Some(chatzos(date, geo_location)? + TimeDelta::minutes(30))
}

/// A generic function for calculating *mincha gedola* that is 6.5 *shaos
/// zmaniyos* (temporal hours) after the start of the day, calculated using the
/// start and end of the day passed to this function.
///
/// Mincha gedola is the
/// earliest time one can pray mincha. The Rambam is of the opinion that it is
/// better to delay mincha until mincha ketana while the Rash, Tur, GRA and
/// others are of the opinion that mincha can be prayed lechatchila starting at
/// mincha gedola.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and *mincha gedola* is calculated as 6.5 of those *shaos
/// zmaniyos* after the beginning of the day
#[must_use]
pub fn mincha_gedola(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> DateTime<Tz> {
    shaos_into_day(day_start, day_end, 6.5)
}

/// A generic function for calculating *samuch lemincha ketana* (the time that
/// eating or other activity can't begin prior to praying *mincha*) that is 9
/// *shaos zmaniyos* (temporal hours) after the start of the day, calculated
/// using the start and end of the day passed to this function.
///
/// See the *Mechaber* and *Mishna Berurah* 232 and 249:2.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and *mincha ketana* is calculated as 9 of those *shaos
/// zmaniyos* after the beginning of the day
#[must_use]
pub fn samuch_lemincha_ketana(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> DateTime<Tz> {
    shaos_into_day(day_start, day_end, 9.0)
}

/// A generic function for calculating *mincha ketana* (preferred earliest time
/// to recite *mincha* in the afternoon) that is 9.5 *shaos zmaniyos* (temporal
/// hours) after the start of the day, calculated using the start and end of the
/// day passed to this function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and *mincha ketana* is calculated as 9.5 of those *shaos
/// zmaniyos* after the beginning of the day
#[must_use]
pub fn mincha_ketana(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> DateTime<Tz> {
    shaos_into_day(day_start, day_end, 9.5)
}

/// A generic function for calculating *plag hamincha* (the earliest time that
/// Shabbos can be started) that is halfway between [*mincha
/// gedola*](mincha_gedola) and [*mincha ketana*](mincha_ketana), or 10.75
/// *shaos zmaniyos* (temporal hours) after the start of the day, calculated
/// using the start and end of the day passed to this function.
#[must_use]
pub fn plag_hamincha(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> DateTime<Tz> {
    shaos_into_day(day_start, day_end, 10.75)
}

/// Returns [sea level sunset](crate::astronomical_calculator::sea_level_sunset)
/// if `use_elevation` is false, or
/// [sunset](crate::astronomical_calculator::sunset) if it is true.
/// This allows relevant *zmanim* to automatically adjust to the
/// elevation setting
#[must_use]
pub fn shkia(
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

/// Returns *tzeis* (nightfall) based on either declination of the sun below the
/// horizon, a fixed time offset, or a minutes *zmaniyos* (temporal minutes)
/// offset after sunset
#[must_use]
pub fn tzeis(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    use_elevation: bool,
    offset: &ZmanOffset,
) -> Option<DateTime<Tz>> {
    match offset {
        ZmanOffset::Degrees(deg) => astronomical_calculator::sunset_offset_by_degrees(
            date,
            geo_location,
            astronomical_calculator::GEOMETRIC_ZENITH + deg,
        ),
        ZmanOffset::Minutes(min) => {
            let sunset = shkia(date, geo_location, use_elevation)?;
            Some(sunset + TimeDelta::nanoseconds((min * MINUTE_NANOS).round() as i64))
        }
        ZmanOffset::MinutesZmaniyos {
            minutes_zmaniyos: minz,
            shaah_zmanis: shaah,
        } => {
            let sunset = shkia(date, geo_location, use_elevation)?;
            Some(offset_by_minutes_zmanis(&sunset, *minz, *shaah))
        }
    }
}

/// Gives the length of a *shaah zmanis* (temporal hour), given the
/// start (usually *hanetz* or *alos*) and end (*shkia* or *tzeis*) of the day
#[must_use]
pub fn shaah_zmanis(day_start: &DateTime<Tz>, day_end: &DateTime<Tz>) -> TimeDelta {
    astronomical_calculator::temporal_hour(day_start, day_end)
}

/// Returns a `DateTime` which is `minutes` minutes *zmaniyos* after `time`,
/// where `shaah_zmanis` is the length in minutes of a *shaah zmanis*
fn offset_by_minutes_zmanis(
    time: &DateTime<Tz>,
    minutes: f64,
    shaah_zmanis: TimeDelta,
) -> DateTime<Tz> {
    // Unfortunately you can't multiply TimeDelta by f64, so this is a bit messy
    *time
        + TimeDelta::nanoseconds(
            ((shaah_zmanis / 60).num_nanoseconds().unwrap() as f64 * minutes).round() as i64,
        )
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
    offset_by_minutes_zmanis(day_start, shaos * 60.0, shaah_zmanis)
}

/// Offset used for *alos* or *tzeis*
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZmanOffset {
    /// Some degrees under the horizon
    Degrees(f64),

    /// Some minutes before/after sunrise/sunset
    Minutes(f64),

    /// Some minutes *zmaniyos* before/after sunrise/sunset
    MinutesZmaniyos {
        /// Number of minutes *zmaniyos*
        minutes_zmaniyos: f64,
        /// Length of a *shaah zmanis* in minutes. Each minute *zmanis* will be
        /// 1/60 of this
        shaah_zmanis: TimeDelta,
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
        assert_eq!(result2, "2025-07-29 10:16:53.333333120 IDT");
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
        assert_eq!(result2, "2025-07-29 10:39:47.301666435 IDT");
    }

    #[test]
    fn test_tzeis() {
        let loc = GeoLocation {
            latitude: 31.79388,
            longitude: 35.03684,
            elevation: 586.19,
            timezone: Jerusalem,
        };

        let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
        let tzeis1 = tzeis(&date1, &loc, false, &ZmanOffset::Degrees(6.0))
            .unwrap()
            .to_string();
        assert_eq!(tzeis1, "2025-08-04 20:00:13.884113380 IDT");

        let date2 = Jerusalem.with_ymd_and_hms(2025, 1, 26, 0, 0, 0).unwrap();
        let tzeis2 = tzeis(&date2, &loc, false, &ZmanOffset::Degrees(6.0))
            .unwrap()
            .to_string();
        assert_eq!(tzeis2, "2025-01-26 17:34:32.830038230 IST");

        let date3 = Jerusalem.with_ymd_and_hms(2005, 5, 15, 0, 0, 0).unwrap();
        let tzeis3 = tzeis(&date3, &loc, false, &ZmanOffset::Degrees(6.0))
            .unwrap()
            .to_string();
        assert_eq!(tzeis3, "2005-05-15 19:56:34.656301355 IDT");
    }

    #[test]
    fn test_offset_by_minutes_zmanis() {
        let midnight = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
        let shaah = TimeDelta::minutes(60);
        let twelve_fourty_three_thirty =
            offset_by_minutes_zmanis(&midnight, 43.5, shaah).to_string();
        assert_eq!(twelve_fourty_three_thirty, "2025-08-04 00:43:30 IDT");
    }

    #[test]
    fn test_elevation_shkia() {
        let loc = GeoLocation {
            latitude: 31.79388,
            longitude: 35.03684,
            elevation: 586.19,
            timezone: Jerusalem,
        };

        let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
        let shkia1 = shkia(&date1, &loc, true).unwrap().to_string();
        assert_eq!(shkia1, "2025-08-04 19:37:50.664750888 IDT");

        let date2 = Jerusalem.with_ymd_and_hms(2025, 1, 26, 0, 0, 0).unwrap();
        let shkia2 = shkia(&date2, &loc, true).unwrap().to_string();
        assert_eq!(shkia2, "2025-01-26 17:12:37.244169500 IST");

        let date3 = Jerusalem.with_ymd_and_hms(2005, 5, 15, 0, 0, 0).unwrap();
        let shkia3 = shkia(&date3, &loc, true).unwrap().to_string();
        assert_eq!(shkia3, "2005-05-15 19:33:44.453393419 IDT");
    }
}
