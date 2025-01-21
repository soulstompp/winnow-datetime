use crate::parsers;
use alloc::string::String;
use winnow::combinator::eof;
use winnow::combinator::terminated;
use winnow::Parser;
use winnow_datetime::Date;

/// Parses a date string.
///
/// A string can have one of the following formats:
///
/// * `2015-11-02` or `20151102`
/// * `2015-W45-01` or `2015W451`
/// * `2015-306` or `2015306`
///
/// ## Example
///
/// ```rust
/// let date = winnow_iso8601::parse_date("2015-11-02").unwrap();
/// ```
pub fn parse_date(mut i: &str) -> Result<Date, String> {
    if let Ok(parsed) = terminated(parsers::date, eof).parse_next(&mut i) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse date: {}", i))
    }
}
