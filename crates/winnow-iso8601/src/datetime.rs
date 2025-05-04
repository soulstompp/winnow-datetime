use crate::date::date;
use crate::time::base_time;
use ::winnow::Parser;
use winnow::combinator::{eof, separated_pair, terminated, trace};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::Result;
use winnow_datetime::DateTime;

/// Parses a datetime string.
///
/// ## Example
///
/// ```rust
/// let dt = winnow_iso8601::parse_datetime("2015-11-03T21:56").unwrap();
/// ```
pub fn parse_datetime(mut i: &str) -> Result<DateTime, InputError<&str>> {
    terminated(datetime, eof).parse_next(&mut i)
}

/// Parses a datetime string.
///
/// A datetime string is a combination of the valid formats for the date and time,
/// separated by a literal `T`.
// Full ISO8601 datetime
pub fn datetime<'i, Input, Error>(input: &mut Input) -> std::result::Result<DateTime, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("datetime", move |input: &mut Input| {
        separated_pair(date, literal("T"), base_time)
            .map(|(d, t)| DateTime { date: d, time: t })
            .parse_next(input)
    })
    .parse_next(input)
}

#[cfg(test)]
mod parsers {
    use crate::datetime::datetime;
    use winnow::error::InputError;
    use winnow_datetime::PartialInput;

    #[test]
    fn format_equivalence() {
        assert_eq!(
            datetime::<_, InputError<_>>(&mut "2001-02-03T04:05:06+07:00"),
            datetime::<_, InputError<_>>(&mut "20010203T040506+0700")
        );
        assert_eq!(
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"2001-02-03T04:05:06+07:00")),
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"20010203T04:05:06+0700"))
        );
        assert_eq!(
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"2001-02-03T04:05:00+07:00")),
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"20010203T0405+0700"))
        );
        assert_eq!(
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"20010203T0405+0700")),
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"2001-02-03T0405+0700"))
        );
        assert_eq!(
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"20010203T040506+0700")),
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"2001-02-03T040506+0700"))
        );
        assert_eq!(
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"20010203T040506+0000")),
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"20010203T040506Z"))
        );
        assert_eq!(
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"2015W056T04:05:06+07:00")),
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"2015-W05-6T04:05:06+07:00"))
        );
    }

    #[test]
    fn test_datetime_error() {
        let test_datetimes = vec!["ppp", "dumd-di-duTmd:iu:m"];

        for iso_string in test_datetimes {
            let res = datetime::<_, InputError<_>>(&mut PartialInput::new(iso_string.as_bytes()));
            assert!(res.is_err());
        }
    }

    #[test]
    fn disallows_notallowed() {
        assert!(
            datetime::<_, InputError<_>>(&mut PartialInput::new(b"2001-w05-6t04:05:06.123z"))
                .is_err()
        );
    }
}
