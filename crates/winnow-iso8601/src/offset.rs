use winnow::combinator::{alt, eof, opt, preceded, terminated, trace};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, Parser, Result};
use winnow_datetime::parser::{sign, time_hour, time_minute};
use winnow_datetime::Offset;

/// Parses an offset string.
///
/// ## Example
///
/// ```rust
/// let dt = winnow_iso8601::parse_offset("Z").unwrap();
/// ```
pub fn parse_offset(mut i: &str) -> Result<Offset, InputError<&str>> {
    terminated(offset, eof).parse_next(&mut i)
}

// (+...|-...)
/// Parses a offset offset string.
///
/// An offset string is a combination of the valid formats specifying a time's UTC offset
///
/// This will accept (Z|+...|-...) as offsets
///
// (Z|+...|-...)
pub fn offset<'i, Input, Error>(input: &mut Input) -> Result<Offset, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("offset", move |input: &mut Input| {
        alt((offset_hour, offset_zulu)).parse_next(input)
    })
    .parse_next(input)
}

// Z
pub fn offset_zulu<'i, Input, Error>(input: &mut Input) -> Result<Offset, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("offset_zulu", move |input: &mut Input| {
        literal("Z")
            .map(|_| Offset::Fixed {
                hours: 0,
                minutes: 0,
                critical: false,
            })
            .parse_next(input)
    })
    .parse_next(input)
}

pub fn offset_hour<'i, Input, Error>(input: &mut Input) -> Result<Offset, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("offset_hour", move |input: &mut Input| {
        seq!((
            sign,
            time_hour,
            opt(preceded(opt(literal(":")), time_minute))
        ))
        .verify(|(s, h, m)| !(*s == -1 && h * 1 == 0 && (m.is_none() || m.unwrap() * 1 == 0)))
        .map(|(s, h, m)| Offset::Fixed {
            hours: s * (h as i32),
            minutes: s * (m.unwrap_or(0) as i32),
            critical: false,
        })
        .parse_next(input)
    })
    .parse_next(input)
}
