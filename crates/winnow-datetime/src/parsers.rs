//! The low-level parsers for date, datetime, duration and time.
//!
//! Using the low-level functions provided here allows to recover leftover input
//! or to combine these parsers with other parser combinators.

use crate::Date;
use core::str;
use std::ops::RangeBounds;
use winnow::ascii::digit1;
use winnow::combinator::{alt, opt, trace};
use winnow::error::{ContextError, ErrMode};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::{literal, take_while};
use winnow::{seq, PResult, Parser};

// UTILITY

pub fn take_digits<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("take_digits", move |input: &mut Input| {
        let digits = take_while(1.., |c: <Input as InputStream>::Token| c.is_dec_digit())
            .parse_next(input)?;

        if digits.as_bstr().is_empty() {
            return Err(ErrMode::Backtrack(ContextError::new()));
        }

        let s = str::from_utf8(digits.as_bstr()).expect("Invalid data, expected UTF-8 string");
        let res = s
            .parse()
            .expect("Invalid string, expected ASCII representation of a number");

        Ok(res)
    })
    .parse_next(i)
}

pub fn take_digits_in_range<'i, Input>(
    i: &mut Input,
    places: usize,
    range: impl RangeBounds<u32>,
) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    let n = take_while(places, |c: <Input as InputStream>::Token| {
        c.as_char().is_digit(10)
    })
    .parse_next(i)?;

    let s = str::from_utf8(n.as_bstr()).expect("Invalid data, expected UTF-8 string");

    let number: u32 = s
        .parse()
        .expect("Invalid string, expected ASCII representation of a number");

    if range.contains(&number) {
        Ok(number)
    } else {
        return Err(ErrMode::Backtrack(ContextError::new()));
    }
}

pub fn sign<'i, Input>(i: &mut Input) -> PResult<i32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    alt((literal("-"), literal("+")))
        .map(|s: <Input as InputStream>::Slice| match s.as_bstr() {
            b"-" => -1,
            _ => 1,
        })
        .parse_next(i)
}

// DATE

// WW
pub fn date_week<'i, Input>(i: &mut Input) -> PResult<u32>
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

pub fn unverified_date_week_day<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("unverified_date_week_day", move |input: &mut Input| {
        take_digits_in_range(input, 1, 0..=9)
    })
        .parse_next(i)
}

pub fn date_week_day<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_week_day", move |input: &mut Input| {
        unverified_date_week_day(input)
    })
        .verify(|d| *d > 0 && *d <= 7)
        .parse_next(i)
}

// YYYY-"W"WW-D
pub fn date_iso_week<'i, Input>(i: &mut Input) -> PResult<crate::Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("", move |input: &mut Input| {
        seq!((
            date_year,                               // y
            seq!((opt(literal("-")), literal("W"))), // [-]W
            date_week,                               // w
            opt(unverified_date_iso_week_day)
        ))
            .verify(|(_, _, _, d)| d.is_none() || d.unwrap() > 0 && d.unwrap() <= 7)
            .map(|(year, _, ww, d)| Date::Week { year, ww, d: d.unwrap_or(0) })
            .parse_next(input)
    })
        .parse_next(i)
}

// [-]D - unverified
pub fn unverified_date_iso_week_day<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
   trace("", move |input: &mut Input| {
        seq!((
            _: opt(literal("-")),                       // [-]
            unverified_date_week_day,                           // d
        )).map(|d| d.0)
        .parse_next(input)
    })
    .parse_next(i)
}

// [-]D
pub fn date_iso_week_day<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
   trace("", move |input: &mut Input| {
        seq!((
            _: opt(literal("-")),                       // [-]
            date_week_day,                           // d
        )).map(|d| d.0)
        .verify(|d| *d > 0 && *d <= 7)
        .parse_next(input)
    })
        .parse_next(i)
}

// [+/-]YYYY
pub fn date_year<'i, Input>(i: &mut Input) -> PResult<i32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_year", move |input: &mut Input| {
        // The sign is optional, but defaults to `+`
        let sign = opt(sign).parse_next(input)?.unwrap_or(1);

        let y = take_while(4, |c: <Input as InputStream>::Token| {
            c.as_char().is_digit(10)
        })
        .parse_next(input)?;
        let year: i32 = str::from_utf8(y.as_bstr()).unwrap().parse().unwrap();

        if year >= 100 && year < 10000 {
            Ok(sign * year)
        } else {
            Err(ErrMode::Backtrack(ContextError::new()))
        }
    })
    .parse_next(i)
}

// MM
pub fn date_month<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_month", move |input: &mut Input| {
        take_digits_in_range(input, 2, 1..=12)
    })
    .parse_next(i)
}

// DD
pub fn date_day<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_day", move |input: &mut Input| {
        take_digits_in_range(input, 2, 1..=31)
    })
    .parse_next(i)
}

// YYYY-MM-DD
pub fn date_ymd<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_ymd", move |input: &mut Input| {
        seq!(Date::YMD {
            year: date_year,      // YYYY
            _: opt(literal("-")), // -
            month: date_month,     // MM
            _: opt(literal("-")), // -
            day: opt(date_day).map(|d| d.unwrap_or(1)),       //DD
        })
        .parse_next(input)
    })
    .parse_next(i)
}

// TIME

// HH
pub fn time_hour<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("time_hour", move |input: &mut Input| {
        take_digits_in_range(input, 2, 0..=23)
    })
    .parse_next(i)
}

// MM
pub fn time_minute<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("time_minute", move |input: &mut Input| {
        take_digits_in_range(input, 2, 0..=59)
    })
    .parse_next(i)
}

// SS
pub fn time_second<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("time_second", move |input: &mut Input| {
        take_digits_in_range(input, 2, 0..=60)
    })
    .parse_next(i)
}

// Converts the fractional part if-any of a number of seconds to milliseconds
// truncating towards zero if there are more than three digits.
// e.g. "" -> 0, "1" -> 100, "12" -> 120, "123" -> 123, "1234" -> 123
pub fn fraction_millisecond<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("fraction_millisecond", move |input: &mut Input| {
        let d = digit1(input)?;
        let mut digits = d.as_bstr();

        let mut l = digits.len();
        if l > 3 {
            digits = digits.get(0..3).unwrap();
        }
        let mut result = 0;
        if l > 0 {
            let digits = str::from_utf8(digits).unwrap(); // This can't panic, `digits` will only include digits.
            result = digits.parse().unwrap();
        }
        while l < 3 {
            result *= 10;
            l += 1;
        }
        Ok(result)
    })
    .parse_next(i)
}
