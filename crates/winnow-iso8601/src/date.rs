use crate::parsers;
use alloc::string::String;
use core::fmt;
use core::fmt::Display;
use core::str::FromStr;
use winnow_datetime::Date;

/// Wrapper around a `Date` that implements `Display` and `FromStr` correctly for the standard.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Iso8601Date(pub Date);

impl Display for Iso8601Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            // like `2015-11-02`
            Date::YMD { year, month, day } => write!(f, "{:04}-{:02}-{:02}", year, month, day),
            // like `2015-W45-01`
            Date::Week { year, ww, d } => write!(f, "{:04}-{:02}-{:02}", year, ww, d),
            // like `2015-306`
            Date::Ordinal { year, ddd } => write!(f, "{:04}-{:03}", year, ddd),
        }
    }
}

impl FromStr for Iso8601Date {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        date(s)
    }
}

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
/// let date = winnow_iso8601::date("2015-11-02").unwrap();
/// ```
pub fn date(mut i: &str) -> Result<Iso8601Date, String> {
    if let Ok(parsed) = parsers::parse_date(&mut i) {
        Ok(Iso8601Date(parsed))
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
        coverage: vec![
            FormatCoverage {
                format: "%Y-%M-%D".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%C".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%X".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%Y".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%Y-%M".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%Y-%O".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%V-W%W".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%V-W%W-%w".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%Y%M%D".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%Y%O".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%VW%W".into(),
                exception: Ok(None),
            },
            FormatCoverage {
                format: "%VW%W%w".into(),
                exception: Ok(None),
            },
        ]
    }
}