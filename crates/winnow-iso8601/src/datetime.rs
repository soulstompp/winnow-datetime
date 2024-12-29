use crate::{parsers, Iso8601Date, Iso8601Time};
use alloc::string::String;
use core::fmt;
use core::fmt::{Debug, Display};
use core::str::FromStr;
use winnow_datetime::DateTime;

/// Wrapper around a `DateTime` that implements `Display` and `FromStr` correctly for the standard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Iso8601DateTime(pub DateTime);

impl Display for Iso8601DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Iso8601Date(self.0.date))?;
        write!(f, "T")?;
        write!(f, "{}", Iso8601Time(self.0.time))
    }
}

impl FromStr for Iso8601DateTime {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        datetime(s)
    }
}

/// Parses a datetime string.
///
/// A datetime string is a combination of the valid formats for the date and time,
/// separated by a literal `T`.
/// See the respective functions for the correct format.
///
/// ## Example
///
/// ```rust
/// let dt = winnow_iso8601::datetime("2015-11-03T21:56").unwrap();
/// ```
pub fn datetime(mut i: &str) -> Result<Iso8601DateTime, String> {
    if let Ok(parsed) = parsers::parse_datetime(&mut i) {
        Ok(Iso8601DateTime(parsed))
    } else {
        Err(format!("Failed to parse datetime: {}", i))
    }
}