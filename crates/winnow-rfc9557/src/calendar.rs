use winnow::combinator::{alt, eof, terminated, trace};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::{Parser, Result};
use winnow_datetime::types::Calendar;

/// Parses a calendar string
///
/// A calendar string which should follow
///
/// This will accept (Z|+...|-...) as offsets
///
/// ## Example
///
/// ```rust
/// let dt = winnow_rfc9557::parse_time_zone("America/Los_Angeles").unwrap();
/// ```
pub fn parse_calendar(mut i: &str) -> Result<Calendar, InputError<&str>> {
    terminated(calendar, eof).parse_next(&mut i)
}

/// Parses a time zone string.
///
/// [A-Z]+/[]A-Z]+
pub fn calendar<'a, Input, Error>(input: &mut Input) -> Result<Calendar, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("calendar", move |input: &mut Input| {
        alt((
            literal("buddhist"),
            literal("chinese"),
            literal("coptic"),
            literal("dangi"),
            literal("ethioaa"),
            literal("ethiopic"),
            literal("gregory"),
            literal("hebrew"),
            literal("indian"),
            literal("islamic-umalqura"),
            literal("islamic-tbla"),
            literal("islamic-civil"),
            literal("islamic-rgsa"),
            literal("islamic"),
            literal("iso8601"),
            literal("japanese"),
            literal("persian"),
            literal("roc"),
            literal("islamicc"),
        ))
        .map(|identifier: <Input as Stream>::Slice| Calendar {
            identifier: String::from_utf8_lossy(identifier.as_bstr()).into(),
            critical: false,
        })
        .parse_next(input)
    })
    .parse_next(input)
}
