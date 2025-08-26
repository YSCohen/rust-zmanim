//! Some constants that mostly exist to label the role of some number in the
//! rest of the code

/// Number of nanoseconds in a second
pub const SECOND_NANOS: f64 = 1_000_000_000.0;

/// Number of minutes in an hour
pub const MINUTE_SECONDS: f64 = 60.0;

/// Number of nanoseconds in a minute
pub const MINUTE_NANOS: f64 = SECOND_NANOS * 60.0;

/// Number of minutes in an hour
pub const HOUR_MINUTES: f64 = 60.0;

/// Number of seconds in an hour
pub const HOUR_SECONDS: f64 = HOUR_MINUTES * MINUTE_SECONDS;
