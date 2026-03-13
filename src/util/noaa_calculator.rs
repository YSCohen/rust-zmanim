//! Implementation of sunrise and sunset functions to calculate astronomical
//! times based on the NOAA algorithm.
//!
//! This calculator uses the algorithm based on the implementation by the
//! National Oceanic and Atmospheric Administration's Surface Radiation Research
//! Branch. NOAA's implementation is based on equations from *Astronomical
//! Algorithms* by Jean Meeus. Added to the algorithm is an adjustment of the
//! zenith to account for elevation. The algorithm can be found in the
//! [Wikipedia Sunrise Equation](https://en.wikipedia.org/wiki/Sunrise_equation)
//! article.

use std::ops::{Add, Sub};

use jiff::{Span, Zoned, civil::Date};

use crate::util::geolocation::GeoLocation;
use crate::util::math_helper::HOUR_SECONDS;
use crate::util::zenith_adjustments::adjusted_zenith;

// Julian Stuff
/// The Julian day of January 1, 2000, known as J2000.0
const JULIAN_DAY_JAN_1_2000: f64 = 2_451_545.0;

/// Julian days per century
const JULIAN_DAYS_PER_CENTURY: f64 = 36_525.0;

/// Return the Julian day (at midnight) from a Zoned datetime
fn datetime_to_julian_day(dt: &Zoned) -> f64 {
    // adjust for weird TZs
    let offset = dt.offset().seconds() / HOUR_SECONDS as i32;
    let date = if offset > 12 {
        // Samoa
        dt.sub(Span::new().days(1))
    } else if offset < -12 {
        // nowhere?
        dt.add(Span::new().days(1))
    } else {
        // most places
        dt.clone()
    };

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
    julian_centuries.mul_add(JULIAN_DAYS_PER_CENTURY, JULIAN_DAY_JAN_1_2000)
}

// some astronomical functions that stand on their own
/// Return the hour angle of the sun in radians for the given latitude.
fn sun_hour_angle_at_horizon(latitude: f64, solar_dec: f64, zenith: f64, mode: &Mode) -> f64 {
    let lat_r = latitude.to_radians();
    let solar_dec_r = solar_dec.to_radians();
    let zenith_r = zenith.to_radians();

    let mut hour_angle = lat_r
        .tan()
        .mul_add(
            -solar_dec_r.tan(),
            zenith_r.cos() / (lat_r.cos() * solar_dec_r.cos()),
        )
        .acos();

    if *mode == Mode::SunsetMidnight {
        hour_angle *= -1.0;
    }
    hour_angle // in radians
}

/// Return the unitless eccentricity of earth's orbit
fn earth_orbit_eccentricity(julian_centuries: f64) -> f64 {
    julian_centuries.mul_add(
        -0.0000001267f64.mul_add(julian_centuries, 0.000042037),
        0.016708634,
    )
    // unitless
}

/// Return the Geometric Mean Anomaly of the Sun in degrees
fn sun_geometric_mean_anomaly(julian_centuries: f64) -> f64 {
    let anomaly = julian_centuries.mul_add(
        0.0001537f64.mul_add(-julian_centuries, 35999.05029),
        357.52911,
    ); // in degrees
    anomaly % 360.0 // normalized (0...360)
}

/// Return the Geometric Mean Longitude of the Sun
fn sun_geometric_mean_longitude(julian_centuries: f64) -> f64 {
    let longitude = julian_centuries.mul_add(
        0.0003032f64.mul_add(julian_centuries, 36000.76983),
        280.46646,
    ); // in degrees
    longitude % 360.0 // normalized (0...360)
}

/// Return the mean obliquity of the ecliptic (Axial tilt)
fn mean_obliquity_of_ecliptic(julian_centuries: f64) -> f64 {
    let seconds = julian_centuries.mul_add(
        -julian_centuries.mul_add(julian_centuries.mul_add(-0.001813, 0.00059), 46.8150),
        21.448,
    );
    23.0 + ((26.0 + (seconds / 60.0)) / 60.0) // in degrees
}

// chain of functions relying on the ones before
/// Return the corrected obliquity of the ecliptic (Axial tilt)
fn obliquity_correction(julian_centuries: f64) -> f64 {
    let obliquity_of_ecliptic = mean_obliquity_of_ecliptic(julian_centuries);

    let omega = 1_934.136f64.mul_add(-julian_centuries, 125.04);
    let correction = 0.00256f64.mul_add(omega.to_radians().cos(), obliquity_of_ecliptic);
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

    let eq_time = (1.25 * eoe * eoe).mul_add(
        -sin2m,
        (0.5 * y * y).mul_add(
            -sin4l0,
            (4.0 * eoe * y * sinm).mul_add(cos2l0, (y * sin2l0) - (2.0 * eoe * sinm)),
        ),
    );
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
    let approx_sol_noon = longitude.mul_add(4.0, 720.0) - approx_eq_time;

    // refinement using output of first pass
    let tnoon = julian_centuries_from_julian_day(century_start - 0.5 + (approx_sol_noon / 1_440.0));
    let eq_time = equation_of_time(tnoon);
    longitude.mul_add(4.0, 720.0) - eq_time
}

