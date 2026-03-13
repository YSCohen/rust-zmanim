//! Basic calculations used for sun-time algorithms (though currently only
//! [the NOAA algorithm](super::noaa_calculator) is available).

/// 90&deg; below the vertical. Used as a basis for most calculations since the
/// location of the sun is 90&deg; below the vertical at sunrise and sunset.
const GEOMETRIC_ZENITH: f64 = 90.0;

/// 34 arcminutes of refraction
const REFRACTION: f64 = 34.0 / 60.0;

/// 16 arcminutes for the sun's radius in the sky
const SOLAR_RADIUS: f64 = 16.0 / 60.0;

/// The commonly used average earth radius in KM
const EARTH_RADIUS: f64 = 6_356.9;

/// Function to return the adjustment to the zenith required to account for the
/// elevation.
///
/// Since a person at a higher elevation can see farther below the horizon, the
/// calculation for sunrise / sunset is calculated below the horizon used at sea
/// level. The algorithm used is:
/// ```text
/// (EARTH_RADIUS / (EARTH_RADIUS + (elevation / 1_000.0))).acos().to_degrees()
/// ```
///
/// The source of this algorithm is *Calendrical Calculations* by Edward M.
/// Reingold and Nachum Dershowitz. An alternate algorithm that produces similar
/// (but not completely accurate) results found in *Ma'aglay Tzedek* by Moishe
/// Kosower and other sources is:
/// ```text
/// 0.0347 * elevation.sqrt();
/// ```
#[must_use]
pub fn elevation_adjustment(elevation: f64) -> f64 {
    (EARTH_RADIUS / (EARTH_RADIUS + (elevation / 1_000.0)))
        .acos()
        .to_degrees()
}

/// Adjusts the zenith of astronomical sunrise and sunset to account for solar
/// refraction, solar radius, and elevation.
///
/// The value for the Sun's zenith and true rise/set zenith is the angle that
/// the center of the Sun makes to a line perpendicular to the Earth's surface.
/// If the Sun were a point and the Earth were without an atmosphere, true
/// sunset and sunrise would correspond to a 90&deg; zenith. Because the Sun is
/// not a point, and because the atmosphere refracts light, this 90&deg; zenith
/// does not, in fact, correspond to true sunset or sunrise, instead the center
/// of the Sun's disk must lie just below the horizon for the upper edge to be
/// obscured. This means that a zenith of just above 90&deg; must be used. The
/// Sun subtends an angle of 16 minutes of arc and atmospheric
/// refraction accounts for 34 minutes or so, giving a total of 50
/// arcminutes. The total value is therefore 90+(5/6) or 90.8333333&deg; for
/// true sunrise/sunset. Since a person at an elevation can see below the
/// horizon of a person at sea level, this will also adjust the zenith to
/// account for elevation if available. Note that this will only adjust the
/// value if the zenith is exactly 90 degrees. For values below and above this
/// no correction is done, as calculations above or below 90&deg; are usually
/// calculated as an offset to 90&deg;.
#[must_use]
pub fn adjusted_zenith(zenith: f64, elevation: f64) -> f64 {
    if zenith != GEOMETRIC_ZENITH {
        return zenith;
    }
    zenith + SOLAR_RADIUS + REFRACTION + elevation_adjustment(elevation)
}
