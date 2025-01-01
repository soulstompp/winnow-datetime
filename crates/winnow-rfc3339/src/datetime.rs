use crate::parsers;
use alloc::string::String;
use winnow_datetime::DateTime;

/// Parses a datetime string.
///
/// A datetime string is a combination of the valid formats for the date and time,
/// separated by a literal `T`.
/// See the respective functions for the correct format.
///
/// ## Example
///
/// ```rust
/// let dt = winnow_rfc3339::parse_datetime("2015-11-03T21:56:00Z").unwrap();
/// ```
pub fn parse_datetime(mut i: &str) -> Result<DateTime, String> {
    if let Ok(parsed) = parsers::parse_datetime(&mut i) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse datetime: {}", i))
    }
}
