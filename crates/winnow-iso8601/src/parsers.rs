//! The low-level parsers for date, datetime, duration and time.
//!
//! The top-level functions [`date()`][`crate::date()`], [`datetime()`][`crate::datetime()`],
//! [`duration()`][`crate::duration()`] and [`time()`][`crate::time()`]
//! provide convenient wrappers around the low-level parsers,
//! but throw away leftover input on success.
//!
//! Using the low-level functions provided here allows to recover leftover input
//! or to combine these parsers with other parser combinators.

use winnow_datetime::parsers::date_day;
use winnow_datetime::parsers::date_month;
use winnow_datetime::parsers::date_iso_week;
use winnow_datetime::{Date, DateTime, Time, Timezone};
use crate::Duration;
use core::str;
use winnow::combinator::opt;
use winnow::combinator::{alt, trace};
use winnow::combinator::{preceded, separated_pair, terminated};
use winnow::stream::{AsBStr, AsChar, Compare, Stream as InputStream, StreamIsPartial};
use winnow::token::literal;
use winnow::token::one_of;
use winnow::{seq, PResult, Parser, Partial};
use winnow_datetime::parsers::sign;
use winnow_datetime::parsers::take_digits;
use winnow_datetime::parsers::take_digits_in_range;
use winnow_datetime::parsers::{date_ymd, time_second, date_year, fraction_millisecond, time_hour, time_minute};

#[cfg(test)]
mod tests;

/// Type for holding partial data for parsers
pub type Stream<'i> = Partial<&'i [u8]>;

// DATE
// ordinal DDD
fn date_ord_day<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_ord_day", move |input: &mut Input| {
        take_digits_in_range(input, 3, 1..=366)
    })
    .parse_next(i)
}

// YYYY-DDD
fn date_ordinal<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_ordinal", move |input: &mut Input| {
        separated_pair(date_year, opt(literal("-")), date_ord_day)
            .map(|(year, ddd)| Date::Ordinal { year, ddd })
            .parse_next(input)
    })
    .parse_next(i)
}

/// Parses a date string specificed as YYYYMMDD
pub fn date_ymd_numeric<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("date_ymd_numeric", move |input: &mut Input| {
        seq!(Date::YMD {
            year: date_year,      // YYYY
            month: date_month,     // MM
            day: date_day,       //DD
        })
            .parse_next(input)
    })
        .parse_next(i)
}

/// Parses a date string.
///
/// See [`date()`][`crate::date()`] for the supported formats.
pub fn parse_date<'i, Input>(i: &mut Input) -> PResult<Date>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("parse_date", move |input: &mut Input| {
        alt((
            date_iso_week, date_ymd_numeric, date_ordinal, date_ymd
        )).parse_next(input)
    })
    .parse_next(i)
}

// TIME

/// Parses a time string with an optional preceding 'T'.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub fn parse_time<'i, Input>(i: &mut Input) -> PResult<Time>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("parse_time", move |input: &mut Input| {
        seq!((
            _: opt(literal("T")),
            base_time
        )).map(|r| r.0).parse_next(input)
    })
    .parse_next(i)
}

/// Parses a time string.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub fn base_time<'i, Input>(i: &mut Input) -> PResult<Time>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("base_time", move |input: &mut Input| {
        let hour = time_hour(input)?;

        let msms = opt(time_minute_second_millisecond).parse_next(input)?; // MM:[SS][.(m*)]

        let timezone = opt(parse_timezone).parse_next(input)?;

        if let Some((minute, second, millisecond)) = msms {
            return Ok(Time {
                hour,                                         // HH
                minute,                                       // MM
                second: second.unwrap_or(0),        // [SS]
                millisecond: millisecond.unwrap_or(0), // [.(m*)]
                timezone,           // [(Z|+...|-...)]
            });
        } else {
            let (minute, second, millisecond) = (0, 0, 0);
            return Ok(Time {
                hour,                                         // HH
                minute,                                       // MM
                second,        // [SS]
                millisecond, // [.(m*)]
                timezone,           // [(Z|+...|-...)]
            });
        }
    })
        .parse_next(i)
}

/// Parses secondary portion of a time string.
fn time_minute_second_millisecond<'i, Input>(i: &mut Input) -> PResult<(u32, Option<u32>, Option<u32>)>
    where
        Input: StreamIsPartial + InputStream + Compare<&'i str>,
        <Input as InputStream>::Slice: AsBStr,
        <Input as InputStream>::Token: AsChar + Clone,
{
    trace("time_minute_second_millisecond", move |input: &mut Input| {
        seq!((
            preceded(opt(literal(":")), time_minute),
            opt(preceded(opt(literal(":")), time_second)),
            opt(preceded(one_of(b",."), fraction_millisecond))
        ))
        .parse_next(input)
    }).parse_next(i)
}

/// Parses a timezone offset string.
///
/// See [`timezone()`][`crate::timezone()`] for the supported formats.
// (Z|+...|-...)
pub fn parse_timezone<'i, Input>(i: &mut Input) -> PResult<Timezone>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("timezone_hour", move |input: &mut Input| {
        alt((timezone_hour, timezone_utc)).parse_next(input)
    })
    .parse_next(i)
}

