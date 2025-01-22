use crate::offset::offset;
use alloc::string::String;
use winnow::combinator::{opt, preceded, trace};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::{literal, one_of};
use winnow::{seq, PResult, Parser};
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
pub fn parse_time(mut i: &str) -> Result<Time, String> {
    if let Ok(parsed) = time(&mut i) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse time: {}", i))
    }
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
pub fn time<'i, Input>(i: &mut Input) -> PResult<Time>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("time", move |input: &mut Input| {
        seq!((
            _: opt(literal("T")),
            base_time
        ))
        .map(|r| r.0)
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a time string.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub(crate) fn base_time<'i, Input>(i: &mut Input) -> PResult<Time>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

/// Parses secondary portion of a time string.
pub(crate) fn time_minute_second_millisecond<'i, Input>(
    i: &mut Input,
) -> PResult<(u32, Option<u32>, Option<u32>)>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use crate::time::time;
    use winnow::stream::AsBStr;
    use winnow_datetime::parser::{time_hour, time_minute, time_second};
    use winnow_datetime::Stream;

    #[test]
    fn test_time_hour() {
        assert_eq!(time_hour(&mut "00".as_bstr()).unwrap(), 0);
        assert_eq!(time_hour(&mut "01".as_bstr()).unwrap(), 1);
        assert_eq!(time_hour(&mut "06".as_bstr()).unwrap(), 6);
        assert_eq!(time_hour(&mut "12".as_bstr()).unwrap(), 12);
        assert_eq!(time_hour(&mut "13".as_bstr()).unwrap(), 13);
        assert_eq!(time_hour(&mut "20".as_bstr()).unwrap(), 20);

        assert!(time_hour(&mut "24".as_bstr()).is_err());
        assert!(time_hour(&mut "25".as_bstr()).is_err());
        assert!(time_hour(&mut "30".as_bstr()).is_err());
        assert!(time_hour(&mut "ab".as_bstr()).is_err());
    }

    #[test]
    fn test_time_minute() {
        assert_eq!(time_minute(&mut "00".as_bstr()).unwrap(), 0);
        assert_eq!(time_minute(&mut "01".as_bstr()).unwrap(), 1);
        assert_eq!(time_minute(&mut "30".as_bstr()).unwrap(), 30);
        assert_eq!(time_minute(&mut "59".as_bstr()).unwrap(), 59);

        assert!(time_minute(&mut Stream::new(b"60")).is_err());
        assert!(time_minute(&mut Stream::new(b"61")).is_err());
        assert!(time_minute(&mut Stream::new(b"ab")).is_err());
    }

    #[test]
    fn test_time_second() {
        assert_eq!(time_second(&mut "00".as_bstr()).unwrap(), 0);
        assert_eq!(time_second(&mut "01".as_bstr()).unwrap(), 1);
        assert_eq!(time_second(&mut "30".as_bstr()).unwrap(), 30);
        assert_eq!(time_second(&mut "59".as_bstr()).unwrap(), 59);
        assert_eq!(time_second(&mut "60".as_bstr()).unwrap(), 60);

        assert!(time_second(&mut Stream::new(b"61")).is_err());
        assert!(time_second(&mut Stream::new(b"ab")).is_err());
    }

    #[test]
    fn test_time() {
        assert!(time(&mut Stream::new(b"20:")).is_err());
        assert!(time(&mut Stream::new(b"pppp")).is_err());
    }

    #[test]
    fn test_time_with_timezone() {
        assert!(time(&mut Stream::new(b"20:")).is_err());
        assert!(time(&mut Stream::new(b"pppp")).is_err());
    }

    #[test]
    fn disallows_notallowed() {
        assert!(time(&mut Stream::new(b"30:90:90")).is_err());
    }
}
