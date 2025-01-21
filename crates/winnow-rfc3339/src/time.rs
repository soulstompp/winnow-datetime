use crate::parsers;
use alloc::string::String;
use winnow_datetime::Time;

/// Parses a time string.
///
/// A string can have one of the following formats:
///
/// * `07:35:[00][.123]` or `0735[00][.123]`
/// * `07:35:[00][.123][(Z|(+|-)00:00)]`
/// * `0735[00][.123][(Z|(+|-)00:00)]`
/// * `0735[00][.123][(Z|(+|-)0000)]`
///
/// ## Example
///
/// ```rust
/// let time = winnow_rfc3339::parse_time("21:56:42Z").unwrap();
/// ```
pub fn parse_time(mut i: &str) -> Result<Time, String> {
    if let Ok(parsed) = parsers::time(&mut i) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse time: {}", i))
    }
}
