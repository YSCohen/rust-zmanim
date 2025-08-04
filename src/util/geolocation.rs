//! Trivial geolocation struct

#[derive(Debug)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub timezone: chrono_tz::Tz,
}
