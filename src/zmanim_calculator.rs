//! Core *zmanim* calculations built on astronomical sunrise/sunset events.
//!
//! This module provides:
//! - direct wrappers for commonly used times (`hanetz`, `shkia`, `chatzos`),
//! - generic combinators based on day boundaries (for example,
//!   `sof_zman_shema`, `mincha_gedola`, `plag_hamincha`), and
//! - flexible dawn/nightfall offsets via [`ZmanOffset`].
//!
//! [`ZmanOffset`] supports three offset strategies:
//! - degrees below the horizon,
//! - fixed clock minutes, and
//! - temporal (*zmaniyos*) minutes derived from a supplied *shaah zmanis*.
//!
//! Most public APIs return `Option<Zoned>` and propagate `None` when the
//! underlying astronomical event does not occur.

use std::ops::{Add, Sub};

use jiff::{SignedDuration, Zoned, civil::Date};

use crate::astronomical_calculator;
use crate::util::geolocation::GeoLocation;
use crate::util::math_helper::MINUTE_NANOS;

/// Returns *alos hashachar* (dawn) based on either declination of the sun below
/// the horizon, a fixed time offset, or a minutes *zmaniyos* (temporal minutes)
/// offset before sunrise
#[must_use]
pub fn alos(
    date: &Date,
    geo_location: &GeoLocation,
    use_elevation: bool,
    offset: &ZmanOffset,
) -> Option<Zoned> {
    match offset {
        ZmanOffset::Degrees(deg) => astronomical_calculator::sunrise_offset_by_degrees(
            date,
            geo_location,
            astronomical_calculator::GEOMETRIC_ZENITH + deg,
        ),
        ZmanOffset::Minutes(min) => Some(
            hanetz(date, geo_location, use_elevation)?
                .sub(SignedDuration::from_nanos((min * MINUTE_NANOS) as i64)),
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
pub fn hanetz(date: &Date, geo_location: &GeoLocation, use_elevation: bool) -> Option<Zoned> {
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
pub fn sof_zman_shema(day_start: &Zoned, day_end: &Zoned) -> Zoned {
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
pub fn sof_zman_tefila(day_start: &Zoned, day_end: &Zoned) -> Zoned {
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
pub fn sof_zman_biur_chametz(day_start: &Zoned, day_end: &Zoned) -> Zoned {
    shaos_into_day(day_start, day_end, 5.0)
}

/// Returns [Astronomical
/// noon](crate::astronomical_calculator::solar_noon).
#[must_use]
pub fn chatzos(date: &Date, geo_location: &GeoLocation) -> Option<Zoned> {
    astronomical_calculator::solar_noon(date, geo_location)
}

/// Returns [Astronomical
/// midnight](crate::astronomical_calculator::solar_midnight).
#[must_use]
pub fn chatzos_halayla(date: &Date, geo_location: &GeoLocation) -> Option<Zoned> {
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
pub fn fixed_local_chatzos(date: &Date, geo_location: &GeoLocation) -> Option<Zoned> {
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
pub fn mincha_gedola_30_minutes(date: &Date, geo_location: &GeoLocation) -> Option<Zoned> {
    Some(chatzos(date, geo_location)?.add(SignedDuration::from_mins(30)))
}

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
pub fn mincha_gedola(day_start: &Zoned, day_end: &Zoned) -> Zoned {
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
pub fn samuch_lemincha_ketana(day_start: &Zoned, day_end: &Zoned) -> Zoned {
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
pub fn mincha_ketana(day_start: &Zoned, day_end: &Zoned) -> Zoned {
    shaos_into_day(day_start, day_end, 9.5)
}

/// A generic function for calculating *plag hamincha* (the earliest time that
/// Shabbos can be started) that is halfway between [*mincha
/// gedola*](mincha_gedola) and [*mincha ketana*](mincha_ketana), or 10.75
/// *shaos zmaniyos* (temporal hours) after the start of the day, calculated
/// using the start and end of the day passed to this function.
#[must_use]
pub fn plag_hamincha(day_start: &Zoned, day_end: &Zoned) -> Zoned {
    shaos_into_day(day_start, day_end, 10.75)
}

/// Returns [sea level sunset](crate::astronomical_calculator::sea_level_sunset)
/// if `use_elevation` is false, or
/// [sunset](crate::astronomical_calculator::sunset) if it is true.
/// This allows relevant *zmanim* to automatically adjust to the
/// elevation setting
#[must_use]
pub fn shkia(date: &Date, geo_location: &GeoLocation, use_elevation: bool) -> Option<Zoned> {
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
    date: &Date,
    geo_location: &GeoLocation,
    use_elevation: bool,
    offset: &ZmanOffset,
) -> Option<Zoned> {
    match offset {
        ZmanOffset::Degrees(deg) => astronomical_calculator::sunset_offset_by_degrees(
            date,
            geo_location,
            astronomical_calculator::GEOMETRIC_ZENITH + deg,
        ),
        ZmanOffset::Minutes(min) => {
            let sunset = shkia(date, geo_location, use_elevation)?;
            Some(sunset.add(SignedDuration::from_nanos((min * MINUTE_NANOS) as i64)))
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
pub fn shaah_zmanis(day_start: &Zoned, day_end: &Zoned) -> SignedDuration {
    astronomical_calculator::temporal_hour(day_start, day_end)
}

/// Returns a `Zoned` datetime which is `minutes` minutes *zmaniyos* after
/// `time`, where `shaah_zmanis` is the length in minutes of a *shaah zmanis*
fn offset_by_minutes_zmanis(time: &Zoned, minutes: f64, shaah_zmanis: SignedDuration) -> Zoned {
    time.add(SignedDuration::from_secs_f64(
        (shaah_zmanis / 60).as_secs_f64() * minutes,
    ))
}

/// A generic function for calculating a given number of *shaos zmaniyos*
/// (temporal hours) after the start of the day, calculated using the start and
/// end of the day passed to this function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and the returned `Zoned` is `shaos` of those *shaos zmaniyos*
/// after the beginning of the day
fn shaos_into_day(day_start: &Zoned, day_end: &Zoned, shaos: f64) -> Zoned {
    let shaah_zmanis = astronomical_calculator::temporal_hour(day_start, day_end);
    offset_by_minutes_zmanis(day_start, shaos * 60.0, shaah_zmanis)
}

#[derive(Debug, Clone, PartialEq)]
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
        shaah_zmanis: SignedDuration,
    },
}
