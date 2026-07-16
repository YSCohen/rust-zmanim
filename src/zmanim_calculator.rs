//! Core *zmanim* calculations built on astronomical sunrise/sunset events.
//!
//! This module provides:
//! - direct wrappers for commonly used times (`hanetz`, `shkia`,
//!   `chatzos_hayom`),
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

use crate::{
    astronomical_calculator,
    util::{geolocation::GeoLocation, math_helper::MINUTE_NANOS},
};

/// Returns *alos hashachar* (dawn).
///
/// The offset can be provided as:
/// - degrees below the horizon,
/// - fixed clock minutes before sunrise, or
/// - minutes *zmaniyos* (temporal minutes) before sunrise.
#[must_use]
pub fn alos(
    date: Date,
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
        _ => Some(offset_before_event(
            &hanetz(date, geo_location, use_elevation)?,
            offset,
        )),
    }
}

/// Applies a [`Minutes`](ZmanOffset::Minutes) or
/// [`MinutesZmaniyos`](ZmanOffset::MinutesZmaniyos) offset backward from
/// `event` (before sunrise for *alos*).
///
/// # Panics
///
/// Panics on [`ZmanOffset::Degrees`], which is not an offset from an event.
pub(crate) fn offset_before_event(event: &Zoned, offset: &ZmanOffset) -> Zoned {
    match offset {
        ZmanOffset::Degrees(_) => unreachable!("degree offsets are not relative to an event"),
        ZmanOffset::Minutes(min) => {
            event.sub(SignedDuration::from_nanos((min * MINUTE_NANOS) as i64))
        }
        ZmanOffset::MinutesZmaniyos {
            minutes_zmaniyos: minz,
            shaah_zmanis: shaah,
        } => offset_by_minutes_zmanis(event, -minz, *shaah),
    }
}

/// Applies a [`Minutes`](ZmanOffset::Minutes) or
/// [`MinutesZmaniyos`](ZmanOffset::MinutesZmaniyos) offset forward from
/// `event` (after sunset for *tzeis*).
///
/// # Panics
///
/// Panics on [`ZmanOffset::Degrees`], which is not an offset from an event.
pub(crate) fn offset_after_event(event: &Zoned, offset: &ZmanOffset) -> Zoned {
    match offset {
        ZmanOffset::Degrees(_) => unreachable!("degree offsets are not relative to an event"),
        ZmanOffset::Minutes(min) => {
            event.add(SignedDuration::from_nanos((min * MINUTE_NANOS) as i64))
        }
        ZmanOffset::MinutesZmaniyos {
            minutes_zmaniyos: minz,
            shaah_zmanis: shaah,
        } => offset_by_minutes_zmanis(event, *minz, *shaah),
    }
}

/// Returns [sea level
/// sunrise](crate::astronomical_calculator::sea_level_sunrise) if
/// `use_elevation` is false,
/// or [sunrise](crate::astronomical_calculator::sunrise) if
/// it is true.
///
/// This allows relevant *zmanim* to automatically adjust to the elevation
/// setting.
#[must_use]
pub fn hanetz(date: Date, geo_location: &GeoLocation, use_elevation: bool) -> Option<Zoned> {
    if use_elevation {
        astronomical_calculator::sunrise(date, geo_location)
    } else {
        astronomical_calculator::sea_level_sunrise(date, geo_location)
    }
}

/// Returns the latest *zman krias shema* (time to recite *Shema* in the
/// morning).
///
/// It is computed as 3 *shaos zmaniyos* (temporal hours) after `day_start`,
/// where the interval from `day_start` to `day_end` is divided into
/// 12 equal temporal hours.
///
/// In other words, this returns `3/12` into the daytime interval.
#[must_use]
pub fn sof_zman_shema(day_start: &Zoned, day_end: &Zoned) -> Zoned {
    shaos_into_day(day_start, day_end, 3.0)
}

/// Returns the latest *zman tefila* (time to recite *shacharis* in the
/// morning).
///
/// It is computed as 4 *shaos zmaniyos* (temporal hours) after `day_start`,
/// where the interval from `day_start` to `day_end` is divided into
/// 12 equal temporal hours.
///
/// In other words, this returns `4/12` into the daytime interval.
#[must_use]
pub fn sof_zman_tefila(day_start: &Zoned, day_end: &Zoned) -> Zoned {
    shaos_into_day(day_start, day_end, 4.0)
}

/// A generic function for calculating the latest time for burning *chametz* on
/// *Erev Pesach* that is 5 *shaos zmaniyos* (temporal hours) after the start of
/// the day, calculated using the start and end of the day passed to this
/// function.
///
/// The time from the start of day to the end of day is divided into
/// 12 *shaos zmaniyos*, and the latest time for burning *chametz* is
/// calculated as 5 of those *shaos zmaniyos* after the beginning of the day.
#[must_use]
pub fn sof_zman_biur_chametz(day_start: &Zoned, day_end: &Zoned) -> Zoned {
    shaos_into_day(day_start, day_end, 5.0)
}

