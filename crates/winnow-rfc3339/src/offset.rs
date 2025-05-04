use winnow::combinator::{alt, eof, terminated, trace};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, Parser, Result};
use winnow_datetime::parser::time_minute;
use winnow_datetime::parser::{sign, time_hour};
use winnow_datetime::Offset;

/// Parses a offset offset string.
///
/// A offset offset string is a combination of the valid formats specifying a time's UTC offset
///
/// This will accept (Z|+...|-...) as offsets
///
/// ## Example
///
/// ```rust
/// let dt = winnow_rfc3339::parse_offset("Z").unwrap();
/// ```
pub fn parse_offset(mut i: &str) -> Result<Option<Offset>, InputError<&str>> {
    terminated(offset, eof).parse_next(&mut i)
}

/// Parses an offset string.
///
/// See [`offset()`][`mod@crate::offset`] for the supported formats.
// (Z|+...|-...)
pub fn offset<'a, Input, Error>(input: &mut Input) -> Result<Option<Offset>, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("offset_hour", move |input: &mut Input| {
        alt((offset_hour, offset_zulu)).parse_next(input)
    })
    .parse_next(input)
}

// Z|z
fn offset_zulu<'a, Input, Error>(input: &mut Input) -> Result<Option<Offset>, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("offset_zulu", move |input: &mut Input| {
        alt((literal("Z"), literal("z")))
            .map(|_| Some(Offset::default()))
            .parse_next(input)
    })
    .parse_next(input)
}

// (+...|-...)
pub fn offset_hour<'a, Input, Error>(
    input: &mut Input,
) -> std::result::Result<Option<Offset>, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("offset_hour", move |input: &mut Input| {
        let s: i32 = sign.parse_next(input)?;

        seq!((
            time_hour,
            _: literal(":"),
            time_minute
        ))
        .map(|(h, m)| {
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
    .parse_next(input)
}
