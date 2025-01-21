use crate::parsers;
use alloc::string::String;
use winnow_datetime::types::Duration;
use winnow_datetime::FractionalDuration;

/// Parses a duration string similiar to duration but allows for decimal places.
pub fn parse_duration(mut i: &str) -> Result<Duration, String> {
    match parsers::duration(&mut i) {
        Ok(p) => Ok(p),
        Err(e) => Err(format!("Failed to parse duration {}: {}", i, e)),
    }
}

/// let duration = winnow_iso8601::parse_fractional_duration("P1,5Y2M3DT4,5H5M6S").unwrap();
/// let duration = winnow_iso8601::parse_fractional_duration("P1,5W").unwrap();
pub fn parse_fractional_duration(mut i: &str) -> Result<FractionalDuration, String> {
    match parsers::fractional_duration(&mut i) {
        Ok(p) => Ok(p),
        Err(e) => Err(format!("Failed to parse duration {}: {}", i, e)),
    }
}