/// Returns [astronomical noon](crate::astronomical_calculator::solar_noon).
#[must_use]
pub fn chatzos_hayom(date: Date, geo_location: &GeoLocation) -> Option<Zoned> {
    astronomical_calculator::solar_noon(date, geo_location)
}

/// Returns [astronomical
/// midnight](crate::astronomical_calculator::solar_midnight).
#[must_use]
pub fn chatzos_halayla(date: Date, geo_location: &GeoLocation) -> Option<Zoned> {
    astronomical_calculator::solar_midnight(date, geo_location)
}

/// Returns the local time for fixed local *chatzos*.
///
/// This time is "clock noon" (or midnight) adjusted from standard time to
/// account for local longitude. The 360&deg; of the globe divided by 24 yields
/// 15&deg; per hour (which is 4 minutes per degree), so at longitudes 0, 15,
/// 30, etc., *chatzos* is exactly 12:00 noon. This is the time of *chatzos*
/// according to the *Aruch Hashulchan* in *Orach Chaim* 233:14 and Rabbi Moshe
/// Feinstein in *Igros Moshe, Orach Chaim* 1:24 and 2:20. Lakewood, N.J., with
/// a longitude of -74.222, is 0.778&deg; away from the nearest multiple of 15
/// at -75&deg;. This is multiplied by 4 to yield about 3 minutes and 7 seconds,
/// for a *chatzos* of approximately 11:56:53. This method is not tied to
/// theoretical 15&deg; time zones, and it adjusts to the actual timezone and
/// daylight saving time.
#[must_use]
pub fn fixed_local_chatzos_hayom(date: Date, geo_location: &GeoLocation) -> Option<Zoned> {
    astronomical_calculator::local_mean_time(date, geo_location, 12.0)
}

/// Returns *chatzos* (midday) calculated as halfway between `day_start` and
/// `day_end`.
#[must_use]
pub fn chatzos_hayom_as_half_day(day_start: &Zoned, day_end: &Zoned) -> Zoned {
    day_start.add(day_end.duration_since(day_start) / 2)
}

/// Returns *mincha gedola* calculated as 30 minutes after *chatzos* and not 1/2
/// of a *shaah zmanis* after *chatzos* as calculated by [`mincha_gedola`].
///
/// Some use this time to delay the start of mincha in the winter when 1/2 of a
/// *shaah zmanis* is less than 30 minutes. One should not use this time to
/// start *mincha* before the standard *mincha gedola*. See *Shulchan Aruch
/// Orach Chayim* 234:1 and the *Shaar Hatziyon seif katan ches*.
#[must_use]
pub fn mincha_gedola_30_minutes(date: Date, geo_location: &GeoLocation) -> Option<Zoned> {
    Some(chatzos_hayom(date, geo_location)?.add(SignedDuration::from_mins(30)))
}

/// Returns *mincha gedola* using `day_start` and `day_end`.
///
/// Mincha gedola is the earliest time one can pray mincha. The Rambam is of the
/// opinion that it is better to delay mincha until mincha ketana while the
/// *Rash*, *Tur*, GRA and others are of the opinion that *mincha* can be prayed
/// *lechatchila* starting at *mincha gedola*.
///
/// The time from the start of day to the end of day is divided into
/// 12 *shaos zmaniyos*, and *mincha gedola* is calculated as 6.5 of those
/// *shaos zmaniyos* after the beginning of the day.
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
/// The time from the start of day to the end of day is divided into 12 *shaos
/// zmaniyos*, and *mincha ketana* is calculated as 9 of those *shaos
/// zmaniyos* after the beginning of the day.
#[must_use]
pub fn samuch_lemincha_ketana(day_start: &Zoned, day_end: &Zoned) -> Zoned {
    shaos_into_day(day_start, day_end, 9.0)
}

/// A generic function for calculating *mincha ketana* (preferred earliest time
/// to recite *mincha* in the afternoon) that is 9.5 *shaos zmaniyos* (temporal
/// hours) after the start of the day, calculated using the start and end of the
/// day passed to this function.
///
/// The time from the start of day to the end of day is divided into 12 *shaos
/// zmaniyos*, and *mincha ketana* is calculated as 9.5 of those *shaos
/// zmaniyos* after the beginning of the day.
#[must_use]
pub fn mincha_ketana(day_start: &Zoned, day_end: &Zoned) -> Zoned {
    shaos_into_day(day_start, day_end, 9.5)
}

/// A generic function for calculating *plag hamincha* (the earliest time that
/// Shabbos can be started) that is halfway between [*mincha
/// ketana*](mincha_ketana) and sunset, or 10.75 *shaos zmaniyos* (temporal
/// hours) after the start of the day, calculated using the start and end of the
/// day passed to this function.
#[must_use]
pub fn plag_hamincha(day_start: &Zoned, day_end: &Zoned) -> Zoned {
    shaos_into_day(day_start, day_end, 10.75)
}

