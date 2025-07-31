//! An API that can calculate sunrise, sunset and Jewish *zmanim* (religious
//! times) for prayers and other Jewish religious duties.
//!
//! This module contains the main functionality of the [rust-zmanim](crate)
//! library. **Elevation based *zmanim* (even sunrise and sunset) should not be
//! used *lekula* without the guidance of a *posek***. According to Rabbi Dovid
//! Yehudah Bursztyn in his Zmanim Kehilchasam, 7th edition chapter 2, section 7
//! (pages 181-182) and section 9 (pages 186-187), no *zmanim* besides sunrise
//! and sunset should use elevation. However, Rabbi Yechiel Avrahom Zilber in
//! the Birur Halacha Vol. 6 Ch. 58 Pages 34 and 42 is of the opinion that
//! elevation should be accounted for in *zmanim* calculations. Related to this,
//! Rabbi Yaakov Karp in Shimush Zekeinim, Ch. 1, page 17 states that
//! obstructing horizons should be factored into *zmanim* calculations

use chrono::{TimeDelta, prelude::*};

use crate::astronomical_calculator::*;
use crate::util::geolocation::GeoLocation;
use crate::util::math_helper::*;

/// Returns [sea level sunrise](sea_level_sunrise) if `use_elevation` is false,
/// or [elevation-adjusted sunrise](crate::astronomical_calculator::sunrise) if
/// it is true. This allows relevant *zmanim* to automatically adjust to the
/// elevation setting
pub fn elevation_adjusted_sunrise(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    if use_elevation {
        sunrise(date, geo_location)
    } else {
        sea_level_sunrise(date, geo_location)
    }
}

/// An alias for [elevation_adjusted_sunrise()](elevation_adjusted_sunrise)
pub fn hanetz(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    elevation_adjusted_sunrise(date, geo_location, use_elevation)
}

/// Returns [sea level sunset](sea_level_sunset) if `use_elevation` is false, or
/// [elevation-adjusted
/// sunset](crate::astronomical_calculator::sea_level_sunset) if it is true.
/// This allows relevant *zmanim* to automatically adjust to the
/// elevation setting
pub fn elevation_adjusted_sunset(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    if use_elevation {
        sunset(date, geo_location)
    } else {
        sea_level_sunset(date, geo_location)
    }
}

/// An alias for [elevation_adjusted_sunset()](elevation_adjusted_sunset)
pub fn shkia(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    elevation_adjusted_sunset(date, geo_location, use_elevation)
}

/// Returns *tzais* (nightfall) based on either declination of the sun below the
/// horizon, a fixed time offset, or a minutes zmaniyos (temporal) offset after
/// sunset
pub fn tzais(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
    offset: ZmanOffset,
) -> DateTime<Utc> {
    match offset {
        ZmanOffset::Degrees(deg) => {
            sunset_offset_by_degrees(date, geo_location, GEOMETRIC_ZENITH + deg)
        }
        ZmanOffset::Minutes(min) => {
            let sunset = elevation_adjusted_sunset(date, geo_location, use_elevation);
            offset_by_minutes(&sunset, min)
        }
        ZmanOffset::MinutesZmaniyos {
            minutes_zmaniyos: minz,
            day_start: start,
            day_end: end,
        } => {
            let sunset = elevation_adjusted_sunset(date, geo_location, use_elevation);
            offset_by_minutes_zmanis(&sunset, minz, &start, &end)
        }
    }
}

/// Returns the *tzais* (nightfall) based on the opinion of *Rabbeinu Tam* that
/// *tzais hakochavim* is calculated as the time it takes to walk 4 *mil* at 18
/// minutes a mil.
///
/// According to the *Machtzis Hashekel* in *Orach Chaim* 235:3, the *Pri
/// Megadim* in *Orach Chaim* 261:2 (see the *Biur Halacha*) and others (see
/// *Hazmanim Bahalacha* 17:3 and 17:5), the 72 minutes are standard clock
/// minutes any time of the year in any location. Depending on `use_elevation`
/// setting, a 72-minute offset from either [sunset] or [sea level
/// sunset](sea_level_sunset) is used
pub fn tzais_72(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    tzais(date, geo_location, use_elevation, ZmanOffset::Minutes(72.0))
}

