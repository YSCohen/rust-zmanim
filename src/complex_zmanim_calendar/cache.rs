use std::cell::{OnceCell, RefCell};

use jiff::Zoned;

/// Lazily computed solar events for a single date/location, so that repeated
/// *zmanim* calculations on the same calendar reuse the underlying
/// astronomical results instead of re-running the solar position algorithm.
#[derive(Debug, Clone, Default)]
pub(crate) struct ZmanCache {
    pub(crate) elevation_sunrise: OnceCell<Option<Zoned>>,
    pub(crate) elevation_sunset: OnceCell<Option<Zoned>>,
    pub(crate) sea_level_sunrise: OnceCell<Option<Zoned>>,
    pub(crate) sea_level_sunset: OnceCell<Option<Zoned>>,
    pub(crate) solar_noon: OnceCell<Option<Zoned>>,
    pub(crate) solar_midnight: OnceCell<Option<Zoned>>,
    // Degree-based events, keyed by `f64::to_bits` of the degrees below
    // GEOMETRIC_ZENITH. Only ~25 distinct values are ever used, so a linear
    // scan over a Vec beats hashing.
    pub(crate) sunrise_by_degrees: RefCell<Vec<(u64, Option<Zoned>)>>,
    pub(crate) sunset_by_degrees: RefCell<Vec<(u64, Option<Zoned>)>>,
}
