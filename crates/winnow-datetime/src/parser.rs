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

/// Converts the fractional part if-any of a number of seconds to nanoseconds, truncating
/// towards zero if there are more than nine digits.
///
/// e.g. "1" -> 100_000_000, "12" -> 120_000_000, "123456" -> 123_456_000,
/// "123456789" -> 123_456_789, "1234567891" -> 123_456_789
///
/// Truncating rather than rounding keeps the parsed instant at or before the one written.
pub fn fraction_nanosecond<Input, Error>(input: &mut Input) -> Result<u32, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar,

    Error: ParserError<Input>,
{
    trace("fraction_nanosecond", move |input: &mut Input| {
        let d = digit1(input)?;
        let mut digits = d.as_bstr();

        if digits.len() > 9 {
            digits = digits.get(0..9).unwrap();
        }

        // `digit1` guarantees at least one digit, and nine digits is at most 999_999_999,
        // which is inside u32.
        let mut result: u32 = str::from_utf8(digits).unwrap().parse().unwrap();

        let mut l = digits.len();
        while l < 9 {
            result *= 10;
            l += 1;
        }

        Ok(result)
    })
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::fraction_nanosecond;
    use winnow::error::InputError;
    use winnow::Parser;

    fn frac(s: &str) -> u32 {
        fraction_nanosecond::<_, InputError<_>>
            .parse(s)
            .unwrap_or_else(|e| panic!("{s:?} did not parse: {e}"))
    }

    /// `.5` is half a second, not five nanoseconds.
    #[test]
    fn a_fraction_is_padded_to_nine_digits() {
        assert_eq!(frac("5"), 500_000_000);
        assert_eq!(frac("05"), 50_000_000);
        assert_eq!(frac("123"), 123_000_000);
        assert_eq!(frac("000000001"), 1);
    }

    /// Digits past the third were once discarded; e.g. MySQL slow logs write six.
    #[test]
    fn digits_past_the_third_survive() {
        assert_eq!(frac("015898"), 15_898_000);
        assert_eq!(frac("4321"), 432_100_000);
        assert_eq!(frac("870479"), 870_479_000);
        assert_eq!(frac("123456789"), 123_456_789);
    }

    /// Digits past the ninth are truncated towards zero, not rounded.
    #[test]
    fn past_nine_digits_is_truncated_not_rounded() {
        assert_eq!(frac("1234567891"), 123_456_789);
        assert_eq!(frac("9999999999"), 999_999_999, "truncated, not rounded up");
        assert_eq!(frac("0000000009"), 0, "below the resolution, so zero");
    }
}