/// Returns *alos hashachar* (dawn) based on either declination of the sun below
/// the horizon, a fixed time offset, or a minutes zmaniyos (temporal) offset
/// before sunrise
pub fn alos(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
    offset: ZmanOffset,
) -> DateTime<Utc> {
    match offset {
        ZmanOffset::Degrees(deg) => DateTime::from_timestamp_micros(
            (utc_sea_level_sunrise(date, GEOMETRIC_ZENITH + deg, geo_location) * SECOND_MICROS)
                .round() as i64,
        )
        .unwrap(),
        ZmanOffset::Minutes(min) => offset_by_minutes(
            &elevation_adjusted_sunrise(date, geo_location, use_elevation),
            -min,
        ),
        ZmanOffset::MinutesZmaniyos {
            minutes_zmaniyos: minz,
            day_start: start,
            day_end: end,
        } => offset_by_minutes_zmanis(
            &elevation_adjusted_sunrise(date, geo_location, use_elevation),
            -minz,
            &start,
            &end,
        ),
    }
}

/// Function to return *alos* (dawn) calculated as 72 minutes before [sunrise]
/// or [sea level sunrise](sea_level_sunrise) (depending on `use_elevation`).
///
/// This time is based on the time to walk the distance of 4 *mil* at 18 minutes
/// per *mil*. The 72-minute time (but not the concept of fixed minutes) is
/// based on the opinion that the time of the *Neshef* (twilight between dawn
/// and sunrise) does not vary by the time of year or location but depends on
/// the time it takes to walk the distance of 4 mil
pub fn alos_72(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    alos(date, geo_location, use_elevation, ZmanOffset::Minutes(72.0))
}

/// Returns [Astronomical
/// *chatzos*](crate::astronomical_calculator::sun_transit).
pub fn chatzos(date: &DateTime<Utc>, geo_location: &GeoLocation) -> DateTime<Utc> {
    // TODO: implement half-day chatzos and add an option here
    sun_transit(date, geo_location)
}

/// A generic function for calculating the latest *zman krias shema* (time to
/// recite shema in the morning) that is 3 *shaos zmaniyos* (temporal hours)
/// after the start of the day, calculated using the start and end of the day
/// passed to this function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and the latest *zman krias shema* is calculated as 3 of those
/// *shaos zmaniyos* after the beginning of the day
pub fn sof_zman_shema(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>) -> DateTime<Utc> {
    shaos_into_day(day_start, day_end, 3.0)
}

/// Returns the latest *zman shema* (time to recite shema in the morning) that
/// is 3 *shaos zmaniyos* (solar hours) after [sunrise] or [sea level
/// sunrise](sea_level_sunrise) (depending on the `use_elevation`) according
/// GRA.
///
/// The day is calculated from [sea level sunrise](sea_level_sunrise) to [sea
/// level sunset](sea_level_sunset) or from [sunrise] to [sunset] (depending on
/// `use_elevation`)
pub fn sof_zman_shema_gra(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    sof_zman_shema(
        &elevation_adjusted_sunrise(date, geo_location, use_elevation),
        &elevation_adjusted_sunset(date, geo_location, use_elevation),
    )
}

/// Returns the latest *zman shema* (time to recite shema in the morning) that
/// is 3 *shaos zmaniyos* (solar hours) after *alos hashachar*, according to the
/// Magen Avraham (MGA).
///
/// The day is calculated from 72 minutes before [sea level
/// sunrise](sea_level_sunrise) to 72 minutes after [sea level
/// sunset](sea_level_sunset) or from 72 minutes before [sunrise] to 72 minutes
/// after [sunset] (depending on `use_elevation`)
pub fn sof_zman_shema_mga(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    sof_zman_shema(
        &alos_72(date, geo_location, use_elevation),
        &tzais_72(date, geo_location, use_elevation),
    )
}

/// A generic function for calculating the latest *zman tefila* (time to recite
/// *shacharis* in the morning) that is 4 *shaos zmaniyos* (temporal hours)
/// after the start of the day, calculated using the start and end of the day
/// passed to this function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and the latest *zman tefila* is calculated as 4 of those *shaos
/// zmaniyos* after the beginning of the day
pub fn sof_zman_tefila(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>) -> DateTime<Utc> {
    shaos_into_day(day_start, day_end, 4.0)
}

/// Returns the latest *zman tefila* (time to recite *shacharis* in the morning)
/// that is 4 *shaos zmaniyos* (solar hours) after [sunrise] or [sea level
/// sunrise](sea_level_sunrise) (depending on the `use_elevation`) according
/// GRA.
///
/// The day is calculated from [sea level sunrise](sea_level_sunrise) to [sea
/// level sunset](sea_level_sunset) or from sunrise to sunset (depending on
/// `use_elevation`)
pub fn sof_zman_tefila_gra(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    sof_zman_tefila(
        &elevation_adjusted_sunrise(date, geo_location, use_elevation),
        &elevation_adjusted_sunset(date, geo_location, use_elevation),
    )
}

