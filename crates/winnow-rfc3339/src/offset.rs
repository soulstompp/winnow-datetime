use alloc::string::String;
use winnow::combinator::{alt, trace};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, PResult, Parser};
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
pub fn parse_offset(mut i: &str) -> Result<Option<Offset>, String> {
    if let Ok(parsed) = offset(&mut i) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse datetime: {}", i))
    }
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
