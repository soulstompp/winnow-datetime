//! The low-level parsers for date, datetime, duration and time.
//!
//! Using the low-level functions provided here allows to recover leftover input
//! or to combine these parsers with other parser combinators.

use core::str;
use std::ops::RangeBounds;
use winnow::ascii::{digit1, Int, Uint};
use winnow::combinator::{alt, trace};
use winnow::error::ParserError;
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::{literal, take_while};
use winnow::Parser;
// UTILITY

/// Exactly 1 digit
pub fn digit_1<Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("digit_1", move |input: &mut Input| {
        take_exact_digits(input, 1)
    })
    .parse_next(input)
}

/// Exactly 2 digits
pub fn digit_2<Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("digit_2", move |input: &mut Input| {
        take_exact_digits(input, 2)
    })
    .parse_next(input)
}

/// Exactly 3 digits
pub fn digit_3<Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("digit_3", move |input: &mut Input| {
        take_exact_digits(input, 3)
    })
    .parse_next(input)
}

/// Exactly 4 digits
pub fn digit_4<Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("digit_4", move |input: &mut Input| {
        take_exact_digits(input, 4)
    })
    .parse_next(input)
}

/// Exactly 6 digits
pub fn digit_6<Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("digit_6", move |input: &mut Input| {
        take_exact_digits(input, 2)
    })
    .parse_next(input)
}

pub(crate) fn take_exact_digits<Input, Error>(
    input: &mut Input,
    places: usize,
) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    let n = take_while(places, AsChar::is_dec_digit).parse_next(input)?;

    let n = String::from_utf8_lossy(n.as_bstr());

    let n = u32::try_from_dec_uint(n.as_ref()).unwrap();

    Ok(n)
}
pub fn take_digits<'i, Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("take_digits", move |input: &mut Input| {
        let out = take_while(1.., AsChar::is_dec_digit).parse_next(input)?;
        let out = String::from_utf8_lossy(out.as_bstr());
        let out = u32::try_from_dec_uint(out.as_ref()).unwrap();

        Ok(out)
    })
    .parse_next(input)
}

pub fn take_digits_in_range<Input, Error>(
    input: &mut Input,
    places: usize,
    range: impl RangeBounds<u32>,
) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    let out = take_while(places, AsChar::is_dec_digit).parse_next(input)?;
    let out = String::from_utf8_lossy(out.as_bstr());
    let out = u32::try_from_dec_uint(out.as_ref()).unwrap();

    if range.contains(&out) {
        Ok(out)
    } else {
        Err(ParserError::from_input(input))
    }
}

pub fn sign<'a, Input, Error>(input: &mut Input) -> Result<i32, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("sign", move |input: &mut Input| {
        let i = alt((literal("-"), literal("+")))
            .map(|s: <Input as Stream>::Slice| match s.as_bstr() {
                b"-" => "-1",
                _ => "1",
            })
            .parse_next(input)?;

        Ok(i32::try_from_dec_int(i).unwrap())
    })
    .parse_next(input)
}

// DATE

// MM
pub fn date_month<Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("date_month", move |input: &mut Input| {
        take_digits_in_range(input, 2, 1..=12)
    })
    .parse_next(input)
}

// DD
pub fn date_day<Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("date_day", move |input: &mut Input| {
        take_digits_in_range(input, 2, 1..=31)
    })
    .parse_next(input)
}

// TIME

// HH
pub fn time_hour<Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("time_hour", move |input: &mut Input| {
        take_digits_in_range(input, 2, 0..=23)
    })
    .parse_next(input)
}

// MM
pub fn time_minute<Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("time_minute", move |input: &mut Input| {
        take_digits_in_range(input, 2, 0..=59)
    })
    .parse_next(input)
}

// SS
pub fn time_second<Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("time_second", move |input: &mut Input| {
        take_digits_in_range(input, 2, 0..=60)
    })
    .parse_next(input)
}

// Converts the fractional part if-any of a number of seconds to milliseconds
// truncating towards zero if there are more than three digits.
// e.g. "" -> 0, "1" -> 100, "12" -> 120, "123" -> 123, "1234" -> 123
pub fn fraction_millisecond<'i, Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar,

    Error: ParserError<Input>,
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
    .parse_next(input)
}
