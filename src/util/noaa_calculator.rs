/// US National Oceanic and Atmospheric Administration Algorithm
use chrono::prelude::*;
use julian_day_converter::unixtime_to_julian_day;

use crate::util::astronomical_calculations;
use crate::util::geolocation::GeoLocation;

const JULIAN_DAY_JAN_1_2000: f64 = 2451545.0;
const JULIAN_DAYS_PER_CENTURY: f64 = 36525.0;

pub fn utc_sunrise(
    target_date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    zenith: f64,
    adjust_for_elevation: bool,
) -> f64 {
    utc_sun_position(
        target_date,
        geo_location,
        zenith,
        adjust_for_elevation,
        &Mode::Sunrise,
    )
}

pub fn utc_sunset(
    target_date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    zenith: f64,
    adjust_for_elevation: bool,
) -> f64 {
    utc_sun_position(
        target_date,
        geo_location,
        zenith,
        adjust_for_elevation,
        &Mode::Sunset,
    )
}

fn julian_centuries_from_julian_day(julian_day: f64) -> f64 {
    (julian_day - JULIAN_DAY_JAN_1_2000) / JULIAN_DAYS_PER_CENTURY
}

fn julian_day_from_julian_centuries(julian_centuries: f64) -> f64 {
    (julian_centuries * JULIAN_DAYS_PER_CENTURY) + JULIAN_DAY_JAN_1_2000
}

fn utc_sun_position(
    target_date: &DateTime<Utc>,
    geo_location: &GeoLocation,
    zenith: f64,
    adjust_for_elevation: bool,
    mode: &Mode,
) -> f64 {
    let elevation = if adjust_for_elevation {
        geo_location.elevation
    } else {
        0.0
    };
    let adjusted_zenith = astronomical_calculations::adjusted_zenith(zenith, elevation);
    let utc_time = calculate_utc_sun_position(
        unixtime_to_julian_day(target_date.timestamp()),
        geo_location.latitude,
        -geo_location.longitude,
        adjusted_zenith,
        mode,
    ); // in minutes
    (utc_time / 60.0) % 24.0 // in hours, normalized (0...24)
}

fn calculate_utc_sun_position(
    julian_day: f64,
    latitude: f64,
    longitude: f64,
    zenith: f64,
    mode: &Mode,
) -> f64 {
    let julian_centuries = julian_centuries_from_julian_day(julian_day);

    // first pass using solar noon
    let noonmin = solar_noon_utc(julian_centuries, longitude);
    let tnoon = julian_centuries_from_julian_day(julian_day + (noonmin / 1440.0));
    let first_pass = approximate_utc_sun_position(tnoon, latitude, longitude, zenith, mode);

    // refine using output of first pass
    let trefinement = julian_centuries_from_julian_day(julian_day + (first_pass / 1440.0));
    approximate_utc_sun_position(trefinement, latitude, longitude, zenith, mode)
}

fn approximate_utc_sun_position(
    approx_julian_centuries: f64,
    latitude: f64,
    longitude: f64,
    zenith: f64,
    mode: &Mode,
) -> f64 {
    let eq_time = equation_of_time(approx_julian_centuries);
    let solar_dec = solar_declination(approx_julian_centuries);
    let hour_angle = sun_hour_angle_at_horizon(latitude, solar_dec, zenith, mode);

    let delta = longitude - hour_angle.to_degrees();
    let time_delta = delta * 4.0;
    720.0 + time_delta - eq_time
}

fn sun_hour_angle_at_horizon(latitude: f64, solar_dec: f64, zenith: f64, mode: &Mode) -> f64 {
    let lat_r = latitude.to_radians();
    let solar_dec_r = solar_dec.to_radians();
    let zenith_r = zenith.to_radians();

    let mut hour_angle = ((zenith_r.cos() / (lat_r.cos() * solar_dec_r.cos()))
        - (lat_r.tan() * solar_dec_r.tan()))
    .acos();

    if Mode::Sunset == *mode {
        hour_angle *= -1.0;
    }
    hour_angle // in radians
}

fn solar_declination(julian_centuries: f64) -> f64 {
    let correction = obliquity_correction(julian_centuries).to_radians();
    let apparent_longitude = sun_apparent_longitude(julian_centuries).to_radians();
    let sint = correction.sin() * apparent_longitude.sin();
    sint.asin().to_degrees() // in degrees
}

fn sun_apparent_longitude(julian_centuries: f64) -> f64 {
    let true_longitude = sun_true_longitude(julian_centuries);
    let omega = 125.04 - (1934.136 * julian_centuries);
    true_longitude - 0.00569 - (0.00478 * omega.to_radians().sin()) // in degrees
}

