//! Implementation of sunrise and sunset functions to calculate astronomical
//! times based on the NOAA algorithm.
//!
//! Astronomical events are calculated based on the NOAA's solar position
//! algorithm. NOAA's
//! [implementation](https://www.srrb.noaa.gov/highlights/sunrise/solareqns.PDF)
//! is based on equations from *Astronomical Algorithms* by Jean Meeus. Added to
//! the algorithm is an adjustment of the zenith to account for elevation. The
//! algorithm can be found in the [Wikipedia Sunrise
//! Equation](https://en.wikipedia.org/wiki/Sunrise_equation) article.

use std::{
    f64::consts::PI,
    ops::{Add, Sub},
};

use jiff::{Span, Zoned, civil::Date, tz::TimeZone};

use crate::util::{
    geolocation::GeoLocation,
    math_helper::{HOUR_MINUTES, HOUR_SECONDS, MINUTE_SECONDS, SECOND_NANOS},
    zenith_adjustments::adjusted_zenith,
};

// Julian Stuff
/// The Julian day of January 1, 2000, known as J2000.0
const JULIAN_DAY_JAN_1_2000: f64 = 2_451_545.0;

/// Julian days per century
const JULIAN_DAYS_PER_CENTURY: f64 = 36_525.0;

/// Returns the civil date adjusted for antimeridian crossover.
///
/// The "absolute time" calculations are based on the longitudinal offset from
/// UTC, so for time zones that cross the antimeridian (such as Samoa, which
/// uses UTC+14 while its longitude of ~-171&deg; would suggest UTC-11), the
/// date is shifted before calculating to conform with the presumption that the
/// date only increases east of the Prime Meridian.
fn antimeridian_adjusted_date(dt: &Zoned) -> Date {
    let offset = dt.offset().seconds() / HOUR_SECONDS as i32;
    if offset > 12 {
        // e.g. Samoa
        dt.sub(Span::new().days(1)).date()
    } else if offset < -12 {
        // nowhere currently AFAIK, but better safe than sorry  :)
        dt.add(Span::new().days(1)).date()
    } else {
        // most places
        dt.date()
    }
}

