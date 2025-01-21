use crate::parsers;
use alloc::string::String;
use winnow_datetime::types::Duration;

/// Parses a duration string.
///
/// A string starts with `P` and can have one of the following formats:
///
/// * Fully-specified duration: `P1Y2M3DT4H5M6S`
/// * Duration in weekly intervals: `P1W`
/// * Fully-specified duration in [`DateTime`](`crate::DateTime`) format: `P<datetime>`
///
/// Both fully-specified formats get parsed into the YMDHMS Duration variant.
/// The weekly interval format gets parsed into the Weeks Duration variant.
///
/// The ranges for each of the individual units are not expected to exceed
/// the next largest unit.
///
/// These ranges (inclusive) are as follows:
///
/// * Year (any valid u32)
/// * Month 0 - 12
/// * Week 0 - 52
/// * Day 0 - 31
/// * Hour 0 - 24
/// * Minute 0 - 60
/// * Second 0 - 60
///
/// ## Examples
///
/// ```rust
/// let duration = winnow_iso8601::parse_duration("P1Y2M3DT4H5M6S").unwrap();
/// let duration = winnow_iso8601::parse_duration("P1W").unwrap();
/// ```
pub fn parse_duration(mut i: &str) -> Result<Duration, String> {
    match parsers::duration(&mut i) {
        Ok(p) => Ok(p),
        Err(e) => Err(format!("Failed to parse duration {}: {}", i, e)),
    }
}
