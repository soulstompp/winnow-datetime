use winnow::combinator::{alt, eof, terminated, trace};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::{Parser, Result};
use winnow_datetime::types::Calendar;

/// Parses a calendar identifier, such as `gregory`.
///
/// ## Example
///
/// ```rust
/// let calendar = winnow_rfc9557::calendar::parse_calendar("gregory").unwrap();
/// assert_eq!(calendar.identifier, "gregory");
/// ```
pub fn parse_calendar(mut i: &str) -> Result<Calendar, InputError<&str>> {
    terminated(calendar, eof).parse_next(&mut i)
}

/// Parses one of the calendar identifiers listed in RFC 9557, such as `gregory` or `islamic-civil`.
pub fn calendar<'a, Input, Error>(input: &mut Input) -> Result<Calendar, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'a str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("calendar", move |input: &mut Input| {
        // `alt` takes the first match, so an identifier must come before any that is a prefix
        // of it (`islamicc` before `islamic`). Grouped because winnow 1.0 limits `alt` to 9
        // alternatives.
        alt((
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
            )),
            alt((
                literal("islamic-umalqura"),
                literal("islamic-tbla"),
                literal("islamic-civil"),
                literal("islamic-rgsa"),
                literal("islamicc"),
                literal("islamic"),
            )),
            alt((
                literal("iso8601"),
                literal("japanese"),
                literal("persian"),
                literal("roc"),
            )),
        ))
        .map(|identifier: <Input as Stream>::Slice| Calendar {
            identifier: String::from_utf8_lossy(identifier.as_bstr()).into(),
            critical: false,
        })
        .parse_next(input)
    })
    .parse_next(input)
}
