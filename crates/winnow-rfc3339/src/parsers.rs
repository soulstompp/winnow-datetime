//! The low-level parsers for date, datetime, duration and time.
//!
//! The top-level functions [`date()`][`crate::date()`], [`datetime()`][`crate::datetime()`],
//! [`duration()`][`crate::duration()`] and [`time()`][`crate::time()`]
//! provide convenient wrappers around the low-level parsers,
//! but throw away leftover input on success.
//!
//! Using the low-level functions provided here allows to recover leftover input
//! or to combine these parsers with other parser combinators.

use core::str;
use winnow::combinator::opt;
use winnow::combinator::{alt, trace};
use winnow::combinator::{preceded, separated_pair};
use winnow::error::{ContextError, ErrMode};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::one_of;
use winnow::token::{literal, take_while};
use winnow::{seq, PResult, Parser, Partial};
use winnow_datetime::parser::date_day;
use winnow_datetime::parser::date_month;
use winnow_datetime::parser::sign;
use winnow_datetime::parser::{fraction_millisecond, time_hour, time_minute, time_second};
use winnow_datetime::{date_ymd_seq, time_seq, Date, DateTime, Offset, Time};

#[cfg(test)]
mod tests;

/// Type for holding partial data for parsers
pub type Stream<'i> = Partial<&'i [u8]>;

// DATE

// DATE

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
        let y = take_while(4, |c: <Input as InputStream>::Token| {
            c.as_char().is_digit(10)
        })
        .parse_next(input)?;

        let year: i32 = str::from_utf8(y.as_bstr()).unwrap().parse().unwrap();

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

// TIME

/// Parses a time string with an optional preceding 'T'.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub fn time<'i, Input>(i: &mut Input) -> PResult<Time>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("parse_time", move |input: &mut Input| {
        time_seq!(Time {
            hour: time_hour,                             // HH
            minute: preceded(literal(":"), time_minute), // MM
            second: preceded(literal(":"), time_second), // [SS]
            millisecond: opt(preceded(one_of(b",."), fraction_millisecond)).map(|d| d.unwrap_or(0)), // [.(m*)]
            offset: offset, // [(Z|+...|-...)]
        })
        .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a offset offset string.
///
/// See [`offset()`][`crate::offset()`] for the supported formats.
// (Z|+...|-...)
pub fn offset<'i, Input>(i: &mut Input) -> PResult<Option<Offset>>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("offset_hour", move |input: &mut Input| {
        alt((offset_hour, offset_zulu)).parse_next(input)
    })
    .parse_next(i)
}

// Z|z
fn offset_zulu<'i, Input>(i: &mut Input) -> PResult<Option<Offset>>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("offset_zulu", move |input: &mut Input| {
        alt((literal("Z"), literal("z")))
            .map(|_| Some(Offset::default()))
            .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a datetime string.
///
/// See [`datetime()`][`crate::datetime()`] for supported formats.
// Full ISO8601 datetime
pub fn parse_datetime<'i, Input>(i: &mut Input) -> PResult<DateTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("parse_datetime", move |input: &mut Input| {
        separated_pair(date, alt((literal("T"), literal("t"))), time)
            .map(|(d, t)| DateTime { date: d, time: t })
            .parse_next(input)
    })
    .parse_next(i)
}

// (+...|-...)
fn offset_hour<'i, Input>(i: &mut Input) -> PResult<Option<Offset>>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("offset_hour", move |input: &mut Input| {
        seq!((
            sign,
            time_hour,
            _: literal(":"),
            time_minute
        ))
        .map(|(s, h, m)| {
            if s == -1 && h == 0 && m == 0 {
                None
            } else {
                Some(Offset {
                    offset_hours: s * (h as i32),
                    offset_minutes: s * (m as i32),
                })
            }
        })
        .parse_next(input)
    })
    .parse_next(i)
}
