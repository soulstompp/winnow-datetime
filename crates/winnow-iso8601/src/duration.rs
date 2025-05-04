use core::str;
use winnow::combinator::{eof, opt, preceded, terminated, trace};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::{literal, one_of};
use winnow::{seq, Parser, Result};
use winnow_datetime::duration_part_seq;
use winnow_datetime::parser::take_digits;
use winnow_datetime::types::{Duration, DurationPart};

/// Parses a duration string.
///
///
/// ## Examples
///
/// ```rust
/// let duration = winnow_iso8601::parse_duration("P1Y2M3DT4H5M6S").unwrap();
/// let duration = winnow_iso8601::parse_duration("P1W").unwrap();
/// ```
pub fn parse_duration(mut i: &str) -> Result<Duration, InputError<&str>> {
    terminated(duration, eof).parse_next(&mut i)
}

/// Parses a duration string with the format P%dY%dM%dDT%dH%dM%dS
///
/// A duration starts with `P` and can have one of the following formats:
///
/// * Fully-specified duration: `P1Y2M3DT4H5M6S`
/// * Duration in weekly intervals: `P1W`
/// * Fully-specified duration in [`DateTime`](`winnow_datetime::DateTime`) format: `P<datetime>`
///
/// Both fully-specified formats get parsed into the YMDHMS Duration variant.
/// The weekly interval format gets parsed into the Weeks Duration variant.
///
/// The ranges for each of the individual units are not expected to exceed
/// the next largest unit.
///
/// These ranges (inclusive) are as follows:
///
/// * Year (any valid u32)
/// * Month 0 - 12
/// * Week 0 - 52
/// * Day 0 - 31
/// * Hour 0 - 24
/// * Minute 0 - 60
/// * Second 0 - 60
pub fn duration<'i, Input, Error>(input: &mut Input) -> std::result::Result<Duration, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("duration", move |input: &mut Input| {
        seq!((
            _: literal("P"),
            opt(duration_part_year),
            opt(duration_part_month),
            opt(duration_part_week),
            opt(duration_part_day),
            opt(preceded(opt(literal("T")), duration_base_time)),
        ))
        .verify(|(y, mo, w, d, time)| {
            let (h, m, s) = time.unwrap_or((None, None, None));

            let p = [y, mo, w, d, &h, &m];

            (p.iter().any(|x| x.is_some() || s.is_some()))
                && p.iter().all(|x| x.is_none() || x.unwrap().frac.is_none())
        })
        .map(|(y, mo, w, d, time)| {
            let time = time.unwrap_or((None, None, None));
            Duration {
                years: y.map(|p| p.whole).unwrap_or(0),
                months: mo.map(|p| p.whole).unwrap_or(0),
                weeks: w.map(|p| p.whole).unwrap_or(0),
                days: d.map(|p| p.whole).unwrap_or(0),
                hours: time.0.map(|p| p.whole).unwrap_or(0),
                minutes: time.1.map(|p| p.whole).unwrap_or(0),
                seconds: time.2.map(|p| p.whole).unwrap_or(0),
                milliseconds: time.2.map(|p| p.frac).unwrap_or(None),
            }
            // at least one element must be present for a valid duration representation
        })
        .parse_next(input)
    })
    .parse_next(input)
}

///    dur-year          = 1*DIGIT "Y" [dur-month]
pub(crate) fn duration_part_year<'i, Input, Error>(
    input: &mut Input,
) -> std::result::Result<DurationPart, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("duration_part_year", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("Y")
        })
        .parse_next(input)
    })
    .parse_next(input)
}

///    dur-month         = 1*DIGIT "M" [dur-day]
pub(crate) fn duration_part_month<'i, Input, Error>(
    input: &mut Input,
) -> std::result::Result<DurationPart, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("duration_part_month", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("M")
        })
        .parse_next(input)
    })
    .parse_next(input)
}

///    dur-week          = 1*DIGIT "W"
pub(crate) fn duration_part_week<'i, Input, Error>(
    input: &mut Input,
) -> std::result::Result<DurationPart, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("duration_part_week", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("W")
        })
        .parse_next(input)
    })
    .parse_next(input)
}

