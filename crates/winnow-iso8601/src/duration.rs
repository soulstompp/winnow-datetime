use alloc::string::String;
use core::str;
use winnow::combinator::{opt, preceded, trace};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::{literal, one_of};
use winnow::{seq, PResult, Parser};
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
pub fn parse_duration(mut i: &str) -> Result<Duration, String> {
    match duration(&mut i) {
        Ok(p) => Ok(p),
        Err(e) => Err(format!("Failed to parse duration {}: {}", i, e)),
    }
}

/// Parses a duration string with the format P%dY%dM%dDT%dH%dM%dS
///
/// A duration starts with `P` and can have one of the following formats:
///
/// * Fully-specified duration: `P1Y2M3DT4H5M6S`
/// * Duration in weekly intervals: `P1W`
/// * Fully-specified duration in [`DateTime`](`crate::DateTime`) format: `P<datetime>`
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
pub fn duration<'i, Input>(i: &mut Input) -> PResult<Duration>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

///    dur-year          = 1*DIGIT "Y" [dur-month]
pub(crate) fn duration_part_year<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

///    dur-month         = 1*DIGIT "M" [dur-day]
pub(crate) fn duration_part_month<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

///    dur-week          = 1*DIGIT "W"
pub(crate) fn duration_part_week<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

//    dur-day           = 1*DIGIT "D"
pub(crate) fn duration_part_day<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

///    dur-hour          = 1*DIGIT "H" [dur-minute]
///    dur-time          = "T" (dur-hour / dur-minute / dur-second)
pub(crate) fn duration_part_hour<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

///    dur-minute        = 1*DIGIT "M" [dur-second]
pub(crate) fn duration_part_minute<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

///    dur-second        = 1*DIGIT "S"
pub(crate) fn duration_part_second<'i, Input>(i: &mut Input) -> PResult<DurationPart>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

/// Parses time portion of a duration
pub fn duration_time<'i, Input>(
    i: &mut Input,
) -> PResult<(
    Option<DurationPart>,
    Option<DurationPart>,
    Option<DurationPart>,
)>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_time", move |input: &mut Input| {
        preceded(opt(literal("T")), duration_base_time).parse_next(input)
    })
    .parse_next(i)
}

pub(crate) fn duration_base_time<'i, Input>(
    i: &mut Input,
) -> PResult<(
    Option<DurationPart>,
    Option<DurationPart>,
    Option<DurationPart>,
)>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
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
    .parse_next(i)
}

#[cfg(test)]
mod parsers {
    use crate::duration::*;
    use winnow_datetime::types::DurationPart;
    use winnow_datetime::Stream;

    #[test]
    fn test_duration_year() {
        assert_eq!(
            duration_part_year(&mut "2019Y".as_bstr()).unwrap(),
            (DurationPart {
                whole: 2019,
                frac: None
            })
        );
        assert_eq!(
            duration_part_year(&mut "0Y".as_bstr()).unwrap(),
            (DurationPart {
                whole: 0,
                frac: None
            })
        );
        assert_eq!(
            duration_part_year(&mut "10000Y".as_bstr()).unwrap(),
            (DurationPart {
                whole: 10000,
                frac: None
            })
        );
        assert!(duration_part_year(&mut Stream::new(b"abcd")).is_err());
        assert!(duration_part_year(&mut Stream::new(b"-1")).is_err());
    }

    #[test]
    fn test_duration_month() {
        assert_eq!(
            duration_part_month(&mut "6M".as_bstr()).unwrap(),
            (DurationPart {
                whole: 6,
                frac: None
            })
        );
        assert_eq!(
            duration_part_month(&mut "0M".as_bstr()).unwrap(),
            (DurationPart {
                whole: 0,
                frac: None
            })
        );
        assert_eq!(
            duration_part_month(&mut "12M".as_bstr()).unwrap(),
            (DurationPart {
                whole: 12,
                frac: None
            })
        );

        assert!(duration_part_month(&mut Stream::new(b"ab")).is_err());
        assert!(duration_part_month(&mut Stream::new(b"-1")).is_err());
        assert!(duration_part_month(&mut Stream::new(b"13")).is_err());
    }