fn sun_true_longitude(julian_centuries: f64) -> f64 {
    let sgml = sun_geometric_mean_longitude(julian_centuries);
    let center = sun_equation_of_center(julian_centuries);
    sgml + center // in degrees
}

fn sun_equation_of_center(julian_centuries: f64) -> f64 {
    let mrad = sun_geometric_mean_anomaly(julian_centuries).to_radians();
    let sinm = mrad.sin();
    let sin2m = (2.0 * mrad).sin();
    let sin3m = (3.0 * mrad).sin();

    (sinm * (1.914602 - (julian_centuries * (0.004817 + (0.000014 * julian_centuries)))))
        + (sin2m * (0.019993 - (0.000101 * julian_centuries)))
        + (sin3m * 0.000289) // in degrees
}

fn solar_noon_utc(julian_centuries: f64, longitude: f64) -> f64 {
    let century_start = julian_day_from_julian_centuries(julian_centuries);

    // first pass to yield approximate solar noon
    let approx_tnoon = julian_centuries_from_julian_day(century_start + (longitude / 360.0));
    let approx_eq_time = equation_of_time(approx_tnoon);
    let approx_sol_noon = 720.0 + (longitude * 4.0) - approx_eq_time;

    // refinement using output of first pass
    let tnoon = julian_centuries_from_julian_day(century_start - 0.5 + (approx_sol_noon / 1440.0));
    let eq_time = equation_of_time(tnoon);
    720.0 + (longitude * 4.0) - eq_time
}

fn equation_of_time(julian_centuries: f64) -> f64 {
    let epsilon = obliquity_correction(julian_centuries).to_radians();
    let sgml = sun_geometric_mean_longitude(julian_centuries).to_radians();
    let sgma = sun_geometric_mean_anomaly(julian_centuries).to_radians();
    let eoe = earth_orbit_eccentricity(julian_centuries);

    let mut y = (epsilon / 2.0).tan();
    y *= y;

    let sin2l0 = (2.0 * sgml).sin();
    let sin4l0 = (4.0 * sgml).sin();
    let cos2l0 = (2.0 * sgml).cos();
    let sinm = (sgma).sin();
    let sin2m = (2.0 * sgma).sin();

    let eq_time = (y * sin2l0) - (2.0 * eoe * sinm) + (4.0 * eoe * y * sinm * cos2l0)
        - (0.5 * y * y * sin4l0)
        - (1.25 * eoe * eoe * sin2m);
    eq_time.to_degrees() * 4.0 // minutes of time
}

fn earth_orbit_eccentricity(julian_centuries: f64) -> f64 {
    0.016708634 - (julian_centuries * (0.000042037 + (0.0000001267 * julian_centuries))) // unitless
}

fn sun_geometric_mean_anomaly(julian_centuries: f64) -> f64 {
    let anomaly = 357.52911 + (julian_centuries * (35999.05029 - (0.0001537 * julian_centuries))); // in degrees

    anomaly % 360.0 // normalized (0...360)
}

fn sun_geometric_mean_longitude(julian_centuries: f64) -> f64 {
    let longitude = 280.46646 + (julian_centuries * (36000.76983 + (0.0003032 * julian_centuries))); // in degrees

    longitude % 360.0 // normalized (0...360)
}

fn obliquity_correction(julian_centuries: f64) -> f64 {
    let obliquity_of_ecliptic = mean_obliquity_of_ecliptic(julian_centuries);

    let omega = 125.04 - (1934.136 * julian_centuries);
    let correction = obliquity_of_ecliptic + (0.00256 * omega.to_radians().cos());
    correction % 360.0 // normalized (0...360)
}

fn mean_obliquity_of_ecliptic(julian_centuries: f64) -> f64 {
    let seconds = 21.448
        - (julian_centuries
            * (46.8150 + (julian_centuries * (0.00059 - (julian_centuries * 0.001813)))));
    23.0 + ((26.0 + (seconds / 60.0)) / 60.0) // in degrees
}

#[derive(PartialEq)]
pub enum Mode {
    Sunrise,
    Sunset,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_obliquity_of_ecliptic() {
        assert_eq!(mean_obliquity_of_ecliptic(123.456), 22.77896577741124);
        assert_eq!(mean_obliquity_of_ecliptic(375.175), 45.13219969674679);
        assert_eq!(mean_obliquity_of_ecliptic(0.00345), 23.439246246734182);
    }

    #[test]
    fn test_obliquity_correction() {
        assert_eq!(obliquity_correction(123.456), 22.781298123556926);
        assert_eq!(obliquity_correction(375.175), 45.13114077427744);
        assert_eq!(obliquity_correction(0.00345), 23.438029936888373);
    }

