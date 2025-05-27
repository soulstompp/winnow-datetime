use crate::offset::offset;
use winnow::combinator::empty;
use winnow::combinator::preceded;
use winnow::combinator::trace;
use winnow::combinator::{eof, opt, terminated};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::token::one_of;
use winnow::{seq, Parser, Result};
use winnow_datetime::parser::fraction_millisecond;
use winnow_datetime::parser::time_hour;
use winnow_datetime::parser::time_minute;
use winnow_datetime::parser::time_second;
use winnow_datetime::{time_seq, Time};

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
/// let time = winnow_rfc9557::parse_time("21:56:42Z").unwrap();
/// ```
pub fn parse_time(mut i: &str) -> Result<Time, InputError<&str>> {
    terminated(time, eof).parse_next(&mut i)
}

/// Parses a time string with an optional preceding 'T'.
///
/// See [`time()`][`mod@crate::time`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub fn time<'i, Input, Error>(input: &mut Input) -> Result<Time, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("parse_time", move |input: &mut Input| {
        time_seq!(Time {
            hour: time_hour,                             // HH
            minute: preceded(literal(":"), time_minute), // MM
            second: preceded(literal(":"), time_second), // [SS]
            millisecond: opt(preceded(one_of(b",."), fraction_millisecond)).map(|d| d.unwrap_or(0)), // [.(m*)]
            offset: offset.map(|o| Some(o)), // [(Z|+...|-...)]
            time_zone: empty.map(|_| None),
            calendar: empty.map(|_| None),
        })
        .parse_next(input)
    })
    .parse_next(input)
}

#[cfg(test)]
mod parsers {
    use crate::time::time;
    use winnow::error::InputError;
    use winnow::stream::AsBStr;
    use winnow_datetime::parser::{time_hour, time_minute, time_second};
    use winnow_datetime::PartialInput;

    #[test]
    fn test_time_hour() {
        assert_eq!(time_hour::<_, InputError<_>>(&mut "00").unwrap(), 0);
        assert_eq!(time_hour::<_, InputError<_>>(&mut "01").unwrap(), 1);
        assert_eq!(time_hour::<_, InputError<_>>(&mut "06").unwrap(), 6);
        assert_eq!(time_hour::<_, InputError<_>>(&mut "12").unwrap(), 12);
        assert_eq!(time_hour::<_, InputError<_>>(&mut "13").unwrap(), 13);
        assert_eq!(time_hour::<_, InputError<_>>(&mut "20").unwrap(), 20);

        assert!(time_hour::<_, InputError<_>>(&mut "24").is_err());
        assert!(time_hour::<_, InputError<_>>(&mut "25").is_err());
        assert!(time_hour::<_, InputError<_>>(&mut "30").is_err());
        assert!(time_hour::<_, InputError<_>>(&mut "ab").is_err());
    }

    #[test]
    fn test_time_minute() {
        assert_eq!(time_minute::<_, InputError<_>>(&mut "00").unwrap(), 0);
        assert_eq!(time_minute::<_, InputError<_>>(&mut "01").unwrap(), 1);
        assert_eq!(time_minute::<_, InputError<_>>(&mut "30").unwrap(), 30);
        assert_eq!(time_minute::<_, InputError<_>>(&mut "59").unwrap(), 59);

        assert!(time_minute::<_, InputError<_>>(&mut PartialInput::new(b"60")).is_err());
        assert!(time_minute::<_, InputError<_>>(&mut PartialInput::new(b"61")).is_err());
        assert!(time_minute::<_, InputError<_>>(&mut PartialInput::new(b"ab")).is_err());
    }

    #[test]
    fn test_time_second() {
        assert_eq!(
            time_second::<_, InputError<_>>(&mut "00".as_bstr()).unwrap(),
            0
        );
        assert_eq!(
            time_second::<_, InputError<_>>(&mut "01".as_bstr()).unwrap(),
            1
        );
        assert_eq!(
            time_second::<_, InputError<_>>(&mut "30".as_bstr()).unwrap(),
            30
        );
        assert_eq!(
            time_second::<_, InputError<_>>(&mut "59".as_bstr()).unwrap(),
            59
        );
        assert_eq!(
            time_second::<_, InputError<_>>(&mut "60".as_bstr()).unwrap(),
            60
        );

        assert!(time_second::<_, InputError<_>>(&mut PartialInput::new(b"61")).is_err());
        assert!(time_second::<_, InputError<_>>(&mut PartialInput::new(b"ab")).is_err());
    }

    #[test]
    fn test_time() {
        assert!(time::<_, InputError<_>>(&mut PartialInput::new(b"20:")).is_err());
        assert!(time::<_, InputError<_>>(&mut PartialInput::new(b"pppp")).is_err());
    }

    #[test]
    fn test_time_with_timezone() {
        assert!(time::<_, InputError<_>>(&mut PartialInput::new(b"20:")).is_err());
        assert!(time::<_, InputError<_>>(&mut PartialInput::new(b"pppp")).is_err());
    }

    #[test]
    fn disallows_notallowed() {
        assert!(time::<_, InputError<_>>(&mut PartialInput::new(b"30:90:90")).is_err());
    }
}
