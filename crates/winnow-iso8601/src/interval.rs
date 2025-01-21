use crate::parsers;
use alloc::string::String;
use winnow_datetime::types::Interval;

/// Parses an interval string.
///
/// A string that optionally starts with `R` and contains a combination of partial date-times in the
/// following permissible formats:
///
pub fn parse_interval(mut i: &str) -> Result<Interval, String> {
    match parsers::interval(&mut i) {
        Ok(p) => Ok(p),
        Err(e) => Err(format!("Failed to parse interval {}: {}", i, e)),
    }
}
