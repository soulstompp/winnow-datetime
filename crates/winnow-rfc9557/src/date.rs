use winnow::combinator::trace;
use winnow::combinator::{eof, preceded, terminated};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, Parser};
use winnow_datetime::parser::date_month;
use winnow_datetime::parser::{date_day, digit_4};
use winnow_datetime::{date_ymd_seq, Date};

/// Parses a date string.
///
/// ## Example
///
/// ```rust
/// use winnow::Result;
/// let date = winnow_rfc9557::parse_date("2015-11-02").unwrap();
/// ```
pub fn parse_date(mut i: &str) -> Result<Date, InputError<&str>> {
    terminated(date, eof).parse_next(&mut i)
}

/// Parses a date
///
/// A string can have one of the following formats:
///
/// * `2015-11-02` or `20151102`
/// * `2015-W45-01` or `2015W451`
/// * `2015-306` or `2015306`
///
pub fn date<'i, Input, Error>(input: &mut Input) -> Result<Date, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("parse_date", move |input: &mut Input| date_ymd(input)).parse_next(input)
}

/// Date separator -
pub fn date_sep<'a, Input, Error>(input: &mut Input) -> Result<char, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar,
    Error: ParserError<Input>,
{
    trace("date_sep", move |input: &mut Input| {
        literal("-").parse_next(input).map(|_| '-')
    })
    .parse_next(input)
}

/// Parse 4 digit year with no sign withing range 0000-9999
// YYYY&DataType::String
pub fn date_year<Input, Error>(input: &mut Input) -> Result<i32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("date_year", move |input: &mut Input| match digit_4(input) {
        Ok(d) => Ok(d as i32),
        Err(e) => Err(e),
    })
    .parse_next(input)
}

/// Parses a date string in the format `YYYY-MM-DD`.
// YYYY-MM-DD
pub fn date_ymd<'a, Input, Error>(input: &mut Input) -> Result<Date, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("date_ymd", move |input: &mut Input| {
        date_ymd_seq!(Date::YMD {
            year: date_year,                       // YYYY
            month: preceded(date_sep, date_month), // MM
            day: preceded(date_sep, date_day),     //DD
        })
        .parse_next(input)
    })
    .parse_next(input)
}

#[cfg(test)]
mod parsers {
    use crate::date::date;
    use winnow::error::InputError;

    use winnow_datetime::parser::{date_day, date_month};
    use winnow_datetime::PartialInput;

    //TODO: should be in winnow_datetime
    #[test]
    fn test_date_month() {
        assert_eq!(date_month::<_, InputError<_>>(&mut "01").unwrap(), 1);
        assert_eq!(date_month::<_, InputError<_>>(&mut "06").unwrap(), 6);
        assert_eq!(date_month::<_, InputError<_>>(&mut "12").unwrap(), 12);
        assert_eq!(date_month::<_, InputError<_>>(&mut "12-").unwrap(), 12);

        assert!(date_month::<_, InputError<_>>(&mut PartialInput::new(b"13\n")).is_err());
        assert!(date_month::<_, InputError<_>>(&mut PartialInput::new(b"00\n")).is_err());
    }

    //TODO: should be in winnow_datetime
    #[test]
    fn test_date_day() {
        assert_eq!(date_day::<_, InputError<_>>(&mut "01").unwrap(), 1);
        assert_eq!(date_day::<_, InputError<_>>(&mut "12").unwrap(), 12);
        assert_eq!(date_day::<_, InputError<_>>(&mut "20").unwrap(), 20);
        assert_eq!(date_day::<_, InputError<_>>(&mut "28").unwrap(), 28);
        assert_eq!(date_day::<_, InputError<_>>(&mut "30").unwrap(), 30);
        assert_eq!(date_day::<_, InputError<_>>(&mut "31").unwrap(), 31);
        assert_eq!(date_day::<_, InputError<_>>(&mut "31-").unwrap(), 31);

        assert!(date_day::<_, InputError<_>>(&mut PartialInput::new(b"00")).is_err());
        assert!(date_day::<_, InputError<_>>(&mut PartialInput::new(b"32")).is_err());
    }

    #[test]
    fn test_date() {
        assert!(date::<_, InputError<_>>(&mut PartialInput::new(b"201")).is_err());
        assert!(date::<_, InputError<_>>(&mut PartialInput::new(b"2015p00p00")).is_err());
        assert!(date::<_, InputError<_>>(&mut PartialInput::new(b"pppp")).is_err());
    }

    #[test]
    fn disallows_notallowed() {
        assert!(date::<_, InputError<_>>(&mut PartialInput::new(b"0000-20-40")).is_err());
    }
}
