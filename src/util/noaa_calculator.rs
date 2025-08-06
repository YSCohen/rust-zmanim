//! Implementation of sunrise and sunset functions to calculate astronomical
//! times based on the NOAA algorithm.
//!
//! This calculator uses the algorithm based on the implementation by the
//! National Oceanic and Atmospheric Administration's Surface Radiation Research
//! Branch. NOAA's implementation is based on equations from *Astronomical
//! Algorithms* by Jean Meeus. Added to the algorithm is an adjustment of the
//! zenith to account for elevation. The algorithm can be found in the
//! <a href="https://en.wikipedia.org/wiki/Sunrise_equation">Wikipedia Sunrise Equation</a>
//! article

use chrono::prelude::*;
use chrono_tz::Tz;

use crate::util::astronomical_basics::adjusted_zenith;
use crate::util::geolocation::GeoLocation;

// Julian Stuff
/// The Julian day of January 1, 2000, known as J2000.0
const JULIAN_DAY_JAN_1_2000: f64 = 2_451_545.0;

/// Julian days per century
const JULIAN_DAYS_PER_CENTURY: f64 = 36_525.0;

/// Return the Julian day (at midnight) from a DateTime
fn datetime_to_julian_day(date: &DateTime<Tz>) -> f64 {
    // let date  = date.with_timezone(&Tz::UTC);
    let mut year = date.year() as f64;
    let mut month = date.month() as f64;
    let day = date.day() as f64;
    if month <= 2.0 {
        year -= 1.0;
        month += 12.0;
    }
    let a = (year / 100.0).floor();
    let b = (2.0 - a + a / 4.0).floor();
    (365.25 * (year + 4716.0)).floor() + (30.6001 * (month + 1.0)).floor() + day + b - 1524.5
}

/// Convert Julian day to centuries since J2000.0
fn julian_centuries_from_julian_day(julian_day: f64) -> f64 {
    (julian_day - JULIAN_DAY_JAN_1_2000) / JULIAN_DAYS_PER_CENTURY
}

/// Convert centuries since J2000.0 to Julian day
fn julian_day_from_julian_centuries(julian_centuries: f64) -> f64 {
    (julian_centuries * JULIAN_DAYS_PER_CENTURY) + JULIAN_DAY_JAN_1_2000
}

// some astronomical functions that stand on their own
/// Return the hour angle of the sun in radians at for the latitude
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

/// Return the unitless eccentricity of earth's orbit
fn earth_orbit_eccentricity(julian_centuries: f64) -> f64 {
    0.016708634 - (julian_centuries * (0.000042037 + (0.0000001267 * julian_centuries))) // unitless
}

/// Return the Geometric Mean Anomaly of the Sun in degrees
fn sun_geometric_mean_anomaly(julian_centuries: f64) -> f64 {
    let anomaly = 357.52911 + (julian_centuries * (35999.05029 - (0.0001537 * julian_centuries))); // in degrees
    anomaly % 360.0 // normalized (0...360)
}

/// Return the Geometric Mean Longitude of the Sun
fn sun_geometric_mean_longitude(julian_centuries: f64) -> f64 {
    let longitude = 280.46646 + (julian_centuries * (36000.76983 + (0.0003032 * julian_centuries))); // in degrees
    longitude % 360.0 // normalized (0...360)
}

/// Return the mean obliquity of the ecliptic (Axial tilt)
fn mean_obliquity_of_ecliptic(julian_centuries: f64) -> f64 {
    let seconds = 21.448
        - (julian_centuries
            * (46.8150 + (julian_centuries * (0.00059 - (julian_centuries * 0.001813)))));
    23.0 + ((26.0 + (seconds / 60.0)) / 60.0) // in degrees
}

// chain of functions relying on the ones before
/// Return the corrected obliquity of the ecliptic (Axial tilt)
fn obliquity_correction(julian_centuries: f64) -> f64 {
    let obliquity_of_ecliptic = mean_obliquity_of_ecliptic(julian_centuries);

    let omega = 125.04 - (1_934.136 * julian_centuries);
    let correction = obliquity_of_ecliptic + (0.00256 * omega.to_radians().cos());
    correction % 360.0 // normalized (0...360)
}