/// Return the equation of center for the sun in degrees
fn sun_equation_of_center(julian_centuries: f64) -> f64 {
    let mrad = sun_geometric_mean_anomaly(julian_centuries).to_radians();
    let sinm = mrad.sin();
    let sin2m = (2.0 * mrad).sin();
    let sin3m = (3.0 * mrad).sin();

    sinm.mul_add(
        julian_centuries.mul_add(-0.000014f64.mul_add(julian_centuries, 0.004817), 1.914602),
        sin2m * 0.000101f64.mul_add(-julian_centuries, 0.019993),
    ) + (sin3m * 0.000289) // in degrees
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
    let omega = 1_934.136f64.mul_add(-julian_centuries, 125.04);
    0.00478f64.mul_add(-omega.to_radians().sin(), true_longitude - 0.00569) // in degrees
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
/// [`utc_sun_rise_set`] for accuracy.
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
/// zenith](crate::util::zenith_adjustments::adjusted_zenith) for refraction,
/// solar radius, and optionally elevation).
fn utc_sun_rise_set(
    date: &Date,
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
    let julian_day = datetime_to_julian_day(&date.to_zoned(geo_location.timezone.clone()).unwrap());

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

    let time = approximate_utc_sun_position(
        trefinement,
        geo_location.latitude,
        -geo_location.longitude,
        adjusted_zenith,
        mode,
    ) / 60.0;

    if time.is_nan() {
        None
    } else if time > 0.0 {
        // ensure that the time is < 24
        Some(time % 24.0)
    } else {
        // and >= 0
        Some(time % 24.0 + 24.0)
    }
}

// public interface for utc_sun_rise_set
/// Get the UTC of sunrise in hours, adjusting the zenith for refraction, solar
/// radius, and optionally elevation
#[must_use]
pub fn utc_sunrise(
    date: &Date,
    geo_location: &GeoLocation,
    zenith: f64,
    adjust_for_elevation: bool,
) -> Option<f64> {
    utc_sun_rise_set(
        date,
        geo_location,
        zenith,
        adjust_for_elevation,
        &Mode::SunriseNoon,
    )
}

/// Get the UTC of sunset in hours, adjusting the zenith for refraction, solar
/// radius, and optionally elevation
#[must_use]
pub fn utc_sunset(
    date: &Date,
    geo_location: &GeoLocation,
    zenith: f64,
    adjust_for_elevation: bool,
) -> Option<f64> {
    utc_sun_rise_set(
        date,
        geo_location,
        zenith,
        adjust_for_elevation,
        &Mode::SunsetMidnight,
    )
}

// noon and midnight
/// Return the UTC of the current day's solar noon or the upcoming midnight
/// (about 12 hours after solar noon) of the given day at the given location on
/// earth.
fn utc_solar_noon_midnight(date: &Date, geo_location: &GeoLocation, mode: &Mode) -> Option<f64> {
    let mut julian_day =
        datetime_to_julian_day(&date.to_zoned(geo_location.timezone.clone()).unwrap());
    let longitude = -geo_location.longitude;

    if *mode == Mode::SunsetMidnight {
        julian_day += 0.5;
    }
    // First pass for approximate solar noon to calculate equation of time
    let tnoon = julian_centuries_from_julian_day(julian_day + longitude / 360.0);
    let eot = equation_of_time(tnoon);
    let sol_noon_utc = longitude.mul_add(4.0, -eot); // minutes

    // second pass
    let newt = julian_centuries_from_julian_day(julian_day + sol_noon_utc / 1440.0);
    let eot = equation_of_time(newt);
    if eot.is_nan() {
        None
    } else {
        let minutes = longitude.mul_add(
            4.0,
            if *mode == Mode::SunriseNoon {
                720.0
            } else {
                1440.0
            },
        ) - eot;

        let time = minutes / 60.0;
        Some(if time > 0.0 {
            time % 24.0
        } else {
            (time % 24.0) + 24.0
        })
    }
}

/// Return the UTC of solar noon for the given day at the given location on
/// earth. This implementation returns true solar noon as opposed to the time
/// halfway between sunrise and sunset. Other calculators may return a more
/// simplified calculation of halfway between sunrise and sunset.
#[must_use]
pub fn utc_noon(date: &Date, geo_location: &GeoLocation) -> Option<f64> {
    utc_solar_noon_midnight(date, geo_location, &Mode::SunriseNoon)
}

/// Return the UTC of the solar midnight for the end of the given civil day at
/// the given location on earth (about 12 hours after solar noon). This
/// implementation returns true solar midnight as opposed to the time halfway
/// between sunrise and sunset. Other calculators may return a more simplified
/// calculation of halfway between sunrise and sunset.
#[must_use]
pub fn utc_midnight(date: &Date, geo_location: &GeoLocation) -> Option<f64> {
    utc_solar_noon_midnight(date, geo_location, &Mode::SunsetMidnight)
}

/// Used internally to specify which solar event should be calculated, to a
/// function that calculates both.
#[derive(PartialEq)]
enum Mode {
    /// Calculate sunrise (for `utc_sun_rise_set`) or noon (for
    /// `utc_solar_noon_midnight`)
    SunriseNoon,
    /// Calculate sunset (for `utc_sun_rise_set`) or midnight (for
    /// `utc_solar_noon_midnight`)
    SunsetMidnight,
}