//    dur-day           = 1*DIGIT "D"
pub(crate) fn duration_part_day<'i, Input, Error>(
    input: &mut Input,
) -> std::result::Result<DurationPart, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("duration_part_day", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("D")
        })
        .parse_next(input)
    })
    .parse_next(input)
}

///    dur-hour          = 1*DIGIT "H" [dur-minute]
///    dur-time          = "T" (dur-hour / dur-minute / dur-second)
pub(crate) fn duration_part_hour<'i, Input, Error>(
    input: &mut Input,
) -> std::result::Result<DurationPart, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("duration_part_hour", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("H")
        })
        .parse_next(input)
    })
    .parse_next(input)
}

///    dur-minute        = 1*DIGIT "M" [dur-second]
pub(crate) fn duration_part_minute<'i, Input, Error>(
    input: &mut Input,
) -> std::result::Result<DurationPart, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("duration_part_minute", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("M")
        })
        .parse_next(input)
    })
    .parse_next(input)
}

///    dur-second        = 1*DIGIT "S"
pub(crate) fn duration_part_second<'i, Input, Error>(
    input: &mut Input,
) -> std::result::Result<DurationPart, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("duration_part_second", move |input: &mut Input| {
        duration_part_seq!({
            whole: take_digits,
            sep: one_of(b",."),
            fraction: take_digits,
            end: literal("S")
        })
        .parse_next(input)
    })
    .parse_next(input)
}

/// Parses time portion of a duration
pub fn duration_time<'i, Input, Error>(
    input: &mut Input,
) -> std::result::Result<
    (
        Option<DurationPart>,
        Option<DurationPart>,
        Option<DurationPart>,
    ),
    Error,
>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("duration_time", move |input: &mut Input| {
        preceded(opt(literal("T")), duration_base_time).parse_next(input)
    })
    .parse_next(input)
}

pub fn duration_base_time<'i, Input, Error>(
    input: &mut Input,
) -> std::result::Result<
    (
        Option<DurationPart>,
        Option<DurationPart>,
        Option<DurationPart>,
    ),
    Error,
>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("duration_base_time", move |input: &mut Input| {
        seq!((
            opt(duration_part_hour),
            opt(duration_part_minute),
            opt(duration_part_second),
        ))
        .verify(|(h, m, s)| h.is_some() || m.is_some() || s.is_some())
        .map(|(h, m, s)| (h, m, s))
        .parse_next(input)
    })
    .parse_next(input)
}

#[cfg(test)]
mod parsers {
    use crate::duration::*;
    use winnow::error::InputError;
    use winnow_datetime::types::DurationPart;
    use winnow_datetime::PartialInput;

    #[test]
    fn test_duration_year() {
        assert_eq!(
            duration_part_year::<_, InputError<_>>(&mut "2019Y").unwrap(),
            (DurationPart {
                whole: 2019,
                frac: None
            })
        );
        assert_eq!(
            duration_part_year::<_, InputError<_>>(&mut "0Y").unwrap(),
            (DurationPart {
                whole: 0,
                frac: None
            })
        );
        assert_eq!(
            duration_part_year::<_, InputError<_>>(&mut "10000Y").unwrap(),
            (DurationPart {
                whole: 10000,
                frac: None
            })
        );
        assert!(duration_part_year::<_, InputError<_>>(&mut PartialInput::new(b"abcd")).is_err());
        assert!(duration_part_year::<_, InputError<_>>(&mut PartialInput::new(b"-1")).is_err());
    }

