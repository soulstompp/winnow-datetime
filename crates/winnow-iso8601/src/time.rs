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
/// let time = winnow_iso8601::parse_time("21:56:42").unwrap();
/// ```
pub fn parse_time(mut i: &str) -> Result<Time, String> {
    if let Ok(parsed) = parsers::time(&mut i) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse time: {}", i))
    }
}

#[cfg(feature = "testing")]
use winnow_datetime_assert::{FormatCoverage, TimeCoverage};

#[cfg(feature = "testing")]
/// builds a list of assertions the date parser should pass
pub fn coverage() -> TimeCoverage {
    TimeCoverage {
        coverage: vec![
            FormatCoverage {
                format: "%h".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%,1h".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%.1h".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h:%m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h:%,1m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h:%.1m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h:%m:%s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h:%m:%.1s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h:%m:%.2s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h:%m:%,3s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h:%m:%.3s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h:%m:%s,%u".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h:%m:%s.%u".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%,1h".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%.1h".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h:%m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h:%,1m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h:%.1m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h:%m:%s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h:%m:%.1s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h:%m:%.2s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h:%m:%,3s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h:%m:%.3s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h:%m:%s,%u".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h:%m:%s.%u".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h%m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h%,1m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h%.1m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h%m%s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h%m%.1s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h%m%.2s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h%m%,3s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h%m%.3s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h%m%s,%u".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%h%m%s.%u".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h%m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h%,1m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h%.1m".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h%m%s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h%m%.1s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h%m%.2s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h%m%,3s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h%m%.3s".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h%m%s,%u".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "T%h%m%s.%u".into(),
                exception: Ok(None),
                complete: true,
            },
        ],
        separators: vec![None],
        timezone_coverage: crate::offset::coverage(),
    }
}
