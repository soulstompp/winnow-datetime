use std::prelude::rust_2015::String;
use winnow_datetime::Offset;
use winnow_datetime_assert::FormatCoverage;

/// Parses a offset offset string.
///
/// A offset offset string is a combination of the valid formats specifying a time's UTC offset
///
/// This will accept (Z|+...|-...) as offsets
///
/// ## Example
///
/// ```rust
/// let dt = winnow_iso8601::parse_offset("Z").unwrap();
/// ```
pub fn parse_offset(mut i: &str) -> Result<Option<Offset>, String> {
    if let Ok(parsed) = crate::parsers::offset(&mut i) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse datetime: {}", i))
    }
}

#[cfg(feature = "testing")]
use winnow_datetime_assert::OffsetCoverage;

#[cfg(feature = "testing")]
/// builds a list of assertions the date parser should pass
pub fn coverage() -> OffsetCoverage {
    OffsetCoverage {
        coverage: vec![
            FormatCoverage {
                format: "%Z:%z".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "Z".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "+00:00".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Z".into(),
                exception: Ok(None),
                complete: true,
            },
            FormatCoverage {
                format: "%Z%z".into(),
                exception: Ok(None),
                complete: true,
            },
        ],
    }
}