    #[test]
    fn test_duration_week() {
        assert_eq!(
            duration_part_week(&mut "26W".as_bstr()).unwrap(),
            DurationPart {
                whole: 26,
                frac: None
            }
        );
        assert_eq!(
            duration_part_week(&mut "0W".as_bstr()).unwrap(),
            DurationPart {
                whole: 0,
                frac: None
            }
        );
        assert_eq!(
            duration_part_week(&mut "52W".as_bstr()).unwrap(),
            DurationPart {
                whole: 52,
                frac: None
            }
        );
        assert!(duration_part_week(&mut Stream::new(b"ab")).is_err());
        assert!(duration_part_week(&mut Stream::new(b"-1")).is_err());
        assert!(duration_part_week(&mut Stream::new(b"53")).is_err());
    }

    #[test]
    fn test_duration_day() {
        assert_eq!(
            duration_part_day(&mut "16D".as_bstr()).unwrap(),
            DurationPart {
                whole: 16,
                frac: None
            }
        );
        assert_eq!(
            duration_part_day(&mut "0D".as_bstr()).unwrap(),
            DurationPart {
                whole: 0,
                frac: None
            }
        );
        assert_eq!(
            duration_part_day(&mut "31D".as_bstr()).unwrap(),
            DurationPart {
                whole: 31,
                frac: None
            }
        );
        assert!(duration_part_day(&mut Stream::new(b"ab")).is_err());
        assert!(duration_part_day(&mut Stream::new(b"-1")).is_err());
        assert!(duration_part_day(&mut Stream::new(b"32")).is_err());
    }

    #[test]
    fn test_duration_hour() {
        assert_eq!(
            duration_part_hour(&mut "12H".as_bstr()).unwrap(),
            DurationPart {
                whole: 12,
                frac: None
            }
        );
        assert_eq!(
            duration_part_hour(&mut "0H".as_bstr()).unwrap(),
            DurationPart {
                whole: 0,
                frac: None
            }
        );
        assert_eq!(
            duration_part_hour(&mut "24H".as_bstr()).unwrap(),
            DurationPart {
                whole: 24,
                frac: None
            }
        );
        assert!(duration_part_hour(&mut Stream::new(b"ab")).is_err());
        assert!(duration_part_hour(&mut Stream::new(b"-1")).is_err());
        assert!(duration_part_hour(&mut Stream::new(b"25")).is_err());
    }

    #[test]
    fn test_duration_minute() {
        assert_eq!(
            duration_part_minute(&mut "30M".as_bstr()).unwrap(),
            DurationPart {
                whole: 30,
                frac: None
            }
        );
        assert_eq!(
            duration_part_minute(&mut "0M".as_bstr()).unwrap(),
            DurationPart {
                whole: 0,
                frac: None
            }
        );
        assert_eq!(
            duration_part_minute(&mut "60M".as_bstr()).unwrap(),
            DurationPart {
                whole: 60,
                frac: None
            }
        );
        assert!(duration_part_minute(&mut Stream::new(b"ab")).is_err());
        assert!(duration_part_minute(&mut Stream::new(b"-1")).is_err());
        assert!(duration_part_minute(&mut Stream::new(b"61")).is_err());
    }