/// Returns [sea level sunset](crate::astronomical_calculator::sea_level_sunset)
/// if `use_elevation` is false, or
/// [sunset](crate::astronomical_calculator::sunset) if it is true.
///
/// This allows relevant *zmanim* to automatically adjust to the elevation
/// setting.
#[must_use]
pub fn shkia(date: Date, geo_location: &GeoLocation, use_elevation: bool) -> Option<Zoned> {
    if use_elevation {
        astronomical_calculator::sunset(date, geo_location)
    } else {
        astronomical_calculator::sea_level_sunset(date, geo_location)
    }
}

/// Returns *tzeis* (nightfall).
///
/// The offset can be provided as:
/// - degrees below the horizon,
/// - fixed clock minutes after sunset, or
/// - *minutes zmaniyos* (temporal minutes) after sunset.
#[must_use]
pub fn tzeis(
    date: Date,
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
        _ => Some(offset_after_event(
            &shkia(date, geo_location, use_elevation)?,
            offset,
        )),
    }
}

/// Returns the length of a *shaah zmanis* (temporal hour).
///
/// This is computed from the provided day start and day end (commonly
/// *hanetz*/*alos* to *shkia*/*tzeis*).
#[must_use]
pub fn shaah_zmanis(day_start: &Zoned, day_end: &Zoned) -> SignedDuration {
    astronomical_calculator::temporal_hour(day_start, day_end)
}

/// Returns a `Zoned` datetime which is `minutes` minutes *zmaniyos* after
/// `time`, where `shaah_zmanis` is the length of a *shaah zmanis*.
fn offset_by_minutes_zmanis(time: &Zoned, minutes: f64, shaah_zmanis: SignedDuration) -> Zoned {
    let total_nanos = (shaah_zmanis.as_nanos() as f64 * (minutes / 60.0)) as i64;
    time.add(SignedDuration::from_nanos(total_nanos))
}

/// A generic function for calculating a given number of *shaos zmaniyos*
/// (temporal hours) after the start of the day, calculated using the start and
/// end of the day passed to this function.
///
/// The time from the start of day to the end of day is divided into 12 *shaos
/// zmaniyos*, and the returned `Zoned` is `shaos` of those *shaos zmaniyos*
/// after the beginning of the day.
fn shaos_into_day(day_start: &Zoned, day_end: &Zoned, shaos: f64) -> Zoned {
    let shaah_zmanis = astronomical_calculator::temporal_hour(day_start, day_end);
    offset_by_minutes_zmanis(day_start, shaos * 60.0, shaah_zmanis)
}

/// Returns the length of a *shaah zmanis* based on 1/6 of a half-day,
/// calculated from the start to the end of the half-day passed to this
/// function.
///
/// For morning-based *zmanim* the half-day runs from *alos* or *hanetz* to
/// *chatzos*, and for afternoon-based *zmanim* it runs from *chatzos* to
/// *shkia* or *tzeis*.
#[must_use]
pub fn half_day_based_shaah_zmanis(
    start_of_half_day: &Zoned,
    end_of_half_day: &Zoned,
) -> SignedDuration {
    end_of_half_day.duration_since(start_of_half_day) / 6
}

/// A generic function for calculating a *zman* based on one half of the day,
/// split at *chatzos*.
///
/// The half-day is divided into 6 *shaos zmaniyos*
/// ([`half_day_based_shaah_zmanis`]). A non-negative `hours` value offsets
/// forward from `start_of_half_day` (used for morning *zmanim* such as *sof
/// zman krias shema* with `hours` of 3, or *mincha gedola* as 0.5 after
/// *chatzos*), while a negative `hours` value offsets backward from
/// `end_of_half_day`.
#[must_use]
pub fn half_day_based_zman(
    start_of_half_day: &Zoned,
    end_of_half_day: &Zoned,
    hours: f64,
) -> Zoned {
    let shaah_zmanis = half_day_based_shaah_zmanis(start_of_half_day, end_of_half_day);
    let offset = SignedDuration::from_nanos((shaah_zmanis.as_nanos() as f64 * hours) as i64);
    if hours >= 0.0 {
        start_of_half_day.add(offset)
    } else {
        end_of_half_day.add(offset)
    }
}

/// The way a *zman* is offset from a reference event such as sunrise or sunset.
///
/// Different calculation methods express their offset in different units -
/// astronomical degrees below the horizon, fixed clock minutes, or temporal
/// (*zmaniyos*) minutes - and this enum captures which one applies.
#[derive(Debug, Clone, PartialEq)]
pub enum ZmanOffset {
    /// Degrees below the horizon.
    Degrees(f64),

    /// Fixed clock minutes before/after sunrise/sunset.
    Minutes(f64),

    /// Minutes *zmaniyos* (temporal minutes) before/after sunrise/sunset.
    MinutesZmaniyos {
        /// Number of minutes *zmaniyos*.
        minutes_zmaniyos: f64,
        /// Length of a *shaah zmanis* in "clock minutes". Each **minute
        /// *zmanis*** is 1/60 of this.
        shaah_zmanis: SignedDuration,
    },
}