    #[test]
    fn test_duration_month() {
        assert_eq!(
            duration_part_month::<_, InputError<_>>(&mut "6M").unwrap(),
            (DurationPart {
                whole: 6,
                frac: None
            })
        );
        assert_eq!(
            duration_part_month::<_, InputError<_>>(&mut "0M").unwrap(),
            (DurationPart {
                whole: 0,
                frac: None
            })
        );
        assert_eq!(
            duration_part_month::<_, InputError<_>>(&mut "12M").unwrap(),
            (DurationPart {
                whole: 12,
                frac: None
            })
        );

        assert!(duration_part_month::<_, InputError<_>>(&mut PartialInput::new(b"ab")).is_err());
        assert!(duration_part_month::<_, InputError<_>>(&mut PartialInput::new(b"-1")).is_err());
        assert!(duration_part_month::<_, InputError<_>>(&mut PartialInput::new(b"13")).is_err());
    }

    #[test]
    fn test_duration_week() {
        assert_eq!(
            duration_part_week::<_, InputError<_>>(&mut "26W").unwrap(),
            DurationPart {
                whole: 26,
                frac: None
            }
        );
        assert_eq!(
            duration_part_week::<_, InputError<_>>(&mut "0W").unwrap(),
            DurationPart {
                whole: 0,
                frac: None
            }
        );
        assert_eq!(
            duration_part_week::<_, InputError<_>>(&mut "52W").unwrap(),
            DurationPart {
                whole: 52,
                frac: None
            }
        );
        assert!(duration_part_week::<_, InputError<_>>(&mut PartialInput::new(b"ab")).is_err());
        assert!(duration_part_week::<_, InputError<_>>(&mut PartialInput::new(b"-1")).is_err());
        assert!(duration_part_week::<_, InputError<_>>(&mut PartialInput::new(b"53")).is_err());
    }

    #[test]
    fn test_duration_day() {
        assert_eq!(
            duration_part_day::<_, InputError<_>>(&mut "16D").unwrap(),
            DurationPart {
                whole: 16,
                frac: None
            }
        );
        assert_eq!(
            duration_part_day::<_, InputError<_>>(&mut "0D").unwrap(),
            DurationPart {
                whole: 0,
                frac: None
            }
        );
        assert_eq!(
            duration_part_day::<_, InputError<_>>(&mut "31D").unwrap(),
            DurationPart {
                whole: 31,
                frac: None
            }
        );
        assert!(duration_part_day::<_, InputError<_>>(&mut PartialInput::new(b"ab")).is_err());
        assert!(duration_part_day::<_, InputError<_>>(&mut PartialInput::new(b"-1")).is_err());
        assert!(duration_part_day::<_, InputError<_>>(&mut PartialInput::new(b"32")).is_err());
    }

    #[test]
    fn test_duration_hour() {
        assert_eq!(
            duration_part_hour::<_, InputError<_>>(&mut "12H").unwrap(),
            DurationPart {
                whole: 12,
                frac: None
            }
        );
        assert_eq!(
            duration_part_hour::<_, InputError<_>>(&mut "0H").unwrap(),
            DurationPart {
                whole: 0,
                frac: None
            }
        );
        assert_eq!(
            duration_part_hour::<_, InputError<_>>(&mut "24H").unwrap(),
            DurationPart {
                whole: 24,
                frac: None
            }
        );
        assert!(duration_part_hour::<_, InputError<_>>(&mut PartialInput::new(b"ab")).is_err());
        assert!(duration_part_hour::<_, InputError<_>>(&mut PartialInput::new(b"-1")).is_err());
        assert!(duration_part_hour::<_, InputError<_>>(&mut PartialInput::new(b"25")).is_err());
    }

    #[test]
    fn test_duration_minute() {
        assert_eq!(
            duration_part_minute::<_, InputError<_>>(&mut "30M").unwrap(),
            DurationPart {
                whole: 30,
                frac: None
            }
        );
        assert_eq!(
            duration_part_minute::<_, InputError<_>>(&mut "0M").unwrap(),
            DurationPart {
                whole: 0,
                frac: None
            }
        );
        assert_eq!(
            duration_part_minute::<_, InputError<_>>(&mut "60M").unwrap(),
            DurationPart {
                whole: 60,
                frac: None
            }
        );
        assert!(duration_part_minute::<_, InputError<_>>(&mut PartialInput::new(b"ab")).is_err());
        assert!(duration_part_minute::<_, InputError<_>>(&mut PartialInput::new(b"-1")).is_err());
        assert!(duration_part_minute::<_, InputError<_>>(&mut PartialInput::new(b"61")).is_err());
    }

