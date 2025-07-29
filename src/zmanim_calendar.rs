use chrono::{TimeDelta, prelude::*};

use crate::astronomical_calendar::*;
use crate::util::geolocation::GeoLocation;
use crate::util::math_helper::*;

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

pub fn hanetz(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    elevation_adjusted_sunrise(date, geo_location, use_elevation)
}

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

pub fn shkia(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    elevation_adjusted_sunset(date, geo_location, use_elevation)
}

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
        ZmanOffset::MinutesZmanis(minz) => {
            let sunset = elevation_adjusted_sunset(date, geo_location, use_elevation);
            offset_by_minutes_zmanis(&sunset, minz, geo_location, use_elevation)
        }
    }
}

pub fn tzais_72(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    tzais(date, geo_location, use_elevation, ZmanOffset::Minutes(72.0))
}

pub fn alos(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
    offset: ZmanOffset,
) -> DateTime<Utc> {
    match offset {
        ZmanOffset::Degrees(deg) => DateTime::from_timestamp(
            utc_sea_level_sunrise(date, GEOMETRIC_ZENITH + deg, geo_location) as i64,
            0,
        )
        .unwrap(),
        ZmanOffset::Minutes(min) => offset_by_minutes(
            &elevation_adjusted_sunrise(date, geo_location, use_elevation),
            -min,
        ),
        ZmanOffset::MinutesZmanis(minz) => offset_by_minutes_zmanis(
            &elevation_adjusted_sunrise(date, geo_location, use_elevation),
            -minz,
            geo_location,
            use_elevation,
        ),
    }
}

pub fn alos_72(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    alos(date, geo_location, use_elevation, ZmanOffset::Minutes(72.0))
}

pub fn chatzos(date: &DateTime<Utc>, geo_location: &GeoLocation) -> DateTime<Utc> {
    sun_transit(date, geo_location)
}

pub fn sof_zman_shma(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>) -> DateTime<Utc> {
    shaos_into_day(day_start, day_end, 3.0)
}

pub fn sof_zman_shma_gra(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    sof_zman_shma(
        &elevation_adjusted_sunrise(date, geo_location, use_elevation),
        &elevation_adjusted_sunset(date, geo_location, use_elevation),
    )
}

pub fn sof_zman_shma_mga(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    sof_zman_shma(
        &alos_72(date, geo_location, use_elevation),
        &tzais_72(date, geo_location, use_elevation),
    )
}

pub fn sof_zman_tfila(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>) -> DateTime<Utc> {
    shaos_into_day(day_start, day_end, 4.0)
}

pub fn sof_zman_tfila_gra(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    sof_zman_tfila(
        &elevation_adjusted_sunrise(date, geo_location, use_elevation),
        &elevation_adjusted_sunset(date, geo_location, use_elevation),
    )
}

pub fn sof_zman_tfila_mga(
    date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    sof_zman_tfila(
        &alos_72(date, geo_location, use_elevation),
        &tzais_72(date, geo_location, use_elevation),
    )
}

pub fn mincha_gedola(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>) -> DateTime<Utc> {
    shaos_into_day(day_start, day_end, 6.5)
}

pub fn mincha_ketana(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>) -> DateTime<Utc> {
    shaos_into_day(day_start, day_end, 9.5)
}

pub fn plag_hamincha(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>) -> DateTime<Utc> {
    shaos_into_day(day_start, day_end, 10.75)
}

pub fn shaah_zmanis(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>) -> f64 {
    temporal_hour(day_start, day_end)
}

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

fn shaos_into_day(day_start: &DateTime<Utc>, day_end: &DateTime<Utc>, shaos: f64) -> DateTime<Utc> {
    let shaah_zmanis = temporal_hour(day_start, day_end);
    offset_by_minutes(day_start, (shaah_zmanis / MINUTE_MILLIS) * shaos)
}

fn offset_by_minutes(time: &DateTime<Utc>, minutes: f64) -> DateTime<Utc> {
    *time + TimeDelta::microseconds((minutes * MINUTE_MICROS).round() as i64)
}

