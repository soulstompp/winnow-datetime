use winnow::combinator::separated_pair;
use winnow::combinator::terminated;
use winnow::combinator::{alt, eof, opt, preceded, trace};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, Parser, Result};
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
pub fn parse_date(mut i: &str) -> Result<Date, InputError<&str>> {
    terminated(date, eof).parse_next(&mut i)
}

/// Parses a date
///
/// The date can have one of the following formats:
///
/// * `2015-11-02` or `20151102`
/// * `2015-W45-01` or `2015W451`
/// * `2015-306` or `2015306`
///
/// See [`date()`][`mod@crate::date`] for the supported formats.
pub fn date<'i, Input, Error>(input: &mut Input) -> Result<Date, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("date", move |input: &mut Input| {
        alt((date_ywd, date_ymd_numeric, date_yddd, date_ymd)).parse_next(input)
    })
    .parse_next(input)
}

/// Date separator -
pub fn date_sep<'a, Input, Error>(input: &mut Input) -> Result<char, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("date_sep", move |input: &mut Input| {
        literal("-").parse_next(input).map(|_| '-')
    })
    .parse_next(input)
}

/// Parses 2 digit week of the year within range 01-52
// WW
fn date_week<'i, Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("date_week", move |input: &mut Input| {
        let _ = preceded(opt(literal("-")), literal("W")).parse_next(input)?; // [-]Ww

        week_of_year(input)
    })
    .parse_next(input)
}

/// Parses 2 digit week of the year within range 01-52
// WW
pub(crate) fn week_of_year<'i, Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("week_of_year", move |input: &mut Input| {
        take_digits_in_range(input, 2, 1..=52)
    })
    .parse_next(input)
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
pub(crate) fn day_of_week<'i, Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("day_of_week", move |input: &mut Input| {
        take_digits_in_range(input, 1, 1..=7)
    })
    .parse_next(input)
}

/// Parses 2 digit week of the year within range 01-52
/// Parses a date string as ISO 8601 week date.
// YYYY-"W"WW-D
pub(crate) fn date_ywd<'i, Input, Error>(input: &mut Input) -> Result<Date, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
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
    .parse_next(input)
}

// [-]D - unverified
/// Parses a year with +/- and will eventually support 6 digit year
// [+/-]YYYY
pub(crate) fn date_year<'i, Input, Error>(input: &mut Input) -> Result<i32, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("date_year", move |input: &mut Input| {
        // The sign is optional, but defaults to `+`
        let sign = opt(sign).parse_next(input)?.unwrap_or(1i32);

        let year = digit_4(input)?;

        if year >= 100 && year < 10000 {
            Ok(sign * year as i32)
        } else {
            Err(ParserError::from_input(input))
        }
    })
    .parse_next(input)
}

pub(crate) fn date_ymd<'i, Input, Error>(input: &mut Input) -> Result<Date, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
    // YYYY-MM-DD
{
    trace("date_ymd", move |input: &mut Input| {
        date_ymd_seq!(Date::YMD {
            year: date_year,                                                     // YYYY
            month: preceded(date_sep, date_month),                               // MM
            day: opt(preceded(opt(date_sep), date_day)).map(|d| d.unwrap_or(1)), //DD
        })
        .parse_next(input)
    })
    .parse_next(input)
}

// ordinal DDD
pub(crate) fn date_day_of_year<'i, Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("date_day_of_year", move |input: &mut Input| {
        take_digits_in_range(input, 3, 1..=366)
    })
    .parse_next(input)
}

// YYYY-DDD
pub(crate) fn date_yddd<'i, Input, Error>(input: &mut Input) -> Result<Date, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("date_yddd", move |input: &mut Input| {
        separated_pair(date_year, opt(literal("-")), date_day_of_year)
            .map(|(year, day)| Date::Ordinal { year, day })
            .parse_next(input)
    })
    .parse_next(input)
}

/// Parses a date string specificed as YYYYMMDD
pub(crate) fn date_ymd_numeric<'i, Input, Error>(input: &mut Input) -> Result<Date, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("date_ymd_numeric", move |input: &mut Input| {
        seq!(Date::YMD {
            year: date_year,   // YYYY
            month: date_month, // MM
            day: date_day,     //DD
        })
        .parse_next(input)
    })
    .parse_next(input)
}

#[cfg(test)]
mod parsers {
    use crate::date::{date, date_yddd, date_year, date_ywd, day_of_week};
    use winnow::error::InputError;

    use winnow::Parser;
    use winnow_datetime::parser::{date_day, date_month};
    use winnow_datetime::PartialInput;

    #[test]
    fn test_date_year() {
        assert_eq!(date_year::<_, InputError<_>>(&mut "2015").unwrap(), 2015);
        assert_eq!(date_year::<_, InputError<_>>(&mut "+2015").unwrap(), 2015);
        assert!(date_year::<_, InputError<_>>(&mut "-333").is_err());
        assert_eq!(date_year::<_, InputError<_>>(&mut "-0333").unwrap(), -333);
        assert_eq!(date_year::<_, InputError<_>>(&mut "2015-").unwrap(), 2015);
        assert!(date_year::<_, InputError<_>>(&mut PartialInput::new(b"abcd")).is_err());
        assert!(date_year::<_, InputError<_>>(&mut PartialInput::new(b"2a03")).is_err());
    }

    #[test]
    fn test_date_month() {
        assert_eq!(date_month::<_, InputError<_>>(&mut "01").unwrap(), 1);
        assert_eq!(date_month::<_, InputError<_>>(&mut "06").unwrap(), 6);
        assert_eq!(date_month::<_, InputError<_>>(&mut "12").unwrap(), 12);
        assert_eq!(date_month::<_, InputError<_>>(&mut "12-").unwrap(), 12);

        assert!(date_month::<_, InputError<_>>(&mut PartialInput::new(b"13\n")).is_err());
        assert!(date_month::<_, InputError<_>>(&mut PartialInput::new(b"00\n")).is_err());
    }

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
    fn test_date_iso_week_date() {
        assert!(date_ywd::<_, InputError<_>>
            .parse_next(&mut PartialInput::new(b"2015-W06-8"))
            .is_err());
        assert!(date_ywd::<_, InputError<_>>
            .parse_next(&mut PartialInput::new(b"2015-W068"))
            .is_err());
        assert!(date_ywd::<_, InputError<_>>
            .parse_next(&mut PartialInput::new(b"2015-W06-0"))
            .is_err());
        assert!(date_ywd::<_, InputError<_>>
            .parse_next(&mut PartialInput::new(b"2015-W00-2"))
            .is_err());
        assert!(date_ywd::<_, InputError<_>>
            .parse_next(&mut PartialInput::new(b"2015-W54-2"))
            .is_err());
        assert!(date_ywd::<_, InputError<_>>
            .parse_next(&mut PartialInput::new(b"2015-W542"))
            .is_err());
    }

    #[test]
    fn test_date_ordinal_date() {
        // not valid here either
        assert!(date_yddd::<_, InputError<_>>(&mut PartialInput::new(b"2015-400")).is_err());
    }

    #[test]
    fn test_day_of_week() {
        assert_eq!(day_of_week::<_, InputError<_>>(&mut "1").unwrap(), 1);
        assert_eq!(day_of_week::<_, InputError<_>>(&mut "7").unwrap(), 7);
        assert!(day_of_week::<_, InputError<_>>(&mut "8").is_err()); // Invalid day
    }

    #[test]
    fn disallows_notallowed() {
        assert!(date::<_, InputError<_>>(&mut PartialInput::new(b"0000-20-40")).is_err());
    }
}
