use crate::parsers;
use alloc::string::String;
use core::fmt;
use core::fmt::Display;
use core::str::FromStr;
use winnow_datetime::Time;

/// Wrapper around a `Time` that implements `Display` and `FromStr` correctly for the standard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Iso8601Time(pub Time);

impl Display for Iso8601Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let timezone = self.0.timezone.unwrap_or_default();
        // like `16:43:16.123+00:00`
        write!(
            f,
            "{:02}:{:02}:{:02}.{}+{:02}:{:02}",
            self.0.hour,
            self.0.minute,
            self.0.second,
            self.0.millisecond,
            timezone.offset_hours,
            timezone.offset_minutes
        )
    }
}

impl FromStr for Iso8601Time {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        time(s)
    }
}

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
/// let time = winnow_iso8601::time("21:56:42").unwrap();
/// ```
pub fn time(mut i: &str) -> Result<Iso8601Time, String> {
    if let Ok(parsed) = parsers::parse_time(&mut i) {
        Ok(Iso8601Time(parsed))
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
            FormatCoverage { format: "%h".into(), exception: Ok(None) },
            FormatCoverage { format: "%,1h".into(), exception: Ok(None) },
            FormatCoverage { format: "%.1h".into(), exception: Ok(None) },
            FormatCoverage { format: "%h:%m".into(), exception: Ok(None) },
            FormatCoverage { format: "%h:%,1m".into(), exception: Ok(None) },
            FormatCoverage { format: "%h:%.1m".into(), exception: Ok(None) },
            FormatCoverage { format: "%h:%m:%s".into(), exception: Ok(None) },
            FormatCoverage { format: "%h:%m:%.1s".into(), exception: Ok(None) },
            FormatCoverage { format: "%h:%m:%.2s".into(), exception: Ok(None) },
            FormatCoverage { format: "%h:%m:%,3s".into(), exception: Ok(None) },
            FormatCoverage { format: "%h:%m:%.3s".into(), exception: Ok(None) },
            FormatCoverage { format: "%h:%m:%s,%u".into(), exception: Ok(None) },
            FormatCoverage { format: "%h:%m:%s.%u".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h".into(), exception: Ok(None) },
            FormatCoverage { format: "T%,1h".into(), exception: Ok(None) },
            FormatCoverage { format: "T%.1h".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h:%m".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h:%,1m".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h:%.1m".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h:%m:%s".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h:%m:%.1s".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h:%m:%.2s".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h:%m:%,3s".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h:%m:%.3s".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h:%m:%s,%u".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h:%m:%s.%u".into(), exception: Ok(None) },
            FormatCoverage { format: "%h%m".into(), exception: Ok(None) },
            FormatCoverage { format: "%h%,1m".into(), exception: Ok(None) },
            FormatCoverage { format: "%h%.1m".into(), exception: Ok(None) },
            FormatCoverage { format: "%h%m%s".into(), exception: Ok(None) },
            FormatCoverage { format: "%h%m%.1s".into(), exception: Ok(None) },
            FormatCoverage { format: "%h%m%.2s".into(), exception: Ok(None) },
            FormatCoverage { format: "%h%m%,3s".into(), exception: Ok(None) },
            FormatCoverage { format: "%h%m%.3s".into(), exception: Ok(None) },
            FormatCoverage { format: "%h%m%s,%u".into(), exception: Ok(None) },
            FormatCoverage { format: "%h%m%s.%u".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h%m".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h%,1m".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h%.1m".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h%m%s".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h%m%.1s".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h%m%.2s".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h%m%,3s".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h%m%.3s".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h%m%s,%u".into(), exception: Ok(None) },
            FormatCoverage { format: "T%h%m%s.%u".into(), exception: Ok(None) },
        ],
        separators: vec![Some("Z"), None],
        timezone_coverage: crate::timezone::coverage(),
    }
}
