use crate::parsers;
use alloc::string::String;
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
/// let date = winnow_rfc3339::parse_date("2015-11-02").unwrap();
/// ```
pub fn parse_date(mut i: &str) -> Result<Date, String> {
    if let Ok(parsed) = parsers::date(&mut i) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse date: {}", i))
    }
}

#[cfg(feature = "testing")]
use winnow_datetime_assert::DateCoverage;
#[cfg(feature = "testing")]
use winnow_datetime_assert::FormatCoverage;

#[cfg(feature = "testing")]
/// builds a list of assertions the date parser should pass
pub fn coverage() -> DateCoverage {
    DateCoverage {
        coverage: vec![FormatCoverage {
            format: "%Y-%M-%D".into(),
            exception: Ok(None),
            complete: true,
        }],
    }
}