/// Return the Equation of Time - the difference between true solar time and
/// mean solar time
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

/// Return the UTC of solar noon for the given day at the given location on
/// earth. This implementation returns true solar noon as opposed to the time
/// halfway between sunrise and sunset. Other calculators may return a more
/// simplified calculation of halfway between sunrise and sunset
fn solar_noon_utc(julian_centuries: f64, longitude: f64) -> f64 {
    let century_start = julian_day_from_julian_centuries(julian_centuries);

    // first pass to yield approximate solar noon
    let approx_tnoon = julian_centuries_from_julian_day(century_start + (longitude / 360.0));
    let approx_eq_time = equation_of_time(approx_tnoon);
    let approx_sol_noon = 720.0 + (longitude * 4.0) - approx_eq_time;

    // refinement using output of first pass
    let tnoon = julian_centuries_from_julian_day(century_start - 0.5 + (approx_sol_noon / 1_440.0));
    let eq_time = equation_of_time(tnoon);
    720.0 + (longitude * 4.0) - eq_time
}

/// Return the equation of center for the sun in degrees
fn sun_equation_of_center(julian_centuries: f64) -> f64 {
    let mrad = sun_geometric_mean_anomaly(julian_centuries).to_radians();
    let sinm = mrad.sin();
    let sin2m = (2.0 * mrad).sin();
    let sin3m = (3.0 * mrad).sin();

    (sinm * (1.914602 - (julian_centuries * (0.004817 + (0.000014 * julian_centuries)))))
        + (sin2m * (0.019993 - (0.000101 * julian_centuries)))
        + (sin3m * 0.000289) // in degrees
}

/// Return the true longitude of the sun, in degrees
fn sun_true_longitude(julian_centuries: f64) -> f64 {
    let sgml = sun_geometric_mean_longitude(julian_centuries);
    let center = sun_equation_of_center(julian_centuries);
    sgml + center // in degrees
}

/// Return the apparent longitude of the sun, in degrees
fn sun_apparent_longitude(julian_centuries: f64) -> f64 {
    let true_longitude = sun_true_longitude(julian_centuries);
    let omega = 125.04 - (1_934.136 * julian_centuries);
    true_longitude - 0.00569 - (0.00478 * omega.to_radians().sin()) // in degrees
}

/// Return the declination of the sun, in degrees
fn solar_declination(julian_centuries: f64) -> f64 {
    let correction = obliquity_correction(julian_centuries).to_radians();
    let apparent_longitude = sun_apparent_longitude(julian_centuries).to_radians();
    let sint = correction.sin() * apparent_longitude.sin();
    sint.asin().to_degrees() // in degrees
}

/// Return the approximate UTC in minutes of a given sun position for the given
/// day at the given location on earth. Used twice within
/// [calculate_utc_sun_position] for accuracy
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

/// Return the UTC in minutes of a given sun position for the given day at the
/// given location on earth, ([adjusts the
/// zenith](astronomical_basics::adjusted_zenith) for refraction, solar radius
/// and optionally elevation)
fn utc_sun_position(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    zenith: f64,
    adjust_for_elevation: bool,
    mode: &Mode,
) -> Option<f64> {
    let elevation = if adjust_for_elevation {
        geo_location.elevation
    } else {
        0.0
    };

    let adjusted_zenith = adjusted_zenith(zenith, elevation);
    let julian_day = datetime_to_julian_day(date);

    // first pass using solar noon
    let noonmin = solar_noon_utc(
        julian_centuries_from_julian_day(julian_day),
        -geo_location.longitude,
    );
    let tnoon = julian_centuries_from_julian_day(julian_day + (noonmin / 1_440.0));
    let first_pass = approximate_utc_sun_position(
        tnoon,
        geo_location.latitude,
        -geo_location.longitude,
        adjusted_zenith,
        mode,
    );

    // refine using output of first pass
    let trefinement = julian_centuries_from_julian_day(julian_day + (first_pass / 1_440.0));

    let tim = approximate_utc_sun_position(
        trefinement,
        geo_location.latitude,
        -geo_location.longitude,
        adjusted_zenith,
        mode,
    ) / 60.0;

    if tim.is_nan() {
        None
    } else {
        Some(if tim > 0.0 {
            tim % 24.0
        } else {
            tim % 24.0 + 24.0
        })
    } // ensure that the time is >= 0 and < 24
}