/// Returns the Julian day (at midnight) from a Zoned datetime
fn datetime_to_julian_day(dt: &Zoned) -> f64 {
    let date = antimeridian_adjusted_date(dt);

    let mut year = f64::from(date.year());
    let mut month = f64::from(date.month());
    let day = f64::from(date.day());
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
    let mean_lon = sun_geometric_mean_longitude(julian_centuries).to_radians();
    let mean_anom = sun_geometric_mean_anomaly(julian_centuries).to_radians();
    let eoe = earth_orbit_eccentricity(julian_centuries);

    let mut y = (epsilon / 2.0).tan();
    y *= y;

    let sin2l0 = (2.0 * mean_lon).sin();
    let sin4l0 = (4.0 * mean_lon).sin();
    let cos2l0 = (2.0 * mean_lon).cos();
    let sin_anom = (mean_anom).sin();
    let sin_2_anom = (2.0 * mean_anom).sin();

    let eq_time = (1.25 * eoe * eoe).mul_add(
        -sin_2_anom,
        (0.5 * y * y).mul_add(
            -sin4l0,
            (4.0 * eoe * y * sin_anom).mul_add(cos2l0, (y * sin2l0) - (2.0 * eoe * sin_anom)),
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
    let sin_2_anom = (2.0 * mrad).sin();
    let sin_3_anom = (3.0 * mrad).sin();

    sinm.mul_add(
        julian_centuries.mul_add(-0.000014f64.mul_add(julian_centuries, 0.004817), 1.914602),
        sin_2_anom * 0.000101f64.mul_add(-julian_centuries, 0.019993),
    ) + (sin_3_anom * 0.000289) // in degrees
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
    date: Date,
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

    let zoned = date.to_zoned(geo_location.timezone.clone()).ok()?;
    let adjusted_date = antimeridian_adjusted_date(&zoned);
    let adjusted_zenith = adjusted_zenith(zenith, elevation, adjusted_date);
    let julian_day = datetime_to_julian_day(&zoned);

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
    );

    normalize_time(time)
}

/// Converts a UTC time in minutes to fractional hours normalized into
/// `[0, 24)`, or `None` if the input is NaN.
fn normalize_time(minutes: f64) -> Option<f64> {
    let time = minutes / 60.0;
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

/// Returns the UTC of the current day's solar noon or the upcoming midnight
/// (about 12 hours after solar noon) of the given day at the given location on
/// earth.
fn utc_solar_noon_midnight(date: Date, geo_location: &GeoLocation, mode: &Mode) -> Option<f64> {
    let julian_day = datetime_to_julian_day(&date.to_zoned(geo_location.timezone.clone()).ok()?);
    let longitude = -geo_location.longitude;
    let base_minutes = if *mode == Mode::SunriseNoon {
        720.0
    } else {
        1440.0
    };

    // First pass for approximate solar noon to calculate equation of time.
    // Note that the first approximation deliberately omits `base_minutes`.
    let tnoon = julian_centuries_from_julian_day(julian_day + longitude / 360.0);
    let eot = equation_of_time(tnoon);
    let mut sol_noon_utc = longitude.mul_add(4.0, -eot); // minutes

    // two refinement passes
    for _ in 0..2 {
        let newt = julian_centuries_from_julian_day(julian_day + sol_noon_utc / 1440.0);
        let eot = equation_of_time(newt);
        if eot.is_nan() {
            return None;
        }
        sol_noon_utc = longitude.mul_add(4.0, base_minutes) - eot;
    }

    normalize_time(sol_noon_utc)
}

// degree-based trig helpers, in degrees as per NOAA convention

/// Sine of an angle given in degrees
fn sin_deg(deg: f64) -> f64 {
    deg.to_radians().sin()
}

/// Cosine of an angle given in degrees
fn cos_deg(deg: f64) -> f64 {
    deg.to_radians().cos()
}

/// Tangent of an angle given in degrees
fn tan_deg(deg: f64) -> f64 {
    deg.to_radians().tan()
}

/// Arc-cosine returning degrees
fn acos_deg(x: f64) -> f64 {
    x.acos().to_degrees()
}

/// Computes the sun's position at the given datetime and location, returning
/// the solar zenith angle (in degrees), the hour angle (in radians), and the
/// solar declination (in degrees). Shared by [`solar_elevation`] and
/// [`solar_azimuth`]. The position is based on sea level (it is not adjusted
/// for altitude).
fn solar_position(instant: &Zoned, loc: &GeoLocation) -> (f64, f64, f64) {
    let utc = instant.with_time_zone(TimeZone::UTC);
    let fractional_day = (f64::from(utc.hour())
        + (f64::from(utc.minute())
            + (f64::from(utc.second()) + f64::from(utc.subsec_nanosecond()) / SECOND_NANOS)
                / MINUTE_SECONDS)
            / HOUR_MINUTES)
        / 24.0;
    let julian_day = datetime_to_julian_day(&utc) + fractional_day;
    let julian_centuries = julian_centuries_from_julian_day(julian_day);
    let declination = solar_declination(julian_centuries);
    let eq_time = equation_of_time(julian_centuries);

    let true_solar_time =
        ((fractional_day + eq_time / 1_440.0 + loc.longitude / 360.0) + 2.0) % 1.0;
    let hour_angle = true_solar_time.mul_add(2.0 * PI, -PI);

    let cos_zenith = sin_deg(loc.latitude).mul_add(
        sin_deg(declination),
        cos_deg(loc.latitude) * cos_deg(declination) * hour_angle.cos(),
    );
    let zenith = acos_deg(cos_zenith.clamp(-1.0, 1.0));
    (zenith, hour_angle, declination)
}

/// Applies an atmospheric refraction adjustment to a solar `elevation` (in
/// degrees), returning the adjustment in degrees.
///
/// Note: this is a full atmospheric refraction model to report the sun's
/// apparent elevation, distinct from
/// [`zenith_adjustments::adjusted_zenith`](crate::util::zenith_adjustments::adjusted_zenith),
/// which applies a fixed refraction constant for adjusting the sunrise/sunset
/// zenith.
fn adjust_elevation_for_refraction(elevation: f64) -> f64 {
    if elevation > 85.0 {
        return 0.0;
    }

    let te = tan_deg(elevation);
    let correction = if elevation > 5.0 {
        (58.1 / te) - (0.07 / te.powi(3)) + (0.000086 / te.powi(5))
    } else if elevation > -0.575 {
        elevation.mul_add(
            elevation.mul_add(
                elevation.mul_add(0.711f64.mul_add(elevation, -12.79), 103.4),
                -518.2,
            ),
            1735.0,
        )
    } else {
        -20.774 / te
    };
    correction / 3_600.0
}

/// Returns the UTC (in hours) of the time when the sun reaches the given
/// `target_azimuth` for the given day at the given location. Only azimuths of
/// 90&deg; (directly east) and 270&deg; (directly west) are supported; any
/// other value is treated as 270&deg;.
///
/// Returns `None` when the azimuth is never reached for the date and location
/// (for example in the tropics, at the poles, or on the equator).
#[must_use]
pub fn utc_time_at_azimuth_90_or_270(
    date: Date,
    geo_location: &GeoLocation,
    target_azimuth: f64,
) -> Option<f64> {
    let julian_day = datetime_to_julian_day(&date.to_zoned(geo_location.timezone.clone()).ok()?);
    let solar_noon_base = 0.5 - (geo_location.longitude / 360.0);
    let quarter = if target_azimuth == 90.0 { 0.25 } else { 0.75 };
    let mut date_time = solar_noon_base + quarter;

    for _ in 0..3 {
        let julian_centuries = julian_centuries_from_julian_day(julian_day + date_time);
        // Handle Tropics, the Poles, and Equator line divisions
        let ratio = tan_deg(solar_declination(julian_centuries)) / tan_deg(geo_location.latitude);
        if ratio.is_nan() || !(-1.0..=1.0).contains(&ratio) {
            return None;
        }
        let sign = if target_azimuth == 90.0 { -1.0 } else { 1.0 };
        let offset = sign * (acos_deg(ratio) / 360.0);
        date_time = solar_noon_base + offset - (equation_of_time(julian_centuries) / 1_440.0);
    }

    Some((date_time * 24.0 % 24.0 + 24.0) % 24.0)
}

// public interface for utc_sun_rise_set
/// Returns the UTC of sunrise in hours, adjusting the zenith for refraction,
/// solar radius, and optionally elevation
#[must_use]
pub fn utc_sunrise(
    date: Date,
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

/// Returns the UTC of sunset in hours, adjusting the zenith for refraction,
/// solar radius, and optionally elevation
#[must_use]
pub fn utc_sunset(
    date: Date,
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

/// Returns the UTC of solar noon for the given day at the given location on
/// earth. This implementation returns true solar noon as opposed to the time
/// halfway between sunrise and sunset.
#[must_use]
pub fn utc_noon(date: Date, geo_location: &GeoLocation) -> Option<f64> {
    utc_solar_noon_midnight(date, geo_location, &Mode::SunriseNoon)
}

/// Returns the UTC of the solar midnight for the end of the given civil day at
/// the given location on earth (about 12 hours after solar noon). This
/// implementation returns true solar midnight as opposed to the time halfway
/// between sunrise and sunset.
#[must_use]
pub fn utc_midnight(date: Date, geo_location: &GeoLocation) -> Option<f64> {
    utc_solar_noon_midnight(date, geo_location, &Mode::SunsetMidnight)
}

/// Returns the solar elevation (in degrees) at the given datetime and
/// location. Can be negative if the sun is below the horizon.
#[must_use]
pub fn solar_elevation(instant: &Zoned, geo_location: &GeoLocation) -> f64 {
    let (zenith, _, _) = solar_position(instant, geo_location);
    let elevation = 90.0 - zenith;
    elevation + adjust_elevation_for_refraction(elevation)
}

/// Returns the solar azimuth (in degrees, clockwise from due north) at the
/// given datetime and location.
#[must_use]
pub fn solar_azimuth(instant: &Zoned, geo_location: &GeoLocation) -> f64 {
    let (zenith, hour_angle, declination) = solar_position(instant, geo_location);
    let az_denominator = cos_deg(geo_location.latitude) * sin_deg(zenith);
    let azimuth = if az_denominator.abs() > 0.001 {
        let az = sin_deg(geo_location.latitude).mul_add(cos_deg(zenith), -sin_deg(declination))
            / az_denominator;
        let sign = if hour_angle > 0.0 { -1.0 } else { 1.0 };
        acos_deg(az.clamp(-1.0, 1.0)).mul_add(-sign, 180.0)
    } else if geo_location.latitude > 0.0 {
        180.0
    } else {
        0.0
    };
    (azimuth + 360.0) % 360.0
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