    #[test]
    fn test_duration_second_and_millisecond1() {
        assert_eq!(
            duration_part_second(&mut "30S".as_bstr()).unwrap(),
            DurationPart {
                whole: 30,
                frac: None
            }
        );
        assert_eq!(
            duration_part_second(&mut "0S".as_bstr()).unwrap(),
            DurationPart {
                whole: 0,
                frac: None
            }
        );
        assert_eq!(
            duration_part_second(&mut "60S".as_bstr()).unwrap(),
            DurationPart {
                whole: 60,
                frac: None
            }
        );
        assert_eq!(
            duration_part_second(&mut "1,23S".as_bstr()).unwrap(),
            DurationPart {
                whole: 1,
                frac: Some(0.23)
            }
        );
        assert_eq!(
            duration_part_second(&mut "2.34S".as_bstr()).unwrap(),
            DurationPart {
                whole: 2,
                frac: Some(0.34)
            }
        );
        assert!(duration_part_second(&mut Stream::new(b"abS")).is_err());
        assert!(duration_part_second(&mut Stream::new(b"-1S")).is_err());
    }

    #[test]
    fn test_duration_time() {
        assert_eq!(
            duration_time(&mut "T1H2M3S".as_bstr()).unwrap(),
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
            duration_time(&mut "T10H12M30S".as_bstr()).unwrap(),
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
            duration_time(&mut "T1H3S".as_bstr()).unwrap(),
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
            duration_time(&mut "T2M".as_bstr()).unwrap(),
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
            duration_time(&mut "T1H2M3,4S".as_bstr()).unwrap(),
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
            duration_time(&mut "T1H23.4S".as_bstr()).unwrap(),
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
            duration_time(&mut "T0,123S".as_bstr()).unwrap(),
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
            duration_time(&mut "T0123S".as_bstr()).unwrap(),
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
        assert!(duration(&mut Stream::new(b"")).is_err());
        assert!(duration(&mut Stream::new(b"P")).is_err()); // empty duration is not 0 seconds
        assert!(duration(&mut Stream::new(b"1Y2M3DT4H5M6S")).is_err()); // missing P at start
        assert!(duration(&mut Stream::new(b"T4H5M6S")).is_err()); // missing P,
    }

    #[test]
    fn test_duration_weeks_error() {
        assert!(duration(&mut Stream::new(b"")).is_err());
        assert!(duration(&mut Stream::new(b"P")).is_err()); // empty duration is not 0 seconds
        assert!(duration(&mut Stream::new(b"P1")).is_err()); // missing W after number
        assert!(duration(&mut Stream::new(b"PW")).is_err()); // missing number
    }

    #[test]
    fn test_duration_second() {
        assert_eq!(
            duration(&mut "PT30S".as_bstr()).unwrap(),
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
            duration(&mut "PT30.123S".as_bstr()).unwrap(),
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
            duration(&mut "P2021Y11M16DT23H26M59.123S".as_bstr()).unwrap(),
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
            duration(&mut "P0W".as_bstr()).unwrap(),
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
            duration(&mut "P2021Y11M16DT23H26M59S".as_bstr()).unwrap(),
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
            duration(&mut "P2021Y11M16DT23H26M".as_bstr()).unwrap(),
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
            duration(&mut "P2021Y11M16DT23H".as_bstr()).unwrap(),
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
            duration(&mut "P2021Y11M16D".as_bstr()).unwrap(),
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
            duration(&mut "P2021Y11M16DT1S".as_bstr()).unwrap(),
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
            duration(&mut "P2021Y11M16DT0.471S".as_bstr()).unwrap(),
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
            duration(&mut "P2021Y11M".as_bstr()).unwrap(),
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
            duration(&mut "P11M".as_bstr()).unwrap(),
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
            duration(&mut "P16D".as_bstr()).unwrap(),
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
            duration(&mut "P0D".as_bstr()).unwrap(),
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
            duration(&mut "PT12H".as_bstr()).unwrap(),
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
            duration(&mut "PT8760H".as_bstr()).unwrap(),
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
            duration(&mut "PT15M".as_bstr()).unwrap(),
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
            duration(&mut "PT600M".as_bstr()).unwrap(),
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
            duration(&mut "PT16S".as_bstr()).unwrap(),
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
            duration(&mut "PT900S".as_bstr()).unwrap(),
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
            duration(&mut "P365D".as_bstr()).unwrap(),
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
            duration(&mut "P36500D".as_bstr()).unwrap(),
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