fn offset_by_minutes_zmanis(
    time: &DateTime<Utc>,
    minutes: f64,
    geo_location: &GeoLocation,
    use_elevation: bool,
) -> DateTime<Utc> {
    let shaah_zmanis_skew = shaah_zmanis_gra(time, geo_location, use_elevation) / HOUR_MILLIS;
    *time + TimeDelta::microseconds((minutes * MINUTE_MICROS * shaah_zmanis_skew).round() as i64)
}

pub enum ZmanOffset {
    Degrees(f64),
    Minutes(f64),
    MinutesZmanis(f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sof_zman_tfila() {
        let start = Utc.with_ymd_and_hms(2025, 7, 29, 6, 00, 00).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 7, 29, 18, 0, 00).unwrap();
        let result1 = format!("{}", sof_zman_tfila(&start, &end));
        assert_eq!(result1, "2025-07-29 10:00:00 UTC");

        let start = Utc.with_ymd_and_hms(2025, 7, 29, 5, 47, 29).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 7, 29, 19, 15, 42).unwrap();
        let result2 = format!("{}", sof_zman_tfila(&start, &end));
        assert_eq!(result2, "2025-07-29 10:16:53.333333 UTC");
    }

    #[test]
    fn test_shaos_into_day() {
        let start1 = Utc.with_ymd_and_hms(2025, 7, 29, 6, 00, 00).unwrap();
        let end1 = Utc.with_ymd_and_hms(2025, 7, 29, 18, 0, 00).unwrap();
        let result1 = format!("{}", shaos_into_day(&start1, &end1, 4.34));
        assert_eq!(result1, "2025-07-29 10:20:24 UTC");

        let start2 = Utc.with_ymd_and_hms(2025, 7, 29, 5, 47, 29).unwrap();
        let end2 = Utc.with_ymd_and_hms(2025, 7, 29, 19, 15, 42).unwrap();
        // let result2 = Utc.with_ymd_and_hms(2025, 7, 29, 10, 39, 47).unwrap();
        let result2 = format!("{}", shaos_into_day(&start2, &end2, 4.34));
        assert_eq!(result2, "2025-07-29 10:39:47.301667 UTC");
    }

    #[test]
    fn test_sof_zman_shma_gra() {
        let (date, loc) = date_loc();
        let result = format!("{}", sof_zman_shma_gra(&date, &loc, false));
        assert_eq!(result, "2025-07-29 06:20:04.898816 UTC");
    }

    #[test]
    fn test_sof_zman_tfila_gra() {
        let (date, loc) = date_loc();
        let result = format!("{}", sof_zman_tfila_gra(&date, &loc, false));
        assert_eq!(result, "2025-07-29 07:28:46.612436 UTC");
    }

    #[test]
    fn test_tzais() {
        let (date, loc) = date_loc();
        let result1 = format!("{}", tzais(&date, &loc, false, ZmanOffset::Degrees(6.0)));
        assert_eq!(result1, "2025-07-29 17:04:59.441464 UTC"); // cheated: added one microsecond
        let result2 = format!("{}", tzais(&date, &loc, false, ZmanOffset::Minutes(6.0)));
        assert_eq!(result2, "2025-07-29 16:44:20.321397 UTC");
        let result3 = format!(
            "{}",
            tzais(&date, &loc, false, ZmanOffset::MinutesZmanis(96.0))
        );
        assert_eq!(result3, "2025-07-29 18:28:15.063189 UTC");
    }

    #[test]
    fn test_chatzos() {
        let (date, loc) = date_loc();
        let result = format!("{}", chatzos(&date, &loc));
        assert_eq!(result, "2025-07-29 09:46:10.039676 UTC")
    }

    #[test]
    fn test_plag_hamincha() {
        let start = Utc.with_ymd_and_hms(2025, 7, 29, 6, 00, 00).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 7, 29, 18, 0, 00).unwrap();
        let result1 = format!("{}", plag_hamincha(&start, &end));
        assert_eq!(result1, "2025-07-29 16:45:00 UTC");

        let start = Utc.with_ymd_and_hms(2025, 7, 29, 5, 47, 29).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 7, 29, 19, 15, 42).unwrap();
        let result2 = format!("{}", plag_hamincha(&start, &end));
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
