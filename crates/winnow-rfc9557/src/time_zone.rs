use crate::offset::offset;
use winnow::ascii::alpha1;
use winnow::combinator::repeat;
use winnow::combinator::{alt, eof, terminated, trace};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, Parser, Result};
use winnow_datetime::types::{NamedTimeZone, TimeZone};

/// Parses a time zone string.
///
/// A time zone string is a combination of the valid formats specifying a time's UTC offset
///
/// This will accept (Z|+...|-...) as offsets
///
/// ## Example
///
/// ```rust
/// let dt = winnow_rfc9557::parse_time_zone("America/Los_Angeles").unwrap();
/// ```
pub fn parse_time_zone(mut i: &str) -> Result<TimeZone, InputError<&str>> {
    terminated(time_zone, eof).parse_next(&mut i)
}

/// Parses a time zone string.
///
/// [A-Z]+/[]A-Z]+
pub fn time_zone<'a, Input, Error>(input: &mut Input) -> Result<TimeZone, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("time_zone", move |input: &mut Input| {
        alt((named_time_zone, fixed_time_zone)).parse_next(input)
    })
    .parse_next(input)
}

/// Parses offset as a fixed time zone.
///
/// [A-Z]+/[]A-Z]+
pub fn fixed_time_zone<'a, Input, Error>(input: &mut Input) -> Result<TimeZone, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("time_zone", move |input: &mut Input| {
        offset
            .map(|offset| TimeZone::Fixed{ offset })
            .parse_next(input)
    })
    .parse_next(input)
}

/// Parses a time zone string.
///
/// [A-Z]+/[]A-Z]+
pub fn named_time_zone<'a, Input, Error>(input: &mut Input) -> Result<TimeZone, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("time_zone", move |input: &mut Input| {
        seq!(
            named_time_zone_part,
            _: literal("/"),
            named_time_zone_part,
        )
        .map(|(r, l): (String, String)| {
            let mut name = String::new();
            name.push_str(&r);
            name.push('/');
            name.push_str(&l);

            TimeZone::Named{ zone: NamedTimeZone {
                identifier: name,
                critical: Default::default(),
            }}
        })
        .parse_next(input)
    })
    .parse_next(input)
}

fn named_time_zone_part<'a, Input, Error>(input: &mut Input) -> Result<String, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("time_zone_part", move |input: &mut Input| {
        seq!(alpha1, repeat(0.., seq!(literal("_"), alpha1,)))
            .map(
                |(lhs, rhs): (<Input as Stream>::Slice, Vec<(_, <Input as Stream>::Slice)>)| {
                    let mut name = String::new();
                    name.push_str(&String::from_utf8_lossy(lhs.as_bstr()));

                    for part in rhs {
                        name.push('_');
                        name.push_str(&String::from_utf8_lossy(part.1.as_bstr()));
                    }

                    name
                },
            )
            .parse_next(input)
    })
    .parse_next(input)
}