// Z
fn timezone_utc<'i, Input>(i: &mut Input) -> PResult<Timezone>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("timezone_utc", move |input: &mut Input| {
        literal("Z").map(|_| Timezone::default()).parse_next(input)
    })
    .parse_next(i)
}

/// Parses a datetime string.
///
/// See [`datetime()`][`crate::datetime()`] for supported formats.
// Full ISO8601 datetime
pub fn parse_datetime<'i, Input>(i: &mut Input) -> PResult<DateTime>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("parse_datetime", move |input: &mut Input| {
        separated_pair(parse_date, literal("T"), base_time)
            .map(|(d, t)| DateTime { date: d, time: t })
            .parse_next(input)
    })
    .parse_next(i)
}

// DURATION

///    dur-year          = 1*DIGIT "Y" [dur-month]
fn duration_year<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_year", move |input: &mut Input| {
        (terminated(take_digits, literal("Y"))).parse_next(input)
    })
    .parse_next(i)
}

///    dur-month         = 1*DIGIT "M" [dur-day]
fn duration_month<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_month", move |input: &mut Input| {
        (terminated(take_digits, literal("M"))).parse_next(input)
    })
    .parse_next(i)
}

///    dur-week          = 1*DIGIT "W"
fn duration_week<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_week", move |input: &mut Input| {
        let d = take_digits(input)?;
        let _ = literal("W").parse_next(input)?;

        Ok(d)
    })
    .parse_next(i)
}

//    dur-day           = 1*DIGIT "D"
fn duration_day<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_day", move |input: &mut Input| {
        terminated(take_digits, literal("D")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-hour          = 1*DIGIT "H" [dur-minute]
///    dur-time          = "T" (dur-hour / dur-minute / dur-second)
fn duration_hour<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_hour", move |input: &mut Input| {
        terminated(take_digits, literal("H")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-minute        = 1*DIGIT "M" [dur-second]
fn duration_minute<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("", move |input: &mut Input| {
        terminated(take_digits, literal("M")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-second        = 1*DIGIT "S"
fn duration_second<'i, Input>(i: &mut Input) -> PResult<u32>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_second", move |input: &mut Input| {
        terminated(take_digits, literal("S")).parse_next(input)
    })
    .parse_next(i)
}

///    dur-second-ext    = 1*DIGIT (,|.) 1*DIGIT "S"
fn duration_second_and_millisecond<'i, Input>(i: &mut Input) -> PResult<(u32, u32)>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace(
        "duration_second_and_millisecond",
        move |input: &mut Input| {
            alt((
                // no milliseconds
                duration_second.map(|m| (m, 0)),
                terminated(
                    // with milliseconds
                    separated_pair(take_digits, one_of(b",."), fraction_millisecond),
                    literal("S"),
                ),
            ))
            .parse_next(input)
        },
    )
    .parse_next(i)
}

fn duration_time<'i, Input>(i: &mut Input) -> PResult<(u32, u32, u32, u32)>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("duration_time", move |input: &mut Input| {
        seq!((
            literal("T"),
            opt(duration_hour),
            opt(duration_minute),
            opt(duration_second_and_millisecond),
        ))
        .map(|(_, h, m, s)| {
            let (s, ms) = s.unwrap_or((0, 0));

            (h.unwrap_or(0), m.unwrap_or(0), s, ms)
        })
        .parse_next(input)
    })
    .parse_next(i)
}

pub(crate) fn duration<'i, Input>(i: &mut Input) -> PResult<Duration>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("", move |input: &mut Input| {
        seq!((
            _: literal("P"),
            opt(duration_year),
            opt(duration_month),
            opt(duration_week),
            opt(duration_day),
            opt(duration_time),
        ))
        .verify(|(y, mo, w, d, time)| {
            if y.is_none() && mo.is_none() && w.is_none() && d.is_none() && time.is_none() {
                false
            } else {
                true
            }
        })
        .map(|(y, mo, w, d, time)| {
            // at least one element must be present for a valid duration representation

            let (h, mi, s, ms) = time.unwrap_or((0, 0, 0, 0));

            Duration {
                years: y.unwrap_or(0),
                months: mo.unwrap_or(0),
                weeks: w.unwrap_or(0),
                days: d.unwrap_or(0),
                hours: h,
                minutes: mi,
                seconds: s,
                milliseconds: ms,
            }
        })
        .parse_next(input)
    })
    .parse_next(i)
}

// (+...|-...)
fn timezone_hour<'i, Input>(i: &mut Input) -> PResult<Timezone>
where
    Input: StreamIsPartial + InputStream + Compare<&'i str>,
    <Input as InputStream>::Slice: AsBStr,
    <Input as InputStream>::Token: AsChar + Clone,
{
    trace("timezone_hour", move |input: &mut Input| {
        seq!((
            sign,
            time_hour,
            opt(preceded(opt(literal(":")), time_minute))
        ))
        .map(|(s, h, m)| Timezone {
            offset_hours: s * (h as i32),
            offset_minutes: s * (m.unwrap_or(0) as i32),
        })
        .parse_next(input)
    })
    .parse_next(i)
}