    #[test]
    fn test_duration_second_and_millisecond1() {
        assert_eq!(
            duration_part_second::<_, InputError<_>>(&mut "30S").unwrap(),
            DurationPart {
                whole: 30,
                frac: None
            }
        );
        assert_eq!(
            duration_part_second::<_, InputError<_>>(&mut "0S").unwrap(),
            DurationPart {
                whole: 0,
                frac: None
            }
        );
        assert_eq!(
            duration_part_second::<_, InputError<_>>(&mut "60S").unwrap(),
            DurationPart {
                whole: 60,
                frac: None
            }
        );
        assert_eq!(
            duration_part_second::<_, InputError<_>>(&mut "1,23S").unwrap(),
            DurationPart {
                whole: 1,
                frac: Some(0.23)
            }
        );
        assert_eq!(
            duration_part_second::<_, InputError<_>>(&mut "2.34S").unwrap(),
            DurationPart {
                whole: 2,
                frac: Some(0.34)
            }
        );
        assert!(duration_part_second::<_, InputError<_>>(&mut PartialInput::new(b"abS")).is_err());
        assert!(duration_part_second::<_, InputError<_>>(&mut PartialInput::new(b"-1S")).is_err());
    }

    #[test]
    fn test_duration_time() {
        assert_eq!(
            duration_time::<_, InputError<_>>(&mut "T1H2M3S").unwrap(),
            (
                Some(DurationPart {
                    whole: 1,
                    frac: None
                }),
                Some(DurationPart {
                    whole: 2,
                    frac: None
                }),
                Some(DurationPart {
                    whole: 3,
                    frac: None
                })
            )
        );
        assert_eq!(
            duration_time::<_, InputError<_>>(&mut "T10H12M30S").unwrap(),
            (
                Some(DurationPart {
                    whole: 10,
                    frac: None
                }),
                Some(DurationPart {
                    whole: 12,
                    frac: None
                }),
                Some(DurationPart {
                    whole: 30,
                    frac: None
                })
            )
        );
        assert_eq!(
            duration_time::<_, InputError<_>>(&mut "T1H3S").unwrap(),
            (
                Some(DurationPart {
                    whole: 1,
                    frac: None
                }),
                None,
                Some(DurationPart {
                    whole: 3,
                    frac: None
                })
            )
        );

        assert_eq!(
            duration_time::<_, InputError<_>>(&mut "T2M").unwrap(),
            (
                None,
                Some(DurationPart {
                    whole: 2,
                    frac: None
                }),
                None
            )
        );
        assert_eq!(
            duration_time::<_, InputError<_>>(&mut "T1H2M3,4S").unwrap(),
            (
                Some(DurationPart {
                    whole: 1,
                    frac: None
                }),
                Some(DurationPart {
                    whole: 2,
                    frac: None
                }),
                Some(DurationPart {
                    whole: 3,
                    frac: Some(0.4)
                })
            )
        );
        assert_eq!(
            duration_time::<_, InputError<_>>(&mut "T1H23.4S").unwrap(),
            (
                Some(DurationPart {
                    whole: 1,
                    frac: None
                }),
                None,
                Some(DurationPart {
                    whole: 23,
                    frac: Some(0.4)
                })
            )
        );
        assert_eq!(
            duration_time::<_, InputError<_>>(&mut "T0,123S").unwrap(),
            (
                None,
                None,
                Some(DurationPart {
                    whole: 0,
                    frac: Some(0.123)
                })
            )
        );
        assert_eq!(
            duration_time::<_, InputError<_>>(&mut "T0123S").unwrap(),
            (
                None,
                None,
                Some(DurationPart {
                    whole: 123,
                    frac: None
                },)
            )
        );
    }

