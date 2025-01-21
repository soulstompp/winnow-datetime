use std::prelude::rust_2015::String;
use winnow_datetime::Offset;

/// Parses a offset offset string.
///
/// A offset offset string is a combination of the valid formats specifying a time's UTC offset
///
/// This will accept (Z|+...|-...) as offsets
///
/// ## Example
///
/// ```rust
/// let dt = winnow_rfc3339::parse_offset("Z").unwrap();
/// ```
pub fn parse_offset(mut i: &str) -> Result<Option<Offset>, String> {
    if let Ok(parsed) = crate::parsers::offset(&mut i) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse datetime: {}", i))
    }
}
