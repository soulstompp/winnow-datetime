use crate::offset::offset;
use core::str;
use winnow::combinator::{alt, empty, fail, opt, preceded, trace};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::literal;
use winnow::token::one_of;
use winnow::{seq, PResult, Parser};
use winnow_datetime::parser::fraction_millisecond;
use winnow_datetime::parser::time_hour;
use winnow_datetime::parser::time_minute;
use winnow_datetime::parser::time_second;
use winnow_datetime::time_seq;
use winnow_datetime::types::PartialTime;

/// Parses a partial time string with an optional preceding 'T'.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub(crate) fn partial_time<'i, Input>(i: &mut Input) -> PResult<PartialTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("partial_time", move |input: &mut Input| {
        seq!((
            _: opt(alt((literal(" "), literal("T")))),
            partial_base_time
        ))
        .map(|r| r.0)
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a partial time string.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub(crate) fn partial_base_time<'i, Input>(i: &mut Input) -> PResult<PartialTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("partial_base_time", move |input: &mut Input| {
        seq!(PartialTime {
            hour: time_hour.map(Some),                        // HH
            minute: opt(preceded(literal(":"), time_minute)), // MM
            second: opt(preceded(literal(":"), time_second)), // SS
            millisecond: opt(preceded(
                alt((literal("."), literal(","))),
                fraction_millisecond
            )), // .mmm
            offset: opt(offset).map(|o| o.unwrap_or(None)),   // [(Z|+...|-...)]
        })
        .parse_next(input)
    })
    .parse_next(i)
}

// NOTE: this is marked as dead code because this is likely going to be made public
#[allow(dead_code)]
pub(crate) fn partial_end_time<'i, Input>(
    i: &mut Input,
    start_time: &PartialTime,
) -> PResult<PartialTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("partial_end_time", move |input: &mut Input| {
        let _ = opt(alt((literal(" "), literal("T")))).parse_next(input)?;

        partial_end_base_time(input, start_time)
    })
    .parse_next(i)
}