// public interface for utc_sun_position
/// Get the UTC of sunrise in hours, adjusting the zenith for refraction, solar
/// radius, and optionally elevation
pub fn utc_sunrise(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    zenith: f64,
    adjust_for_elevation: bool,
) -> Option<f64> {
    utc_sun_position(
        date,
        geo_location,
        zenith,
        adjust_for_elevation,
        &Mode::Sunrise,
    )
}

/// Get the UTC of sunset in hours, adjusting the zenith for refraction, solar
/// radius, and optionally elevation
pub fn utc_sunset(
    date: &DateTime<Tz>,
    geo_location: &GeoLocation,
    zenith: f64,
    adjust_for_elevation: bool,
) -> Option<f64> {
    utc_sun_position(
        date,
        geo_location,
        zenith,
        adjust_for_elevation,
        &Mode::Sunset,
    )
}

#[derive(PartialEq)]
pub enum Mode {
    Sunrise,
    Sunset,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::Asia::Jerusalem;

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
    fn test_datetime_to_julian_day() {
        let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
        let jd1 = datetime_to_julian_day(&date1);
        assert_eq!(jd1, 2460891.5);

        let date2 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 6, 7, 8).unwrap();
        let jd2 = datetime_to_julian_day(&date2);
        assert_eq!(jd2, 2460891.5);

        let date3 = Jerusalem.with_ymd_and_hms(2025, 1, 26, 0, 0, 0).unwrap();
        let jd3 = datetime_to_julian_day(&date3);
        assert_eq!(jd3, 2460701.5);

        let date4 = Jerusalem.with_ymd_and_hms(2005, 5, 15, 0, 0, 0).unwrap();
        let jd4 = datetime_to_julian_day(&date4);
        assert_eq!(jd4, 2453505.5);
    }

    #[test]
    fn test_utc_sunset() {
        let loc = GeoLocation {
            latitude: 31.79388,
            longitude: 35.03684,
            elevation: 586.19,
            timezone: Jerusalem,
        };

        let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
        let set1 = utc_sunset(&date1, &loc, 90.0, false).unwrap();
        assert_eq!(set1, 16.56541683664536);

        let date2 = Jerusalem.with_ymd_and_hms(2025, 1, 26, 0, 0, 0).unwrap();
        let set2 = utc_sunset(&date2, &loc, 90.0, false).unwrap();
        assert_eq!(set2, 15.14486178521462);

        let date3 = Jerusalem.with_ymd_and_hms(2005, 5, 15, 0, 0, 0).unwrap();
        let set3 = utc_sunset(&date3, &loc, 90.0, false).unwrap();
        assert_eq!(set3, 16.495841422194637);
    }

    #[test]
    fn test_utc_sunrise() {
        let loc = GeoLocation {
            latitude: 31.79388,
            longitude: 35.03684,
            elevation: 586.19,
            timezone: Jerusalem,
        };

        let date1 = Jerusalem.with_ymd_and_hms(2025, 8, 4, 0, 0, 0).unwrap();
        let rise1 = utc_sunrise(&date1, &loc, 90.0, false).unwrap();
        assert_eq!(rise1, 2.959544300030197);

        let date2 = Jerusalem.with_ymd_and_hms(2025, 1, 26, 0, 0, 0).unwrap();
        let rise2 = utc_sunrise(&date2, &loc, 90.0, false).unwrap();
        assert_eq!(rise2, 4.607887155117016);

        let date3 = Jerusalem.with_ymd_and_hms(2005, 5, 15, 0, 0, 0).unwrap();
        let rise3 = utc_sunrise(&date3, &loc, 90.0, false).unwrap();
        assert_eq!(rise3, 2.7169504039525774);
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
}
