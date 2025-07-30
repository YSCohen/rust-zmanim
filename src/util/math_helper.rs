//! Some constants that would be easy and destructive to get wrong

/// Number of microseconds in a second
pub const SECOND_MICROS: f64 = 1_000_000.0;

/// Number of milliseconds in a minute
pub const MINUTE_MILLIS: f64 = 60.0 * 1_000.0;

/// Number of microseconds in a minute
pub const MINUTE_MICROS: f64 = SECOND_MICROS * 60.0;

/// Number of milliseconds in an hour
pub const HOUR_MILLIS: f64 = MINUTE_MILLIS * 60.0;

/// Number of milliseconds in a day
pub const DAY_MILLIS: f64 = HOUR_MILLIS * 24.0;

/// Number of microseconds in a day
pub const DAY_MICROS: f64 = DAY_MILLIS * 1_000.0;
