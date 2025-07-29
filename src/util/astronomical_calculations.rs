pub const GEOMETRIC_ZENITH: f64 = 90.0;
pub const REFRACTION: f64 = 34.0 / 60.0;
pub const SOLAR_RADIUS: f64 = 16.0 / 60.0;
pub const EARTH_RADIUS: f64 = 6356.9; // km

pub fn elevation_adjustment(elevation: f64) -> f64 {
    (EARTH_RADIUS / (EARTH_RADIUS + (elevation / 1000.0)))
        .acos()
        .to_degrees()
}

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
