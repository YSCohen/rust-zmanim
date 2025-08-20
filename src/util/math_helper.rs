//! Some constants that mostly exist to label the role of some number in the
//! rest of the code

/// Number of microseconds in a second
pub const SECOND_MICROS: f64 = 1_000_000.0;

/// Number of minutes in an hour
pub const MINUTE_SECONDS: f64 = 60.0;

/// Number of microseconds in a minute
pub const MINUTE_MICROS: f64 = SECOND_MICROS * 60.0;

/// Number of minutes in an hour
pub const HOUR_MINUTES: f64 = 60.0;

/// Number of seconds in an hour
pub const HOUR_SECONDS: f64 = HOUR_MINUTES * MINUTE_SECONDS;

/// Number of milliseconds in a second
pub const HOUR_MILLIS: f64 = MINUTE_SECONDS * 1_000.0;
