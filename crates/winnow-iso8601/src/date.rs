use alloc::string::String;
use std::str;
use winnow::combinator::separated_pair;
use winnow::combinator::terminated;
use winnow::combinator::{alt, eof, opt, preceded, trace};
use winnow::error::ContextError;
use winnow::error::ErrMode;
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, PResult, Parser};
use winnow_datetime::parser::date_day;
use winnow_datetime::parser::digit_1;
use winnow_datetime::parser::{date_month, digit_4};
use winnow_datetime::parser::{sign, take_digits_in_range};
use winnow_datetime::Date;
use winnow_datetime::{date_ymd_seq, date_ywd_seq};

/// Parses a date string.
///
/// ## Example
///
/// ```rust
/// let date = winnow_iso8601::parse_date("2015-11-02").unwrap();
/// ```
pub fn parse_date(mut i: &str) -> Result<Date, String> {
    if let Ok(parsed) = terminated(date, eof).parse_next(&mut i) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse date: {}", i))
    }
}

/// Parses a date
///
/// The date can have one of the following formats:
///
/// * `2015-11-02` or `20151102`
/// * `2015-W45-01` or `2015W451`
/// * `2015-306` or `2015306`
///
/// See [`date()`][`crate::date()`] for the supported formats.
pub fn date<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date", move |input: &mut Input| {
        alt((date_ywd, date_ymd_numeric, date_yddd, date_ymd)).parse_next(input)
    })
    .parse_next(i)
}

/// Date separator -
pub fn date_sep<'i, Input>(i: &mut Input) -> PResult<char>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_sep", move |input: &mut Input| {
        literal("-").parse_next(input).map(|_| '-')
    })
    .parse_next(i)
}

/// Parses 2 digit week of the year within range 01-52
// WW
fn date_week<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_week", move |input: &mut Input| {
        let _ = preceded(opt(literal("-")), literal("W")).parse_next(input)?; // [-]Ww

        week_of_year(input)
    })
    .parse_next(i)
}

/// Parses 2 digit week of the year within range 01-52
// WW
pub(crate) fn week_of_year<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_week", move |input: &mut Input| {
        take_digits_in_range(input, 2, 1..=52)
    })
    .parse_next(i)
}

/// Verifies an opt value and calls a verification function on Some
fn verify_opt<V, F>(verify_fn: F) -> impl Fn(Option<V>) -> bool
where
    F: Fn(V) -> bool,
{
    move |day: Option<V>| match day {
        None => true,
        Some(day) => verify_fn(day),
    }
}
/// Verifies a day_of_week range (1-7)
fn verify_day_of_week(day: u32) -> bool {
    day >= 1 && day <= 7
}

/// Parses 2 digit week of the year within range 01-7
pub(crate) fn day_of_week<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("day_of_week", move |input: &mut Input| {
        take_digits_in_range(input, 1, 1..=7)
    })
    .parse_next(i)
}

/// Parses 2 digit week of the year within range 01-52
/// Parses a date string as ISO 8601 week date.
// YYYY-"W"WW-D
pub(crate) fn date_ywd<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("", move |input: &mut Input| {
        date_ywd_seq!(Date::Week {
            year: date_year, // y
            week: date_week, // w
            day: opt(preceded(opt(date_sep), digit_1))
                .verify(|d| verify_opt(verify_day_of_week)(*d))
                .map(|d| d.unwrap_or(1)), // d
        })
        .parse_next(input)
    })
    .parse_next(i)
}

// [-]D - unverified
/// Parses a year with +/- and will eventually support 6 digit year
// [+/-]YYYY
pub(crate) fn date_year<'i, Input>(i: &mut Input) -> PResult<i32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_year", move |input: &mut Input| {
        // The sign is optional, but defaults to `+`
        let sign = opt(sign).parse_next(input)?.unwrap_or(1);

        let year = digit_4(input)? as i32;

        if year >= 100 && year < 10000 {
            Ok(sign * year)
        } else {
            Err(ErrMode::Backtrack(ContextError::new()))
        }
    })
    .parse_next(i)
}

// YYYY-MM-DD
pub(crate) fn date_ymd<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_ymd", move |input: &mut Input| {
        date_ymd_seq!(Date::YMD {
            year: date_year,                                                     // YYYY
            month: preceded(date_sep, date_month),                               // MM
            day: opt(preceded(opt(date_sep), date_day)).map(|d| d.unwrap_or(1)), //DD
        })
        .parse_next(input)
    })
    .parse_next(i)
}

// ordinal DDD
pub(crate) fn date_day_of_year<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_day_of_year", move |input: &mut Input| {
        take_digits_in_range(input, 3, 1..=366)
    })
    .parse_next(i)
}