    #[test]
    fn test_sun_geometric_mean_longitude() {
        assert_eq!(sun_geometric_mean_longitude(123.456), 236.12778008915484);
        assert_eq!(sun_geometric_mean_longitude(375.175), 71.96473453752697);
        assert_eq!(sun_geometric_mean_longitude(0.00345), 44.6691159171088);
    }

    #[test]
    fn test_sun_geometric_mean_anomaly() {
        assert_eq!(sun_geometric_mean_anomaly(123.456), 93.93911152798682);
        assert_eq!(sun_geometric_mean_anomaly(375.175), 159.58742041699588);
        assert_eq!(sun_geometric_mean_anomaly(0.00345), 121.72583349867057);
    }

    #[test]
    fn test_earth_orbit_eccentricity() {
        assert_eq!(earth_orbit_eccentricity(123.456), 0.0095878307833088);
        assert_eq!(earth_orbit_eccentricity(375.175), -0.016896418230187497);
        assert_eq!(earth_orbit_eccentricity(0.00345), 0.016708488970841952);
    }

    #[test]
    fn test_equation_of_time() {
        assert_eq!(equation_of_time(1.234), 2.594140502554649);
        assert_eq!(equation_of_time(543.21), 413.73066872728435);
    }

    #[test]
    fn test_solar_noon_utc() {
        assert_eq!(solar_noon_utc(1.234, 35.03), 857.5377415545428);
        assert_eq!(solar_noon_utc(9.876, 12.34), 775.5788864035511);
        assert_eq!(solar_noon_utc(0.043, -34.56), 580.6941934839474);
    }

    #[test]
    fn test_sun_equation_of_center() {
        assert_eq!(sun_equation_of_center(123.456), 1.1026068465371244);
        assert_eq!(sun_equation_of_center(375.175), -0.6378888936858589);
        assert_eq!(sun_equation_of_center(0.00345), 1.6106382351198698);
    }

    #[test]
    fn test_solar_declination() {
        assert_eq!(solar_declination(123.456), -18.999680822749635)
    }

    #[test]
    fn test_approximate_utc_sun_position() {
        assert_eq!(
            approximate_utc_sun_position(123.45, 31.78, 35.03, 91.57037635711369, &Mode::Sunset),
            1236.5426352099616
        );
        assert_eq!(
            approximate_utc_sun_position(45.45, 67.31, 3.35, 89.123, &Mode::Sunrise),
            564.659110379488
        );
    }

    #[test]
    fn test_calculate_utc_sun_position() {
        let (date, _) = date_loc();
        let julian_date = unixtime_to_julian_day(date.timestamp());
        let pos1 = calculate_utc_sun_position(julian_date, 67.31, 3.35, 89.123, &Mode::Sunset);
        assert_eq!(pos1, 1295.754354007962);
        let julian_date = julian_date + 456.987;
        let pos2 = calculate_utc_sun_position(julian_date, 45.31, 76.35, 92.567, &Mode::Sunrise);
        assert_eq!(pos2, 690.6400685857874);
    }

    #[test]
    fn test_utc_sun_position() {
        let (date, loc) = date_loc();
        let pos1 = utc_sun_position(&date, &loc, 90.0, true, &Mode::Sunrise);
        assert_eq!(pos1, 2.8371795341279333);
        let pos2 = utc_sun_position(&date, &loc, 90.0, false, &Mode::Sunset);
        assert_eq!(pos2, 16.63897816594255);
    }

    #[test]
    fn test_utc_sunset() {
        let (date, loc) = date_loc();
        let set1 = utc_sunset(&date, &loc, 90.0, true);
        assert_eq!(set1, 16.701593810321235);
        let set2 = utc_sunset(&date, &loc, 90.0, false);
        assert_eq!(set2, 16.63897816594255);
        let set3 = utc_sunset(&date, &loc, 95.678, false);
        assert_eq!(set3, 17.055106811966073);
    }

    #[test]
    fn test_utc_sunrise() {
        let (date, loc) = date_loc();
        let set1 = utc_sunrise(&date, &loc, 90.0, true);
        assert_eq!(set1, 2.8371795341279333);
        let set2 = utc_sunrise(&date, &loc, 90.0, false);
        assert_eq!(set2, 2.8999327653799707);
        let set3 = utc_sunrise(&date, &loc, 95.678, false);
        assert_eq!(set3, 2.48281688676995);
    }

    #[test]
    fn test_julian_centuries_from_julian_day() {
        assert_eq!(
            julian_centuries_from_julian_day(5678678.456),
            88.3540987268994
        );
        assert_eq!(julian_centuries_from_julian_day(0.056), -67.11964254620123);
    }

    #[test]
    fn test_julian_day_from_julian_centuries() {
        assert_eq!(julian_day_from_julian_centuries(-8.456), 2142689.6);
        assert_eq!(julian_day_from_julian_centuries(0.056), 2453590.4);
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
