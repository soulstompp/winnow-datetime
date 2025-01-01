//! The low-level parsers for date, datetime, duration and time.
//!
//! Using the low-level functions provided here allows to recover leftover input
//! or to combine these parsers with other parser combinators.

use core::str;
use std::ops::RangeBounds;
use winnow::ascii::digit1;
use winnow::combinator::{alt, trace};
use winnow::error::{ContextError, ErrMode};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::{literal, take_while};
use winnow::{PResult, Parser};

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

#[macro_export]
macro_rules! define_date_parser {
    ($name:ident, $($parser:expr),+) => {
        pub fn $name<'i, Input>(input: &mut Input) -> PResult<Date>
        where
            Input: StreamIsPartial + InputStream + Compare<&'i str>,
            <Input as InputStream>::Slice: AsBStr,
            <Input as InputStream>::Token: AsChar + Clone,
        {
            trace(stringify!($name), |input| {
                alt(($($parser),+)).parse_next(input)
            })
            .parse_next(input)
        }
    };
}