/// a partial time string which can be truncated depending on a partial start time
pub(crate) fn partial_end_base_time<'i, Input>(
    i: &mut Input,
    start_time: &PartialTime,
) -> PResult<PartialTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("partial_end_base_time", move |input: &mut Input| {
        match [
            start_time.hour.is_some(),
            start_time.minute.is_some(),
            start_time.second.is_some(),
            start_time.millisecond.is_some(),
        ] {
            // Case 1: Full precision (%H:%M:%S.%ms)
            [true, true, true, true] => alt((
                time_seq!(PartialTime {
                    hour: time_hour.map(Some),
                    minute: preceded(literal(":"), time_minute).map(Some),
                    second: preceded(literal(":"), time_second).map(Some),
                    millisecond: opt(preceded(one_of(['.', ',']), fraction_millisecond)),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
                time_seq!(PartialTime {
                    hour: opt(empty).map(|_| start_time.hour),
                    minute: time_minute.map(Some),
                    second: preceded(literal(":"), time_second).map(Some),
                    millisecond: opt(preceded(one_of(['.', ',']), fraction_millisecond)),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
                time_seq!(PartialTime {
                    hour: opt(empty).map(|_| start_time.hour),
                    minute: opt(empty).map(|_| start_time.minute),
                    second: time_second.map(Some),
                    millisecond: opt(preceded(one_of(['.', ',']), fraction_millisecond)),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
                time_seq!(PartialTime {
                    hour: opt(empty).map(|_| start_time.hour),
                    minute: opt(empty).map(|_| start_time.minute),
                    second: opt(empty).map(|_| start_time.second),
                    millisecond: opt(preceded(one_of(['.', ',']), fraction_millisecond)),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
            ))
            .parse_next(input),
            // Case 2: HH:MM:SS (no milliseconds)
            [true, true, true, false] => alt((
                time_seq!(PartialTime {
                    hour: time_hour.map(Some),
                    minute: preceded(literal(":"), time_minute).map(Some),
                    second: preceded(literal(":"), time_second).map(Some),
                    millisecond: opt(empty).map(|_| None),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
                time_seq!(PartialTime {
                    hour: opt(empty).map(|_| start_time.hour),
                    minute: time_minute.map(Some),
                    second: preceded(literal(":"), time_second).map(Some),
                    millisecond: opt(empty).map(|_| None),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
                time_seq!(PartialTime {
                    hour: opt(empty).map(|_| start_time.hour),
                    minute: opt(empty).map(|_| start_time.minute),
                    second: time_second.map(Some),
                    millisecond: opt(empty).map(|_| None),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
            ))
            .parse_next(input),

            // Case 3: HH:MM (no seconds or milliseconds)
            [true, true, false, false] => alt((
                time_seq!(PartialTime {
                    hour: time_hour.map(Some),
                    minute: preceded(literal(":"), time_minute).map(Some),
                    second: opt(empty).map(|_| None),
                    millisecond: opt(empty).map(|_| None),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
                time_seq!(PartialTime {
                    hour: opt(empty).map(|_| start_time.hour),
                    minute: time_minute.map(Some),
                    second: opt(empty).map(|_| None),
                    millisecond: opt(empty).map(|_| None),
                    offset: opt(offset).map(|o| o.unwrap_or(None)),
                }),
            ))
            .parse_next(input),

            // Case 4: HH only (no minutes, seconds, or milliseconds)
            [true, false, false, false] => time_seq!(PartialTime {
                hour: time_hour.map(Some),
                minute: opt(empty).map(|_| None),
                second: opt(empty).map(|_| None),
                millisecond: opt(empty).map(|_| None),
                offset: opt(offset).map(|o| o.unwrap_or(None)),
            })
            .parse_next(input),

            // Case 5: Invalid (no hour provided)
            [_, _, _, _] => fail.parse_next(input),
        }
    })
    .parse_next(i)
}

#[cfg(test)]
mod parsers {
    use crate::partial_time::{partial_end_time, partial_time};
    use winnow::stream::AsBStr;
    use winnow_datetime::types::PartialTime;

    #[test]
    fn partial_time_parsing() {
        assert_eq!(
            partial_time(&mut "12:01:30".as_bstr()).unwrap(),
            PartialTime {
                hour: Some(12),
                minute: Some(1),
                second: Some(30),
                millisecond: None,
                offset: None,
            }
        );
        assert_eq!(
            partial_time(&mut "12:01".as_bstr()).unwrap(),
            PartialTime {
                hour: Some(12),
                minute: Some(1),
                second: None,
                millisecond: None,
                offset: None,
            }
        );
        assert_eq!(
            partial_time(&mut "12:01:30.123".as_bstr()).unwrap(),
            PartialTime {
                hour: Some(12),
                minute: Some(1),
                second: Some(30),
                millisecond: Some(123),
                offset: None,
            }
        );
    }

    #[test]
    fn partial_end_time_parsing() {
        assert_eq!(
            partial_end_time(
                &mut "12:01:30".as_bstr(),
                &PartialTime {
                    hour: Some(12),
                    minute: Some(1),
                    second: Some(29),
                    millisecond: None,
                    offset: None,
                }
            )
            .unwrap(),
            PartialTime {
                hour: Some(12),
                minute: Some(1),
                second: Some(30),
                millisecond: None,
                offset: None,
            }
        );
        assert_eq!(
            partial_end_time(
                &mut "12:01".as_bstr(),
                &PartialTime {
                    hour: Some(12),
                    minute: Some(0),
                    second: None,
                    millisecond: None,
                    offset: None,
                }
            )
            .unwrap(),
            PartialTime {
                hour: Some(12),
                minute: Some(1),
                second: None,
                millisecond: None,
                offset: None,
            }
        );
        assert_eq!(
            partial_end_time(
                &mut "12:01:30.123".as_bstr(),
                &PartialTime {
                    hour: Some(12),
                    minute: Some(1),
                    second: Some(30),
                    millisecond: Some(122),
                    offset: None,
                }
            )
            .unwrap(),
            PartialTime {
                hour: Some(12),
                minute: Some(1),
                second: Some(30),
                millisecond: Some(123),
                offset: None,
            }
        );
    }
}