/// Returns the latest *zman tefila* (time to recite *shacharis* in the morning)
/// that is 4 *shaos zmaniyos* (solar hours) after *alos hashachar*, according
/// to the Magen Avraham (MGA).
///
/// The day is calculated from 72 minutes before [sea level
/// sunrise](sea_level_sunrise) to 72 minutes after [sea level
/// sunset](sea_level_sunset) or from 72 minutes before [sunrise] to 72 minutes
/// after [sunset] (depending on `use_elevation`)
pub fn sof_zman_tefila_mga(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    sof_zman_tefila(
        &alos_72(date, geo_location, use_elevation),
        &tzais_72(date, geo_location, use_elevation),
    )
}

/// A generic function for calculating *mincha gedola* (earliest time to recite
/// *mincha* in the afternoon) that is 6.5 *shaos zmaniyos* (temporal hours)
/// after the start of the day, calculated using the start and end of the day
/// passed to this function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and *mincha gedola* is calculated as 6.5 of those *shaos
/// zmaniyos* after the beginning of the day
pub fn mincha_gedola(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>) -> DateTime<Utc> {
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
pub fn mincha_ketana(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>) -> DateTime<Utc> {
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
pub fn plag_hamincha(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>) -> DateTime<Utc> {
    shaos_into_day(day_start, day_end, 10.75)
}

/// An alias for
/// [temporal_hour()](crate::astronomical_calculator::temporal_hour)
pub fn shaah_zmanis(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>) -> f64 {
    temporal_hour(day_start, day_end)
}

/// Returns a *shaah zmanis* according to the opinion of the GRA.
///
/// This calculation divides the day based on the opinion of the *GRA* that the
/// day runs from from [sea level sunrise](sea_level_sunrise) to [sea level
/// sunset](sea_level_sunset) or [sunrise] to [sunset] (depending on
/// `use_elevation`). The day is split into 12 equal parts with each one being a
/// *shaah zmanis*
pub fn shaah_zmanis_gra(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> f64 {
    shaah_zmanis(
        &elevation_adjusted_sunrise(date, geo_location, use_elevation),
        &elevation_adjusted_sunset(date, geo_location, use_elevation),
    )
}

/// Returns a *shaah zmanis* according to the opinion of the Magen Avraham
/// (MGA).
///
/// This calculation divides the day based on the opinion of the *MGA* that the
/// day runs from from 72 minutes before [sea level sunrise](sea_level_sunrise)
/// to 72 minutes after [sea level sunset](sea_level_sunset) or 72 minutes
/// before [sunrise] to 72 minutes after [sunset] (depending on
/// `use_elevation`). The day is split into 12 equal parts with each one being a
/// *shaah zmanis*
pub fn shaah_zmanis_mga(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> f64 {
    shaah_zmanis(
        &alos_72(date, geo_location, use_elevation),
        &tzais_72(date, geo_location, use_elevation),
    )
}

/// A generic function for calculating a given number of *shaos zmaniyos*
/// (temporal hours) after the start of the day, calculated using the start and
/// end of the day passed to this function.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos*, and the returned `DateTime` is `shaos` of those *shaos zmaniyos*
/// after the beginning of the day
fn shaos_into_day(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>, shaos: f64) -> DateTime<Utc> {
    let shaah_zmanis = temporal_hour(day_start, day_end);
    offset_by_minutes(day_start, (shaah_zmanis / MINUTE_MILLIS) * shaos)
}

/// Returns a `DateTime` which is `minutes` minutes after `time`
fn offset_by_minutes(time: &DateTime<Utc>, minutes: f64) -> DateTime<Utc> {
    *time + TimeDelta::microseconds((minutes * MINUTE_MICROS).round() as i64)
}

/// Returns a `DateTime` which is `minutes` minutes *zmaniyos* (temporal
/// minutes) after `time`.
///
/// The time from the start of day to the end of day are divided into 12 *shaos
/// zmaniyos* and the returned `DateTime` is `minutes` 60ths of those *shaos
/// zmaniyos* after `time`
fn offset_by_minutes_zmanis(
    time: &DateTime<Utc>,
    minutes: f64,
    day_start: &DateTime<Utc>,
    day_end: &DateTime<Utc>,
) -> DateTime<Utc> {
    let shaah_zmanis_skew = shaah_zmanis(day_start, day_end) / HOUR_MILLIS;
    *time + TimeDelta::microseconds((minutes * MINUTE_MICROS * shaah_zmanis_skew).round() as i64)
}

pub enum ZmanOffset {
    Degrees(f64),
    Minutes(f64),
    MinutesZmaniyos {
        minutes_zmaniyos: f64,
        day_start: DateTime<Utc>,
        day_end: DateTime<Utc>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sof_zman_tefila() {
        let start = Utc.with_ymd_and_hms(2025, 7, 29, 6, 00, 00).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 7, 29, 18, 0, 00).unwrap();
        let result1 = sof_zman_tefila(&start, &end).to_string();
        assert_eq!(result1, "2025-07-29 10:00:00 UTC");

        let start = Utc.with_ymd_and_hms(2025, 7, 29, 5, 47, 29).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 7, 29, 19, 15, 42).unwrap();
        let result2 = sof_zman_tefila(&start, &end).to_string();
        assert_eq!(result2, "2025-07-29 10:16:53.333333 UTC");
    }

    #[test]
    fn test_shaos_into_day() {
        let start1 = Utc.with_ymd_and_hms(2025, 7, 29, 6, 00, 00).unwrap();
        let end1 = Utc.with_ymd_and_hms(2025, 7, 29, 18, 0, 00).unwrap();
        let result1 = shaos_into_day(&start1, &end1, 4.34).to_string();
        assert_eq!(result1, "2025-07-29 10:20:24 UTC");

        let start2 = Utc.with_ymd_and_hms(2025, 7, 29, 5, 47, 29).unwrap();
        let end2 = Utc.with_ymd_and_hms(2025, 7, 29, 19, 15, 42).unwrap();
        // let result2 = Utc.with_ymd_and_hms(2025, 7, 29, 10, 39, 47).unwrap();
        let result2 = shaos_into_day(&start2, &end2, 4.34).to_string();
        assert_eq!(result2, "2025-07-29 10:39:47.301667 UTC");
    }

    #[test]
    fn test_sof_zman_shema_gra() {
        let (date, loc) = date_loc();
        let result = sof_zman_shema_gra(&date, &loc, false).to_string();
        assert_eq!(result, "2025-07-29 06:20:04.898816 UTC");
    }

    #[test]
    fn test_sof_zman_tefila_gra() {
        let (date, loc) = date_loc();
        let result = sof_zman_tefila_gra(&date, &loc, false).to_string();
        assert_eq!(result, "2025-07-29 07:28:46.612436 UTC");
    }

    #[test]
    fn test_tzais() {
        let (date, loc) = date_loc();
        let result1 = tzais(&date, &loc, false, ZmanOffset::Degrees(6.0)).to_string();
        assert_eq!(result1, "2025-07-29 17:04:59.441464 UTC"); // cheated: added one microsecond
        let result2 = tzais(&date, &loc, false, ZmanOffset::Minutes(6.0)).to_string();
        assert_eq!(result2, "2025-07-29 16:44:20.321397 UTC");
        let start = sea_level_sunrise(&date, &loc);
        let end = sea_level_sunset(&date, &loc);
        let result3 = format!(
            "{}",
            tzais(
                &date,
                &loc,
                false,
                ZmanOffset::MinutesZmaniyos {
                    minutes_zmaniyos: 96.0,
                    day_start: start,
                    day_end: end
                }
            )
        );
        assert_eq!(result3, "2025-07-29 18:28:15.063189 UTC");
    }

    #[test]
    fn test_chatzos() {
        let (date, loc) = date_loc();
        let result = chatzos(&date, &loc).to_string();
        assert_eq!(result, "2025-07-29 09:46:10.039676 UTC")
    }

    #[test]
    fn test_plag_hamincha() {
        let start = Utc.with_ymd_and_hms(2025, 7, 29, 6, 00, 00).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 7, 29, 18, 0, 00).unwrap();
        let result1 = plag_hamincha(&start, &end).to_string();
        assert_eq!(result1, "2025-07-29 16:45:00 UTC");

        let start = Utc.with_ymd_and_hms(2025, 7, 29, 5, 47, 29).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 7, 29, 19, 15, 42).unwrap();
        let result2 = plag_hamincha(&start, &end).to_string();
        assert_eq!(result2, "2025-07-29 17:51:30.645833 UTC");
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
