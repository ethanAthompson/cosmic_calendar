/// Sum wrapper for timezones
///
/// limit: -12 <- 0 -> 12
///
/// desc: each variant has a utc offset value (limit)
///

/// Timezones:
///
/// EARTH
///

/// Pacific Standard Time (North America)
pub const PST: i32 = 8 * 3600;

/// Philippine Standard Time
pub const PSTP: i32 = -8 * 3600;

/// Eastern Standard Time
pub const EST: i32 = -5 * 3600;

/// Niue Standard Time
pub const NUT: i32 = -11 * 3600;

// UTC times
// Western European Time
// Greenwich Mean Time

/// Timezones:
///
/// Inner Planets
///

pub const MYT: i32 = 0;
