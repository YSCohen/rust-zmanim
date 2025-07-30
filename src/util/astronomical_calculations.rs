//! Basic calculations used for all sun time calculation algorithms (though
//! currently only [the NOAA algorithm](super::noaa_calculator) is used)

/// 90&deg; below the vertical. Used as a basis for most calculations since the
/// location of the sun is 90&deg; below the vertical at sunrise and sunset.
///
/// **Note**: for sunrise and sunset the [adjusted zenith](adjusted_zenith) is
/// required to account for the radius of the sun and refraction. The adjusted
/// zenith should not be used for calculations above or below 90&deg; since they
/// are usually calculated as an offset to 90&deg;
pub const GEOMETRIC_ZENITH: f64 = 90.0;

/// 34 archminutes of refraction
pub const REFRACTION: f64 = 34.0 / 60.0;

/// 16 archminutes for the sun's radius in the sky
pub const SOLAR_RADIUS: f64 = 16.0 / 60.0;

/// The commonly used average earth radius in KM
pub const EARTH_RADIUS: f64 = 6_356.9;

/// Function to return the adjustment to the zenith required to account for the
/// elevation.
///
/// Since a person at a higher elevation can see farther below the
/// horizon, the calculation for sunrise / sunset is calculated below the
/// horizon used at sea level. The algorithm used is
///
/// <pre>
/// (EARTH_RADIUS / (EARTH_RADIUS + (elevation / 1_000.0))).acos().to_degrees()
/// </pre>
///
/// The source of this algorithm is *Calendrical Calculations* by Edward M.
/// Reingold and Nachum Dershowitz. An alternate algorithm that produces similar
/// (but not completely accurate) result found in *Ma'aglay Tzedek* by Moishe
/// Kosower and other sources is:
///
/// <pre>
/// 0.0347 * elevationMeters.sqrt();
/// </pre>
pub fn elevation_adjustment(elevation: f64) -> f64 {
    (EARTH_RADIUS / (EARTH_RADIUS + (elevation / 1_000.0)))
        .acos()
        .to_degrees()
}

/// Adjusts the zenith of astronomical sunrise and sunset to account for solar
/// refraction, solar radius and elevation.
///
/// The value for Sun's zenith and true
/// rise/set Zenith is the angle that the center of the Sun makes to a line
/// perpendicular to the Earth's surface. If the Sun were a point and the
/// Earth were without an atmosphere, true sunset and sunrise would correspond
/// to a 90&deg; zenith. Because the Sun is not a point, and because the
/// atmosphere refracts light, this 90&deg; zenith does not, in fact, correspond
/// to true sunset or sunrise, instead the center of the Sun's disk must lie
/// just below the horizon for the upper edge to be obscured. This means that a
/// zenith of just above 90&deg; must be used. The Sun subtends an angle of
/// [16 minutes of arc](SOLAR_RADIUS) and atmospheric refraction accounts for
/// [34 minutes](REFRACTION) or so, giving a total of 50 arcminutes. The total
/// value is therefore 90+(5/6) or 90.8333333&deg; for true sunrise/sunset.
/// Since a person at an elevation can see below the horizon of a person at sea
/// level, this will also adjust the zenith to account for elevation if
/// available. Note that this will only adjust the value if the zenith is
/// exactly 90 degrees. For values below and above this no correction is done
pub fn adjusted_zenith(zenith: f64, elevation: f64) -> f64 {
    if zenith != GEOMETRIC_ZENITH {
        return zenith;
    }
    zenith + SOLAR_RADIUS + REFRACTION + elevation_adjustment(elevation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjusted_zenith() {
        assert_eq!(adjusted_zenith(90.0, 123.4), 91.1903339839496);
        assert_eq!(adjusted_zenith(96.0, 234.5), 96.0);
        assert_eq!(adjusted_zenith(75.4, 543.2), 75.4);
    }

    #[test]
    fn test_elevation_adjustment() {
        assert_eq!(elevation_adjustment(526.0), 0.7370430237803639);
        assert_eq!(elevation_adjustment(100.0), 0.3213750031005735);
        assert_eq!(elevation_adjustment(0.0), 0.0);
        assert_eq!(elevation_adjustment(3.456789), 0.059751839197933074);
    }
}
