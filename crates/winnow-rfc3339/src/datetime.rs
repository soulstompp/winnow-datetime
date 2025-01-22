use crate::date::date;
use crate::time::time;
use alloc::string::String;
use winnow::combinator::{alt, separated_pair, trace};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::literal;
use winnow::{PResult, Parser};
use winnow_datetime::DateTime;

/// Parses a datetime string.
///
/// A datetime string is a combination of the valid formats for the date and time,
/// separated by a literal `T`.
/// See the respective functions for the correct format.
///
/// ## Example
///
/// ```rust
/// let dt = winnow_rfc3339::parse_datetime("2015-11-03T21:56:00Z").unwrap();
/// ```
pub fn parse_datetime(mut i: &str) -> Result<DateTime, String> {
    if let Ok(parsed) = datetime(&mut i) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse datetime: {}", i))
    }
}

/// Parses a datetime string.
///
/// See [`datetime()`][`crate::datetime()`] for supported formats.
// Full ISO8601 datetime
pub fn datetime<'i, Input>(i: &mut Input) -> PResult<DateTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("parse_datetime", move |input: &mut Input| {
        separated_pair(date, alt((literal("T"), literal("t"))), time)
            .map(|(d, t)| DateTime { date: d, time: t })
            .parse_next(input)
    })
    .parse_next(i)
}

#[cfg(test)]
mod parsers {
    use crate::datetime::datetime;
    use winnow_datetime::Stream;

    #[test]
    fn test_datetime_error() {
        let test_datetimes = vec!["ppp", "dumd-di-duTmd:iu:m"];

        for iso_string in test_datetimes {
            let res = datetime(&mut Stream::new(iso_string.as_bytes()));
            assert!(res.is_err());
        }
    }

    #[test]
    fn disallows_notallowed() {
        assert!(datetime(&mut Stream::new(b"2001-w05-6t04:05:06.123z")).is_err());
    }
}
