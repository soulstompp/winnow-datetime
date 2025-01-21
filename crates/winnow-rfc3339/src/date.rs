use alloc::string::String;
use std::str;
use winnow::combinator::preceded;
use winnow::combinator::trace;
use winnow::error::{ContextError, ErrMode};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, PResult, Parser};
use winnow_datetime::parser::{date_day, digit_4};
use winnow_datetime::parser::date_month;
use winnow_datetime::{date_ymd_seq, Date};

/// Parses a date string.
///
/// A string can have one of the following formats:
///
/// * `2015-11-02` or `20151102`
/// * `2015-W45-01` or `2015W451`
/// * `2015-306` or `2015306`
///
/// ## Example
///
/// ```rust
/// let date = winnow_rfc3339::parse_date("2015-11-02").unwrap();
/// ```
pub fn parse_date(mut i: &str) -> Result<Date, String> {
    if let Ok(parsed) = date(&mut i) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse date: {}", i))
    }
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

/// Parse 4 digit year with no sign withing range 0000-9999
// YYYY
pub fn date_year<'i, Input>(i: &mut Input) -> PResult<i32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_year", move |input: &mut Input| {
        let year = digit_4(input)? as i32;

        if year >= 0 && year <= 9999 {
            Ok(year)
        } else {
            Err(ErrMode::Backtrack(ContextError::new()))
        }
    })
    .parse_next(i)
}

/// Parses a date string in the format `YYYY-MM-DD`.
// YYYY-MM-DD
pub fn date_ymd<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_ymd", move |input: &mut Input| {
        date_ymd_seq!(Date::YMD {
            year: date_year,                       // YYYY
            month: preceded(date_sep, date_month), // MM
            day: preceded(date_sep, date_day),     //DD
        })
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a date string.
///
/// See [`date()`][`crate::date()`] for the supported formats.
pub fn date<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("parse_date", move |input: &mut Input| date_ymd(input)).parse_next(i)
}

#[cfg(test)]
mod parsers {
    use crate::date::date;
    use winnow::stream::AsBStr;
    use winnow_datetime::parser::{date_day, date_month};
    use winnow_datetime::Stream;

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
    fn disallows_notallowed() {
        assert!(date(&mut Stream::new(b"0000-20-40")).is_err());
    }
}