// YYYY-DDD
pub(crate) fn date_yddd<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_yddd", move |input: &mut Input| {
        separated_pair(date_year, opt(literal("-")), date_day_of_year)
            .map(|(year, day)| Date::Ordinal { year, day })
            .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a date string specificed as YYYYMMDD
pub(crate) fn date_ymd_numeric<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_ymd_numeric", move |input: &mut Input| {
        seq!(Date::YMD {
            year: date_year,   // YYYY
            month: date_month, // MM
            day: date_day,     //DD
        })
        .parse_next(input)
    })
    .parse_next(i)
}

#[cfg(test)]
mod parsers {
    use crate::date::{date, date_yddd, date_year, date_ywd, day_of_week};
    use winnow::stream::AsBStr;
    use winnow::Parser;
    use winnow_datetime::parser::{date_day, date_month};
    use winnow_datetime::Stream;

    #[test]
    fn test_date_year() {
        assert_eq!(date_year(&mut "2015".as_bstr()).unwrap(), 2015);
        assert_eq!(date_year(&mut "+2015".as_bstr()).unwrap(), 2015);
        assert!(date_year(&mut "-333".as_bstr()).is_err());
        assert_eq!(date_year(&mut "-0333".as_bstr()).unwrap(), -333);
        assert_eq!(date_year(&mut "2015-".as_bstr()).unwrap(), 2015);
        assert!(date_year(&mut Stream::new(b"abcd")).is_err());
        assert!(date_year(&mut Stream::new(b"2a03")).is_err());
    }

    #[test]
    fn test_date_month() {
        assert_eq!(date_month(&mut "01".as_bstr()).unwrap(), 1);
        assert_eq!(date_month(&mut "06".as_bstr()).unwrap(), 6);
        assert_eq!(date_month(&mut "12".as_bstr()).unwrap(), 12);
        assert_eq!(date_month(&mut "12-".as_bstr()).unwrap(), 12);

        assert!(date_month(&mut Stream::new(b"13\n")).is_err());
        assert!(date_month(&mut Stream::new(b"00\n")).is_err());
    }

    #[test]
    fn test_date_day() {
        assert_eq!(date_day(&mut "01".as_bstr()).unwrap(), 1);
        assert_eq!(date_day(&mut "12".as_bstr()).unwrap(), 12);
        assert_eq!(date_day(&mut "20".as_bstr()).unwrap(), 20);
        assert_eq!(date_day(&mut "28".as_bstr()).unwrap(), 28);
        assert_eq!(date_day(&mut "30".as_bstr()).unwrap(), 30);
        assert_eq!(date_day(&mut "31".as_bstr()).unwrap(), 31);
        assert_eq!(date_day(&mut "31-".as_bstr()).unwrap(), 31);

        assert!(date_day(&mut Stream::new(b"00")).is_err());
        assert!(date_day(&mut Stream::new(b"32")).is_err());
    }

    #[test]
    fn test_date() {
        assert!(date(&mut Stream::new(b"201")).is_err());
        assert!(date(&mut Stream::new(b"2015p00p00")).is_err());
        assert!(date(&mut Stream::new(b"pppp")).is_err());
    }

    #[test]
    fn test_date_iso_week_date() {
        assert!(date_ywd
            .parse_next(&mut Stream::new(b"2015-W06-8"))
            .is_err());
        assert!(date_ywd.parse_next(&mut Stream::new(b"2015-W068")).is_err());
        assert!(date_ywd
            .parse_next(&mut Stream::new(b"2015-W06-0"))
            .is_err());
        assert!(date_ywd
            .parse_next(&mut Stream::new(b"2015-W00-2"))
            .is_err());
        assert!(date_ywd
            .parse_next(&mut Stream::new(b"2015-W54-2"))
            .is_err());
        assert!(date_ywd.parse_next(&mut Stream::new(b"2015-W542")).is_err());
    }

    #[test]
    fn test_date_ordinal_date() {
        // not valid here either
        assert!(date_yddd(&mut Stream::new(b"2015-400")).is_err());
    }

    #[test]
    fn test_day_of_week() {
        assert_eq!(day_of_week(&mut "1".as_bstr()).unwrap(), 1);
        assert_eq!(day_of_week(&mut "7".as_bstr()).unwrap(), 7);
        assert!(day_of_week(&mut "8".as_bstr()).is_err()); // Invalid day
    }

    #[test]
    fn disallows_notallowed() {
        assert!(date(&mut Stream::new(b"0000-20-40")).is_err());
    }
}
