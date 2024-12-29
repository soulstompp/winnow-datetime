use core::str::FromStr;
use std::prelude::rust_2015::String;
use winnow_datetime::Timezone;
use winnow_datetime_assert::FormatCoverage;

/// Wrapper around a `Timezone` that implements `Display` and `FromStr` correctly for the standard.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Iso8601Timezone(Timezone);

impl From<Timezone> for Iso8601Timezone {
    fn from(tz: Timezone) -> Self {
        Iso8601Timezone(tz)
    }
}

impl FromStr for Iso8601Timezone {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        timezone(s)
    }
}

/// Parses a timezone offset string.
///
/// A timezone offset string is a combination of the valid formats specifying a time's UTC offset
///
/// This will accept (Z|+...|-...) as offsets
///
/// ## Example
///
/// ```rust
/// let dt = winnow_iso8601::timezone("Z").unwrap();
/// ```
pub fn timezone(mut i: &str) -> Result<Iso8601Timezone, String> {
    if let Ok(parsed) = crate::parsers::parse_timezone(&mut i) {
        Ok(Iso8601Timezone(parsed))
    } else {
        Err(format!("Failed to parse datetime: {}", i))
    }
}

#[cfg(feature = "testing")]
use winnow_datetime_assert::TimezoneCoverage;

#[cfg(feature = "testing")]
/// builds a list of assertions the date parser should pass
pub fn coverage() -> TimezoneCoverage {
    TimezoneCoverage {
        coverage: vec![
            FormatCoverage {
                format: "%Z:%z".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "Z".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "-00:00".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%Z".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%Z%z".into(),
                exception: Ok(None),
            },

        ],
    }
}
