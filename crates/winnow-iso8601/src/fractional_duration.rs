use crate::duration::{
    duration_base_time, duration_part_day, duration_part_month, duration_part_week,
    duration_part_year,
};
use winnow::combinator::{eof, opt, preceded, terminated, trace};
use winnow::error::{InputError, ParserError};
use winnow::stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::literal;
use winnow::{seq, Parser, Result};
use winnow_datetime::FractionalDuration;

/// Parses a duration with the same formating rules but allows for decimal places.
/// let duration = winnow_iso8601::parse_fractional_duration("P1,5Y2M3DT4,5H5M6S").unwrap();
/// let duration = winnow_iso8601::parse_fractional_duration("P1,5W").unwrap();
pub fn parse_fractional_duration(mut i: &str) -> Result<FractionalDuration, InputError<&str>> {
    terminated(fractional_duration, eof).parse_next(&mut i)
}

/// Parses a duration string with the format P%dY%dM%dDT%dH%dM%dS
pub fn fractional_duration<'i, Input, Error>(
    input: &mut Input,
) -> std::result::Result<FractionalDuration, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'i str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace("fractional_duration", move |input: &mut Input| {
        seq!((
            _: literal("P"),
            opt(duration_part_year),
            opt(duration_part_month),
            opt(duration_part_week),
            opt(duration_part_day),
            opt(preceded(opt(literal("T")), duration_base_time)),
        ))
        .verify(|(y, mo, w, d, time)| {
            if y.is_none() && mo.is_none() && w.is_none() && d.is_none() && time.is_none() {
                false
            } else {
                true
            }
        })
        .map(|(y, mo, w, d, time)| {
            let time = time.unwrap_or((None, None, None));

            FractionalDuration {
                years: y.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                months: mo.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                weeks: w.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                days: d.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                hours: time.0.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                minutes: time.1.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
                seconds: time.2.map(|p| (p.whole, p.frac)).unwrap_or((0, None)),
            }
            // at least one element must be present for a valid duration representation
        })
        .parse_next(input)
    })
    .parse_next(input)
}
