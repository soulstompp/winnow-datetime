use crate::offset::offset;
use winnow::combinator::{eof, opt, preceded, terminated, trace};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::{literal, one_of};
use winnow::{seq, Parser, Result};
use winnow_datetime::parser::{fraction_millisecond, time_hour, time_minute, time_second};
use winnow_datetime::Time;

/// Parses a time string.
///
///
/// ## Example
///
/// ```rust
/// let time = winnow_iso8601::parse_time("21:56:42").unwrap();
/// ```
pub fn parse_time(mut i: &str) -> Result<Time, InputError<&str>> {
    terminated(time, eof).parse_next(&mut i)
}

/// Parses a time with an optional preceding 'T'.
///
/// A string can have one of the following formats:
///
/// * `07:35:[00][.123]` or `0735[00][.123]`
/// * `07:35:[00][.123][(Z|(+|-)00:00)]`
/// * `0735[00][.123][(Z|(+|-)00:00)]`
/// * `0735[00][.123][(Z|(+|-)0000)]`
///
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub fn time<'i, Input, Error>(input: &mut Input) -> std::result::Result<Time, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("time", move |input: &mut Input| {
        seq!((
            _: opt(literal("T")),
            base_time
        ))
        .map(|r| r.0)
        .parse_next(input)
    })
    .parse_next(input)
}

/// Parses a time string.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub(crate) fn base_time<'i, Input, Error>(input: &mut Input) -> std::result::Result<Time, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("base_time", move |input: &mut Input| {
        let hour = time_hour(input)?;

        let msms = opt(time_minute_second_millisecond).parse_next(input)?; // MM:[SS][.(m*)]

        let offset = opt(offset).parse_next(input)?;

        let (minute, second, millisecond) = msms.unwrap_or((0, None, None));

        Ok(Time {
            hour,                                  // HH
            minute,                                // MM
            second: second.unwrap_or(0),           // [SS]
            millisecond: millisecond.unwrap_or(0), // [.(m*)]
            offset: offset.unwrap_or(None),        // [(Z|+...|-...)]
        })
    })
    .parse_next(input)
}

/// Parses secondary portion of a time string.
pub(crate) fn time_minute_second_millisecond<'i, Input, Error>(
    input: &mut Input,
) -> std::result::Result<(u32, Option<u32>, Option<u32>), Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace(
        "time_minute_second_millisecond",
        move |input: &mut Input| {
            seq!(
                preceded(opt(literal(":")), time_minute),
                opt(preceded(opt(literal(":")), time_second)),
                opt(preceded(one_of(b",."), fraction_millisecond))
            )
            .parse_next(input)
        },
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use crate::time::time;
    use winnow::error::InputError;

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
        assert_eq!(time_second::<_, InputError<_>>(&mut "00").unwrap(), 0);
        assert_eq!(time_second::<_, InputError<_>>(&mut "01").unwrap(), 1);
        assert_eq!(time_second::<_, InputError<_>>(&mut "30").unwrap(), 30);
        assert_eq!(time_second::<_, InputError<_>>(&mut "59").unwrap(), 59);
        assert_eq!(time_second::<_, InputError<_>>(&mut "60").unwrap(), 60);

        assert!(time_second::<_, InputError<_>>(&mut PartialInput::new(b"61")).is_err());
        assert!(time_second::<_, InputError<_>>(&mut PartialInput::new(b"ab")).is_err());
    }

    #[test]
    fn disallows_notallowed() {
        assert!(time::<_, InputError<_>>(&mut PartialInput::new(b"30:90:90")).is_err());
    }
}