    #[test]
    fn test_duration_ymdhms_error() {
        assert!(duration::<_, InputError<_>>(&mut PartialInput::new(b"")).is_err());
        assert!(duration::<_, InputError<_>>(&mut PartialInput::new(b"P")).is_err()); // empty duration is not 0 seconds
        assert!(duration::<_, InputError<_>>(&mut PartialInput::new(b"1Y2M3DT4H5M6S")).is_err()); // missing P at start
        assert!(duration::<_, InputError<_>>(&mut PartialInput::new(b"T4H5M6S")).is_err());
        // missing P,
    }

    #[test]
    fn test_duration_weeks_error() {
        assert!(duration::<_, InputError<_>>(&mut PartialInput::new(b"")).is_err());
        assert!(duration::<_, InputError<_>>(&mut PartialInput::new(b"P")).is_err()); // empty duration is not 0 seconds
        assert!(duration::<_, InputError<_>>(&mut PartialInput::new(b"P1")).is_err()); // missing W after number
        assert!(duration::<_, InputError<_>>(&mut PartialInput::new(b"PW")).is_err());
        // missing number
    }

    #[test]
    fn test_duration_second() {
        assert_eq!(
            duration::<_, InputError<_>>(&mut "PT30S").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 30,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "PT30.123S").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 30,
                milliseconds: Some(0.123)
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P2021Y11M16DT23H26M59.123S").unwrap(),
            Duration {
                years: 2021,
                months: 11,
                weeks: 0,
                days: 16,
                hours: 23,
                minutes: 26,
                seconds: 59,
                milliseconds: Some(0.123)
            }
        );
    }

    #[test]
    fn duration_roundtrip() {
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P0W").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P2021Y11M16DT23H26M59S").unwrap(),
            Duration {
                years: 2021,
                months: 11,
                weeks: 0,
                days: 16,
                hours: 23,
                minutes: 26,
                seconds: 59,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P2021Y11M16DT23H26M").unwrap(),
            Duration {
                years: 2021,
                months: 11,
                weeks: 0,
                days: 16,
                hours: 23,
                minutes: 26,
                seconds: 0,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P2021Y11M16DT23H").unwrap(),
            Duration {
                years: 2021,
                months: 11,
                weeks: 0,
                days: 16,
                hours: 23,
                minutes: 0,
                seconds: 0,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P2021Y11M16D").unwrap(),
            Duration {
                years: 2021,
                months: 11,
                weeks: 0,
                days: 16,
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P2021Y11M16DT1S").unwrap(),
            Duration {
                years: 2021,
                months: 11,
                weeks: 0,
                days: 16,
                hours: 0,
                minutes: 0,
                seconds: 1,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P2021Y11M16DT0.471S").unwrap(),
            Duration {
                years: 2021,
                months: 11,
                weeks: 0,
                days: 16,
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: Some(0.471)
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P2021Y11M").unwrap(),
            Duration {
                years: 2021,
                months: 11,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P11M").unwrap(),
            Duration {
                years: 0,
                months: 11,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P16D").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 16,
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P0D").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: None
            }
        );
    }

    #[test]
    fn duration_multi_digit_hour() {
        assert_eq!(
            duration::<_, InputError<_>>(&mut "PT12H").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 12,
                minutes: 0,
                seconds: 0,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "PT8760H").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 365 * 24,
                minutes: 0,
                seconds: 0,
                milliseconds: None
            }
        );
    }

    #[test]
    fn duration_multi_digit_minute() {
        assert_eq!(
            duration::<_, InputError<_>>(&mut "PT15M").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 15,
                seconds: 0,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "PT600M").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 600,
                seconds: 0,
                milliseconds: None
            }
        );
    }

    #[test]
    fn duration_multi_digit_second() {
        assert_eq!(
            duration::<_, InputError<_>>(&mut "PT16S").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 16,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "PT900S").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 900,
                milliseconds: None
            }
        );
    }

    #[test]
    fn duration_multi_digit_day() {
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P365D").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 365,
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: None
            }
        );
        assert_eq!(
            duration::<_, InputError<_>>(&mut "P36500D").unwrap(),
            Duration {
                years: 0,
                months: 0,
                weeks: 0,
                days: 36500,
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: None
            }
        );
    }
}
